//! Bell synth builders
//!
//! This module contains synth builders that produce bell-like sounds using harmonic relationships:
//! - PrettyBellSynthBuilder: Bell sound with inharmonic partials
//! - DullBellSynthBuilder: Duller bell using triangle wave base

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Pretty bell synth with inharmonic partials
pub struct PrettyBellSynthBuilder;

impl SynthBuilder for PrettyBellSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Bell-like sound with multiple harmonics
        let fundamental = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> sine();
        let harmonic1 =
            var_fn(&pitch_bend_shared, move |bend| freq * 2.51 * bend) >> (sine() * 0.3);
        let harmonic2 =
            var_fn(&pitch_bend_shared, move |bend| freq * 3.99 * bend) >> (sine() * 0.15);

        let left = fundamental + harmonic1 + harmonic2;
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
        SynthMetadata::new("pretty_bell", "Pretty bell sound")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("bell")
            .with_tag("keys")
    }
}

/// Dull bell synth
pub struct DullBellSynthBuilder;

impl SynthBuilder for DullBellSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Duller bell with triangle wave base
        let fundamental = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> triangle();
        let harmonic1 =
            var_fn(&pitch_bend_shared, move |bend| freq * 2.0 * bend) >> (triangle() * 0.25);

        let left = fundamental + harmonic1;
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
        SynthMetadata::new("dull_bell", "Dull bell sound")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("bell")
            .with_tag("keys")
    }
}
