//! SoundFont Synth Adapter - integrates SoundFont with FunDSP
//!
//! This module provides two integration approaches:
//!
//! 1. **SoundFontUnit** - A single AudioUnit that owns the Synthesizer directly.
//!    Best for when you want one SoundFont renderer in your audio graph.
//!    No Arc<Mutex> needed - direct ownership for lowest latency.
//!
//! 2. **SoundFontPlayer** - Higher-level API with note tracking and controls.
//!    Uses SoundFontUnit internally but adds polyphony management.
//!
//! 3. **SoundFontSynthBuilder** - For SynthRegistry integration.
//!    Uses shared ownership (Arc<Mutex>) since multiple voices need to
//!    share the same underlying synthesizer.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use fundsp::audiounit::AudioUnit;
use fundsp::hacker32::*;
use fundsp::shared::Shared;

use super::SoundFontManager;
use crate::synth::registry::{SynthBuilder, SynthMetadata, VoiceControls};

// ============================================================================
// SoundFontUnit - Direct ownership, lowest latency
// ============================================================================

/// A single AudioUnit that owns a SoundFontManager directly.
///
/// This is the recommended approach when you need one SoundFont renderer
/// in your audio graph. It avoids Arc<Mutex> overhead entirely.
///
/// # Example
///
/// ```rust,no_run
/// use fundsp_rack::soundfont::{SoundFontManager, SoundFontUnit};
///
/// let mut manager = SoundFontManager::new(44100);
/// manager.load_soundfont("path/to/soundfont.sf2").unwrap();
///
/// // Create the unit - takes ownership of manager
/// let unit = SoundFontUnit::new(manager);
///
/// // Use in audio graph
/// // let graph = unit >> ...
/// ```
#[derive(Clone)]
pub struct SoundFontUnit {
    manager: SoundFontManager,
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    buffer_pos: usize,
}

impl SoundFontUnit {
    /// Buffer size for internal rendering
    const BUFFER_SIZE: usize = 64;

    /// Create a new SoundFontUnit that takes ownership of a SoundFontManager
    pub fn new(manager: SoundFontManager) -> Self {
        Self {
            manager,
            buffer_l: vec![0.0; Self::BUFFER_SIZE],
            buffer_r: vec![0.0; Self::BUFFER_SIZE],
            buffer_pos: Self::BUFFER_SIZE,
        }
    }

    /// Get mutable access to the underlying manager for MIDI control
    pub fn manager_mut(&mut self) -> &mut SoundFontManager {
        &mut self.manager
    }

    /// Get read access to the underlying manager
    pub fn manager(&self) -> &SoundFontManager {
        &self.manager
    }

    /// Send note on
    pub fn note_on(&mut self, channel: u8, note: u8, velocity: u8) {
        self.manager.note_on(channel, note, velocity);
    }

    /// Send note off
    pub fn note_off(&mut self, channel: u8, note: u8) {
        self.manager.note_off(channel, note);
    }

    /// Set program for a channel
    pub fn program_change(&mut self, channel: u8, program: u8) {
        self.manager.program_change(channel, program);
    }
}

impl AudioUnit for SoundFontUnit {
    fn inputs(&self) -> usize {
        0
    }

    fn outputs(&self) -> usize {
        2
    }

    fn reset(&mut self) {
        self.buffer_pos = Self::BUFFER_SIZE;
        self.manager.reset();
    }

    fn set_sample_rate(&mut self, _sample_rate: f64) {
        // Sample rate is set when creating the manager
    }

    fn tick(&mut self, _input: &[f32], output: &mut [f32]) {
        // Refill buffer if needed
        if self.buffer_pos >= Self::BUFFER_SIZE {
            self.manager.render(&mut self.buffer_l, &mut self.buffer_r);
            self.buffer_pos = 0;
        }

        output[0] = self.buffer_l[self.buffer_pos];
        output[1] = self.buffer_r[self.buffer_pos];
        self.buffer_pos += 1;
    }

