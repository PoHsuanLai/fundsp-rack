//! Low-Frequency Oscillator (LFO) for modulation
//!
//! LFOs are used to modulate synth parameters like pitch, amplitude,
//! filter cutoff, etc. at sub-audio rates (typically 0.1 - 20 Hz).

use fundsp::hacker32::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// LFO waveform types
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LFOWaveform {
    /// Sine wave (smooth modulation)
    Sine,
    /// Triangle wave (linear ramp up/down)
    Triangle,
    /// Sawtooth wave (ramp up, instant reset)
    Sawtooth,
    /// Square wave (on/off modulation)
    Square,
    /// Random sample & hold
    Random,
}

/// LFO configuration
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LFOConfig {
    /// LFO rate in Hz (typically 0.1 - 20)
    pub rate: f32,
    /// Modulation depth (0.0 to 1.0)
    pub depth: f32,
    /// Waveform shape
    pub waveform: LFOWaveform,
    /// Initial phase offset (0.0 to 1.0)
    pub phase: f32,
}

impl LFOConfig {
    /// Create a new LFO configuration
    pub fn new(rate: f32, depth: f32, waveform: LFOWaveform) -> Self {
        Self {
            rate: rate.clamp(0.001, 100.0),
            depth: depth.clamp(0.0, 1.0),
            waveform,
            phase: 0.0,
        }
    }

    /// Create a slow sine LFO (good for vibrato)
    pub fn vibrato() -> Self {
        Self::new(5.0, 0.5, LFOWaveform::Sine)
    }

    /// Create a slow sine LFO (good for tremolo)
    pub fn tremolo() -> Self {
        Self::new(4.0, 0.3, LFOWaveform::Sine)
    }

    /// Create a fast sine LFO (good for chorus-like effects)
    pub fn chorus() -> Self {
        Self::new(0.3, 0.2, LFOWaveform::Sine)
    }

    /// Create a slow random LFO (good for evolving textures)
    pub fn random_slow() -> Self {
        Self::new(0.5, 0.5, LFOWaveform::Random)
    }

    /// Create a triangle LFO for filter sweeps
    pub fn filter_sweep() -> Self {
        Self::new(0.2, 0.7, LFOWaveform::Triangle)
    }
}

impl Default for LFOConfig {
    fn default() -> Self {
        Self::new(2.0, 0.5, LFOWaveform::Sine)
    }
}

/// Create an LFO AudioNode from configuration
/// Returns a mono signal that oscillates between -depth and +depth
pub fn create_lfo_sine(rate: f32, depth: f32) -> An<impl AudioNode> {
    sine_hz(rate) * depth
}

/// Create a triangle LFO
pub fn create_lfo_triangle(rate: f32, depth: f32) -> An<impl AudioNode> {
    triangle_hz(rate) * depth
}

/// Create a sawtooth LFO
pub fn create_lfo_sawtooth(rate: f32, depth: f32) -> An<impl AudioNode> {
    saw_hz(rate) * depth
}

/// Create a square LFO
pub fn create_lfo_square(rate: f32, depth: f32) -> An<impl AudioNode> {
    square_hz(rate) * depth
}

/// Create a random sample & hold LFO
pub fn create_lfo_random(rate: f32, depth: f32) -> An<impl AudioNode> {
    (noise() >> hold_hz(rate, 0.0)) * depth
}

/// LFO modulation target - what parameter to modulate
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LFOTarget {
    /// Modulate pitch (frequency)
    Pitch,
    /// Modulate amplitude
    Amplitude,
    /// Modulate filter cutoff
    FilterCutoff,
    /// Modulate filter resonance
    FilterResonance,
    /// Modulate pulse width (for pulse waves)
    PulseWidth,
    /// Modulate pan position
    Pan,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfo_config_creation() {
        let lfo = LFOConfig::new(5.0, 0.8, LFOWaveform::Sine);
        assert_eq!(lfo.rate, 5.0);
        assert_eq!(lfo.depth, 0.8);
        assert_eq!(lfo.waveform, LFOWaveform::Sine);
    }

    #[test]
    fn test_lfo_config_clamps() {
        let lfo = LFOConfig::new(150.0, 1.5, LFOWaveform::Sine);
        assert_eq!(lfo.rate, 100.0); // Clamped
        assert_eq!(lfo.depth, 1.0); // Clamped
    }

    #[test]
    fn test_lfo_presets() {
        let vibrato = LFOConfig::vibrato();
        assert!(vibrato.rate > 3.0);
        assert_eq!(vibrato.waveform, LFOWaveform::Sine);

        let random = LFOConfig::random_slow();
        assert_eq!(random.waveform, LFOWaveform::Random);
    }
}
