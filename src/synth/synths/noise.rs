//! Noise synth builders
//!
//! This module contains synth builders for noise-based sounds:
//! - NoiseSynthBuilder: White noise generator

use crate::params::ParameterDef;
use super::super::registry::{ SynthBuilder, SynthCategory, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// White noise generator
pub struct NoiseSynthBuilder;

impl SynthBuilder for NoiseSynthBuilder {
    fn build(
        &self,
        _freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = noise();
        let right = noise();
        let synth = Box::new((left | right) * (var(&amp_shared) | var(&amp_shared)));

        let controls = VoiceControls {
            amp: amp_shared,
            cutoff: None,
            resonance: None,
            pitch_bend: pitch_bend_shared,
            pressure: pressure_shared,
        };

        (synth, controls)
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata {
            name: "noise".to_string(),
            description: "White noise generator".to_string(),
            parameters: vec![ParameterDef {
                name: "amp".to_string(),
                default: 1.0,
                min: 0.0,
                max: 2.0,
            }],
            category: SynthCategory::Noise,
        }
    }
}
