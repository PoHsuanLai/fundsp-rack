//! SoundFont Manager - loads and manages SF2 files

use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::{Error, Result};

/// Manager for SoundFont-based synthesis
///
/// Wraps rustysynth's Synthesizer with additional features:
/// - Multiple SoundFont loading with fallback
/// - Per-channel program/bank management
/// - Volume/pan control per channel
#[derive(Clone)]
pub struct SoundFontManager {
    /// The underlying synthesizer
    synth: Option<Synthesizer>,
    /// Loaded SoundFont (Arc for sharing)
    soundfont: Option<Arc<SoundFont>>,
    /// Sample rate
    sample_rate: u32,
    /// Current program for each channel (0-15)
    channel_programs: [u8; 16],
    /// Current bank for each channel
    channel_banks: [u8; 16],
    /// Master volume (0.0 - 1.0)
    master_volume: f32,
    /// Path to loaded soundfont (for reference)
    soundfont_path: Option<String>,
}

impl SoundFontManager {
    /// Create a new SoundFont manager
    ///
    /// # Arguments
    /// * `sample_rate` - Audio sample rate (e.g., 44100, 48000)
    pub fn new(sample_rate: u32) -> Self {
        Self {
            synth: None,
            soundfont: None,
            sample_rate,
            channel_programs: [0; 16],
            channel_banks: [0; 16],
            master_volume: 1.0,
            soundfont_path: None,
        }
    }

    /// Check if a SoundFont is loaded
    pub fn is_loaded(&self) -> bool {
        self.synth.is_some()
    }

    /// Get the path to the currently loaded SoundFont
    pub fn soundfont_path(&self) -> Option<&str> {
        self.soundfont_path.as_deref()
    }

    /// Load a SoundFont file
    ///
    /// # Arguments
    /// * `path` - Path to the SF2 file
    ///
    /// # Example
    /// ```rust,no_run
    /// use fundsp_rack::soundfont::SoundFontManager;
    ///
    /// let mut manager = SoundFontManager::new(44100);
    /// manager.load_soundfont("TimGM6mb.sf2").unwrap();
    /// ```
    pub fn load_soundfont<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path_ref = path.as_ref();
        let path_str = path_ref.display().to_string();

        info!("Loading SoundFont: {}", path_str);

        let mut file = File::open(path_ref)
            .map_err(|e| Error::SoundFontError(format!("Failed to open SF2 file: {}", e)))?;

        let soundfont = SoundFont::new(&mut file)
            .map_err(|e| Error::SoundFontError(format!("Failed to parse SF2 file: {:?}", e)))?;

        let soundfont = Arc::new(soundfont);

        // Create synthesizer settings
        let settings = SynthesizerSettings::new(self.sample_rate as i32);

        // Create synthesizer
        let synth = Synthesizer::new(&soundfont, &settings)
            .map_err(|e| Error::SoundFontError(format!("Failed to create synthesizer: {:?}", e)))?;

        self.soundfont = Some(soundfont);
        self.synth = Some(synth);
        self.soundfont_path = Some(path_str.clone());

