//! Fluent builder API for creating effects
//!
//! This module provides a chainable, ergonomic API for creating and configuring effects.
//!
//! # Examples
//!
//! ```
//! use fundsp_rack::prelude::*;
//!
//! // Simple effect creation
//! let (effect, controls) = Effect::new("lpf")
//!     .param("cutoff", 2000.0)
//!     .param("res", 0.7)
//!     .build()
//!     .unwrap();
//!
//! // Using registry with fluent API
//! let registry = EffectRegistry::with_builtin();
//!
//! let (effect, controls) = registry.effect("compressor")
//!     .param("threshold", -20.0)
//!     .param("ratio", 4.0)
//!     .build()
//!     .unwrap();
//! ```

use super::registry::{EffectControls, EffectRegistry};
use crate::Result;
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Fluent builder for creating effects with a registry reference
pub struct EffectBuilder<'a> {
    registry: &'a EffectRegistry,
    effect_name: String,
    parameters: HashMap<String, f32>,
}

impl<'a> EffectBuilder<'a> {
    /// Create a new effect builder
    pub fn new(registry: &'a EffectRegistry, effect_name: impl Into<String>) -> Self {
        Self {
            registry,
            effect_name: effect_name.into(),
            parameters: HashMap::new(),
        }
    }

    /// Set an effect parameter
    pub fn param(mut self, name: impl Into<String>, value: f32) -> Self {
        self.parameters.insert(name.into(), value);
        self
    }

    /// Set multiple parameters at once
    pub fn params(mut self, params: HashMap<String, f32>) -> Self {
        self.parameters.extend(params);
        self
    }

    /// Set filter cutoff frequency
    pub fn cutoff(self, cutoff: f32) -> Self {
        self.param("cutoff", cutoff)
    }

    /// Set filter resonance
    pub fn resonance(self, resonance: f32) -> Self {
        self.param("res", resonance)
    }

    /// Set filter resonance (alias)
    pub fn res(self, resonance: f32) -> Self {
        self.resonance(resonance)
    }

    /// Set mix/wet amount (0.0 = dry, 1.0 = wet)
    pub fn mix(self, mix: f32) -> Self {
        self.param("mix", mix)
    }

    /// Set drive/gain amount
    pub fn drive(self, drive: f32) -> Self {
        self.param("drive", drive)
    }

    /// Set delay time
    pub fn delay_time(self, time: f32) -> Self {
        self.param("time", time)
    }

    /// Set feedback amount
    pub fn feedback(self, feedback: f32) -> Self {
        self.param("feedback", feedback)
    }

    /// Set threshold (for dynamics)
    pub fn threshold(self, threshold: f32) -> Self {
        self.param("threshold", threshold)
    }

    /// Set ratio (for compressor/expander)
    pub fn ratio(self, ratio: f32) -> Self {
        self.param("ratio", ratio)
    }

    /// Set attack time
    pub fn attack(self, attack: f32) -> Self {
        self.param("attack", attack)
    }

    /// Set release time
    pub fn release(self, release: f32) -> Self {
        self.param("release", release)
    }

    /// Set rate (for modulation effects)
    pub fn rate(self, rate: f32) -> Self {
        self.param("rate", rate)
    }

    /// Set depth (for modulation effects)
    pub fn depth(self, depth: f32) -> Self {
        self.param("depth", depth)
    }

    /// Build the effect
    pub fn build(self) -> Result<(Box<dyn AudioUnit>, EffectControls)> {
        self.registry.build(&self.effect_name, &self.parameters)
    }
}

/// Standalone effect builder (doesn't require a registry reference)
pub struct Effect {
    effect_name: String,
    parameters: HashMap<String, f32>,
}

impl Effect {
    /// Create a new effect builder
    pub fn new(effect_name: impl Into<String>) -> Self {
        Self {
            effect_name: effect_name.into(),
            parameters: HashMap::new(),
        }
    }

    /// Set an effect parameter
    pub fn param(mut self, name: impl Into<String>, value: f32) -> Self {
        self.parameters.insert(name.into(), value);
        self
    }

    /// Set multiple parameters at once
    pub fn params(mut self, params: HashMap<String, f32>) -> Self {
        self.parameters.extend(params);
        self
    }

    /// Set filter cutoff frequency
    pub fn cutoff(self, cutoff: f32) -> Self {
        self.param("cutoff", cutoff)
    }

    /// Set filter resonance
    pub fn resonance(self, resonance: f32) -> Self {
        self.param("res", resonance)
    }

    /// Set filter resonance (alias)
    pub fn res(self, resonance: f32) -> Self {
        self.resonance(resonance)
    }

    /// Set mix/wet amount (0.0 = dry, 1.0 = wet)
    pub fn mix(self, mix: f32) -> Self {
        self.param("mix", mix)
    }

    /// Set drive/gain amount
    pub fn drive(self, drive: f32) -> Self {
        self.param("drive", drive)
    }

    /// Set delay time
    pub fn delay_time(self, time: f32) -> Self {
        self.param("time", time)
    }

    /// Set feedback amount
    pub fn feedback(self, feedback: f32) -> Self {
        self.param("feedback", feedback)
    }

    /// Set threshold (for dynamics)
    pub fn threshold(self, threshold: f32) -> Self {
        self.param("threshold", threshold)
    }

    /// Set ratio (for compressor/expander)
    pub fn ratio(self, ratio: f32) -> Self {
        self.param("ratio", ratio)
    }

    /// Set attack time
    pub fn attack(self, attack: f32) -> Self {
        self.param("attack", attack)
    }

    /// Set release time
    pub fn release(self, release: f32) -> Self {
        self.param("release", release)
    }

    /// Set rate (for modulation effects)
    pub fn rate(self, rate: f32) -> Self {
        self.param("rate", rate)
    }

    /// Set depth (for modulation effects)
    pub fn depth(self, depth: f32) -> Self {
        self.param("depth", depth)
    }

    /// Build the effect using the default registry
    pub fn build(self) -> Result<(Box<dyn AudioUnit>, EffectControls)> {
        let registry = EffectRegistry::with_builtin();
        registry.build(&self.effect_name, &self.parameters)
    }

    /// Build the effect using a custom registry
    pub fn build_with(
        self,
        registry: &EffectRegistry,
    ) -> Result<(Box<dyn AudioUnit>, EffectControls)> {
        registry.build(&self.effect_name, &self.parameters)
    }
}

/// Extension trait for EffectRegistry to enable fluent API
pub trait EffectRegistryExt {
    /// Start building an effect with fluent API
    fn effect(&self, name: impl Into<String>) -> EffectBuilder<'_>;
}

impl EffectRegistryExt for EffectRegistry {
    fn effect(&self, name: impl Into<String>) -> EffectBuilder<'_> {
        EffectBuilder::new(self, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluent_builder() {
        let registry = EffectRegistry::with_builtin();

        let result = registry.effect("lpf").cutoff(2000.0).res(0.7).build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_standalone_effect() {
        let result = Effect::new("lpf").cutoff(1000.0).build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_chainable_params() {
        let registry = EffectRegistry::with_builtin();

        let result = registry
            .effect("compressor")
            .threshold(-20.0)
            .ratio(4.0)
            .attack(0.01)
            .release(0.1)
            .build();

        assert!(result.is_ok());
    }
}
