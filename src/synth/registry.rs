//! Synth plugin system
//!
//! This module provides a flexible plugin architecture for synthesizers.
//! Each synth implements the SynthBuilder trait, allowing for easy registration
//! and extensibility without modifying core backend code.

use crate::params::ParameterDef;
use crate::Result;
use fundsp::hacker32::*;
use fundsp::shared::Shared;
use std::collections::HashMap;
use std::sync::Arc;

use super::synths::*;

/// Trait for building custom synths
pub trait SynthBuilder: Send + Sync {
    /// Build the synth with given parameters
    fn build(
        &self,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> (Box<dyn AudioUnit>, VoiceControls);

    /// Get synth metadata
    fn metadata(&self) -> SynthMetadata;
}

/// Voice controls for dynamic parameter changes
/// These shared variables allow real-time parameter control during playback
#[derive(Clone)]
pub struct VoiceControls {
    /// Amplitude control (0.0 to 1.0+)
    pub amp: Shared,
    /// Filter cutoff frequency (Hz) - if applicable
    pub cutoff: Option<Shared>,
    /// Filter resonance (0.0 to 1.0) - if applicable
    pub resonance: Option<Shared>,
    /// Pitch bend multiplier (1.0 = no bend, 2.0 = up one octave, 0.5 = down one octave)
    pub pitch_bend: Shared,
    /// Aftertouch/pressure (0.0 to 1.0, normalized from MIDI 0-127)
    pub pressure: Shared,
}

/// Metadata about a synth
#[derive(Debug, Clone)]
pub struct SynthMetadata {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterDef>,
    pub category: SynthCategory,
}

impl SynthMetadata {
    /// Create new metadata with no parameters
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        category: SynthCategory,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: vec![],
            category,
        }
    }

    /// Add a parameter definition
    pub fn with_param(mut self, name: impl Into<String>, default: f32, min: f32, max: f32) -> Self {
        self.parameters.push(ParameterDef::new(name, default, min, max));
        self
    }
}

/// Synth categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthCategory {
    /// Basic waveforms (sine, saw, square)
    Basic,
    /// Analog-style synths (tb303, prophet, juno)
    Analog,
    /// Digital synths (fm, wavetable)
    Digital,
    /// Physical modeling (pluck, piano, organ)
    Physical,
    /// Noise generators
    Noise,
}

/// Registry for all available synths
pub struct SynthRegistry {
    builders: HashMap<String, Arc<dyn SynthBuilder>>,
}

impl SynthRegistry {
    /// Create a new empty synth registry
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    /// Create a new registry with all built-in synths registered
    pub fn with_builtin() -> Self {
        let mut registry = Self::new();
        registry.register_builtin();
        registry
    }

