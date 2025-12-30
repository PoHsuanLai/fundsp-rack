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

/// Effect category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectCategory {
    /// Time-based effects (reverb, delay, echo)
    Time,
    /// Modulation effects (chorus, flanger, phaser)
    Modulation,
    /// Filter effects (lowpass, highpass, bandpass)
    Filter,
    /// Dynamics effects (compressor, limiter, gate)
    Dynamics,
    /// Distortion effects (overdrive, fuzz, bitcrusher)
    Distortion,
    /// Spatial effects (panning, stereo width)
    Spatial,
    /// Other/utility effects
    Other,
}

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
#[derive(Debug, Clone)]
pub struct EffectMetadata {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterDef>,
    pub category: EffectCategory,
    /// Latency introduced by this effect (in samples)
    pub latency_samples: usize,
}

impl EffectMetadata {
    /// Create new metadata with no parameters and zero latency
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        category: EffectCategory,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: vec![],
            category,
            latency_samples: 0,
        }
    }

    /// Add a parameter definition
    pub fn with_param(mut self, name: impl Into<String>, default: f32, min: f32, max: f32) -> Self {
        self.parameters.push(ParameterDef::new(name, default, min, max));
        self
    }

    /// Set latency in samples
    pub fn with_latency(mut self, samples: usize) -> Self {
        self.latency_samples = samples;
        self
    }
}

/// Parameter range
#[derive(Debug, Clone, Copy)]
pub struct ParameterRange {
    pub min: f32,
    pub max: f32,
}

/// Effect registry for managing available effects
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

    /// List effects by category
    pub fn list_by_category(&self, category: EffectCategory) -> Vec<EffectMetadata> {
        self.list_effects()
            .into_iter()
            .filter(|meta| meta.category == category)
            .collect()
    }
}

impl Default for EffectRegistry {
    fn default() -> Self {
        Self::new()
    }
}
