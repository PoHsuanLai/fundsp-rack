//! Effect chain for processing audio through multiple effects
//!
//! Provides an ordered chain of effects that can be applied to audio streams.
use crate::metrics::CpuMeter;
use crate::Result;
use super::registry::{EffectControls, EffectRegistry};
#[cfg(feature = "serde")]
use super::serialize::{ChainState, EffectState};
use super::sidechain::SidechainAwareEffect;
use super::EffectId;
use fundsp::hacker32::*;
use std::collections::HashMap;
use std::sync::Arc;

/// An effect instance with its audio processing unit and controls
pub struct Effect {
    /// Optional ID for IR synchronization
    pub id: Option<EffectId>,
    /// Name of the effect
    pub name: String,
    /// Controllable parameters
    pub controls: EffectControls,
    /// The actual audio processing unit (stereo)
    pub processor: Box<dyn AudioUnit>,
    /// Optional sidechain-aware processor (if this effect supports sidechain)
    pub sidechain_processor: Option<Box<dyn SidechainAwareEffect>>,
    /// Latency introduced by this effect (in samples)
    pub latency_samples: usize,
    /// Whether this effect is bypassed (passes audio through unchanged)
    pub bypassed: bool,
    /// Whether this effect is muted (outputs silence)
    pub muted: bool,
    /// Latest input levels (RMS L, RMS R, Peak L, Peak R) for metering
    pub last_input_levels: (f32, f32, f32, f32),
    /// Latest output levels (RMS L, RMS R, Peak L, Peak R) for metering
    pub last_output_levels: (f32, f32, f32, f32),
    /// Sample buffer for calculating input RMS (rolling average)
    input_level_buffer: Vec<(f32, f32)>,
    /// Sample buffer for calculating output RMS (rolling average)
    output_level_buffer: Vec<(f32, f32)>,
    /// CPU meter for performance tracking
    pub cpu_meter: CpuMeter,
}

/// A chain of audio effects that are processed in order
pub struct EffectChain {
    /// The effects in order of processing
    pub effects: Vec<Effect>,
    /// Whether the effect chain is bypassed
    pub bypassed: bool,
    /// Reference to the effect registry for creating new effects
    registry: Option<Arc<EffectRegistry>>,
    /// Sample rate for effect processing
    sample_rate: f64,
}

