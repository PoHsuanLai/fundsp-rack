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
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct SynthMetadata {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterDef>,
    /// Tags for categorization and source tracking
    /// Examples: "bass", "lead", "pad", "source:builtin", "source:vst3"
    #[cfg_attr(feature = "serde", serde(default))]
    pub tags: Vec<String>,
}

impl SynthMetadata {
    /// Create new metadata with no parameters
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: vec![],
            tags: vec![],
        }
    }

    /// Add a parameter definition
    pub fn with_param(mut self, name: impl Into<String>, default: f32, min: f32, max: f32) -> Self {
        self.parameters
            .push(ParameterDef::new(name, default, min, max));
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add multiple tags
    pub fn with_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.into()));
        self
    }

    /// Check if this synth has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }
}

/// Registry for all available synths
#[derive(Clone)]
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

    /// Find synths by tag
    ///
    /// Returns a list of synth names that have the specified tag.
    pub fn find_by_tag(&self, tag: &str) -> Vec<String> {
        self.builders
            .iter()
            .filter(|(_, builder)| builder.metadata().has_tag(tag))
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Get the first synth that matches a tag, or a fallback if none found
    ///
    /// Useful for MIDI program change where we need a single synth for a category.
    pub fn first_with_tag(&self, tag: &str) -> Option<String> {
        self.find_by_tag(tag).into_iter().next()
    }

    /// Get a synth for a General MIDI program number (0-127)
    ///
    /// Priority order:
    /// 1. SoundFont-based synth (sf_*) if available - provides realistic samples
    /// 2. Tag-based lookup for synthesis fallback
    /// 3. Ultimate fallback to "sine"
    pub fn synth_for_gm_program(&self, program: u8) -> String {
        // First, try to find a SoundFont-based synth for this program
        // SoundFont synths are registered with "sf_" prefix and have the "soundfont" tag
        let sf_synths = self.find_by_tag("soundfont");
        if !sf_synths.is_empty() {
            // SoundFont synths are available - find the one for this program
            // They're registered as sf_{program_name} with program-specific metadata
            for synth_name in &sf_synths {
                if let Some(builder) = self.get(synth_name) {
                    let meta = builder.metadata();
                    // Check if the description contains this program number
                    if meta.description.contains(&format!("GM Program {}", program)) {
                        return synth_name.clone();
                    }
                }
            }
        }

        // Fallback to tag-based synthesis
        let (primary_tag, fallback_tags): (&str, &[&str]) = match program {
            // Piano (1-8)
            0..=7 => ("piano", &["keys", "synth"]),
            // Chromatic Percussion (9-16)
            8..=15 => ("bell", &["keys", "synth"]),
            // Organ (17-24)
            16..=23 => ("organ", &["keys", "synth"]),
            // Guitar (25-32)
            24..=31 => ("pluck", &["synth"]),
            // Bass (33-40)
            32..=39 => ("bass", &["sub", "synth"]),
            // Strings (41-48)
            40..=47 => ("strings", &["pad", "synth"]),
            // Ensemble (49-56)
            48..=55 => ("pad", &["strings", "synth"]),
            // Brass (56-64)
            56..=63 => ("brass", &["lead", "synth"]),
            // Reed (65-72)
            64..=71 => ("lead", &["synth"]), // No reed tag, use lead
            // Pipe (73-80)
            72..=79 => ("synth", &[]), // No specific pipe, use basic synth
            // Synth Lead (81-88)
            80..=87 => ("lead", &["synth"]),
            // Synth Pad (89-96)
            88..=95 => ("pad", &["ambient", "strings"]),
            // Synth Effects (97-104)
            96..=103 => ("fm", &["synth"]),
            // Ethnic (105-112)
            104..=111 => ("pluck", &["synth"]),
            // Percussive (113-120)
            112..=119 => ("drum", &["noise", "synth"]),
            // Sound Effects (121-128)
            120..=127 => ("noise", &["synth"]),
            _ => ("synth", &[]),
        };

        // Try primary tag first (excluding soundfont synths for cleaner fallback)
        for synth_name in self.find_by_tag(primary_tag) {
            if !synth_name.starts_with("sf_") {
                return synth_name;
            }
        }

        // Try fallback tags
        for tag in fallback_tags {
            for synth_name in self.find_by_tag(tag) {
                if !synth_name.starts_with("sf_") {
                    return synth_name;
                }
            }
        }

        // Ultimate fallback
        "sine".to_string()
    }

    /// Check if SoundFont synths are registered
    pub fn has_soundfont_synths(&self) -> bool {
        !self.find_by_tag("soundfont").is_empty()
    }
}

impl Default for SynthRegistry {
    fn default() -> Self {
        Self::new()
    }
}
