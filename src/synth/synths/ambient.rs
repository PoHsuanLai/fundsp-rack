//! Ambient and pad synth builders
//!
//! This module contains synth builders designed for ambient and pad sounds:
//! - DarkAmbienceSynthBuilder: Dark ambient pad with triangle and sub oscillator
//! - GrowlSynthBuilder: Growling bass sound with low-frequency modulation
//! - HollowSynthBuilder: Hollow, airy ambient sound with detuned sines

use crate::params::ParameterDef;
use super::super::registry::{ SynthBuilder, SynthCategory, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Dark ambience synth
pub struct DarkAmbienceSynthBuilder;

impl SynthBuilder for DarkAmbienceSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(0.5);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Dark pad sound - triangle with sub oscillator
        let main = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> triangle();
        let sub = var_fn(&pitch_bend_shared, move |bend| freq * 0.5 * bend) >> (sine() * 0.5);

        let left = main + sub;
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
            name: "dark_ambience".to_string(),
            description: "Dark ambient pad".to_string(),
            parameters: vec![ParameterDef {
                name: "amp".to_string(),
                default: 0.5,
                min: 0.0,
                max: 2.0,
            }],
            category: SynthCategory::Analog,
        }
    }
}

/// Growl synth
pub struct GrowlSynthBuilder;

impl SynthBuilder for GrowlSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Growl - saw with low-frequency modulation
        let left = (sine_hz(1.5) * freq * 0.3
            + var_fn(&pitch_bend_shared, move |bend| freq * bend))
            >> saw();
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
            name: "growl".to_string(),
            description: "Growling bass sound".to_string(),
            parameters: vec![ParameterDef {
                name: "amp".to_string(),
                default: 1.0,
                min: 0.0,
                max: 2.0,
            }],
            category: SynthCategory::Analog,
        }
    }
}

/// Hollow synth
pub struct HollowSynthBuilder;

impl SynthBuilder for HollowSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(0.7);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Hollow, airy sound with detuned sines
        let detune = 0.02;
        let left = (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune)) >> sine())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> sine());
        let right = (var_fn(&pitch_bend_shared, move |bend| {
            freq * bend * (1.0 + detune * 0.5)
        }) >> sine())
            + (var_fn(&pitch_bend_shared, move |bend| {
                freq * bend * (1.0 - detune * 0.5)
            }) >> sine());
        let synth =
            Box::new(((left * 0.5) | (right * 0.5)) * (var(&amp_shared) | var(&amp_shared)));

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
            name: "hollow".to_string(),
            description: "Hollow, airy ambient sound".to_string(),
            parameters: vec![ParameterDef {
                name: "amp".to_string(),
                default: 0.7,
                min: 0.0,
                max: 2.0,
            }],
            category: SynthCategory::Analog,
        }
    }
}
