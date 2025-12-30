//! Synth preset system for saving and loading configurations
//!
//! Presets store synth parameters, envelope settings, and LFO configurations
//! in a serializable format.
//!
//! This module is only available with the `serde` feature enabled.

use crate::envelope::EnvelopeConfig;
use crate::lfo::LFOConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A complete synthesizer preset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SynthPreset {
    /// Unique ID for this preset
    pub id: Uuid,
    /// Preset name
    pub name: String,
    /// Synth type (e.g., "tb303", "fm", "saw")
    pub synth_type: String,
    /// Synth parameters (e.g., cutoff, resonance, ratio)
    pub parameters: HashMap<String, f32>,
    /// Envelope configuration (optional)
    pub envelope: Option<EnvelopeConfig>,
    /// LFO configuration (optional)
    pub lfo: Option<LFOConfig>,
    /// Author/creator of the preset
    pub author: Option<String>,
    /// Description or notes
    pub description: Option<String>,
    /// Tags for categorization (e.g., "bass", "lead", "pad")
    pub tags: Vec<String>,
}

impl SynthPreset {
    /// Create a new preset
    pub fn new(name: impl Into<String>, synth_type: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            synth_type: synth_type.into(),
            parameters: HashMap::new(),
            envelope: None,
            lfo: None,
            author: None,
            description: None,
            tags: Vec::new(),
        }
    }

    /// Set a parameter value
    pub fn with_parameter(mut self, name: impl Into<String>, value: f32) -> Self {
        self.parameters.insert(name.into(), value);
        self
    }

    /// Set multiple parameters
    pub fn with_parameters(mut self, params: HashMap<String, f32>) -> Self {
        self.parameters.extend(params);
        self
    }

    /// Set envelope configuration
    pub fn with_envelope(mut self, envelope: EnvelopeConfig) -> Self {
        self.envelope = Some(envelope);
        self
    }

    /// Set LFO configuration
    pub fn with_lfo(mut self, lfo: LFOConfig) -> Self {
        self.lfo = Some(lfo);
        self
    }

    /// Set author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add multiple tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }

    /// Serialize preset to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize preset from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Save preset to file
    #[cfg(feature = "std")]
    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let json = self.to_json().map_err(std::io::Error::other)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Load preset from file
    #[cfg(feature = "std")]
    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        use std::fs;

        let json = fs::read_to_string(path)?;
        Self::from_json(&json).map_err(std::io::Error::other)
    }
}

/// A collection of presets (preset bank)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetBank {
    /// Bank name
    pub name: String,
    /// Presets in this bank
    pub presets: Vec<SynthPreset>,
}

impl PresetBank {
    /// Create a new preset bank
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            presets: Vec::new(),
        }
    }

    /// Add a preset to the bank
    pub fn add_preset(&mut self, preset: SynthPreset) {
        self.presets.push(preset);
    }

    /// Get preset by ID
    pub fn get_by_id(&self, id: &Uuid) -> Option<&SynthPreset> {
        self.presets.iter().find(|p| p.id == *id)
    }

    /// Get preset by name
    pub fn get_by_name(&self, name: &str) -> Option<&SynthPreset> {
        self.presets.iter().find(|p| p.name == name)
    }

    /// Get all presets for a synth type
    pub fn get_by_synth_type(&self, synth_type: &str) -> Vec<&SynthPreset> {
        self.presets
            .iter()
            .filter(|p| p.synth_type == synth_type)
            .collect()
    }

    /// Get all presets with a specific tag
    pub fn get_by_tag(&self, tag: &str) -> Vec<&SynthPreset> {
        self.presets
            .iter()
            .filter(|p| p.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Serialize bank to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize bank from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Save bank to file
    #[cfg(feature = "std")]
    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let json = self.to_json().map_err(std::io::Error::other)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Load bank from file
    #[cfg(feature = "std")]
    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        use std::fs;

        let json = fs::read_to_string(path)?;
        Self::from_json(&json).map_err(std::io::Error::other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::ADSR;

    #[test]
    fn test_preset_creation() {
        let preset = SynthPreset::new("Bass 1", "tb303")
            .with_parameter("cutoff", 800.0)
            .with_parameter("res", 0.7)
            .with_envelope(EnvelopeConfig::ADSR(ADSR::bass()))
            .with_tag("bass")
            .with_tag("acid");

        assert_eq!(preset.name, "Bass 1");
        assert_eq!(preset.synth_type, "tb303");
        assert_eq!(preset.parameters.get("cutoff"), Some(&800.0));
        assert!(preset.envelope.is_some());
        assert_eq!(preset.tags.len(), 2);
    }

    #[test]
    fn test_preset_serialization() {
        let preset = SynthPreset::new("Test", "sine").with_parameter("amp", 0.5);

        let json = preset.to_json().unwrap();
        let deserialized = SynthPreset::from_json(&json).unwrap();

        assert_eq!(preset.name, deserialized.name);
        assert_eq!(preset.synth_type, deserialized.synth_type);
        assert_eq!(preset.parameters, deserialized.parameters);
    }

    #[test]
    fn test_preset_bank() {
        let mut bank = PresetBank::new("My Bank");

        let preset1 = SynthPreset::new("Lead 1", "saw");
        let preset2 = SynthPreset::new("Bass 1", "tb303");

        bank.add_preset(preset1.clone());
        bank.add_preset(preset2);

        assert_eq!(bank.presets.len(), 2);
        assert!(bank.get_by_name("Lead 1").is_some());
        assert!(bank.get_by_name("Nonexistent").is_none());
    }
}
