//! Detuned oscillator synths
//!
//! This module contains synth builders with multiple detuned oscillators for rich, chorusing sounds:
//! - DSawSynthBuilder: Detuned sawtooth waves
//! - DPulseSynthBuilder: Detuned pulse/square waves
//! - DTriSynthBuilder: Detuned triangle waves

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
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
        SynthMetadata::new("dsaw", "Detuned sawtooth waves")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("detune", 0.1, 0.0, 0.5)
            .with_tag("synth")
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
        SynthMetadata::new("dpulse", "Detuned pulse waves")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("detune", 0.1, 0.0, 0.5)
            .with_tag("synth")
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
        SynthMetadata::new("dtri", "Detuned triangle waves")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("detune", 0.1, 0.0, 0.5)
            .with_tag("synth")
    }
}
