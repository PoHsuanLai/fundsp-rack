//! Analog-style synthesizers
//!
//! This module contains classic analog synth emulations:
//! - TB-303 acid bass
//! - Prophet-5 style
//! - Supersaw
//! - Hoover rave synth

use crate::params::ParameterDef;
use super::super::registry::{ SynthBuilder, SynthCategory, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Classic TB-303 acid bass synth
pub struct TB303SynthBuilder;

impl SynthBuilder for TB303SynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(1000.0);
        let initial_resonance = params.get("res").copied().unwrap_or(0.5);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);
        let cutoff_shared = shared(initial_cutoff);
        let resonance_shared = shared(initial_resonance);

        let left = ((var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw())
            | var(&cutoff_shared)
            | var(&resonance_shared))
            >> moog();
        let right = ((var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw())
            | var(&cutoff_shared)
            | var(&resonance_shared))
            >> moog();
        let synth = Box::new((left | right) * (var(&amp_shared) | var(&amp_shared)));

        let controls = VoiceControls {
            amp: amp_shared,
            cutoff: Some(cutoff_shared),
            resonance: Some(resonance_shared),
            pitch_bend: pitch_bend_shared,
            pressure: pressure_shared,
        };

        (synth, controls)
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata {
            name: "tb303".to_string(),
            description: "Classic TB-303 acid bass synth".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: SynthCategory::Analog,
        }
    }
}

/// Prophet-style synth with filter
pub struct ProphetSynthBuilder;

impl SynthBuilder for ProphetSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(2000.0);
        let initial_resonance = params.get("res").copied().unwrap_or(0.3);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);
        let cutoff_shared = shared(initial_cutoff);
        let resonance_shared = shared(initial_resonance);

        // Mix saw and square waves for classic analog sound
        let left_saw = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw();
        let left_square = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> square();
        let left =
            ((left_saw * 0.5 + left_square * 0.5) | var(&cutoff_shared) | var(&resonance_shared))
                >> moog();

        let right_saw = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw();
        let right_square = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> square();
        let right =
            ((right_saw * 0.5 + right_square * 0.5) | var(&cutoff_shared) | var(&resonance_shared))
                >> moog();

        let synth = Box::new((left | right) * (var(&amp_shared) | var(&amp_shared)));

        let controls = VoiceControls {
            amp: amp_shared,
            cutoff: Some(cutoff_shared),
            resonance: Some(resonance_shared),
            pitch_bend: pitch_bend_shared,
            pressure: pressure_shared,
        };

        (synth, controls)
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata {
            name: "prophet".to_string(),
            description: "Prophet-style analog synth".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 2000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.3,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: SynthCategory::Analog,
        }
    }
}

/// Supersaw synth with detuned oscillators
pub struct SupersawSynthBuilder;

impl SynthBuilder for SupersawSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let detune = params.get("detune").copied().unwrap_or(0.02);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Create 7 detuned saw waves
        let left = (var_fn(&pitch_bend_shared, move |bend| {
            freq * bend * (1.0 - detune * 3.0)
        }) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| {
                freq * bend * (1.0 - detune * 2.0)
            }) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune)) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| {
                freq * bend * (1.0 + detune * 2.0)
            }) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| {
                freq * bend * (1.0 + detune * 3.0)
            }) >> saw());

        let right = left.clone();
        let synth = Box::new((left | right) * 0.14 * (var(&amp_shared) | var(&amp_shared)));

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
            name: "supersaw".to_string(),
            description: "Supersaw with detuned oscillators".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "detune".to_string(),
                    default: 0.02,
                    min: 0.0,
                    max: 0.1,
                },
            ],
            category: SynthCategory::Analog,
        }
    }
}

/// Hoover-style rave synth
pub struct HooverSynthBuilder;

impl SynthBuilder for HooverSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        // Hoover is multiple detuned saws with resonant filter
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(1500.0);
        let initial_resonance = params.get("res").copied().unwrap_or(0.7);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);
        let cutoff_shared = shared(initial_cutoff);
        let resonance_shared = shared(initial_resonance);

        // Stack multiple detuned saws
        let detune = 0.05;
        let left = ((((var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune))
            >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> saw()))
            * 0.33)
            | var(&cutoff_shared)
            | var(&resonance_shared))
            >> (moog() * var(&amp_shared));

        let pitch_bend_shared2 = pitch_bend_shared.clone();
        let cutoff_shared2 = cutoff_shared.clone();
        let resonance_shared2 = resonance_shared.clone();
        let amp_shared2 = amp_shared.clone();

        let right = ((((var_fn(&pitch_bend_shared2, move |bend| {
            freq * bend * (1.0 - detune)
        }) >> saw())
            + (var_fn(&pitch_bend_shared2, move |bend| freq * bend) >> saw())
            + (var_fn(&pitch_bend_shared2, move |bend| {
                freq * bend * (1.0 + detune)
            }) >> saw()))
            * 0.33)
            | var(&cutoff_shared2)
            | var(&resonance_shared2))
            >> (moog() * var(&amp_shared2));

        let synth = Box::new(left | right);

        (
            synth,
            VoiceControls {
                amp: amp_shared,
                cutoff: Some(cutoff_shared),
                resonance: Some(resonance_shared),
                pitch_bend: pitch_bend_shared,
                pressure: pressure_shared,
            },
        )
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata {
            name: "hoover".to_string(),
            description: "Hoover-style rave synth".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "amp".to_string(),
                    default: 1.0,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1500.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.7,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: SynthCategory::Analog,
        }
    }
}
