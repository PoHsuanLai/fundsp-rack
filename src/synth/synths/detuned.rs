//! Detuned oscillator synths
//!
//! This module contains synth builders with multiple detuned oscillators for rich, chorusing sounds:
//! - DSawSynthBuilder: Detuned sawtooth waves
//! - DPulseSynthBuilder: Detuned pulse/square waves
//! - DTriSynthBuilder: Detuned triangle waves

use crate::params::ParameterDef;
use super::super::registry::{ SynthBuilder, SynthCategory, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Detuned saw waves
pub struct DSawSynthBuilder;

impl SynthBuilder for DSawSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let detune = params.get("detune").copied().unwrap_or(0.1);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune)) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> saw());
        let right = left.clone();
        let synth = Box::new((left | right) * 0.5 * (var(&amp_shared) | var(&amp_shared)));

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
            name: "dsaw".to_string(),
            description: "Detuned sawtooth waves".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "detune".to_string(),
                    default: 0.1,
                    min: 0.0,
                    max: 0.5,
                },
            ],
            category: SynthCategory::Analog,
        }
    }
}

/// Detuned pulse waves
pub struct DPulseSynthBuilder;

impl SynthBuilder for DPulseSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let detune = params.get("detune").copied().unwrap_or(0.1);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune))
            >> square())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> square());
        let right = left.clone();
        let synth = Box::new((left | right) * 0.5 * (var(&amp_shared) | var(&amp_shared)));

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
            name: "dpulse".to_string(),
            description: "Detuned pulse waves".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "detune".to_string(),
                    default: 0.1,
                    min: 0.0,
                    max: 0.5,
                },
            ],
            category: SynthCategory::Analog,
        }
    }
}

/// Detuned triangle waves
pub struct DTriSynthBuilder;

impl SynthBuilder for DTriSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let detune = params.get("detune").copied().unwrap_or(0.1);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune))
            >> triangle())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> triangle());
        let right = left.clone();
        let synth = Box::new((left | right) * 0.5 * (var(&amp_shared) | var(&amp_shared)));

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
            name: "dtri".to_string(),
            description: "Detuned triangle waves".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "detune".to_string(),
                    default: 0.1,
                    min: 0.0,
                    max: 0.5,
                },
            ],
            category: SynthCategory::Analog,
        }
    }
}
