//! Lead synthesizers
//!
//! This module contains monophonic lead synths:
//! - Lead: Classic mono lead with filter
//! - Sub: Pure sub bass

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Classic mono lead synth (Moog-style)
pub struct LeadSynthBuilder;

impl SynthBuilder for LeadSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(2500.0);
        let initial_resonance = params.get("res").copied().unwrap_or(0.4);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);
        let cutoff_shared = shared(initial_cutoff);
        let resonance_shared = shared(initial_resonance);

        // Classic lead: saw + square mixed, through Moog filter
        let osc = (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw()) * 0.6
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> square()) * 0.4;

        let filtered = (osc | var(&cutoff_shared) | var(&resonance_shared)) >> moog();

        let left = filtered.clone();
        let right = filtered;
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
        SynthMetadata::new("lead", "Classic mono lead synth")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("cutoff", 2500.0, 100.0, 15000.0)
            .with_param("res", 0.4, 0.0, 1.0)
            .with_param("glide", 0.0, 0.0, 1.0)
            .with_tag("lead")
    }
}

/// Pure sub bass
pub struct SubSynthBuilder;

impl SynthBuilder for SubSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let shape = params.get("shape").copied().unwrap_or(0.0); // 0 = sine, 1 = triangle

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Sub bass: pure low frequency, optionally with some triangle for harmonics
        let sine_level = 1.0 - shape;
        let tri_level = shape;

        let sub = (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> sine()) * sine_level
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> triangle()) * tri_level;

        let left = sub.clone();
        let right = sub;
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
        SynthMetadata::new("sub", "Pure sub bass")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("shape", 0.0, 0.0, 1.0)
            .with_tag("bass")
            .with_tag("sub")
    }
}

/// Brass stab synth
pub struct BrassSynthBuilder;

impl SynthBuilder for BrassSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(3000.0);
        let initial_resonance = params.get("res").copied().unwrap_or(0.3);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);
        let cutoff_shared = shared(initial_cutoff);
        let resonance_shared = shared(initial_resonance);

        // Brass: saw waves with slight detuning, filtered
        let detune = 0.005;
        let brass = (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 - detune)) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend) >> saw())
            + (var_fn(&pitch_bend_shared, move |bend| freq * bend * (1.0 + detune)) >> saw());

        let filtered = ((brass * 0.33) | var(&cutoff_shared) | var(&resonance_shared)) >> moog();

        let left = filtered.clone();
        let right = filtered;
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
        SynthMetadata::new("brass", "Brass stab synth")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("cutoff", 3000.0, 100.0, 15000.0)
            .with_param("res", 0.3, 0.0, 1.0)
            .with_tag("lead")
            .with_tag("brass")
    }
}
