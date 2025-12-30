//! Modulated oscillator synths
//!
//! This module contains synth builders that use LFO-modulated oscillators:
//! - ModSawSynthBuilder: Modulated sawtooth wave
//! - ModSineSynthBuilder: Modulated sine wave
//! - ModTriSynthBuilder: Modulated triangle wave
//! - ModPulseSynthBuilder: Modulated pulse/square wave

use crate::params::ParameterDef;
use super::super::registry::{ SynthBuilder, SynthCategory, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Modulated saw wave
pub struct ModSawSynthBuilder;

impl SynthBuilder for ModSawSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let mod_freq = params.get("mod_freq").copied().unwrap_or(5.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Modulate frequency with an LFO
        let left = (sine_hz(mod_freq) * freq * 0.1 + dc(freq)) >> saw();
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
            name: "mod_saw".to_string(),
            description: "Modulated sawtooth wave".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "mod_freq".to_string(),
                    default: 5.0,
                    min: 0.1,
                    max: 20.0,
                },
            ],
            category: SynthCategory::Digital,
        }
    }
}

/// Modulated sine wave
pub struct ModSineSynthBuilder;

impl SynthBuilder for ModSineSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let mod_freq = params.get("mod_freq").copied().unwrap_or(5.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = (sine_hz(mod_freq) * freq * 0.1 + dc(freq)) >> sine();
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
            name: "mod_sine".to_string(),
            description: "Modulated sine wave".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "mod_freq".to_string(),
                    default: 5.0,
                    min: 0.1,
                    max: 20.0,
                },
            ],
            category: SynthCategory::Digital,
        }
    }
}

/// Modulated triangle wave
pub struct ModTriSynthBuilder;

impl SynthBuilder for ModTriSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let mod_freq = params.get("mod_freq").copied().unwrap_or(5.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = (sine_hz(mod_freq) * freq * 0.1 + dc(freq)) >> triangle();
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
            name: "mod_tri".to_string(),
            description: "Modulated triangle wave".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "mod_freq".to_string(),
                    default: 5.0,
                    min: 0.1,
                    max: 20.0,
                },
            ],
            category: SynthCategory::Digital,
        }
    }
}

/// Modulated pulse wave
pub struct ModPulseSynthBuilder;

impl SynthBuilder for ModPulseSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let mod_freq = params.get("mod_freq").copied().unwrap_or(5.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = (sine_hz(mod_freq) * freq * 0.1 + dc(freq)) >> square();
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
            name: "mod_pulse".to_string(),
            description: "Modulated pulse wave".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "mod_freq".to_string(),
                    default: 5.0,
                    min: 0.1,
                    max: 20.0,
                },
            ],
            category: SynthCategory::Digital,
        }
    }
}
