//! Effect plugin system
//!
//! This module provides a flexible plugin architecture for audio effects.
//! Each effect implements the EffectBuilder trait, allowing for easy registration
//! and extensibility without modifying core backend code.

use crate::error::Error;
pub use crate::params::ParameterDef;
use crate::Result;
use fundsp::hacker32::*;
use fundsp::shared::Shared;
use std::collections::HashMap;
use std::sync::Arc;

/// Trait for building custom effects
pub trait EffectBuilder: Send + Sync {
    /// Build the effect with given parameters
    /// Returns a stereo effect processor (2 in, 2 out) and controllable parameters
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls);

    /// Get effect metadata
    fn metadata(&self) -> EffectMetadata;
}

/// Controllable parameters for an effect instance
/// Uses Shared variables for real-time control
#[derive(Clone)]
pub struct EffectControls {
    /// Effect-specific parameters stored as Shared for real-time control
    pub params: HashMap<String, Shared>,
}

impl EffectControls {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Set a parameter value
    pub fn set(&self, name: &str, value: f32) {
        if let Some(shared) = self.params.get(name) {
            shared.set(value);
        }
    }

    /// Get a parameter value
    pub fn get(&self, name: &str) -> Option<f32> {
        self.params.get(name).map(|s| s.value())
    }
}

impl Default for EffectControls {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata about an effect
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EffectMetadata {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterDef>,
    /// Latency introduced by this effect (in samples)
    pub latency_samples: usize,
    /// Tags for categorization and source tracking
    /// Examples: "filter", "dynamics", "delay", "source:builtin", "source:vst3"
    #[cfg_attr(feature = "serde", serde(default))]
    pub tags: Vec<String>,
}

impl EffectMetadata {
    /// Create new metadata with no parameters and zero latency
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: vec![],
            latency_samples: 0,
            tags: vec![],
        }
    }

    /// Add a parameter definition
    pub fn with_param(mut self, name: impl Into<String>, default: f32, min: f32, max: f32) -> Self {
        self.parameters
            .push(ParameterDef::new(name, default, min, max));
        self
    }

    /// Set latency in samples
    pub fn with_latency(mut self, samples: usize) -> Self {
        self.latency_samples = samples;
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

    /// Check if this effect has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }
}

/// Parameter range
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParameterRange {
    pub min: f32,
    pub max: f32,
}

/// Effect registry for managing available effects
#[derive(Clone)]
pub struct EffectRegistry {
    builders: HashMap<String, Arc<dyn EffectBuilder>>,
}

impl EffectRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    /// Create a new registry with all built-in effects registered
    pub fn with_builtin() -> Self {
        let mut registry = Self::new();
        registry.register_builtin();
        registry
    }

    /// Register all built-in effects
    pub fn register_builtin(&mut self) {
        super::builtin::register_all(self);
    }

    /// Register an effect builder
    pub fn register(&mut self, name: impl Into<String>, builder: Arc<dyn EffectBuilder>) {
        self.builders.insert(name.into(), builder);
    }

    /// Get an effect builder by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn EffectBuilder>> {
        self.builders.get(name).cloned()
    }

    /// Create an effect instance
    pub fn create(
        &self,
        name: &str,
        params: &HashMap<String, f32>,
    ) -> Option<(Box<dyn AudioUnit>, EffectControls)> {
        self.get(name).map(|builder| builder.build(params))
    }

    /// Build an effect with error handling
    pub fn build(
        &self,
        name: &str,
        params: &HashMap<String, f32>,
    ) -> Result<(Box<dyn AudioUnit>, EffectControls)> {
        self.create(name, params)
            .ok_or_else(|| Error::InvalidEffect(name.to_string()))
    }

    /// Get metadata for an effect
    pub fn get_metadata(&self, name: &str) -> Option<EffectMetadata> {
        self.get(name).map(|builder| builder.metadata())
    }

    /// Check if an effect exists in the registry
    pub fn contains(&self, name: &str) -> bool {
        self.builders.contains_key(name)
    }

    /// List all registered effect names
    pub fn list_names(&self) -> Vec<String> {
        self.builders.keys().cloned().collect()
    }

    /// List all effect metadata
    pub fn list_effects(&self) -> Vec<EffectMetadata> {
        self.builders
            .values()
            .map(|builder| builder.metadata())
            .collect()
    }
}

impl Default for EffectRegistry {
    fn default() -> Self {
        Self::new()
    }
}
