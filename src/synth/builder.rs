//! Fluent builder API for creating synths
//!
//! This module provides a chainable, ergonomic API for creating and configuring synths.
//!
//! # Examples
//!
//! ```
//! use fundsp_synth::prelude::*;
//!
//! // Simple synth creation
//! let (synth, controls) = Synth::new("tb303")
//!     .freq(55.0)
//!     .param("cutoff", 800.0)
//!     .param("res", 0.7)
//!     .amp(0.5)
//!     .build()
//!     .unwrap();
//!
//! // Using registry with fluent API
//! let registry = SynthRegistry::with_builtin();
//!
//! let (synth, controls) = registry.synth("fm")
//!     .freq(440.0)
//!     .param("ratio", 3.5)
//!     .param("index", 2.0)
//!     .build()
//!     .unwrap();
//! ```

use super::registry::{SynthRegistry, VoiceControls};
use crate::Result;
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Fluent builder for creating synths
pub struct SynthBuilder<'a> {
    registry: &'a SynthRegistry,
    synth_name: String,
    frequency: f32,
    parameters: HashMap<String, f32>,
    amplitude: Option<f32>,
}

impl<'a> SynthBuilder<'a> {
    /// Create a new synth builder
    pub fn new(registry: &'a SynthRegistry, synth_name: impl Into<String>) -> Self {
        Self {
            registry,
            synth_name: synth_name.into(),
            frequency: 440.0, // Default A440
            parameters: HashMap::new(),
            amplitude: None,
        }
    }

    /// Set the frequency (in Hz)
    pub fn freq(mut self, freq: f32) -> Self {
        self.frequency = freq;
        self
    }

    /// Set the frequency from a MIDI note number
    pub fn note(mut self, midi_note: u8) -> Self {
        self.frequency = midi_to_hz(midi_note);
        self
    }

    /// Set a synth parameter
    pub fn param(mut self, name: impl Into<String>, value: f32) -> Self {
        self.parameters.insert(name.into(), value);
        self
    }

    /// Set multiple parameters at once
    pub fn params(mut self, params: HashMap<String, f32>) -> Self {
        self.parameters.extend(params);
        self
    }

    /// Set the amplitude (0.0 to 1.0+)
    pub fn amp(mut self, amplitude: f32) -> Self {
        self.amplitude = Some(amplitude);
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

    /// Set FM ratio (for FM synths)
    pub fn ratio(self, ratio: f32) -> Self {
        self.param("ratio", ratio)
    }

    /// Set FM index (for FM synths)
    pub fn index(self, index: f32) -> Self {
        self.param("index", index)
    }

    /// Set detune amount (for detuned oscillators)
    pub fn detune(self, detune: f32) -> Self {
        self.param("detune", detune)
    }

    /// Set pulse width (for pulse wave synths)
    pub fn pulse_width(self, width: f32) -> Self {
        self.param("pulse_width", width)
    }

    /// Build the synth
    pub fn build(self) -> Result<(Box<dyn AudioUnit>, VoiceControls)> {
        let (synth, controls) =
            self.registry
                .create(&self.synth_name, self.frequency, &self.parameters)?;

        // Set amplitude if specified
        if let Some(amp) = self.amplitude {
            controls.amp.set(amp);
        }

        Ok((synth, controls))
    }
}

/// Standalone synth builder (doesn't require a registry reference)
pub struct Synth {
    synth_name: String,
    frequency: f32,
    parameters: HashMap<String, f32>,
    amplitude: Option<f32>,
}

impl Synth {
    /// Create a new synth builder
    pub fn new(synth_name: impl Into<String>) -> Self {
        Self {
            synth_name: synth_name.into(),
            frequency: 440.0,
            parameters: HashMap::new(),
            amplitude: None,
        }
    }

    /// Set the frequency (in Hz)
    pub fn freq(mut self, freq: f32) -> Self {
        self.frequency = freq;
        self
    }

    /// Set the frequency from a MIDI note number
    pub fn note(mut self, midi_note: u8) -> Self {
        self.frequency = midi_to_hz(midi_note);
        self
    }

    /// Set a synth parameter
    pub fn param(mut self, name: impl Into<String>, value: f32) -> Self {
        self.parameters.insert(name.into(), value);
        self
    }

    /// Set multiple parameters at once
    pub fn params(mut self, params: HashMap<String, f32>) -> Self {
        self.parameters.extend(params);
        self
    }

    /// Set the amplitude (0.0 to 1.0+)
    pub fn amp(mut self, amplitude: f32) -> Self {
        self.amplitude = Some(amplitude);
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

    /// Set FM ratio (for FM synths)
    pub fn ratio(self, ratio: f32) -> Self {
        self.param("ratio", ratio)
    }

    /// Set FM index (for FM synths)
    pub fn index(self, index: f32) -> Self {
        self.param("index", index)
    }

    /// Set detune amount (for detuned oscillators)
    pub fn detune(self, detune: f32) -> Self {
        self.param("detune", detune)
    }

    /// Set pulse width (for pulse wave synths)
    pub fn pulse_width(self, width: f32) -> Self {
        self.param("pulse_width", width)
    }

    /// Build the synth using the default registry
    pub fn build(self) -> Result<(Box<dyn AudioUnit>, VoiceControls)> {
        let registry = SynthRegistry::with_builtin();
        let (synth, controls) =
            registry.create(&self.synth_name, self.frequency, &self.parameters)?;

        // Set amplitude if specified
        if let Some(amp) = self.amplitude {
            controls.amp.set(amp);
        }

        Ok((synth, controls))
    }

    /// Build the synth using a custom registry
    pub fn build_with(
        self,
        registry: &SynthRegistry,
    ) -> Result<(Box<dyn AudioUnit>, VoiceControls)> {
        let (synth, controls) =
            registry.create(&self.synth_name, self.frequency, &self.parameters)?;

        // Set amplitude if specified
        if let Some(amp) = self.amplitude {
            controls.amp.set(amp);
        }

        Ok((synth, controls))
    }
}

/// Extension trait for SynthRegistry to enable fluent API
pub trait SynthRegistryExt {
    /// Start building a synth with fluent API
    fn synth(&self, name: impl Into<String>) -> SynthBuilder<'_>;
}

impl SynthRegistryExt for SynthRegistry {
    fn synth(&self, name: impl Into<String>) -> SynthBuilder<'_> {
        SynthBuilder::new(self, name)
    }
}

/// Convert MIDI note number to frequency in Hz
fn midi_to_hz(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_to_hz() {
        assert_eq!(midi_to_hz(69), 440.0); // A4
        assert!((midi_to_hz(60) - 261.63).abs() < 0.01); // C4
    }

    #[test]
    fn test_fluent_builder() {
        let registry = SynthRegistry::with_builtin();

        let result = registry.synth("sine").freq(440.0).amp(0.5).build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_standalone_synth() {
        let result = Synth::new("sine").freq(440.0).amp(0.3).build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_note_conversion() {
        let registry = SynthRegistry::with_builtin();

        let result = registry
            .synth("sine")
            .note(69) // A4 = 440 Hz
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_chainable_params() {
        let registry = SynthRegistry::with_builtin();

        let result = registry
            .synth("tb303")
            .freq(55.0)
            .cutoff(800.0)
            .res(0.7)
            .amp(0.5)
            .build();

        assert!(result.is_ok());
    }
}
