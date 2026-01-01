//! Modulated oscillator synths
//!
//! This module contains synth builders that use LFO-modulated oscillators:
//! - ModSawSynthBuilder: Modulated sawtooth wave
//! - ModSineSynthBuilder: Modulated sine wave
//! - ModTriSynthBuilder: Modulated triangle wave
//! - ModPulseSynthBuilder: Modulated pulse/square wave

use super::super::registry::{SynthBuilder, SynthMetadata, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Modulated saw wave
pub struct ModSawSynthBuilder;

impl SynthBuilder for ModSawSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let mod_freq = params.get("mod_freq").copied().unwrap_or(5.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        // Modulate frequency with an LFO
        let left = (sine_hz(mod_freq) * freq * 0.1 + dc(freq)) >> saw();
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
        SynthMetadata::new("mod_saw", "Modulated sawtooth wave")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("mod_freq", 5.0, 0.1, 20.0)
            .with_tag("synth")
    }
}

/// Modulated sine wave
pub struct ModSineSynthBuilder;

impl SynthBuilder for ModSineSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let mod_freq = params.get("mod_freq").copied().unwrap_or(5.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = (sine_hz(mod_freq) * freq * 0.1 + dc(freq)) >> sine();
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
        SynthMetadata::new("mod_sine", "Modulated sine wave")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("mod_freq", 5.0, 0.1, 20.0)
            .with_tag("synth")
    }
}

/// Modulated triangle wave
pub struct ModTriSynthBuilder;

impl SynthBuilder for ModTriSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let mod_freq = params.get("mod_freq").copied().unwrap_or(5.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = (sine_hz(mod_freq) * freq * 0.1 + dc(freq)) >> triangle();
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
        SynthMetadata::new("mod_tri", "Modulated triangle wave")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("mod_freq", 5.0, 0.1, 20.0)
            .with_tag("synth")
    }
}

/// Modulated pulse wave
pub struct ModPulseSynthBuilder;

impl SynthBuilder for ModPulseSynthBuilder {
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls) {
        let initial_amp = params.get("amp").copied().unwrap_or(1.0);
        let mod_freq = params.get("mod_freq").copied().unwrap_or(5.0);

        let amp_shared = shared(initial_amp);
        let pitch_bend_shared = shared(1.0);
        let pressure_shared = shared(0.0);

        let left = (sine_hz(mod_freq) * freq * 0.1 + dc(freq)) >> square();
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
        SynthMetadata::new("mod_pulse", "Modulated pulse wave")
            .with_param("amp", 1.0, 0.0, 2.0)
            .with_param("mod_freq", 5.0, 0.1, 20.0)
            .with_tag("synth")
    }
}