    /// Register all built-in synths
    pub fn register_builtin(&mut self) {
        // Basic waveforms
        self.register("sine", Arc::new(SineSynthBuilder));
        self.register("beep", Arc::new(SineSynthBuilder)); // alias
        self.register("saw", Arc::new(SawSynthBuilder));
        self.register("square", Arc::new(SquareSynthBuilder));
        self.register("tri", Arc::new(TriangleSynthBuilder));
        self.register("triangle", Arc::new(TriangleSynthBuilder)); // alias
        self.register("pulse", Arc::new(PulseSynthBuilder));

        // Analog synths
        self.register("tb303", Arc::new(TB303SynthBuilder));
        self.register("acid", Arc::new(TB303SynthBuilder)); // alias
        self.register("prophet", Arc::new(ProphetSynthBuilder));
        self.register("supersaw", Arc::new(SupersawSynthBuilder));
        self.register("hoover", Arc::new(HooverSynthBuilder));

        // Modulated oscillators
        self.register("mod_saw", Arc::new(ModSawSynthBuilder));
        self.register("mod_sine", Arc::new(ModSineSynthBuilder));
        self.register("mod_tri", Arc::new(ModTriSynthBuilder));
        self.register("mod_pulse", Arc::new(ModPulseSynthBuilder));

        // Detuned oscillators
        self.register("dsaw", Arc::new(DSawSynthBuilder));
        self.register("dpulse", Arc::new(DPulseSynthBuilder));
        self.register("dtri", Arc::new(DTriSynthBuilder));

        // FM synthesis
        self.register("fm", Arc::new(FMSynthBuilder));

        // Bells
        self.register("pretty_bell", Arc::new(PrettyBellSynthBuilder));
        self.register("dull_bell", Arc::new(DullBellSynthBuilder));

        // Physical modeling
        self.register("piano", Arc::new(PianoSynthBuilder));
        self.register("pluck", Arc::new(PluckSynthBuilder));

        // Keyboard instruments
        self.register("organ", Arc::new(OrganSynthBuilder));
        self.register("hammond", Arc::new(OrganSynthBuilder)); // alias
        self.register("electric_piano", Arc::new(ElectricPianoSynthBuilder));
        self.register("rhodes", Arc::new(ElectricPianoSynthBuilder)); // alias
        self.register("ep", Arc::new(ElectricPianoSynthBuilder)); // alias

        // Lead synths
        self.register("lead", Arc::new(LeadSynthBuilder));
        self.register("mono_lead", Arc::new(LeadSynthBuilder)); // alias
        self.register("sub", Arc::new(SubSynthBuilder));
        self.register("subbass", Arc::new(SubSynthBuilder)); // alias
        self.register("brass", Arc::new(BrassSynthBuilder));

        // Pad synths
        self.register("strings", Arc::new(StringsSynthBuilder));
        self.register("string_ensemble", Arc::new(StringsSynthBuilder)); // alias
        self.register("pad", Arc::new(PadSynthBuilder));
        self.register("warm_pad", Arc::new(PadSynthBuilder)); // alias

        // Noise
        self.register("noise", Arc::new(NoiseSynthBuilder));

        // Bass synths
        self.register("bass_foundation", Arc::new(BassFoundationSynthBuilder));
        self.register("bass_highend", Arc::new(BassHighendSynthBuilder));

        // Ambient/pad synths
        self.register("dark_ambience", Arc::new(DarkAmbienceSynthBuilder));
        self.register("growl", Arc::new(GrowlSynthBuilder));

        // Tech synths
        self.register("tech_saws", Arc::new(TechSawsSynthBuilder));
        self.register("blade", Arc::new(BladeSynthBuilder));
        self.register("zawa", Arc::new(ZawaSynthBuilder));

        // Additional synths
        self.register("subpulse", Arc::new(SubpulseSynthBuilder));
        self.register("hollow", Arc::new(HollowSynthBuilder));
    }

    /// Register a custom synth
    pub fn register(&mut self, name: &str, builder: Arc<dyn SynthBuilder>) {
        self.builders.insert(name.to_string(), builder);
    }

    /// Build a synth by name
    pub fn build(
        &self,
        name: &str,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> Result<(Box<dyn AudioUnit>, VoiceControls)> {
        let builder = self
            .builders
            .get(name)
            .ok_or_else(|| crate::error::Error::InvalidSynth(name.to_string()))?;
        Ok(builder.build(freq, params))
    }

    /// Create a synth by name (alias for build)
    pub fn create(
        &self,
        name: &str,
        freq: f32,
        params: &HashMap<String, f32>,
    ) -> Result<(Box<dyn AudioUnit>, VoiceControls)> {
        self.build(name, freq, params)
    }

    /// Get a synth builder by name
    pub fn get(&self, name: &str) -> Option<&Arc<dyn SynthBuilder>> {
        self.builders.get(name)
    }

    /// List all available synths
    pub fn list_synths(&self) -> Vec<SynthMetadata> {
        self.builders.values().map(|b| b.metadata()).collect()
    }

    /// Check if a synth exists
    pub fn contains(&self, name: &str) -> bool {
        self.builders.contains_key(name)
    }
}

impl Default for SynthRegistry {
    fn default() -> Self {
        Self::new()
    }
}
