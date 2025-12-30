//! Serialization support for saving and loading effect chains
//!
//! This module provides serialization for effect parameters and chain state.
//! Useful for presets, project files, or any application state persistence.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Serializable representation of an effect in a chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectState {
    /// Optional UUID for IR synchronization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,

    /// Effect name (must match registry)
    pub name: String,

    /// Effect parameters
    pub parameters: HashMap<String, f32>,

    /// Whether this effect is bypassed
    #[serde(default)]
    pub bypassed: bool,

    /// Whether this effect is muted
    #[serde(default)]
    pub muted: bool,
}

/// Serializable representation of an effect chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainState {
    /// Format version for future compatibility
    #[serde(default = "default_version")]
    pub version: u32,

    /// Sample rate when saved
    pub sample_rate: f64,

    /// Chain-level bypass
    #[serde(default)]
    pub bypassed: bool,

    /// Effects in order
    pub effects: Vec<EffectState>,
}

fn default_version() -> u32 {
    1
}

impl ChainState {
    /// Create a new empty chain state
    pub fn new(sample_rate: f64) -> Self {
        Self {
            version: default_version(),
            sample_rate,
            bypassed: false,
            effects: Vec::new(),
        }
    }

    /// Add an effect to the chain state
    pub fn add_effect(&mut self, effect: EffectState) {
        self.effects.push(effect);
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize to JSON bytes
    pub fn to_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec_pretty(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Deserialize from JSON bytes
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

impl EffectState {
    /// Create a new effect state
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            parameters: HashMap::new(),
            bypassed: false,
            muted: false,
        }
    }

    /// Create with an ID
    pub fn with_id(name: impl Into<String>, id: Uuid) -> Self {
        Self {
            id: Some(id),
            name: name.into(),
            parameters: HashMap::new(),
            bypassed: false,
            muted: false,
        }
    }

    /// Builder pattern: add a parameter
    pub fn with_param(mut self, name: impl Into<String>, value: f32) -> Self {
        self.parameters.insert(name.into(), value);
        self
    }

    /// Builder pattern: set bypassed
    pub fn with_bypass(mut self, bypassed: bool) -> Self {
        self.bypassed = bypassed;
        self
    }

    /// Builder pattern: set muted
    pub fn with_mute(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    /// Set a parameter value
    pub fn set_param(&mut self, name: impl Into<String>, value: f32) {
        self.parameters.insert(name.into(), value);
    }

    /// Get a parameter value
    pub fn get_param(&self, name: &str) -> Option<f32> {
        self.parameters.get(name).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_state_builder() {
        let effect = EffectState::new("reverb")
            .with_param("room", 0.8)
            .with_param("time", 2.0)
            .with_bypass(false);

        assert_eq!(effect.name, "reverb");
        assert_eq!(effect.get_param("room"), Some(0.8));
        assert_eq!(effect.get_param("time"), Some(2.0));
        assert!(!effect.bypassed);
    }

    #[test]
    fn test_chain_serialization() {
        let mut chain = ChainState::new(48000.0);

        chain.add_effect(
            EffectState::new("lpf")
                .with_param("cutoff", 1000.0)
                .with_param("res", 0.7),
        );

        chain.add_effect(EffectState::new("reverb").with_param("room", 0.9));

        // Serialize
        let json = chain.to_json().unwrap();

        // Deserialize
        let loaded = ChainState::from_json(&json).unwrap();

        assert_eq!(loaded.sample_rate, 48000.0);
        assert_eq!(loaded.effects.len(), 2);
        assert_eq!(loaded.effects[0].name, "lpf");
        assert_eq!(loaded.effects[0].get_param("cutoff"), Some(1000.0));
    }

    #[test]
    fn test_json_roundtrip() {
        let id = Uuid::new_v4();
        let effect = EffectState::with_id("distortion", id)
            .with_param("amount", 0.7)
            .with_bypass(true);

        let mut chain = ChainState::new(44100.0);
        chain.add_effect(effect.clone());

        let json = chain.to_json().unwrap();
        let loaded = ChainState::from_json(&json).unwrap();

        assert_eq!(loaded.effects[0].id, effect.id);
        assert_eq!(loaded.effects[0].name, effect.name);
        assert_eq!(loaded.effects[0].bypassed, effect.bypassed);
    }
}
