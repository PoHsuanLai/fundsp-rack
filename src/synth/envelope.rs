//! Envelope generators for amplitude and modulation control
//!
//! This module provides ADSR (Attack, Decay, Sustain, Release) envelopes
//! for shaping synth parameters over time.

use fundsp::hacker32::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ADSR envelope parameters
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ADSR {
    /// Attack time in seconds (time to reach peak)
    pub attack: f32,
    /// Decay time in seconds (time to reach sustain level)
    pub decay: f32,
    /// Sustain level (0.0 to 1.0)
    pub sustain: f32,
    /// Release time in seconds (time to fade to zero after gate off)
    pub release: f32,
}

impl ADSR {
    /// Create a new ADSR envelope
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack: attack.max(0.001), // Minimum 1ms
            decay: decay.max(0.001),   // Minimum 1ms
            sustain: sustain.clamp(0.0, 1.0),
            release: release.max(0.001), // Minimum 1ms
        }
    }

    /// Create a pluck-style envelope (fast attack, no sustain)
    pub fn pluck() -> Self {
        Self::new(0.001, 0.1, 0.0, 0.5)
    }

    /// Create a pad-style envelope (slow attack and release)
    pub fn pad() -> Self {
        Self::new(0.5, 0.2, 0.8, 1.0)
    }

    /// Create a percussive envelope (fast attack, short decay, no sustain)
    pub fn percussive() -> Self {
        Self::new(0.001, 0.1, 0.0, 0.1)
    }

    /// Create a piano-style envelope
    pub fn piano() -> Self {
        Self::new(0.002, 0.1, 0.7, 0.3)
    }

    /// Create an organ-style envelope (instant on/off)
    pub fn organ() -> Self {
        Self::new(0.001, 0.001, 1.0, 0.05)
    }

    /// Create a bass-style envelope
    pub fn bass() -> Self {
        Self::new(0.005, 0.1, 0.5, 0.2)
    }
}

impl Default for ADSR {
    fn default() -> Self {
        // Standard synth envelope
        Self::new(0.01, 0.1, 0.7, 0.3)
    }
}

/// Simple AHD (Attack, Hold, Decay) envelope
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AHD {
    /// Attack time in seconds
    pub attack: f32,
    /// Hold time in seconds (sustain at peak)
    pub hold: f32,
    /// Decay time in seconds
    pub decay: f32,
}

impl AHD {
    /// Create a new AHD envelope
    pub fn new(attack: f32, hold: f32, decay: f32) -> Self {
        Self {
            attack: attack.max(0.001),
            hold: hold.max(0.0),
            decay: decay.max(0.001),
        }
    }
}

impl Default for AHD {
    fn default() -> Self {
        Self::new(0.01, 0.1, 0.3)
    }
}

/// AR (Attack, Release) envelope - simplest envelope
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AR {
    /// Attack time in seconds
    pub attack: f32,
    /// Release time in seconds
    pub release: f32,
}

impl AR {
    /// Create a new AR envelope
    pub fn new(attack: f32, release: f32) -> Self {
        Self {
            attack: attack.max(0.001),
            release: release.max(0.001),
        }
    }
}

impl Default for AR {
    fn default() -> Self {
        Self::new(0.01, 0.3)
    }
}

/// Envelope configuration that can be used in synths
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvelopeConfig {
    /// ADSR envelope
    ADSR(ADSR),
    /// AHD envelope
    AHD(AHD),
    /// AR envelope
    AR(AR),
    /// No envelope (always 1.0)
    None,
}

impl EnvelopeConfig {
    /// Get the total envelope time (attack + decay/hold + release)
    /// This is useful for estimating voice lifetime
    pub fn total_time(&self) -> f32 {
        match self {
            EnvelopeConfig::ADSR(adsr) => adsr.attack + adsr.decay + adsr.release,
            EnvelopeConfig::AHD(ahd) => ahd.attack + ahd.hold + ahd.decay,
            EnvelopeConfig::AR(ar) => ar.attack + ar.release,
            EnvelopeConfig::None => 0.0,
        }
    }

    /// Create an ADSR envelope generator using an LFO-style time function
    /// Returns an AudioNode that outputs the envelope value (0.0 to 1.0)
    /// Note: For full ADSR with gate control, integrate this in your DAW's voice manager
    pub fn create_time_based_envelope(adsr: ADSR) -> An<impl AudioNode> {
        // Create an ADSR using FunDSP's lfo/envelope function
        // The envelope function generates values based on time
        lfo(move |t| {
            // Simple ADSR logic using time
            if t < adsr.attack {
                // Attack phase: 0 -> 1
                t / adsr.attack
            } else if t < adsr.attack + adsr.decay {
                // Decay phase: 1 -> sustain
                let decay_t = (t - adsr.attack) / adsr.decay;
                1.0 - (1.0 - adsr.sustain) * decay_t
            } else {
                // Sustain phase
                adsr.sustain
            }
        })
    }
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        EnvelopeConfig::ADSR(ADSR::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adsr_creation() {
        let env = ADSR::new(0.1, 0.2, 0.7, 0.3);
        assert_eq!(env.attack, 0.1);
        assert_eq!(env.decay, 0.2);
        assert_eq!(env.sustain, 0.7);
        assert_eq!(env.release, 0.3);
    }

    #[test]
    fn test_adsr_clamps_sustain() {
        let env = ADSR::new(0.1, 0.2, 1.5, 0.3);
        assert_eq!(env.sustain, 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_adsr_presets() {
        let pluck = ADSR::pluck();
        assert!(pluck.attack < 0.01);
        assert_eq!(pluck.sustain, 0.0);

        let pad = ADSR::pad();
        assert!(pad.attack > 0.3);
        assert!(pad.release > 0.5);
    }

    #[test]
    fn test_envelope_total_time() {
        let adsr = ADSR::new(0.1, 0.2, 0.7, 0.3);
        let env = EnvelopeConfig::ADSR(adsr);
        assert_eq!(env.total_time(), 0.6);
    }
}
