//! Effect preset system for saving and loading configurations
//!
//! Presets store effect chain configurations with metadata for organization.
//!
//! This module is only available with the `serde` feature enabled.
//!
//! ## Submodules
//!
//! - [`mastering`] - Built-in mastering chain presets
//! - [`mixing`] - Built-in mixing effect presets

pub mod mastering;
pub mod mixing;

use crate::effects::serialize::EffectState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub use mastering::{mastering_bank, MasteringPresets, PresetBankMasteringExt};
pub use mixing::{mixing_bank, MixingPresets, PresetBankMixingExt};

/// A complete effect chain preset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EffectPreset {
    /// Unique ID for this preset
    pub id: Uuid,
    /// Preset name
    pub name: String,
    /// Effect chain (ordered list of effects with parameters)
    pub effects: Vec<EffectState>,
    /// Author/creator of the preset
    pub author: Option<String>,
    /// Description or notes
    pub description: Option<String>,
    /// Tags for categorization (e.g., "mastering", "vocal", "guitar")
    pub tags: Vec<String>,
    /// Target sample rate (optional, for sample-rate dependent effects)
    pub sample_rate: Option<f64>,
}

impl EffectPreset {
    /// Create a new preset
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            effects: Vec::new(),
            author: None,
            description: None,
            tags: Vec::new(),
            sample_rate: None,
        }
    }

    /// Add an effect to the chain
    pub fn with_effect(mut self, effect: EffectState) -> Self {
        self.effects.push(effect);
        self
    }

    /// Add an effect by name with parameters
    pub fn with_effect_params(
        mut self,
        name: impl Into<String>,
        params: HashMap<String, f32>,
    ) -> Self {
        let mut effect = EffectState::new(name);
        effect.parameters = params;
        self.effects.push(effect);
        self
    }

    /// Convenience: add a simple effect with no params
    pub fn with_simple_effect(self, name: impl Into<String>) -> Self {
        self.with_effect(EffectState::new(name))
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

    /// Set target sample rate
    pub fn with_sample_rate(mut self, sample_rate: f64) -> Self {
        self.sample_rate = Some(sample_rate);
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
    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let json = self.to_json().map_err(std::io::Error::other)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Load preset from file
    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        use std::fs;

        let json = fs::read_to_string(path)?;
        Self::from_json(&json).map_err(std::io::Error::other)
    }
}

/// A collection of effect presets (preset bank)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectPresetBank {
    /// Bank name
    pub name: String,
    /// Presets in this bank
    pub presets: Vec<EffectPreset>,
}

impl EffectPresetBank {
    /// Create a new preset bank
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            presets: Vec::new(),
        }
    }

    /// Add a preset to the bank
    pub fn add_preset(&mut self, preset: EffectPreset) {
        self.presets.push(preset);
    }

    /// Get preset by ID
    pub fn get_by_id(&self, id: &Uuid) -> Option<&EffectPreset> {
        self.presets.iter().find(|p| p.id == *id)
    }

    /// Get preset by name
    pub fn get_by_name(&self, name: &str) -> Option<&EffectPreset> {
        self.presets.iter().find(|p| p.name == name)
    }

    /// Get all presets with a specific tag
    pub fn get_by_tag(&self, tag: &str) -> Vec<&EffectPreset> {
        self.presets
            .iter()
            .filter(|p| p.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Get all presets containing a specific effect
    pub fn get_by_effect(&self, effect_name: &str) -> Vec<&EffectPreset> {
        self.presets
            .iter()
            .filter(|p| p.effects.iter().any(|e| e.name == effect_name))
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
    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let json = self.to_json().map_err(std::io::Error::other)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Load bank from file
    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        use std::fs;

        let json = fs::read_to_string(path)?;
        Self::from_json(&json).map_err(std::io::Error::other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_creation() {
        let preset = EffectPreset::new("Warm Vocal")
            .with_effect(EffectState::new("eq_3band").with_param("low", 2.0))
            .with_effect(EffectState::new("compressor").with_param("ratio", 4.0))
            .with_effect(EffectState::new("reverb").with_param("room", 0.3))
            .with_tag("vocal")
            .with_tag("warm");

        assert_eq!(preset.name, "Warm Vocal");
        assert_eq!(preset.effects.len(), 3);
        assert_eq!(preset.tags.len(), 2);
    }

    #[test]
    fn test_preset_serialization() {
        let preset = EffectPreset::new("Test")
            .with_effect(EffectState::new("lpf").with_param("cutoff", 1000.0));

        let json = preset.to_json().unwrap();
        let deserialized = EffectPreset::from_json(&json).unwrap();

        assert_eq!(preset.name, deserialized.name);
        assert_eq!(preset.effects.len(), deserialized.effects.len());
    }

    #[test]
    fn test_preset_bank() {
        let mut bank = EffectPresetBank::new("My Effects");

        let preset1 = EffectPreset::new("Preset 1").with_tag("mastering");
        let preset2 = EffectPreset::new("Preset 2").with_tag("vocal");

        bank.add_preset(preset1.clone());
        bank.add_preset(preset2);

        assert_eq!(bank.presets.len(), 2);
        assert!(bank.get_by_name("Preset 1").is_some());
        assert!(bank.get_by_name("Nonexistent").is_none());
        assert_eq!(bank.get_by_tag("mastering").len(), 1);
    }
}
