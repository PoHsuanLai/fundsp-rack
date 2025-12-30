//! Pad and string synthesizers
//!
//! This module contains lush pad and string sounds:
//! - Strings: String ensemble pad
//! - Pad: Generic warm pad

use crate::params::ParameterDef;
use super::super::registry::{ SynthBuilder, SynthCategory, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// String ensemble pad
pub struct StringsSynthBuilder;

impl SynthBuilder for StringsSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let detune = params.get("detune").copied().unwrap_or(0.003);
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(3000.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);
        let cutoff_shared = shared(initial_cutoff);

        // String ensemble: multiple detuned saw waves with lowpass filter
        // Creates that lush, warm string sound
        let strings = (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune * 2.0)) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune)) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune * 2.0)) >> saw());

        // Apply lowpass filter for warmth
        let filtered = ( (strings * 0.2) | var(&cutoff_shared) | dc(0.5)) >> lowpass();

        let left = filtered.clone();
        let right = filtered;
        let synth = Box::new((left | right) * (var(&amp_shared) | var(&amp_shared)));

        let controls = VoiceControls {
            amp: amp_shared,
            cutoff: Some(cutoff_shared),
            resonance: None,
            pitch_bend: pitch_bend_shared,
            pressure: pressure_shared,
        };

        (synth, controls)
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata {
            name: "strings".to_string(),
            description: "String ensemble pad".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "detune".to_string(),
                    default: 0.003,
                    min: 0.0,
                    max: 0.02,
                },
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 3000.0,
                    min: 100.0,
                    max: 10000.0,
                },
            ],
            category: SynthCategory::Analog,
        }
    }
}

/// Warm pad synth
pub struct PadSynthBuilder;

impl SynthBuilder for PadSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let warmth = params.get("warmth").copied().unwrap_or(0.5);
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(2000.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);
        let cutoff_shared = shared(initial_cutoff);

        // Mix of saw and triangle for warmth control
        let saw_level = 1.0 - warmth * 0.5;
        let tri_level = warmth;

        let pad = (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw()) * saw_level
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> triangle()) * tri_level
            + (var_fn(&pitch_bend_shared, move |bend| freq * 0.5 * bend) >> sine()) * 0.3; // Sub

        let filtered = ( (pad * 0.4) | var(&cutoff_shared) | dc(0.3)) >> lowpass();

        let left = filtered.clone();
        let right = filtered;
        let synth = Box::new((left | right) * (var(&amp_shared) | var(&amp_shared)));

        let controls = VoiceControls {
            amp: amp_shared,
            cutoff: Some(cutoff_shared),
            resonance: None,
            pitch_bend: pitch_bend_shared,
            pressure: pressure_shared,
        };

        (synth, controls)
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata {
            name: "pad".to_string(),
            description: "Warm pad synth".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "warmth".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 1.0,
                },
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 2000.0,
                    min: 100.0,
                    max: 10000.0,
                },
            ],
            category: SynthCategory::Analog,
        }
    }
}
