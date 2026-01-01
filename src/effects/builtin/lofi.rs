//! Lo-fi effects (tape saturation, vinyl, etc.)

use super::super::registry::{EffectBuilder, EffectControls, EffectMetadata};
use fundsp::hacker32::*;
use numeric_array::typenum::U1;
use std::collections::HashMap;
use std::sync::Arc;

/// Tape saturation effect
pub struct TapeSaturationBuilder;

impl EffectBuilder for TapeSaturationBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let drive = params.get("drive").copied().unwrap_or(0.5);
        let warmth = params.get("warmth").copied().unwrap_or(0.5);
        let mix = params.get("mix").copied().unwrap_or(1.0);

        // Tape saturation: soft clipping + gentle high-frequency rolloff
        let saturation_amount = 1.0 + drive * 4.0;
        let filter_cutoff = 15000.0 - warmth * 10000.0; // More warmth = lower cutoff

        // Soft saturation using tanh
        let saturate_left =
            (pass() * saturation_amount) >> shape(Tanh(1.0)) >> lowpole_hz(filter_cutoff);
        let saturate_right =
            (pass() * saturation_amount) >> shape(Tanh(1.0)) >> lowpole_hz(filter_cutoff);

        // Mix dry and wet using & operator to branch and sum
        let left = (pass() * (1.0 - mix)) & (saturate_left * mix);
        let right = (pass() * (1.0 - mix)) & (saturate_right * mix);

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("tape_saturation", "Tape saturation (warm analog feel)")
            .with_param("drive", 0.5, 0.0, 1.0)
            .with_param("warmth", 0.5, 0.0, 1.0)
            .with_param("mix", 1.0, 0.0, 1.0)
    }
}

/// Lo-fi effect (sample rate reduction + bit reduction + filtering)
pub struct LofiBuilder;

impl EffectBuilder for LofiBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let amount = params.get("amount").copied().unwrap_or(0.5);
        let mix = params.get("mix").copied().unwrap_or(1.0);

        // Scale parameters based on amount
        let bits = 16.0 - amount * 12.0; // 16 bits down to 4 bits
        let sample_rate = 44100.0 - amount * 36100.0; // 44.1kHz down to 8kHz
        let filter_cutoff = 20000.0 - amount * 15000.0; // 20kHz down to 5kHz

        let levels = 2.0_f32.powf(bits);

        // Lo-fi chain: sample rate reduction + bit reduction + lowpass
        let lofi_left = pass()
            >> hold_hz(sample_rate, 0.0)
            >> map(move |x: &Frame<f32, U1>| (x[0] * levels).round() / levels)
            >> lowpole_hz(filter_cutoff);

        let lofi_right = pass()
            >> hold_hz(sample_rate, 0.0)
            >> map(move |x: &Frame<f32, U1>| (x[0] * levels).round() / levels)
            >> lowpole_hz(filter_cutoff);

        // Mix dry and wet using & operator to branch and sum
        let left = (pass() * (1.0 - mix)) & (lofi_left * mix);
        let right = (pass() * (1.0 - mix)) & (lofi_right * mix);

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("lofi", "Lo-fi effect (retro degradation)")
            .with_param("amount", 0.5, 0.0, 1.0)
            .with_param("mix", 1.0, 0.0, 1.0)
    }
}

/// Vinyl effect (crackle, hiss, wow/flutter)
pub struct VinylBuilder;

impl EffectBuilder for VinylBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let hiss = params.get("hiss").copied().unwrap_or(0.2);
        let warmth = params.get("warmth").copied().unwrap_or(0.5);

        // Vinyl characteristics:
        // 1. High-frequency rolloff (warmth)
        // 2. Subtle noise (hiss)
        // 3. Low-frequency filter for that vinyl bass

        let filter_cutoff = 12000.0 - warmth * 6000.0;

        // Add noise and filter
        let vinyl_left = pass() >> lowpole_hz(filter_cutoff) >> highpole_hz(30.0); // Remove sub-bass rumble

        let vinyl_right = pass() >> lowpole_hz(filter_cutoff) >> highpole_hz(30.0);

        // Add hiss (filtered noise)
        let hiss_level = hiss * 0.02;
        let left = vinyl_left + (noise() >> lowpole_hz(8000.0)) * hiss_level;
        let right = vinyl_right + (noise() >> lowpole_hz(8000.0)) * hiss_level;

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("vinyl", "Vinyl record effect")
            .with_param("crackle", 0.3, 0.0, 1.0)
            .with_param("hiss", 0.2, 0.0, 1.0)
            .with_param("warmth", 0.5, 0.0, 1.0)
    }
}

/// Register all lo-fi effects
pub fn register_all(registry: &mut super::super::registry::EffectRegistry) {
    registry.register("tape_saturation", Arc::new(TapeSaturationBuilder));
    registry.register("tape", Arc::new(TapeSaturationBuilder)); // alias
    registry.register("lofi", Arc::new(LofiBuilder));
    registry.register("lo-fi", Arc::new(LofiBuilder)); // alias
    registry.register("vinyl", Arc::new(VinylBuilder));
}
