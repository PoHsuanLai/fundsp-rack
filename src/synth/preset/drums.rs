//! Built-in drum presets
//!
//! Drums are preset configurations of existing synths with specific
//! envelope and parameter settings.

use super::{PresetBank, SynthPreset};
use crate::synth::envelope::{EnvelopeConfig, ADSR};

/// Factory functions for creating drum presets
pub struct DrumPresets;

impl DrumPresets {
    /// Kick drum - pitch-dropping sine wave
    pub fn kick() -> SynthPreset {
        SynthPreset::new("kick", "sine")
            .with_description("Bass drum with pitch drop")
            .with_tag("drum")
            .with_parameter("amp", 0.8)
            .with_parameter("freq", 60.0)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::new(0.001, 0.15, 0.0, 0.1)))
    }

    /// Snare drum - noise + tone
    pub fn snare() -> SynthPreset {
        SynthPreset::new("snare", "noise")
            .with_description("Snare with noise and tone")
            .with_tag("drum")
            .with_parameter("amp", 0.7)
            .with_parameter("tone_mix", 0.3)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::new(0.001, 0.1, 0.0, 0.1)))
    }

    /// Closed hi-hat - short filtered noise
    pub fn hihat() -> SynthPreset {
        SynthPreset::new("hihat", "noise")
            .with_description("Closed hi-hat")
            .with_tag("drum")
            .with_parameter("amp", 0.4)
            .with_parameter("highpass", 8000.0)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::new(0.001, 0.025, 0.0, 0.025)))
    }

    /// Open hi-hat - longer filtered noise
    pub fn open_hihat() -> SynthPreset {
        SynthPreset::new("open_hihat", "noise")
            .with_description("Open hi-hat")
            .with_tag("drum")
            .with_parameter("amp", 0.4)
            .with_parameter("highpass", 8000.0)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::new(0.001, 0.15, 0.0, 0.15)))
    }

    /// Clap - noise bursts
    pub fn clap() -> SynthPreset {
        SynthPreset::new("clap", "noise")
            .with_description("Hand clap")
            .with_tag("drum")
            .with_parameter("amp", 0.6)
            .with_parameter("bandpass", 1000.0)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::new(0.001, 0.075, 0.0, 0.075)))
    }

    /// Tom - pitch-dropping sine
    pub fn tom() -> SynthPreset {
        SynthPreset::new("tom", "sine")
            .with_description("Tom drum")
            .with_tag("drum")
            .with_parameter("amp", 0.7)
            .with_parameter("freq", 100.0)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::new(0.001, 0.125, 0.0, 0.125)))
    }

    /// Crash cymbal - long noise decay
    pub fn crash() -> SynthPreset {
        SynthPreset::new("crash", "noise")
            .with_description("Crash cymbal")
            .with_tag("drum")
            .with_parameter("amp", 0.5)
            .with_parameter("highpass", 4000.0)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::new(0.001, 0.5, 0.0, 0.5)))
    }

    /// Ride cymbal - metallic tone + noise
    pub fn ride() -> SynthPreset {
        SynthPreset::new("ride", "fm")
            .with_description("Ride cymbal")
            .with_tag("drum")
            .with_parameter("amp", 0.5)
            .with_parameter("ratio", 2.5)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::new(0.001, 0.4, 0.0, 0.4)))
    }

    /// Cowbell - metallic tone
    pub fn cowbell() -> SynthPreset {
        SynthPreset::new("cowbell", "fm")
            .with_description("Cowbell")
            .with_tag("drum")
            .with_parameter("amp", 0.5)
            .with_parameter("freq", 560.0)
            .with_parameter("ratio", 1.5)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::new(0.001, 0.2, 0.0, 0.2)))
    }

    /// Get all built-in drum presets
    pub fn all() -> Vec<SynthPreset> {
        vec![
            Self::kick(),
            Self::snare(),
            Self::hihat(),
            Self::open_hihat(),
            Self::clap(),
            Self::tom(),
            Self::crash(),
            Self::ride(),
            Self::cowbell(),
        ]
    }
}

/// Extension trait for PresetBank to add drum presets
pub trait PresetBankDrumsExt {
    /// Add all built-in drum presets to this bank
    fn with_builtin_drums(self) -> Self;
}

