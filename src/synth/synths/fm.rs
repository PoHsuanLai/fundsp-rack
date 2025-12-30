//! FM synthesis synths
//!
//! This module contains synth builders that use frequency modulation:
//! - FMSynthBuilder: Simple FM synthesis with harmonic modulator relationship

use crate::params::ParameterDef;
use super::super::registry::{ SynthBuilder, SynthCategory, SynthMetadata, VoiceControls};
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
        SynthMetadata {
            name: "fm".to_string(),
            description: "FM synthesis".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "mod_index".to_string(),
                    default: 2.0,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: SynthCategory::Digital,
        }
    }
}