impl EffectChain {
    /// Create a new empty effect chain
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
            bypassed: false,
            registry: None,
            sample_rate: 48000.0, // Default sample rate
        }
    }

    /// Create a new effect chain with a registry for creating effects
    pub fn with_registry(registry: EffectRegistry) -> Self {
        Self {
            effects: Vec::new(),
            bypassed: false,
            registry: Some(Arc::new(registry)),
            sample_rate: 48000.0, // Default sample rate
        }
    }

    /// Create a new effect chain with a shared registry (Arc)
    pub fn with_shared_registry(registry: Arc<EffectRegistry>) -> Self {
        Self {
            effects: Vec::new(),
            bypassed: false,
            registry: Some(registry),
            sample_rate: 48000.0, // Default sample rate
        }
    }

    /// Set the registry (builder pattern)
    pub fn registry(mut self, registry: EffectRegistry) -> Self {
        self.registry = Some(Arc::new(registry));
        self
    }

    /// Set the sample rate (builder pattern)
    pub fn with_sample_rate(mut self, sample_rate: f64) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    /// Set the sample rate for this effect chain
    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
    }

    /// Add an effect to the end of the chain by name
    pub fn add_effect(
        &mut self,
        name: &str,
        params: &HashMap<String, f32>,
    ) -> Result<usize> {
        if let Some(registry) = &self.registry {
            let (processor, controls) = registry.build(name, params)?;
            let metadata = registry.get_metadata(name).ok_or_else(|| {
                crate::Error::InvalidEffect(format!("Effect not found: {}", name))
            })?;

            // Check if this is a sidechain effect and build sidechain processor
            let sidechain_processor =
                super::sidechain::build_sidechain_effect(name, params, self.sample_rate as f32);

            let effect = Effect {
                id: None,
                name: name.to_string(),
                controls,
                processor,
                sidechain_processor,
                latency_samples: metadata.latency_samples,
                bypassed: false,
                muted: false,
                last_input_levels: (0.0, 0.0, 0.0, 0.0),
                last_output_levels: (0.0, 0.0, 0.0, 0.0),
                input_level_buffer: Vec::with_capacity(2048), // ~43ms at 48kHz
                output_level_buffer: Vec::with_capacity(2048), // ~43ms at 48kHz
                cpu_meter: CpuMeter::new(self.sample_rate),
            };
            self.effects.push(effect);
            Ok(self.effects.len() - 1)
        } else {
            Err(crate::Error::InvalidEffect(
                "No registry available".to_string(),
            ))
        }
    }

    /// Add an effect with parameters as key-value pairs (chainable, consumes self)
    ///
    /// # Example
    /// ```no_run
    /// # use fundsp_effects::prelude::*;
    /// let chain = EffectChain::new()
    ///     .registry(EffectRegistry::with_builtin())
    ///     .effect("lpf", &[("cutoff", 2000.0), ("res", 1.0)])?
    ///     .effect("reverb", &[])?;
    /// ```
    pub fn effect(mut self, name: &str, params: &[(&str, f32)]) -> Result<Self> {
        let params_map: HashMap<String, f32> =
            params.iter().map(|(k, v)| (k.to_string(), *v)).collect();
        self.add_effect(name, &params_map)?;
        Ok(self)
    }

    /// Add an effect with parameters (chainable, borrows self)
    ///
    /// This is useful when you need to keep a mutable reference to the chain
    /// for later use (e.g., processing audio).
    ///
    /// # Example
    /// ```no_run
    /// # use fundsp_effects::prelude::*;
    /// let mut chain = EffectChain::with_registry(EffectRegistry::with_builtin());
    /// chain
    ///     .add("chorus", &[("rate", 0.5), ("depth", 0.3)])?
    ///     .add("hall", &[("mix", 0.4)])?;
    ///
    /// // Can still use chain for processing
    /// let (out_l, out_r) = chain.process(0.5, 0.5);
    /// ```
    pub fn add(&mut self, name: &str, params: &[(&str, f32)]) -> Result<&mut Self> {
        let params_map: HashMap<String, f32> =
            params.iter().map(|(k, v)| (k.to_string(), *v)).collect();
        self.add_effect(name, &params_map)?;
        Ok(self)
    }

    /// Add an effect with a specific ID (for IR synchronization)
    pub fn add_effect_with_id(
        &mut self,
        id: EffectId,
        name: &str,
        params: HashMap<String, f32>,
    ) -> Result<usize> {
        if let Some(registry) = &self.registry {
            let (processor, controls) = registry.build(name, &params)?;
            let metadata = registry.get_metadata(name).ok_or_else(|| {
                crate::Error::InvalidEffect(format!("Effect not found: {}", name))
            })?;

            // Check if this is a sidechain effect and build sidechain processor
            let sidechain_processor =
                super::sidechain::build_sidechain_effect(name, &params, self.sample_rate as f32);

            let effect = Effect {
                id: Some(id),
                name: name.to_string(),
                controls,
                processor,
                sidechain_processor,
                latency_samples: metadata.latency_samples,
                bypassed: false,
                muted: false,
                last_input_levels: (0.0, 0.0, 0.0, 0.0),
                last_output_levels: (0.0, 0.0, 0.0, 0.0),
                input_level_buffer: Vec::with_capacity(2048), // ~43ms at 48kHz
                output_level_buffer: Vec::with_capacity(2048), // ~43ms at 48kHz
                cpu_meter: CpuMeter::new(self.sample_rate),
            };
            self.effects.push(effect);
            Ok(self.effects.len() - 1)
        } else {
            Err(crate::Error::InvalidEffect(
                "No registry available".to_string(),
            ))
        }
    }

    /// Find effect index by ID
    pub fn find_effect_index(&self, id: EffectId) -> Option<usize> {
        self.effects.iter().position(|e| e.id == Some(id))
    }

    /// Remove effect by ID
    pub fn remove_effect_by_id(&mut self, id: EffectId) -> Option<Effect> {
        if let Some(index) = self.find_effect_index(id) {
            Some(self.effects.remove(index))
        } else {
            None
        }
    }

    /// Set parameter by effect ID
    pub fn set_effect_param_by_id(&self, id: EffectId, param_name: &str, value: f32) -> bool {
        if let Some(index) = self.find_effect_index(id) {
            self.set_param(index, param_name, value)
        } else {
            false
        }
    }

    /// Reorder effect by ID to new position
    pub fn reorder_effect_by_id(&mut self, id: EffectId, new_index: usize) -> bool {
        if let Some(old_index) = self.find_effect_index(id) {
            if new_index < self.effects.len() {
                let effect = self.effects.remove(old_index);
                self.effects.insert(new_index, effect);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Toggle effect bypass by ID
    pub fn toggle_effect_bypass_by_id(&mut self, _id: EffectId) -> bool {
        // For now, we don't have per-effect bypass, only chain bypass
        // This would require adding a bypassed field to Effect struct
        false
    }

    /// Set effect bypass by ID
    pub fn set_effect_bypass_by_id(&mut self, _id: EffectId, _bypass: bool) -> bool {
        // For now, we don't have per-effect bypass, only chain bypass
        // This would require adding a bypassed field to Effect struct
        false
    }

    /// Remove an effect by index
    pub fn remove_effect(&mut self, index: usize) -> bool {
        if index < self.effects.len() {
            self.effects.remove(index);
            true
        } else {
            false
        }
    }

    /// Set a parameter on an effect in the chain
    pub fn set_param(&self, effect_index: usize, param_name: &str, value: f32) -> bool {
        if let Some(effect) = self.effects.get(effect_index) {
            effect.controls.set(param_name, value);
            true
        } else {
            false
        }
    }

    /// Process stereo audio through the entire effect chain
    #[inline]
    pub fn process(&mut self, left: f32, right: f32) -> (f32, f32) {
        self.process_with_sidechain(left, right, None)
    }

    /// Process stereo audio through the effect chain with optional sidechain input
    ///
    /// # Arguments
    /// * `left` - Left channel input
    /// * `right` - Right channel input
    /// * `sidechain` - Optional sidechain signal (left, right). If None, effects process normally.
    #[inline]
    pub fn process_with_sidechain(
        &mut self,
        left: f32,
        right: f32,
        sidechain: Option<(f32, f32)>,
    ) -> (f32, f32) {
        if self.bypassed || self.effects.is_empty() {
            return (left, right);
        }

        let mut current_left = left;
        let mut current_right = right;

        for effect in &mut self.effects {
            // Capture input levels before processing
            effect
                .input_level_buffer
                .push((current_left, current_right));
            if effect.input_level_buffer.len() > 2048 {
                effect.input_level_buffer.remove(0);
            }
            if effect.input_level_buffer.len() >= 2048 {
                let (rms_l, rms_r, peak_l, peak_r) =
                    calculate_buffer_levels(&effect.input_level_buffer);
                effect.last_input_levels = (rms_l, rms_r, peak_l, peak_r);
            }

            // Handle mute: output silence
            if effect.muted {
                current_left = 0.0;
                current_right = 0.0;
            }
            // Handle bypass: skip processing
            else if !effect.bypassed {
                // Start CPU timing
                let start = effect.cpu_meter.start_timing();

                // Check if this effect has sidechain processing and we have sidechain data
                if let (Some(ref mut sc_processor), Some((sc_left, sc_right))) =
                    (&mut effect.sidechain_processor, sidechain)
                {
                    // Use sidechain-aware processing
                    (current_left, current_right) = sc_processor.process_with_sidechain(
                        current_left,
                        current_right,
                        sc_left,
                        sc_right,
                    );
                } else {
                    // Normal processing
                    (current_left, current_right) =
                        effect.processor.filter_stereo(current_left, current_right);
                }

                // Stop CPU timing
                effect.cpu_meter.stop_timing(start, 1);
            }
            // If bypassed, audio passes through unchanged

            // Capture output levels after processing
            effect
                .output_level_buffer
                .push((current_left, current_right));
            if effect.output_level_buffer.len() > 2048 {
                effect.output_level_buffer.remove(0);
            }
            if effect.output_level_buffer.len() >= 2048 {
                let (rms_l, rms_r, peak_l, peak_r) =
                    calculate_buffer_levels(&effect.output_level_buffer);
                effect.last_output_levels = (rms_l, rms_r, peak_l, peak_r);
            }
        }

        (current_left, current_right)
    }

    /// Set bypass state
    pub fn set_bypass(&mut self, bypass: bool) {
        self.bypassed = bypass;
    }

    /// Get bypass state
    pub fn is_bypassed(&self) -> bool {
        self.bypassed
    }

    /// Get effect levels for metering (effect_id or name -> output levels)
    /// Returns output levels only; for full metering use worker's EffectMeterData
    pub fn get_effect_levels(&self) -> HashMap<String, (f32, f32, f32, f32)> {
        let mut levels = HashMap::new();
        for (idx, effect) in self.effects.iter().enumerate() {
            // Use effect ID if available, otherwise use index-based key
            let key = if let Some(id) = effect.id {
                format!("{}", id)
            } else {
                format!("{}_{}", effect.name, idx)
            };
            levels.insert(key, effect.last_output_levels);
        }
        levels
    }
}

impl Default for EffectChain {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate RMS and peak levels from a buffer of stereo samples
fn calculate_buffer_levels(buffer: &[(f32, f32)]) -> (f32, f32, f32, f32) {
    if buffer.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }

    let mut sum_sq_l = 0.0;
    let mut sum_sq_r = 0.0;
    let mut peak_l = 0.0;
    let mut peak_r = 0.0;

    for &(left, right) in buffer {
        let abs_l = left.abs();
        let abs_r = right.abs();

        sum_sq_l += left * left;
        sum_sq_r += right * right;

        peak_l = peak_l.max(abs_l);
        peak_r = peak_r.max(abs_r);
    }

    let count = buffer.len() as f32;
    let rms_l = (sum_sq_l / count).sqrt();
    let rms_r = (sum_sq_r / count).sqrt();

    (rms_l, rms_r, peak_l, peak_r)
}

// =============================================================================
// New DAW-focused APIs
// =============================================================================

impl EffectChain {
    /// Bypass a specific effect by index (passes audio through unchanged)
    pub fn bypass_effect(&mut self, index: usize, bypassed: bool) -> Result<()> {
        self.effects
            .get_mut(index)
            .ok_or_else(|| {
                crate::Error::InvalidEffect(format!("Effect index {} not found", index))
            })?
            .bypassed = bypassed;
        Ok(())
    }

    /// Mute a specific effect by index (outputs silence)
    pub fn mute_effect(&mut self, index: usize, muted: bool) -> Result<()> {
        self.effects
            .get_mut(index)
            .ok_or_else(|| {
                crate::Error::InvalidEffect(format!("Effect index {} not found", index))
            })?
            .muted = muted;
        Ok(())
    }

    /// Check if an effect is bypassed
    pub fn is_effect_bypassed(&self, index: usize) -> Option<bool> {
        self.effects.get(index).map(|e| e.bypassed)
    }

    /// Check if an effect is muted
    pub fn is_effect_muted(&self, index: usize) -> Option<bool> {
        self.effects.get(index).map(|e| e.muted)
    }

    /// Get total latency of the chain in samples
    pub fn total_latency(&self) -> usize {
        self.effects
            .iter()
            .filter(|e| !e.bypassed)
            .map(|e| e.latency_samples)
            .sum()
    }

    /// Get latency of a specific effect
    pub fn effect_latency(&self, index: usize) -> Option<usize> {
        self.effects.get(index).map(|e| e.latency_samples)
    }

    /// Serialize the chain to JSON
    ///
    /// # Example
    /// ```no_run
    /// # use fundsp_effects::prelude::*;
    /// # let chain = EffectChain::new();
    /// let json = chain.to_json().unwrap();
    /// std::fs::write("my_chain.json", json).unwrap();
    /// ```
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String> {
        let state = self.to_state();
        state
            .to_json()
            .map_err(|e| crate::Error::SerializationError(e.to_string()))
    }

    /// Deserialize and load a chain from JSON
    ///
    /// # Example
    /// ```no_run
    /// # use fundsp_effects::prelude::*;
    /// # let mut chain = EffectChain::with_registry(EffectRegistry::with_builtin());
    /// let json = std::fs::read_to_string("my_chain.json").unwrap();
    /// chain.from_json(&json).unwrap();
    /// ```
    #[cfg(feature = "serde")]
    pub fn from_json(&mut self, json: &str) -> Result<()> {
        let state = ChainState::from_json(json)
            .map_err(|e| crate::Error::SerializationError(e.to_string()))?;
        self.from_state(&state)
    }

    /// Convert to serializable state
    #[cfg(feature = "serde")]
    pub fn to_state(&self) -> ChainState {
        let mut state = ChainState::new(self.sample_rate);
        state.bypassed = self.bypassed;

        for effect in &self.effects {
            let mut effect_state = EffectState::new(effect.name.clone());
            effect_state.id = effect.id;
            effect_state.bypassed = effect.bypassed;
            effect_state.muted = effect.muted;

            // Extract parameters from controls
            for (key, shared) in &effect.controls.params {
                effect_state.set_param(key, shared.value());
            }

            state.add_effect(effect_state);
        }

        state
    }

    /// Load from serializable state
    #[cfg(feature = "serde")]
    pub fn from_state(&mut self, state: &ChainState) -> Result<()> {
        // Clear existing effects
        self.effects.clear();
        self.bypassed = state.bypassed;
        self.sample_rate = state.sample_rate;

        // Rebuild effects from state
        for effect_state in &state.effects {
            let index = if let Some(id) = effect_state.id {
                self.add_effect_with_id(id, &effect_state.name, effect_state.parameters.clone())?
            } else {
                self.add_effect(&effect_state.name, &effect_state.parameters)?
            };

            // Restore bypass/mute state
            if let Some(effect) = self.effects.get_mut(index) {
                effect.bypassed = effect_state.bypassed;
                effect.muted = effect_state.muted;
            }
        }

        Ok(())
    }

    /// Get current sample rate
    pub fn sample_rate(&self) -> f64 {
        self.sample_rate
    }

    /// Get number of effects in chain
    pub fn len(&self) -> usize {
        self.effects.len()
    }

    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    /// Get effect name by index
    pub fn effect_name(&self, index: usize) -> Option<&str> {
        self.effects.get(index).map(|e| e.name.as_str())
    }

    /// Get effect ID by index
    pub fn effect_id(&self, index: usize) -> Option<EffectId> {
        self.effects.get(index).and_then(|e| e.id)
    }

    /// Clear all effects from the chain
    pub fn clear(&mut self) {
        self.effects.clear();
    }

    /// Get CPU usage for a specific effect
    pub fn effect_cpu_usage(&self, index: usize) -> Option<f64> {
        self.effects
            .get(index)
            .map(|e| e.cpu_meter.metrics().cpu_usage)
    }

    /// Get CPU percentage for a specific effect (0-100%)
    pub fn effect_cpu_percent(&self, index: usize) -> Option<f64> {
        self.effects
            .get(index)
            .map(|e| e.cpu_meter.metrics().cpu_percent())
    }

    /// Get total CPU usage across all effects
    pub fn total_cpu_usage(&self) -> f64 {
        self.effects
            .iter()
            .filter(|e| !e.bypassed)
            .map(|e| e.cpu_meter.metrics().cpu_usage)
            .sum()
    }

    /// Get total CPU percentage across all effects (0-100%)
    pub fn total_cpu_percent(&self) -> f64 {
        self.total_cpu_usage() * 100.0
    }

    /// Check if any effect is overloaded (>80% CPU)
    pub fn has_overload(&self) -> bool {
        self.effects
            .iter()
            .any(|e| e.cpu_meter.metrics().is_overloaded())
    }

    /// Reset CPU meters for all effects
    pub fn reset_cpu_meters(&mut self) {
        for effect in &mut self.effects {
            effect.cpu_meter.reset();
        }
    }

    /// Get detailed CPU metrics for all effects
    pub fn cpu_report(&self) -> Vec<(String, f64, bool)> {
        self.effects
            .iter()
            .map(|e| {
                let metrics = e.cpu_meter.metrics();
                (
                    e.name.clone(),
                    metrics.cpu_percent(),
                    metrics.is_overloaded(),
                )
            })
            .collect()
    }
}
