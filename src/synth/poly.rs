//! Polyphonic synthesizer module
//!
//! This module provides a simple polyphony layer on top of the synth registry,
//! managing multiple voices for chord playing.
//!
//! # Example
//!
//! ```rust,no_run
//! use fundsp_rack::prelude::*;
//!
//! // Create a polyphonic synth with 8 voices
//! let mut poly = PolySynth::new("pad", 8);
//!
//! // Play notes (MIDI note numbers)
//! poly.note_on(60, 0.8); // C4
//! poly.note_on(64, 0.8); // E4
//! poly.note_on(67, 0.8); // G4
//!
//! // Process audio
//! let (left, right) = poly.get_stereo();
//!
//! // Release notes
//! poly.note_off(60);
//! ```

use super::registry::{SynthRegistry, VoiceControls};
use fundsp::hacker32::*;
use std::collections::HashMap;

/// Convert MIDI note number to frequency in Hz
pub fn midi_to_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

/// A single voice in the polyphonic synth
struct Voice {
    /// The audio unit for this voice
    unit: Box<dyn AudioUnit>,
    /// Controls for this voice
    controls: VoiceControls,
    /// The MIDI note this voice is playing (None if free)
    note: Option<u8>,
    /// Voice age (for voice stealing - older voices get stolen first)
    age: u64,
}

/// Polyphonic synthesizer that manages multiple voices
pub struct PolySynth {
    /// The synth name to use for creating voices
    synth_name: String,
    /// Additional parameters for synth creation
    params: HashMap<String, f32>,
    /// The synth registry
    registry: SynthRegistry,
    /// All voices (active and inactive)
    voices: Vec<Voice>,
    /// Maximum number of voices
    max_voices: usize,
    /// Voice age counter (increments on each note-on)
    age_counter: u64,
    /// Sample rate
    sample_rate: f64,
}

impl PolySynth {
    /// Create a new polyphonic synth with the given synth name and max voices
    pub fn new(synth_name: &str, max_voices: usize) -> Self {
        Self::with_registry(synth_name, max_voices, SynthRegistry::with_builtin())
    }

