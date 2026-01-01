//! Physical modeling synths
//!
//! This module contains synth builders based on physical modeling techniques:
//! - PianoSynthBuilder: Simple piano-like sound with harmonics
//! - PluckSynthBuilder: Karplus-Strong plucked string algorithm

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Simple piano-like synth
pub struct PianoSynthBuilder;

impl SynthBuilder for PianoSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Simple piano-like sound with harmonics and envelope
        let fundamental = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> sine();
        let harmonic1 = var_fn(&pitch_bend_shared, move |bend| freq * 2.0 * bend) >> (sine() * 0.5);
        let harmonic2 =
            var_fn(&pitch_bend_shared, move |bend| freq * 3.0 * bend) >> (sine() * 0.25);

        let left = fundamental + harmonic1 + harmonic2;
        let right = left.clone();
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
        SynthMetadata::new("piano", "Simple piano-like synth")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("keys")
            .with_tag("piano")
    }
}

/// Karplus-Strong plucked string
pub struct PluckSynthBuilder;

impl SynthBuilder for PluckSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Use noise burst + resonator for pluck sound
        let left = (noise() * 0.5) >> resonator_hz(freq, 20.0);
        let right = (noise() * 0.5) >> resonator_hz(freq, 20.0);
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
        SynthMetadata::new("pluck", "Karplus-Strong plucked string")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("pluck")
    }
}