    fn process(
        &mut self,
        size: usize,
        _input: &fundsp::buffer::BufferRef,
        output: &mut fundsp::buffer::BufferMut,
    ) {
        let mut pos = 0;

        while pos < size {
            // Refill buffer if needed
            if self.buffer_pos >= Self::BUFFER_SIZE {
                self.manager.render(&mut self.buffer_l, &mut self.buffer_r);
                self.buffer_pos = 0;
            }

            // Copy available samples
            let available = Ord::min(Self::BUFFER_SIZE - self.buffer_pos, size - pos);

            for i in 0..available {
                let l = self.buffer_l[self.buffer_pos + i];
                let r = self.buffer_r[self.buffer_pos + i];
                output.set(0, pos + i, F32x::splat(l));
                output.set(1, pos + i, F32x::splat(r));
            }

            self.buffer_pos += available;
            pos += available;
        }
    }

    fn get_id(&self) -> u64 {
        const ID: u64 = 0x536F756E64466F6E; // "SoundFon" in hex
        ID
    }

    fn ping(&mut self, probe: bool, hash: fundsp::prelude::AttoHash) -> fundsp::prelude::AttoHash {
        if probe {
            hash.hash(self.get_id())
        } else {
            hash
        }
    }

    fn route(
        &mut self,
        _input: &fundsp::signal::SignalFrame,
        _frequency: f64,
    ) -> fundsp::signal::SignalFrame {
        fundsp::signal::SignalFrame::new(2)
    }

    fn footprint(&self) -> usize {
        std::mem::size_of::<Self>() + Self::BUFFER_SIZE * 2 * std::mem::size_of::<f32>()
    }

    fn allocate(&mut self) {}
}

// ============================================================================
// SoundFontPlayer - Higher-level polyphonic player
// ============================================================================

/// Active note tracking for polyphony
#[derive(Clone, Debug)]
struct ActiveNote {
    channel: u8,
    note: u8,
}

/// Higher-level SoundFont player with note tracking
///
/// This wraps SoundFontUnit and adds:
/// - Note tracking for proper note-off handling
/// - Program management per channel
/// - Master volume control
pub struct SoundFontPlayer {
    unit: SoundFontUnit,
    active_notes: Vec<ActiveNote>,
    master_volume: f32,
}

impl SoundFontPlayer {
    /// Create a new player with a SoundFontManager
    pub fn new(manager: SoundFontManager) -> Self {
        Self {
            unit: SoundFontUnit::new(manager),
            active_notes: Vec::with_capacity(64),
            master_volume: 1.0,
        }
    }

    /// Load a soundfont file
    pub fn load_soundfont(&mut self, path: &str) -> crate::Result<()> {
        self.unit.manager_mut().load_soundfont(path)
    }

    /// Play a note (velocity 0-127)
    pub fn note_on(&mut self, channel: u8, note: u8, velocity: u8) {
        self.unit.note_on(channel, note, velocity);
        self.active_notes.push(ActiveNote { channel, note });
    }

    /// Stop a note
    pub fn note_off(&mut self, channel: u8, note: u8) {
        self.unit.note_off(channel, note);
        self.active_notes.retain(|n| !(n.channel == channel && n.note == note));
    }

    /// Stop all notes
    pub fn all_notes_off(&mut self) {
        for note in self.active_notes.drain(..) {
            self.unit.note_off(note.channel, note.note);
        }
    }

    /// Set program for a channel
    pub fn program_change(&mut self, channel: u8, program: u8) {
        self.unit.program_change(channel, program);
    }

    /// Set master volume (0.0 - 1.0)
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 2.0);
        self.unit.manager_mut().set_master_volume(volume);
    }

    /// Get the underlying unit for use in audio graphs
    pub fn into_unit(self) -> SoundFontUnit {
        self.unit
    }

    /// Get mutable access to the unit
    pub fn unit_mut(&mut self) -> &mut SoundFontUnit {
        &mut self.unit
    }
}

// ============================================================================
// Shared handle for SynthRegistry integration
// ============================================================================

/// Thread-safe handle to a shared SoundFontManager
///
/// Used when multiple voices need to share the same synthesizer
/// (e.g., in SynthRegistry integration).
pub type SoundFontSynthHandle = Arc<Mutex<SoundFontManager>>;

