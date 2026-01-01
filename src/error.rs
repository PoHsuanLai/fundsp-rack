//! Unified error types for fundsp-rack

use std::fmt;

/// Result type for fundsp-rack operations
pub type Result<T> = std::result::Result<T, Error>;

/// Unified error type for all fundsp-rack operations
#[derive(Debug, Clone)]
pub enum Error {
    /// Synth not found in registry
    InvalidSynth(String),
    /// Effect not found in registry
    InvalidEffect(String),
    /// Invalid parameter name
    InvalidParameter(String),
    /// Invalid parameter value
    InvalidValue {
        param: String,
        value: f32,
        reason: String,
    },
    /// Effect chain error
    ChainError(String),
    /// Index out of bounds
    IndexOutOfBounds { index: usize, len: usize },
    /// Serialization error
    #[cfg(feature = "serde")]
    SerializationError(String),
    /// SoundFont loading/playback error
    #[cfg(feature = "soundfont")]
    SoundFontError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSynth(name) => write!(f, "synth not found: '{}'", name),
            Error::InvalidEffect(name) => write!(f, "effect not found: '{}'", name),
            Error::InvalidParameter(name) => write!(f, "invalid parameter: '{}'", name),
            Error::InvalidValue {
                param,
                value,
                reason,
            } => {
                write!(f, "invalid value {} for '{}': {}", value, param, reason)
            }
            Error::ChainError(msg) => write!(f, "effect chain error: {}", msg),
            Error::IndexOutOfBounds { index, len } => {
                write!(f, "index {} out of bounds (len: {})", index, len)
            }
            #[cfg(feature = "serde")]
            Error::SerializationError(msg) => write!(f, "serialization error: {}", msg),
            #[cfg(feature = "soundfont")]
            Error::SoundFontError(msg) => write!(f, "soundfont error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
