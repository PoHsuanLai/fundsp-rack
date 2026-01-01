//! Pad and string synthesizers
//!
//! This module contains lush pad and string sounds:
//! - Strings: String ensemble pad
//! - Pad: Generic warm pad

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// String ensemble pad
pub struct StringsSynthBuilder;

impl SynthBuilder for StringsSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let detune = params.get("detune").copied().unwrap_or(0.003);
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(3000.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);
        let cutoff_shared = shared(initial_cutoff);

        // String ensemble: multiple detuned saw waves with lowpass filter
        // Creates that lush, warm string sound
        let strings = (var_fn(&pitch_bend_shared, move |bend| {
            freq * bend * (1.0 - detune * 2.0)
        }) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune)) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| {
                freq * bend * (1.0 + detune * 2.0)
            }) >> saw());

        // Apply lowpass filter for warmth
        let filtered = ((strings * 0.2) | var(&cutoff_shared) | dc(0.5)) >> lowpass();

        let left = filtered.clone();
        let right = filtered;
        let synth = Box::new((left | right) * (var(&amp_shared) | var(&amp_shared)));

        let controls = VoiceControls {
            amp: amp_shared,
            cutoff: Some(cutoff_shared),
            resonance: None,
            pitch_bend: pitch_bend_shared,
            pressure: pressure_shared,
        };

        (synth, controls)
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata::new("strings", "String ensemble pad")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("detune", 0.003, 0.0, 0.02)
            .with_param("cutoff", 3000.0, 100.0, 10000.0)
            .with_tag("pad")
            .with_tag("strings")
    }
}

/// Warm pad synth
pub struct PadSynthBuilder;

impl SynthBuilder for PadSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let warmth = params.get("warmth").copied().unwrap_or(0.5);
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(2000.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);
        let cutoff_shared = shared(initial_cutoff);

        // Mix of saw and triangle for warmth control
        let saw_level = 1.0 - warmth * 0.5;
        let tri_level = warmth;

        let pad = (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw()) * saw_level
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> triangle()) * tri_level
            + (var_fn(&pitch_bend_shared, move |bend| freq * 0.5 * bend) >> sine()) * 0.3; // Sub

        let filtered = ((pad * 0.4) | var(&cutoff_shared) | dc(0.3)) >> lowpass();

        let left = filtered.clone();
        let right = filtered;
        let synth = Box::new((left | right) * (var(&amp_shared) | var(&amp_shared)));

        let controls = VoiceControls {
            amp: amp_shared,
            cutoff: Some(cutoff_shared),
            resonance: None,
            pitch_bend: pitch_bend_shared,
            pressure: pressure_shared,
        };

        (synth, controls)
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata::new("pad", "Warm pad synth")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("warmth", 0.5, 0.0, 1.0)
            .with_param("cutoff", 2000.0, 100.0, 10000.0)
            .with_tag("pad")
    }
}
