//! Synthesizer module
//!
//! This module provides:
//! - **Synth plugin trait** - `SynthBuilder` for creating custom synths
//! - **Synth registry** - Register and manage synths by name
//! - **30+ built-in synths** - Sine, saw, square, FM, organ, pad, strings, and more
//! - **Real-time parameter control** - Lock-free parameter updates via `fundsp::shared::Shared`
//! - **Voice controls** - Amplitude, pitch bend, cutoff, resonance, and pressure
//! - **Polyphony** - Easy voice management for chords

pub mod builder;
pub mod envelope;
pub mod lfo;
pub mod poly;
#[cfg(feature = "serde")]
pub mod preset;
pub mod registry;
pub mod synths;

pub use builder::{Synth, SynthBuilder as FluentSynthBuilder, SynthRegistryExt};
pub use envelope::{EnvelopeConfig, ADSR, AHD, AR};
pub use lfo::{LFOConfig, LFOTarget, LFOWaveform};
pub use poly::{midi_to_freq, PolySynth, PolySynthBuilder, SynthRegistryPolyExt};
#[cfg(feature = "serde")]
pub use preset::{
    drum_bank, midi_note_for_token, preset_for_token, DrumPresets, PresetBank, PresetBankDrumsExt, SynthPreset,
};
pub use registry::{SynthBuilder, SynthMetadata, SynthRegistry, VoiceControls};

// Re-export UUID for synth instance tracking (only with serde feature)
#[cfg(feature = "serde")]
pub use uuid::Uuid;

/// Unique identifier for a synth instance (only with serde feature)
#[cfg(feature = "serde")]
pub type SynthId = Uuid;
