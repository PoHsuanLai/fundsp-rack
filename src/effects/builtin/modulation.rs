//! Modulation effects (chorus, flanger, tremolo)

use super::super::registry::{EffectBuilder, EffectControls, EffectMetadata};
use fundsp::hacker32::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Chorus effect
pub struct ChorusBuilder;

impl EffectBuilder for ChorusBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let separation = params.get("separation").copied().unwrap_or(0.02);
        let variation = params.get("variation").copied().unwrap_or(0.5);
        let mod_freq = params.get("mod_frequency").copied().unwrap_or(0.5);

        let left_chorus = chorus(0, separation, variation, mod_freq);
        let right_chorus = chorus(1, separation, variation, mod_freq);

        (Box::new(left_chorus | right_chorus), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("chorus", "Chorus effect")
            .with_param("separation", 0.02, 0.0, 0.1)
            .with_param("variation", 0.5, 0.0, 1.0)
            .with_param("mod_frequency", 0.5, 0.1, 10.0)
    }
}

/// Flanger effect
pub struct FlangerBuilder;

impl EffectBuilder for FlangerBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let min_delay = params.get("min_delay").copied().unwrap_or(0.002);
        let max_delay = params.get("max_delay").copied().unwrap_or(0.01);
        let rate = params.get("rate").copied().unwrap_or(0.5);
        let feedback = params.get("feedback").copied().unwrap_or(0.6);

        // flanger(feedback, min_delay, max_delay, delay_function)
        // The delay function modulates between min and max delay over time
        let left = flanger(feedback, min_delay, max_delay, move |t| {
            fundsp::math::lerp11(min_delay, max_delay, fundsp::math::sin_hz(rate, t))
        });
        let right = flanger(feedback, min_delay, max_delay, move |t| {
            fundsp::math::lerp11(min_delay, max_delay, fundsp::math::cos_hz(rate, t))
        });

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("flanger", "Flanger effect")
            .with_param("depth", 0.005, 0.0, 0.02)
            .with_param("rate", 0.5, 0.1, 10.0)
    }
}

/// Tremolo effect
pub struct TremoloBuilder;

impl EffectBuilder for TremoloBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let rate = params.get("rate").copied().unwrap_or(4.0);
        let depth = params.get("depth").copied().unwrap_or(0.5);

        // Tremolo: modulate amplitude with LFO
        // pass() * (LFO * depth + (1 - depth))
        // LFO is sine_hz which outputs -1 to 1, we need to scale it to 0 to 1
        let lfo = sine_hz(rate) * 0.5 + 0.5; // Now 0 to 1
        let modulator = lfo * depth + (1.0 - depth); // Scale by depth

        let left = pass() * modulator.clone();
        let right = pass() * modulator;

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("tremolo", "Tremolo (amplitude modulation)")
            .with_param("rate", 4.0, 0.1, 20.0)
            .with_param("depth", 0.5, 0.0, 1.0)
    }
}

/// Phaser effect
pub struct PhaserBuilder;

impl EffectBuilder for PhaserBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let _rate = params.get("rate").copied().unwrap_or(0.5);
        let depth = params.get("depth").copied().unwrap_or(0.5);
        let _feedback = params.get("feedback").copied().unwrap_or(0.5);
        let _stages = params.get("stages").copied().unwrap_or(4.0) as usize;

        // Phaser uses cascaded allpass filters
        // Simplified version: just apply allpass stages directly
        // The allpass filters create phase shifts that produce the phasing effect

        let base_delay = 1.0 + (depth * 3.0);

        // Build 4-stage allpass chain for left channel
        let left = pass()
            >> allpole_delay(base_delay)
            >> allpole_delay(base_delay * 1.5)
            >> allpole_delay(base_delay * 2.0)
            >> allpole_delay(base_delay * 2.5);

        // Build 4-stage allpass chain for right channel
        let right = pass()
            >> allpole_delay(base_delay)
            >> allpole_delay(base_delay * 1.5)
            >> allpole_delay(base_delay * 2.0)
            >> allpole_delay(base_delay * 2.5);

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("phaser", "Phaser (sweeping notch filter)")
            .with_param("rate", 0.5, 0.1, 10.0)
            .with_param("depth", 0.5, 0.0, 1.0)
            .with_param("feedback", 0.5, 0.0, 0.95)
            .with_param("stages", 4.0, 2.0, 12.0)
    }
}

/// Vibrato effect (pitch modulation)
pub struct VibratoBuilder;

impl EffectBuilder for VibratoBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let rate = params.get("rate").copied().unwrap_or(5.0);
        let depth = params.get("depth").copied().unwrap_or(0.5);

        // Vibrato is pitch modulation via delay line modulation
        // Calculate delay modulation depth in seconds (related to pitch shift)
        // depth controls the amount of pitch deviation
        let delay_depth_sec = depth * 0.005; // Max ~5ms modulation

        // Create LFO for delay modulation
        let lfo_left = sine_hz(rate).clone();
        let lfo_right = sine_hz(rate);

        // Center delay (where no pitch shift occurs)
        let center_delay = 0.01; // 10ms base delay
        let min_delay = center_delay - delay_depth_sec;
        let max_delay = center_delay + delay_depth_sec;

        // Modulate delay time with LFO
        let delay_mod_left = lfo_left
            >> map(move |x: &Frame<f32, U1>| fundsp::math::lerp11(min_delay, max_delay, x[0]));
        let delay_mod_right = lfo_right
            >> map(move |x: &Frame<f32, U1>| fundsp::math::lerp11(min_delay, max_delay, x[0]));

        // Use tap_linear for smooth variable delay (linear interpolation)
        let left = (pass() | delay_mod_left) >> tap_linear(min_delay, max_delay);
        let right = (pass() | delay_mod_right) >> tap_linear(min_delay, max_delay);

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("vibrato", "Vibrato (pitch modulation)")
            .with_param("rate", 5.0, 0.5, 20.0)
            .with_param("depth", 0.5, 0.0, 1.0)
            .with_latency(441) // ~10ms at 44.1kHz
    }
}

/// Register all modulation effects
pub fn register_all(registry: &mut super::super::registry::EffectRegistry) {
    registry.register("chorus", Arc::new(ChorusBuilder));
    registry.register("flanger", Arc::new(FlangerBuilder));
    registry.register("tremolo", Arc::new(TremoloBuilder));
    registry.register("phaser", Arc::new(PhaserBuilder));
    registry.register("vibrato", Arc::new(VibratoBuilder));
}