impl PresetBankDrumsExt for PresetBank {
    fn with_builtin_drums(mut self) -> Self {
        for preset in DrumPresets::all() {
            self.add_preset(preset);
        }
        self
    }
}

/// Create a preset bank with all drum presets
pub fn drum_bank() -> PresetBank {
    PresetBank::new("Drums").with_builtin_drums()
}

/// Parse a drum token to get the corresponding preset
pub fn preset_for_token(token: &str) -> Option<SynthPreset> {
    match token.to_lowercase().as_str() {
        "kick" | "bd" | "bass" => Some(DrumPresets::kick()),
        "snare" | "sd" => Some(DrumPresets::snare()),
        "hihat" | "hh" | "ch" => Some(DrumPresets::hihat()),
        "open_hihat" | "oh" => Some(DrumPresets::open_hihat()),
        "clap" | "cp" => Some(DrumPresets::clap()),
        "tom" | "tom1" | "tom2" | "tom3" | "lt" | "mt" | "ht" => Some(DrumPresets::tom()),
        "crash" | "cr" | "cy" | "cymbal" => Some(DrumPresets::crash()),
        "ride" | "rd" => Some(DrumPresets::ride()),
        "cowbell" | "cb" => Some(DrumPresets::cowbell()),
        _ => None,
    }
}

/// Get the General MIDI drum note for a token
///
/// Returns a GM drum note number for use in real-time audio.
/// This function is const-friendly and suitable for audio callbacks.
pub fn midi_note_for_token(token: &str) -> Option<u8> {
    // General MIDI drum map notes
    match token {
        // Kick drums
        "bd" | "kick" | "bass" => Some(36), // Bass Drum 1 (C1)
        // Snare drums
        "sd" | "snare" | "sn" => Some(38), // Acoustic Snare (D1)
        "rs" | "rim" | "rimshot" => Some(37), // Side Stick (C#1)
        // Hi-hats
        "hh" | "hihat" | "hat" | "ch" => Some(42), // Closed Hi-Hat (F#1)
        "oh" | "open" | "openhat" | "open_hihat" => Some(46), // Open Hi-Hat (A#1)
        "ph" | "pedal" => Some(44), // Pedal Hi-Hat (G#1)
        // Claps and snaps
        "cp" | "clap" | "handclap" => Some(39), // Hand Clap (D#1)
        // Toms
        "lt" | "lowtom" | "tom" | "tom1" => Some(45), // Low Tom (A1)
        "mt" | "midtom" | "tom2" => Some(47), // Low-Mid Tom (B1)
        "ht" | "hightom" | "tom3" => Some(50), // High Tom (D2)
        // Cymbals
        "cr" | "crash" | "cy" | "cymbal" => Some(49), // Crash Cymbal 1 (C#2)
        "rd" | "ride" => Some(51), // Ride Cymbal 1 (D#2)
        // Other percussion
        "cb" | "cowbell" => Some(56), // Cowbell (G#2)
        "cl" | "claves" => Some(75), // Claves (D#4)
        "ma" | "maracas" => Some(70), // Maracas (A#3)
        "ta" | "tambourine" => Some(54), // Tambourine (F#2)
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drum_presets() {
        let kick = DrumPresets::kick();
        assert_eq!(kick.name, "kick");
        assert!(kick.tags.contains(&"drum".to_string()));
        assert!(kick.envelope.is_some());
    }

    #[test]
    fn test_all_drums() {
        let all = DrumPresets::all();
        assert_eq!(all.len(), 9);
    }

    #[test]
    fn test_drum_bank() {
        let bank = drum_bank();
        assert_eq!(bank.name, "Drums");
        assert!(bank.get_by_name("kick").is_some());
        assert!(bank.get_by_name("snare").is_some());
    }

    #[test]
    fn test_preset_for_token() {
        assert!(preset_for_token("bd").is_some());
        assert!(preset_for_token("kick").is_some());
        assert!(preset_for_token("hh").is_some());
        assert!(preset_for_token("unknown").is_none());
    }

    #[test]
    fn test_preset_bank_ext() {
        let bank = PresetBank::new("My Drums").with_builtin_drums();
        assert!(bank.get_by_tag("drum").len() >= 9);
    }
}
