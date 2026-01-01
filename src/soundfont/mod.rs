//! SoundFont support for realistic instrument samples
//!
//! This module provides SoundFont (SF2) loading and playback integration
//! with the synth registry. SoundFont presets are mapped to General MIDI
//! program numbers, providing realistic instrument samples.
//!
//! # Features
//!
//! - Load SF2 files with full GM instrument support
//! - Sample-accurate playback with proper loop handling
//! - Integration with SynthRegistry for seamless program change
//! - Per-voice real-time control (amplitude, pitch bend)
//!
//! # Example
//!
//! ```rust,no_run
//! use fundsp_rack::soundfont::SoundFontManager;
//!
//! let mut manager = SoundFontManager::new(44100);
//! manager.load_soundfont("path/to/soundfont.sf2").unwrap();
//!
//! // Play a note (channel 0, note 60, velocity 100)
//! manager.note_on(0, 60, 100);
//!
//! // Render audio
//! let mut left = vec![0.0f32; 1024];
//! let mut right = vec![0.0f32; 1024];
//! manager.render(&mut left, &mut right);
//! ```

mod manager;
mod synth_adapter;

pub use manager::SoundFontManager;
pub use synth_adapter::{
    create_soundfont_synth, freq_to_midi, midi_to_freq, register_gm_programs, ChannelAllocator,
    SoundFontPlayer, SoundFontSynthBuilder, SoundFontSynthHandle, SoundFontUnit, SoundFontVoice,
};

/// General MIDI program names (128 programs)
pub static GM_PROGRAM_NAMES: [&str; 128] = [
    // Piano (0-7)
    "Acoustic Grand Piano",
    "Bright Acoustic Piano",
    "Electric Grand Piano",
    "Honky-tonk Piano",
    "Electric Piano 1",
    "Electric Piano 2",
    "Harpsichord",
    "Clavinet",
    // Chromatic Percussion (8-15)
    "Celesta",
    "Glockenspiel",
    "Music Box",
    "Vibraphone",
    "Marimba",
    "Xylophone",
    "Tubular Bells",
    "Dulcimer",
    // Organ (16-23)
    "Drawbar Organ",
    "Percussive Organ",
    "Rock Organ",
    "Church Organ",
    "Reed Organ",
    "Accordion",
    "Harmonica",
    "Tango Accordion",
    // Guitar (24-31)
    "Acoustic Guitar (nylon)",
    "Acoustic Guitar (steel)",
    "Electric Guitar (jazz)",
    "Electric Guitar (clean)",
    "Electric Guitar (muted)",
    "Overdriven Guitar",
    "Distortion Guitar",
    "Guitar Harmonics",
    // Bass (32-39)
    "Acoustic Bass",
    "Electric Bass (finger)",
    "Electric Bass (pick)",
    "Fretless Bass",
    "Slap Bass 1",
    "Slap Bass 2",
    "Synth Bass 1",
    "Synth Bass 2",
    // Strings (40-47)
    "Violin",
    "Viola",
    "Cello",
    "Contrabass",
    "Tremolo Strings",
    "Pizzicato Strings",
    "Orchestral Harp",
    "Timpani",
    // Ensemble (48-55)
    "String Ensemble 1",
    "String Ensemble 2",
    "Synth Strings 1",
    "Synth Strings 2",
    "Choir Aahs",
    "Voice Oohs",
    "Synth Voice",
    "Orchestra Hit",
    // Brass (56-63)
    "Trumpet",
    "Trombone",
    "Tuba",
    "Muted Trumpet",
    "French Horn",
    "Brass Section",
    "Synth Brass 1",
    "Synth Brass 2",
    // Reed (64-71)
    "Soprano Sax",
    "Alto Sax",
    "Tenor Sax",
    "Baritone Sax",
    "Oboe",
    "English Horn",
    "Bassoon",
    "Clarinet",
    // Pipe (72-79)
    "Piccolo",
    "Flute",
    "Recorder",
    "Pan Flute",
    "Blown Bottle",
    "Shakuhachi",
    "Whistle",
    "Ocarina",
    // Synth Lead (80-87)
    "Lead 1 (square)",
    "Lead 2 (sawtooth)",
    "Lead 3 (calliope)",
    "Lead 4 (chiff)",
    "Lead 5 (charang)",
    "Lead 6 (voice)",
    "Lead 7 (fifths)",
    "Lead 8 (bass + lead)",
    // Synth Pad (88-95)
    "Pad 1 (new age)",
    "Pad 2 (warm)",
    "Pad 3 (polysynth)",
    "Pad 4 (choir)",
    "Pad 5 (bowed)",
    "Pad 6 (metallic)",
    "Pad 7 (halo)",
    "Pad 8 (sweep)",
    // Synth Effects (96-103)
    "FX 1 (rain)",
    "FX 2 (soundtrack)",
    "FX 3 (crystal)",
    "FX 4 (atmosphere)",
    "FX 5 (brightness)",
    "FX 6 (goblins)",
    "FX 7 (echoes)",
    "FX 8 (sci-fi)",
    // Ethnic (104-111)
    "Sitar",
    "Banjo",
    "Shamisen",
    "Koto",
    "Kalimba",
    "Bagpipe",
    "Fiddle",
    "Shanai",
    // Percussive (112-119)
    "Tinkle Bell",
    "Agogo",
    "Steel Drums",
    "Woodblock",
    "Taiko Drum",
    "Melodic Tom",
    "Synth Drum",
    "Reverse Cymbal",
    // Sound Effects (120-127)
    "Guitar Fret Noise",
    "Breath Noise",
    "Seashore",
    "Bird Tweet",
    "Telephone Ring",
    "Helicopter",
    "Applause",
    "Gunshot",
];

