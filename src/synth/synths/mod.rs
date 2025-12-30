//! Built-in synthesizer implementations
//!
//! This module contains all the built-in synth builders organized by category:
//!
//! - `basic` - Basic waveforms (sine, saw, square, triangle, pulse)
//! - `analog` - Analog synth emulations (TB-303, Prophet, Supersaw, Hoover)
//! - `modulated` - LFO-modulated oscillators
//! - `detuned` - Detuned oscillator stacks
//! - `fm` - FM synthesis
//! - `bells` - Bell-like sounds
//! - `keys` - Keyboard instruments (organ, electric piano)
//! - `leads` - Lead synths (mono lead, sub, brass)
//! - `pads` - Pad sounds (strings, warm pad)
//! - `physical` - Physical modeling (piano, pluck)
//! - `bass` - Bass synthesizers
//! - `ambient` - Ambient and pad sounds
//! - `tech` - Tech/trance/electronic sounds
//! - `noise` - Noise generators

pub mod ambient;
pub mod analog;
pub mod basic;
pub mod bass;
pub mod bells;
pub mod detuned;
pub mod fm;
pub mod keys;
pub mod leads;
pub mod modulated;
pub mod noise;
pub mod pads;
pub mod physical;
pub mod tech;

// Re-export all synth builders for easy access
pub use ambient::*;
pub use analog::*;
pub use basic::*;
pub use bass::*;
pub use bells::*;
pub use detuned::*;
pub use fm::*;
pub use keys::*;
pub use leads::*;
pub use modulated::*;
pub use noise::*;
pub use pads::*;
pub use physical::*;
pub use tech::*;