    /// Create a builder for a polyphonic synth with fluent API
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use fundsp_rack::prelude::*;
    ///
    /// let mut poly = PolySynth::builder("tb303")
    ///     .voices(4)
    ///     .cutoff(800.0)
    ///     .res(0.7)
    ///     .sample_rate(48000.0)
    ///     .build();
    ///
    /// poly.note_on(36, 0.8); // Bass note
    /// ```
    pub fn builder(synth_name: &str) -> PolySynthBuilder<'_> {
        PolySynthBuilder::new(synth_name)
    }

    /// Create a new polyphonic synth with a custom registry
    pub fn with_registry(synth_name: &str, max_voices: usize, registry: SynthRegistry) -> Self {
        Self {
            synth_name: synth_name.to_string(),
            params: HashMap::new(),
            registry,
            voices: Vec::with_capacity(max_voices),
            max_voices,
            age_counter: 0,
            sample_rate: 44100.0,
        }
    }

    /// Set a parameter for new voices
    pub fn set_param(&mut self, name: &str, value: f32) -> &mut Self {
        self.params.insert(name.to_string(), value);
        self
    }

    /// Set sample rate for all voices
    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        for voice in &mut self.voices {
            voice.unit.set_sample_rate(sample_rate);
        }
    }

    /// Trigger a note on
    ///
    /// Returns the voice index that was used, or None if failed
    pub fn note_on(&mut self, note: u8, velocity: f32) -> Option<usize> {
        let freq = midi_to_freq(note);

        // First, check if this note is already playing (retrigger)
        for (i, voice) in self.voices.iter_mut().enumerate() {
            if voice.note == Some(note) {
                // Retrigger: reset the voice
                voice.controls.amp.set(velocity);
                voice.controls.pitch_bend.set(1.0);
                voice.age = self.age_counter;
                self.age_counter += 1;
                return Some(i);
            }
        }

        // Try to find a free voice
        for (i, voice) in self.voices.iter_mut().enumerate() {
            if voice.note.is_none() {
                // Reuse this voice with new frequency
                // We need to create a new unit since fundsp synths have fixed frequency
                if let Ok((unit, controls)) =
                    self.registry.create(&self.synth_name, freq, &self.params)
                {
                    voice.unit = unit;
                    voice.controls = controls;
                    voice.controls.amp.set(velocity);
                    voice.note = Some(note);
                    voice.age = self.age_counter;
                    self.age_counter += 1;
                    voice.unit.set_sample_rate(self.sample_rate);
                    return Some(i);
                }
                return None;
            }
        }

        // No free voice - either allocate a new one or steal the oldest
        if self.voices.len() < self.max_voices {
            // Allocate new voice
            if let Ok((mut unit, controls)) =
                self.registry.create(&self.synth_name, freq, &self.params)
            {
                unit.set_sample_rate(self.sample_rate);
                let voice = Voice {
                    unit,
                    controls,
                    note: Some(note),
                    age: self.age_counter,
                };
                voice.controls.amp.set(velocity);
                self.age_counter += 1;
                self.voices.push(voice);
                return Some(self.voices.len() - 1);
            }
        } else {
            // Voice stealing: find the oldest voice
            let oldest_idx = self
                .voices
                .iter()
                .enumerate()
                .min_by_key(|(_, v)| v.age)
                .map(|(i, _)| i)?;

            if let Ok((mut unit, controls)) =
                self.registry.create(&self.synth_name, freq, &self.params)
            {
                unit.set_sample_rate(self.sample_rate);
                self.voices[oldest_idx] = Voice {
                    unit,
                    controls,
                    note: Some(note),
                    age: self.age_counter,
                };
                self.voices[oldest_idx].controls.amp.set(velocity);
                self.age_counter += 1;
                return Some(oldest_idx);
            }
        }

        None
    }

    /// Release a note
    pub fn note_off(&mut self, note: u8) {
        for voice in &mut self.voices {
            if voice.note == Some(note) {
                // For now, just silence the voice
                // A proper implementation would trigger release envelope
                voice.controls.amp.set(0.0);
                voice.note = None;
            }
        }
    }

    /// Release all notes
    pub fn all_notes_off(&mut self) {
        for voice in &mut self.voices {
            voice.controls.amp.set(0.0);
            voice.note = None;
        }
    }

    /// Set pitch bend for all active voices (in semitones)
    pub fn pitch_bend(&mut self, semitones: f32) {
        let bend = 2.0_f32.powf(semitones / 12.0);
        for voice in &mut self.voices {
            if voice.note.is_some() {
                voice.controls.pitch_bend.set(bend);
            }
        }
    }

    /// Set cutoff for all active voices (if applicable)
    pub fn set_cutoff(&mut self, cutoff: f32) {
        for voice in &mut self.voices {
            if let Some(ref c) = voice.controls.cutoff {
                c.set(cutoff);
            }
        }
    }

    /// Set resonance for all active voices (if applicable)
    pub fn set_resonance(&mut self, resonance: f32) {
        for voice in &mut self.voices {
            if let Some(ref r) = voice.controls.resonance {
                r.set(resonance);
            }
        }
    }

    /// Get the next stereo sample by summing all active voices
    pub fn get_stereo(&mut self) -> (f32, f32) {
        let mut left = 0.0;
        let mut right = 0.0;

        for voice in &mut self.voices {
            let (l, r) = voice.unit.get_stereo();
            left += l;
            right += r;
        }

        // Simple limiting to prevent clipping
        let scale = if self.voices.len() > 1 {
            1.0 / (self.voices.len() as f32).sqrt()
        } else {
            1.0
        };

        (left * scale, right * scale)
    }

    /// Get the number of currently active voices
    pub fn active_voices(&self) -> usize {
        self.voices.iter().filter(|v| v.note.is_some()).count()
    }

    /// Get the total number of allocated voices
    pub fn allocated_voices(&self) -> usize {
        self.voices.len()
    }

    /// Get the maximum number of voices
    pub fn max_voices(&self) -> usize {
        self.max_voices
    }

    /// Get the currently playing notes
    pub fn playing_notes(&self) -> Vec<u8> {
        self.voices.iter().filter_map(|v| v.note).collect()
    }
}

/// Builder for creating polyphonic synths with a fluent API
///
/// # Example
///
/// ```rust,no_run
/// use fundsp_rack::prelude::*;
///
/// // Create a polyphonic pad synth with custom parameters
/// let mut poly = PolySynth::builder("pad")
///     .voices(8)
///     .cutoff(2000.0)
///     .res(0.5)
///     .sample_rate(48000.0)
///     .build();
///
/// // Or use the registry extension
/// let registry = SynthRegistry::with_builtin();
/// let mut poly = registry.poly("strings")
///     .voices(6)
///     .build();
/// ```
pub struct PolySynthBuilder<'a> {
    synth_name: &'a str,
    max_voices: usize,
    params: HashMap<String, f32>,
    registry: Option<SynthRegistry>,
    sample_rate: f64,
}