/// Create a new SoundFont synth handle
pub fn create_soundfont_synth(sample_rate: u32) -> SoundFontSynthHandle {
    Arc::new(Mutex::new(SoundFontManager::new(sample_rate)))
}

// ============================================================================
// SoundFontVoice - For SynthRegistry integration (shared ownership)
// ============================================================================

/// SoundFont voice that wraps a shared SoundFontManager
///
/// This implements AudioUnit by rendering from the shared SoundFontManager.
/// Multiple voices share the same manager - each voice uses a different MIDI channel.
///
/// Note: This uses Arc<Mutex> which has some overhead. For lowest latency,
/// use SoundFontUnit directly in your audio graph.
#[derive(Clone)]
pub struct SoundFontVoice {
    /// Shared synth manager
    synth: SoundFontSynthHandle,
    /// MIDI channel this voice uses (0-15)
    channel: u8,
    /// Note being played
    note: u8,
    /// Whether this voice is still active
    active: bool,
    /// Amplitude control
    amp: Shared,
    /// Internal buffer for rendering
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    buffer_pos: usize,
}

impl SoundFontVoice {
    /// Buffer size for internal rendering
    const BUFFER_SIZE: usize = 64;

    /// Create a new voice
    pub fn new(
        synth: SoundFontSynthHandle,
        channel: u8,
        note: u8,
        velocity: u8,
        program: u8,
        amp: Shared,
    ) -> Self {
        // Set up the channel and start the note
        if let Ok(mut mgr) = synth.lock() {
            mgr.program_change(channel, program);
            mgr.note_on(channel, note, velocity);
        }

        Self {
            synth,
            channel,
            note,
            active: true,
            amp,
            buffer_l: vec![0.0; Self::BUFFER_SIZE],
            buffer_r: vec![0.0; Self::BUFFER_SIZE],
            buffer_pos: Self::BUFFER_SIZE, // Start at end to trigger first render
        }
    }

    /// Stop this voice
    pub fn stop(&mut self) {
        if self.active {
            if let Ok(mut mgr) = self.synth.lock() {
                mgr.note_off(self.channel, self.note);
            }
            self.active = false;
        }
    }

    /// Check if voice is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Drop for SoundFontVoice {
    fn drop(&mut self) {
        self.stop();
    }
}

impl AudioUnit for SoundFontVoice {
    fn inputs(&self) -> usize {
        0
    }

    fn outputs(&self) -> usize {
        2
    }

    fn reset(&mut self) {
        self.buffer_pos = Self::BUFFER_SIZE;
    }

    fn set_sample_rate(&mut self, _sample_rate: f64) {
        // Sample rate is set on the manager, not per-voice
    }

    fn tick(&mut self, _input: &[f32], output: &mut [f32]) {
        // Refill buffer if needed
        if self.buffer_pos >= Self::BUFFER_SIZE {
            if let Ok(mut mgr) = self.synth.lock() {
                mgr.render(&mut self.buffer_l, &mut self.buffer_r);
            }
            self.buffer_pos = 0;
        }

        let amp = self.amp.value();
        output[0] = self.buffer_l[self.buffer_pos] * amp;
        output[1] = self.buffer_r[self.buffer_pos] * amp;
        self.buffer_pos += 1;
    }

    fn process(
        &mut self,
        size: usize,
        _input: &fundsp::buffer::BufferRef,
        output: &mut fundsp::buffer::BufferMut,
    ) {
        let amp = self.amp.value();
        let mut pos = 0;

        while pos < size {
            // Refill buffer if needed
            if self.buffer_pos >= Self::BUFFER_SIZE {
                if let Ok(mut mgr) = self.synth.lock() {
                    mgr.render(&mut self.buffer_l, &mut self.buffer_r);
                }
                self.buffer_pos = 0;
            }

            // Copy available samples
            let available = Ord::min(Self::BUFFER_SIZE - self.buffer_pos, size - pos);

            for i in 0..available {
                let l = self.buffer_l[self.buffer_pos + i] * amp;
                let r = self.buffer_r[self.buffer_pos + i] * amp;
                output.set(0, pos + i, F32x::splat(l));
                output.set(1, pos + i, F32x::splat(r));
            }

            self.buffer_pos += available;
            pos += available;
        }
    }

