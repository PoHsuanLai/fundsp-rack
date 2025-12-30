//! Basic waveform synthesizers
//!
//! This module contains simple oscillator-based synths:
//! - Sine wave
//! - Sawtooth wave
//! - Square wave
//! - Triangle wave
//! - Pulse wave

use crate::params::ParameterDef;
use super::super::registry::{ SynthBuilder, SynthCategory, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Simple sine wave oscillator
pub struct SineSynthBuilder;

impl SynthBuilder for SineSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> sine();
        let right = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> sine();
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
            name: "sine".to_string(),
            description: "Simple sine wave oscillator".to_string(),
            parameters: vec![ParameterDef {
                name: "amp".to_string(),
                default: 1.0,
                min: 0.0,
                max: 2.0,
            }],
            category: SynthCategory::Basic,
        }
    }
}

/// Sawtooth wave oscillator
pub struct SawSynthBuilder;

impl SynthBuilder for SawSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw();
        let right = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw();
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
            name: "saw".to_string(),
            description: "Sawtooth wave oscillator".to_string(),
            parameters: vec![ParameterDef {
                name: "amp".to_string(),
                default: 1.0,
                min: 0.0,
                max: 2.0,
            }],
            category: SynthCategory::Basic,
        }
    }
}

/// Square wave oscillator
pub struct SquareSynthBuilder;

impl SynthBuilder for SquareSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> square();
        let right = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> square();
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
            name: "square".to_string(),
            description: "Square wave oscillator".to_string(),
            parameters: vec![ParameterDef {
                name: "amp".to_string(),
                default: 1.0,
                min: 0.0,
                max: 2.0,
            }],
            category: SynthCategory::Basic,
        }
    }
}

/// Triangle wave oscillator
pub struct TriangleSynthBuilder;

impl SynthBuilder for TriangleSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> triangle();
        let right = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> triangle();
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
            name: "tri".to_string(),
            description: "Triangle wave oscillator".to_string(),
            parameters: vec![ParameterDef {
                name: "amp".to_string(),
                default: 1.0,
                min: 0.0,
                max: 2.0,
            }],
            category: SynthCategory::Basic,
        }
    }
}

/// Pulse wave oscillator with variable duty cycle
pub struct PulseSynthBuilder;

impl SynthBuilder for PulseSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let _duty = params.get("duty").copied().unwrap_or(0.5);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // FunDSP doesn't have a built-in pulse with variable duty cycle
        // Use square for now (50% duty cycle)
        let left = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> square();
        let right = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> square();
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
            name: "pulse".to_string(),
            description: "Pulse wave oscillator".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "duty".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: SynthCategory::Basic,
        }
    }
}
