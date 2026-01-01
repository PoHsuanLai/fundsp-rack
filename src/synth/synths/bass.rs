//! Bass synth builders
//!
//! This module contains synth builders specialized for bass sounds:
//! - BassFoundationSynthBuilder: Deep sine bass
//! - BassHighendSynthBuilder: Saw bass with harmonics

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Bass foundation synth - deep sine bass
pub struct BassFoundationSynthBuilder;

impl SynthBuilder for BassFoundationSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Deep sine bass
        let left = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> sine();
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
        SynthMetadata::new("bass_foundation", "Deep bass foundation")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("bass")
    }
}

/// Bass highend synth - saw bass with harmonics
pub struct BassHighendSynthBuilder;

impl SynthBuilder for BassHighendSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Saw bass with harmonics
        let left = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw();
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
        SynthMetadata::new("bass_highend", "Bass with high-end harmonics")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("bass")
    }
}