    fn get_id(&self) -> u64 {
        // Combine channel and note into a unique ID
        ((self.channel as u64) << 8) | (self.note as u64)
    }

    fn ping(&mut self, probe: bool, hash: fundsp::prelude::AttoHash) -> fundsp::prelude::AttoHash {
        if probe {
            hash.hash(self.get_id())
        } else {
            hash
        }
    }

    fn route(
        &mut self,
        _input: &fundsp::signal::SignalFrame,
        _frequency: f64,
    ) -> fundsp::signal::SignalFrame {
        fundsp::signal::SignalFrame::new(2)
    }

    fn footprint(&self) -> usize {
        std::mem::size_of::<Self>() + Self::BUFFER_SIZE * 2 * std::mem::size_of::<f32>()
    }

    fn allocate(&mut self) {}
}

// ============================================================================
// Channel allocator
// ============================================================================

/// Channel allocator for SoundFont voices
///
/// Tracks which MIDI channels are in use to avoid conflicts.
#[derive(Clone)]
pub struct ChannelAllocator {
    /// Channels in use (bitmap)
    in_use: u16,
}

impl Default for ChannelAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelAllocator {
    pub fn new() -> Self {
        // Mark channel 9 as always in use (GM drums)
        Self { in_use: 1 << 9 }
    }

    /// Allocate a channel for a melodic instrument
    pub fn allocate(&mut self) -> Option<u8> {
        for ch in 0..16u8 {
            if ch == 9 {
                continue; // Skip drum channel
            }
            if (self.in_use & (1 << ch)) == 0 {
                self.in_use |= 1 << ch;
                return Some(ch);
            }
        }
        // All channels in use, reuse channel 0
        Some(0)
    }

    /// Allocate the drum channel (9)
    pub fn allocate_drums(&mut self) -> u8 {
        self.in_use |= 1 << 9;
        9
    }

    /// Release a channel
    pub fn release(&mut self, channel: u8) {
        if channel != 9 {
            // Don't release drum channel
            self.in_use &= !(1 << channel);
        }
    }

    /// Check if a channel is in use
    pub fn is_in_use(&self, channel: u8) -> bool {
        (self.in_use & (1 << channel)) != 0
    }

    /// Get count of available channels
    pub fn available_count(&self) -> usize {
        (0..16u8)
            .filter(|&ch| ch != 9 && (self.in_use & (1 << ch)) == 0)
            .count()
    }
}

// ============================================================================
// SynthBuilder implementation for SynthRegistry
// ============================================================================

/// SynthBuilder implementation for SoundFont instruments
pub struct SoundFontSynthBuilder {
    synth: SoundFontSynthHandle,
    program: u8,
    name: String,
    channel_allocator: Arc<Mutex<ChannelAllocator>>,
}

impl SoundFontSynthBuilder {
    /// Create a new SoundFont synth builder
    pub fn new(
        synth: SoundFontSynthHandle,
        program: u8,
        name: impl Into<String>,
        channel_allocator: Arc<Mutex<ChannelAllocator>>,
    ) -> Self {
        Self {
            synth,
            program,
            name: name.into(),
            channel_allocator,
        }
    }
}

impl SynthBuilder for SoundFontSynthBuilder {
    fn build(&self, freq: f32, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, VoiceControls) {
        let note = freq_to_midi(freq);
        let velocity = (params.get("velocity").copied().unwrap_or(0.8) * 127.0) as u8;

        // Allocate a channel
        let channel = self
            .channel_allocator
            .lock()
            .map(|mut alloc| alloc.allocate().unwrap_or(0))
            .unwrap_or(0);

        // Create controls
        let amp = shared(params.get("amp").copied().unwrap_or(1.0));
        let pitch_bend = shared(1.0);
        let pressure = shared(0.0);

        let voice = SoundFontVoice::new(
            Arc::clone(&self.synth),
            channel,
            note,
            velocity,
            self.program,
            amp.clone(),
        );

        let controls = VoiceControls {
            amp,
            pitch_bend,
            cutoff: None,
            resonance: None,
            pressure,
        };

        (Box::new(voice), controls)
    }

