//! Tech and electronic synth builders
//!
//! This module contains synth builders designed for electronic and technical sounds:
//! - TechSawsSynthBuilder: Tech/trance detuned saws
//! - BladeSynthBuilder: Sharp, cutting square wave synth
//! - ZawaSynthBuilder: Buzzy, energetic saw + square mix
//! - SubpulseSynthBuilder: Sub-bass pulse wave (one octave lower)

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Tech saws synth
pub struct TechSawsSynthBuilder;

impl SynthBuilder for TechSawsSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        // Similar to supersaw but with specific tech character
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let detune = 0.03;
        let left = (var_fn(&pitch_bend_shared, move |bend| freq * bend * 0.99) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * 1.01) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> saw());

        let right = left.clone();
        let synth = Box::new((left | right) * 0.25 * (var(&amp_shared) | var(&amp_shared)));

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
        SynthMetadata::new("tech_saws", "Tech/trance saws")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("lead")
            .with_tag("tech")
    }
}

/// Blade synth
pub struct BladeSynthBuilder;

impl SynthBuilder for BladeSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Sharp, cutting sound
        let left = var_fn(&pitch_bend_shared, move |bend| freq * bend) >> square();
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
        SynthMetadata::new("blade", "Sharp, cutting synth")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("lead")
            .with_tag("tech")
    }
}

/// Zawa synth
pub struct ZawaSynthBuilder;

impl SynthBuilder for ZawaSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Buzzy, energetic sound
        let left = (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * 1.5 * bend) >> (square() * 0.3));
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
        SynthMetadata::new("zawa", "Buzzy, energetic synth")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("lead")
            .with_tag("tech")
    }
}

/// Subpulse synth
pub struct SubpulseSynthBuilder;

impl SynthBuilder for SubpulseSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Sub-bass pulse wave (one octave lower)
        let sub_freq = freq * 0.5;
        let left = var_fn(&pitch_bend_shared, move |bend| sub_freq * bend) >> square();
        let right = var_fn(&pitch_bend_shared, move |bend| sub_freq * bend) >> square();
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
        SynthMetadata::new("subpulse", "Sub-bass pulse wave")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_tag("bass")
    }
}