        info!("SoundFont loaded successfully: {}", path_str);
        Ok(())
    }

    /// Get the underlying SoundFont (if loaded)
    pub fn soundfont(&self) -> Option<&Arc<SoundFont>> {
        self.soundfont.as_ref()
    }

    /// Set master volume (0.0 - 1.0)
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 2.0);
    }

    /// Get master volume
    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    /// Set program (instrument) for a channel
    ///
    /// # Arguments
    /// * `channel` - MIDI channel (0-15)
    /// * `program` - GM program number (0-127)
    pub fn program_change(&mut self, channel: u8, program: u8) {
        if channel >= 16 {
            warn!("Invalid channel: {}", channel);
            return;
        }

        self.channel_programs[channel as usize] = program;

        if let Some(synth) = &mut self.synth {
            synth.process_midi_message(channel as i32, 0xC0, program as i32, 0);
            debug!("Channel {} program changed to {}", channel, program);
        }
    }

    /// Set bank for a channel
    ///
    /// # Arguments
    /// * `channel` - MIDI channel (0-15)
    /// * `bank` - Bank number (0-127, typically 0 for GM)
    pub fn bank_select(&mut self, channel: u8, bank: u8) {
        if channel >= 16 {
            return;
        }

        self.channel_banks[channel as usize] = bank;

        if let Some(synth) = &mut self.synth {
            // CC 0 = Bank Select MSB
            synth.process_midi_message(channel as i32, 0xB0, 0, bank as i32);
        }
    }

    /// Send note on event
    ///
    /// # Arguments
    /// * `channel` - MIDI channel (0-15, channel 9 is drums)
    /// * `note` - MIDI note number (0-127)
    /// * `velocity` - Note velocity (0-127)
    pub fn note_on(&mut self, channel: u8, note: u8, velocity: u8) {
        if let Some(synth) = &mut self.synth {
            synth.note_on(channel as i32, note as i32, velocity as i32);
            debug!("Note on: ch={} note={} vel={}", channel, note, velocity);
        }
    }

    /// Send note off event
    ///
    /// # Arguments
    /// * `channel` - MIDI channel (0-15)
    /// * `note` - MIDI note number (0-127)
    pub fn note_off(&mut self, channel: u8, note: u8) {
        if let Some(synth) = &mut self.synth {
            synth.note_off(channel as i32, note as i32);
            debug!("Note off: ch={} note={}", channel, note);
        }
    }

    /// Send all notes off for a channel
    pub fn all_notes_off(&mut self, channel: u8) {
        if let Some(synth) = &mut self.synth {
            // CC 123 = All Notes Off
            synth.process_midi_message(channel as i32, 0xB0, 123, 0);
        }
    }

    /// Send all sound off for a channel (immediate silence)
    pub fn all_sound_off(&mut self, channel: u8) {
        if let Some(synth) = &mut self.synth {
            // CC 120 = All Sound Off
            synth.process_midi_message(channel as i32, 0xB0, 120, 0);
        }
    }

    /// Reset all controllers on a channel
    pub fn reset_controllers(&mut self, channel: u8) {
        if let Some(synth) = &mut self.synth {
            // CC 121 = Reset All Controllers
            synth.process_midi_message(channel as i32, 0xB0, 121, 0);
        }
    }

    /// Send pitch bend
    ///
    /// # Arguments
    /// * `channel` - MIDI channel (0-15)
    /// * `value` - Pitch bend value (0-16383, 8192 = center)
    pub fn pitch_bend(&mut self, channel: u8, value: u16) {
        if let Some(synth) = &mut self.synth {
            let lsb = (value & 0x7F) as i32;
            let msb = ((value >> 7) & 0x7F) as i32;
            synth.process_midi_message(channel as i32, 0xE0, lsb, msb);
        }
    }

    /// Send control change
    ///
    /// # Arguments
    /// * `channel` - MIDI channel (0-15)
    /// * `control` - Controller number (0-127)
    /// * `value` - Controller value (0-127)
    pub fn control_change(&mut self, channel: u8, control: u8, value: u8) {
        if let Some(synth) = &mut self.synth {
            synth.process_midi_message(channel as i32, 0xB0, control as i32, value as i32);
        }
    }

    /// Set channel volume (CC 7)
    pub fn set_channel_volume(&mut self, channel: u8, volume: u8) {
        self.control_change(channel, 7, volume);
    }

    /// Set channel pan (CC 10)
    pub fn set_channel_pan(&mut self, channel: u8, pan: u8) {
        self.control_change(channel, 10, pan);
    }

    /// Set channel expression (CC 11)
    pub fn set_channel_expression(&mut self, channel: u8, expression: u8) {
        self.control_change(channel, 11, expression);
    }

    /// Render audio samples
    ///
    /// # Arguments
    /// * `left` - Left channel output buffer
    /// * `right` - Right channel output buffer
    ///
    /// Both buffers must have the same length.
    pub fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        debug_assert_eq!(left.len(), right.len());

        if let Some(synth) = &mut self.synth {
            synth.render(left, right);

            // Apply master volume
            if (self.master_volume - 1.0).abs() > 0.001 {
                for sample in left.iter_mut() {
                    *sample *= self.master_volume;
                }
                for sample in right.iter_mut() {
                    *sample *= self.master_volume;
                }
            }
        } else {
            // No synth loaded, output silence
            left.fill(0.0);
            right.fill(0.0);
        }
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get the current program for a channel
    pub fn channel_program(&self, channel: u8) -> u8 {
        self.channel_programs.get(channel as usize).copied().unwrap_or(0)
    }

    /// Reset the synthesizer to initial state
    pub fn reset(&mut self) {
        if let Some(synth) = &mut self.synth {
            // Reset all channels
            for ch in 0..16 {
                synth.process_midi_message(ch, 0xB0, 121, 0); // Reset controllers
                synth.process_midi_message(ch, 0xB0, 123, 0); // All notes off
            }
        }
        self.channel_programs = [0; 16];
        self.channel_banks = [0; 16];
    }

    /// List available presets in the loaded SoundFont
    pub fn list_presets(&self) -> Vec<(u8, u8, String)> {
        // Note: rustysynth doesn't expose preset enumeration directly
        // We return the standard GM preset names as a reference
        (0..128)
            .map(|p| (0, p, super::GM_PROGRAM_NAMES[p as usize].to_string()))
            .collect()
    }
}

impl Default for SoundFontManager {
    fn default() -> Self {
        Self::new(44100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = SoundFontManager::new(48000);
        assert_eq!(manager.sample_rate(), 48000);
        assert!(!manager.is_loaded());
    }

    #[test]
    fn test_render_without_soundfont() {
        let mut manager = SoundFontManager::new(44100);
        let mut left = vec![1.0f32; 256];
        let mut right = vec![1.0f32; 256];

        manager.render(&mut left, &mut right);

        // Should output silence when no soundfont is loaded
        assert!(left.iter().all(|&s| s == 0.0));
        assert!(right.iter().all(|&s| s == 0.0));
    }
}