impl<'a> PolySynthBuilder<'a> {
    /// Create a new builder for the given synth
    pub fn new(synth_name: &'a str) -> Self {
        Self {
            synth_name,
            max_voices: 8,
            params: HashMap::new(),
            registry: None,
            sample_rate: 44100.0,
        }
    }

    /// Set maximum number of voices (default: 8)
    pub fn voices(mut self, max_voices: usize) -> Self {
        self.max_voices = max_voices;
        self
    }

    /// Set a synth parameter by name
    pub fn param(mut self, name: &str, value: f32) -> Self {
        self.params.insert(name.to_string(), value);
        self
    }

    /// Set the synth registry
    pub fn registry(mut self, registry: SynthRegistry) -> Self {
        self.registry = Some(registry);
        self
    }

    /// Set the sample rate (default: 44100.0)
    pub fn sample_rate(mut self, sample_rate: f64) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    // === Common parameter shortcuts ===

    /// Set filter cutoff frequency (Hz)
    pub fn cutoff(self, hz: f32) -> Self {
        self.param("cutoff", hz)
    }

    /// Set filter resonance (0.0 to 1.0)
    pub fn res(self, value: f32) -> Self {
        self.param("res", value)
    }

    /// Set detune amount
    pub fn detune(self, value: f32) -> Self {
        self.param("detune", value)
    }

    /// Set FM modulation ratio
    pub fn ratio(self, value: f32) -> Self {
        self.param("ratio", value)
    }

    /// Set FM modulation index
    pub fn index(self, value: f32) -> Self {
        self.param("index", value)
    }

    /// Set attack time (seconds)
    pub fn attack(self, value: f32) -> Self {
        self.param("attack", value)
    }

    /// Set decay time (seconds)
    pub fn decay(self, value: f32) -> Self {
        self.param("decay", value)
    }

    /// Set sustain level (0.0 to 1.0)
    pub fn sustain(self, value: f32) -> Self {
        self.param("sustain", value)
    }

    /// Set release time (seconds)
    pub fn release(self, value: f32) -> Self {
        self.param("release", value)
    }

    /// Build the polyphonic synth
    pub fn build(self) -> PolySynth {
        let registry = self.registry.unwrap_or_else(SynthRegistry::with_builtin);
        let mut poly = PolySynth::with_registry(self.synth_name, self.max_voices, registry);
        poly.params = self.params;
        poly.sample_rate = self.sample_rate;
        poly
    }
}

// === Extension trait for SynthRegistry ===

/// Extension trait for creating polyphonic synths from a registry
pub trait SynthRegistryPolyExt {
    /// Create a polyphonic synth builder for the given synth name
    fn poly<'a>(&self, synth_name: &'a str) -> PolySynthBuilder<'a>;
}

impl SynthRegistryPolyExt for SynthRegistry {
    fn poly<'a>(&self, synth_name: &'a str) -> PolySynthBuilder<'a> {
        // We can't store a reference to self in the builder due to lifetime issues,
        // so we'll create a fresh registry in build(). This is a limitation but works fine.
        PolySynthBuilder::new(synth_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_to_freq() {
        // A4 = 440 Hz
        assert!((midi_to_freq(69) - 440.0).abs() < 0.01);
        // C4 = 261.63 Hz
        assert!((midi_to_freq(60) - 261.63).abs() < 0.1);
    }

    #[test]
    fn test_poly_synth_basic() {
        let mut poly = PolySynth::new("sine", 4);

        // Play a chord
        assert!(poly.note_on(60, 0.8).is_some());
        assert!(poly.note_on(64, 0.8).is_some());
        assert!(poly.note_on(67, 0.8).is_some());

        assert_eq!(poly.active_voices(), 3);
        assert_eq!(poly.playing_notes().len(), 3);

        // Release one note
        poly.note_off(64);
        assert_eq!(poly.active_voices(), 2);

        // Release all
        poly.all_notes_off();
        assert_eq!(poly.active_voices(), 0);
    }
}
