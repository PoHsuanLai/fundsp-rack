//! Distortion effects (distortion, bitcrusher, krush)

use super::super::registry::{EffectBuilder, EffectControls, EffectMetadata};
use fundsp::hacker32::*;
use numeric_array::typenum::U1;
use std::collections::HashMap;
use std::sync::Arc;

/// Distortion effect
pub struct DistortionBuilder;

impl EffectBuilder for DistortionBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let amount = params.get("amount").copied().unwrap_or(0.5);

        // Soft clipping distortion using tanh
        let drive = 1.0 + amount * 10.0;
        let left = (pass() * drive) >> shape(Tanh(1.0));
        let right = (pass() * drive) >> shape(Tanh(1.0));

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("distortion", "Distortion effect")
            .with_param("amount", 0.5, 0.0, 1.0)
    }
}

/// Bitcrusher effect
pub struct BitcrusherBuilder;

impl EffectBuilder for BitcrusherBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let bits = params.get("bits").copied().unwrap_or(8.0);

        // Reduce bit depth - use map with Frame
        let levels = 2.0_f32.powf(bits);
        let left = pass() >> map(move |x: &Frame<f32, U1>| (x[0] * levels).round() / levels);
        let right = pass() >> map(move |x: &Frame<f32, U1>| (x[0] * levels).round() / levels);

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("bitcrusher", "Bitcrusher (reduces bit depth)")
            .with_param("bits", 8.0, 1.0, 16.0)
    }
}

/// Pan effect
pub struct PanBuilder;

impl EffectBuilder for PanBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let pan_val = params.get("pan").copied().unwrap_or(0.0); // -1.0 = left, 0.0 = center, 1.0 = right

        let pan_shared = shared(pan_val);
        let mut controls = EffectControls::new();
        controls
            .params
            .insert("pan".to_string(), pan_shared.clone());

        // Use FunDSP's built-in pan function
        let stereo_pan = fundsp::hacker32::pan(pan_val);

        (Box::new(stereo_pan), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("pan", "Pan (stereo positioning)")
            .with_param("pan", 0.0, -1.0, 1.0)
    }
}

/// Krush - Bit reduction and sample rate reduction
pub struct KrushBuilder;

impl EffectBuilder for KrushBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let bits = params.get("bits").copied().unwrap_or(8.0);
        let sample_rate = params.get("sample_rate").copied().unwrap_or(8000.0);
        let mix = params.get("mix").copied().unwrap_or(1.0);

        let levels = 2.0_f32.powf(bits);
        let dry = 1.0 - mix;

        // Sample rate reduction followed by bit reduction
        // hold_hz(frequency, variability) - variability controls randomness (0.0 = none)
        // We use 0.0 for consistent sample-and-hold behavior
        let crush_left = pass()
            >> hold_hz(sample_rate, 0.0)  // Sample rate reduction
            >> map(move |x: &Frame<f32, U1>| {  // Bit reduction
                (x[0] * levels).round() / levels
            });

        let crush_right = pass()
            >> hold_hz(sample_rate, 0.0)  // Sample rate reduction
            >> map(move |x: &Frame<f32, U1>| {  // Bit reduction
                (x[0] * levels).round() / levels
            });

        // Mix dry and wet using & operator to branch and sum
        let left = (pass() * dry) & (crush_left * mix);
        let right = (pass() * dry) & (crush_right * mix);

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("krush", "Bit reduction and sample rate reduction")
            .with_param("bits", 8.0, 1.0, 16.0)
            .with_param("sample_rate", 8000.0, 1000.0, 48000.0)
            .with_param("mix", 1.0, 0.0, 1.0)
    }
}

/// Register all distortion effects
pub fn register_all(registry: &mut super::super::registry::EffectRegistry) {
    registry.register("distortion", Arc::new(DistortionBuilder));
    registry.register("bitcrusher", Arc::new(BitcrusherBuilder));
    registry.register("krush", Arc::new(KrushBuilder));
}