    fn metadata(&self) -> SynthMetadata {
        let category = match self.program {
            0..=7 => "piano",
            8..=15 => "bell",
            16..=23 => "organ",
            24..=31 => "pluck",
            32..=39 => "bass",
            40..=47 => "strings",
            48..=55 => "pad",
            56..=63 => "brass",
            64..=71 => "reed",
            72..=79 => "pipe",
            80..=87 => "lead",
            88..=95 => "pad",
            96..=103 => "fx",
            104..=111 => "ethnic",
            112..=119 => "percussion",
            120..=127 => "sfx",
            _ => "other",
        };

        SynthMetadata::new(
            &self.name,
            format!(
                "GM Program {} - {}",
                self.program,
                super::GM_PROGRAM_NAMES[self.program as usize]
            ),
        )
        .with_param("velocity", 0.8, 0.0, 1.0)
        .with_param("amp", 1.0, 0.0, 2.0)
        .with_tags(["soundfont", "gm", category, "source:soundfont"])
    }
}

// ============================================================================
// Utility functions
// ============================================================================

/// Convert frequency to MIDI note number
pub fn freq_to_midi(freq: f32) -> u8 {
    let midi = 69.0 + 12.0 * (freq / 440.0).log2();
    midi.round().clamp(0.0, 127.0) as u8
}

/// Convert MIDI note number to frequency
pub fn midi_to_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

/// Register all 128 GM programs from a SoundFont into a SynthRegistry
///
/// Synth names are prefixed with "sf_" to avoid conflicts with built-in synths.
pub fn register_gm_programs(registry: &mut crate::synth::SynthRegistry, synth: SoundFontSynthHandle) {
    let channel_allocator = Arc::new(Mutex::new(ChannelAllocator::new()));

    for program in 0..128u8 {
        let name = format!("sf_{}", gm_program_to_name(program));
        let builder = Arc::new(SoundFontSynthBuilder::new(
            Arc::clone(&synth),
            program,
            super::GM_PROGRAM_NAMES[program as usize],
            Arc::clone(&channel_allocator),
        ));
        registry.register(&name, builder);
    }
}

/// Convert GM program number to a snake_case name
fn gm_program_to_name(program: u8) -> String {
    super::GM_PROGRAM_NAMES[program as usize]
        .to_lowercase()
        .replace([' ', '-', '(', ')'], "_")
        .replace("__", "_")
        .trim_end_matches('_')
        .to_string()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_to_midi() {
        assert_eq!(freq_to_midi(440.0), 69); // A4
        assert_eq!(freq_to_midi(261.63), 60); // C4
        assert_eq!(freq_to_midi(880.0), 81); // A5
    }

    #[test]
    fn test_midi_to_freq() {
        assert!((midi_to_freq(69) - 440.0).abs() < 0.01);
        assert!((midi_to_freq(60) - 261.63).abs() < 0.1);
        assert!((midi_to_freq(81) - 880.0).abs() < 0.01);
    }

    #[test]
    fn test_channel_allocator() {
        let mut alloc = ChannelAllocator::new();

        // Should skip channel 9
        for expected in [0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15] {
            assert_eq!(alloc.allocate(), Some(expected));
        }

        // After all channels used, should return 0
        assert_eq!(alloc.allocate(), Some(0));
    }

    #[test]
    fn test_channel_allocator_release() {
        let mut alloc = ChannelAllocator::new();

        let ch1 = alloc.allocate().unwrap();
        let ch2 = alloc.allocate().unwrap();

        assert_eq!(ch1, 0);
        assert_eq!(ch2, 1);

        // Release ch1
        alloc.release(ch1);

        // Next allocation should reuse ch1
        let ch3 = alloc.allocate().unwrap();
        assert_eq!(ch3, 0);
    }

    #[test]
    fn test_gm_program_names() {
        assert_eq!(gm_program_to_name(0), "acoustic_grand_piano");
        assert_eq!(gm_program_to_name(40), "violin");
    }

    #[test]
    fn test_soundfont_unit_creation() {
        let manager = SoundFontManager::new(44100);
        let unit = SoundFontUnit::new(manager);
        assert_eq!(unit.inputs(), 0);
        assert_eq!(unit.outputs(), 2);
    }
}