/// Get the GM program name for a given program number
pub fn gm_program_name(program: u8) -> &'static str {
    GM_PROGRAM_NAMES.get(program as usize).unwrap_or(&"Unknown")
}

/// GM Drum kit note names (channel 10, notes 35-81)
pub static GM_DRUM_NOTES: [(u8, &str); 47] = [
    (35, "Acoustic Bass Drum"),
    (36, "Bass Drum 1"),
    (37, "Side Stick"),
    (38, "Acoustic Snare"),
    (39, "Hand Clap"),
    (40, "Electric Snare"),
    (41, "Low Floor Tom"),
    (42, "Closed Hi-Hat"),
    (43, "High Floor Tom"),
    (44, "Pedal Hi-Hat"),
    (45, "Low Tom"),
    (46, "Open Hi-Hat"),
    (47, "Low-Mid Tom"),
    (48, "Hi-Mid Tom"),
    (49, "Crash Cymbal 1"),
    (50, "High Tom"),
    (51, "Ride Cymbal 1"),
    (52, "Chinese Cymbal"),
    (53, "Ride Bell"),
    (54, "Tambourine"),
    (55, "Splash Cymbal"),
    (56, "Cowbell"),
    (57, "Crash Cymbal 2"),
    (58, "Vibraslap"),
    (59, "Ride Cymbal 2"),
    (60, "Hi Bongo"),
    (61, "Low Bongo"),
    (62, "Mute Hi Conga"),
    (63, "Open Hi Conga"),
    (64, "Low Conga"),
    (65, "High Timbale"),
    (66, "Low Timbale"),
    (67, "High Agogo"),
    (68, "Low Agogo"),
    (69, "Cabasa"),
    (70, "Maracas"),
    (71, "Short Whistle"),
    (72, "Long Whistle"),
    (73, "Short Guiro"),
    (74, "Long Guiro"),
    (75, "Claves"),
    (76, "Hi Wood Block"),
    (77, "Low Wood Block"),
    (78, "Mute Cuica"),
    (79, "Open Cuica"),
    (80, "Mute Triangle"),
    (81, "Open Triangle"),
];

/// Get the drum name for a GM drum note (channel 10)
pub fn gm_drum_name(note: u8) -> Option<&'static str> {
    GM_DRUM_NOTES.iter().find(|(n, _)| *n == note).map(|(_, name)| *name)
}
