//! Keyboard instrument synths (organ, electric piano)
//!
//! This module contains keyboard-style synthesizers:
//! - Organ: Hammond-style drawbar organ
//! - Electric Piano: Rhodes-style electric piano

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Hammond-style drawbar organ
pub struct OrganSynthBuilder;

impl SynthBuilder for OrganSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        // Drawbar levels (0.0 to 1.0) - classic Hammond registrations
        let drawbar_16 = params.get("drawbar_16").copied().unwrap_or(0.8); // Sub-fundamental
        let drawbar_8 = params.get("drawbar_8").copied().unwrap_or(1.0); // Fundamental
        let drawbar_4 = params.get("drawbar_4").copied().unwrap_or(0.6); // 2nd harmonic
        let drawbar_2 = params.get("drawbar_2").copied().unwrap_or(0.4); // 4th harmonic
        let drawbar_1 = params.get("drawbar_1").copied().unwrap_or(0.2); // 8th harmonic

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Hammond organ uses additive synthesis with sine waves at harmonic intervals
        // 16' = sub-octave, 8' = fundamental, 4' = octave, 2' = two octaves, 1' = three octaves
        let organ = (var_fn(&pitch_bend_shared, move |bend| freq * 0.5 * bend) >> sine())
            * drawbar_16
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> sine()) * drawbar_8
            + (var_fn(&pitch_bend_shared, move |bend| freq * 2.0 * bend) >> sine()) * drawbar_4
            + (var_fn(&pitch_bend_shared, move |bend| freq * 4.0 * bend) >> sine()) * drawbar_2
            + (var_fn(&pitch_bend_shared, move |bend| freq * 8.0 * bend) >> sine()) * drawbar_1;

        let left = organ.clone();
        let right = organ;
        let synth = Box::new((left | right) * 0.3 * (var(&amp_shared) | var(&amp_shared)));

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
        SynthMetadata::new("organ", "Hammond-style drawbar organ")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("drawbar_16", 0.8, 0.0, 1.0)
            .with_param("drawbar_8", 1.0, 0.0, 1.0)
            .with_param("drawbar_4", 0.6, 0.0, 1.0)
            .with_param("drawbar_2", 0.4, 0.0, 1.0)
            .with_param("drawbar_1", 0.2, 0.0, 1.0)
            .with_tag("keys")
            .with_tag("organ")
    }
}

/// Rhodes-style electric piano
pub struct ElectricPianoSynthBuilder;

impl SynthBuilder for ElectricPianoSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let brightness = params.get("brightness").copied().unwrap_or(0.5);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Rhodes-style: fundamental + bell-like harmonics
        // Uses sine waves with specific harmonic ratios for that bell-like quality
        let harmonic_2_level = 0.3 + brightness * 0.3;
        let harmonic_3_level = 0.15 + brightness * 0.2;

        let ep = (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> sine())
            + (var_fn(&pitch_bend_shared, move |bend| freq * 2.0 * bend) >> sine())
                * harmonic_2_level
            + (var_fn(&pitch_bend_shared, move |bend| freq * 3.0 * bend) >> sine())
                * harmonic_3_level;

        let left = ep.clone();
        let right = ep;
        let synth = Box::new((left | right) * 0.4 * (var(&amp_shared) | var(&amp_shared)));

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
        SynthMetadata::new("electric_piano", "Rhodes-style electric piano")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("brightness", 0.5, 0.0, 1.0)
            .with_tag("keys")
            .with_tag("piano")
    }
}
