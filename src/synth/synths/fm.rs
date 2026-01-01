//! FM synthesis synths
//!
//! This module contains synth builders that use frequency modulation:
//! - FMSynthBuilder: Simple FM synthesis with harmonic modulator relationship

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Simple FM synthesis
pub struct FMSynthBuilder;

impl SynthBuilder for FMSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let mod_index = params.get("mod_index").copied().unwrap_or(2.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Simple FM: carrier frequency modulated by modulator
        let modulator_freq = freq * 2.0; // Harmonic relationship
        let left = (sine_hz(modulator_freq) * freq * mod_index + dc(freq)) >> sine();
        let right = left.clone();
        let synth = Box::new((left | right) * (var(&amp_shared) | var(&amp_shared)));

        (
            synth,
            VoiceControls {
                amp: amp_shared,
                cutoff: None,
                resonance: None,
                pitch_bend: pitch_bend_shared,
                pressure: pressure_shared,
            },
        )
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata::new("fm", "FM synthesis")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("mod_index", 2.0, 0.0, 10.0)
            .with_tag("fm")
            .with_tag("synth")
    }
}
