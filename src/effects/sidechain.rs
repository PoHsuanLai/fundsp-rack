//! Sidechain-aware effect trait and implementations
//!
//! Effects that can be controlled by external audio signals (sidechain compression, gating, etc.)

use fundsp::hacker32::*;
use std::collections::HashMap;

/// Trait for effects that can process audio with an external sidechain signal
///
/// This extends the standard AudioUnit trait to allow effects to respond to
/// an external control signal (e.g., compressor controlled by kick drum).
pub trait SidechainAwareEffect: AudioUnit {
    /// Process stereo audio with sidechain input
    ///
    /// # Arguments
    /// * `input_left` - Left channel input sample
    /// * `input_right` - Right channel input sample
    /// * `sidechain_left` - Left channel sidechain control signal
    /// * `sidechain_right` - Right channel sidechain control signal
    ///
    /// # Returns
    /// Processed stereo output (left, right)
    fn process_with_sidechain(
        &mut self,
        input_left: f32,
        input_right: f32,
        sidechain_left: f32,
        sidechain_right: f32,
    ) -> (f32, f32);
}

/// Helper function to detect peak level from stereo sidechain signal
#[inline]
pub fn sidechain_peak(left: f32, right: f32) -> f32 {
    left.abs().max(right.abs())
}

/// Helper function to detect RMS level from stereo sidechain signal
#[inline]
pub fn sidechain_rms(left: f32, right: f32) -> f32 {
    ((left * left + right * right) * 0.5).sqrt()
}

/// Convert linear amplitude to decibels
#[inline]
pub fn amplitude_to_db(amplitude: f32) -> f32 {
    20.0 * amplitude.max(1e-10).log10()
}

/// Convert decibels to linear amplitude
#[inline]
pub fn db_to_amplitude(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Sidechain Compressor - compresses audio based on external sidechain signal
#[derive(Clone)]
pub struct SidechainCompressor {
    /// Threshold in dB (when sidechain exceeds this, compression happens)
    pub threshold: Shared,
    /// Compression ratio (e.g., 4.0 = 4:1)
    pub ratio: Shared,
    /// Attack time coefficient
    pub attack_coeff: Shared,
    /// Release time coefficient
    pub release_coeff: Shared,
    /// Current envelope level (for smooth attack/release)
    envelope: Shared,
}

impl SidechainCompressor {
    /// Create a new sidechain compressor
    pub fn new(
        threshold_db: f32,
        ratio: f32,
        attack_sec: f32,
        release_sec: f32,
        sample_rate: f32,
    ) -> Self {
        // Calculate attack/release coefficients
        let attack_coeff = (-1.0 / (attack_sec * sample_rate)).exp();
        let release_coeff = (-1.0 / (release_sec * sample_rate)).exp();

        Self {
            threshold: shared(threshold_db),
            ratio: shared(ratio),
            attack_coeff: shared(attack_coeff),
            release_coeff: shared(release_coeff),
            envelope: shared(0.0),
        }
    }

    /// Calculate gain reduction based on sidechain level
    #[inline]
    fn calculate_gain_reduction(&mut self, sidechain_level: f32) -> f32 {
        let threshold = self.threshold.value();
        let ratio = self.ratio.value();
        let attack_coeff = self.attack_coeff.value();
        let release_coeff = self.release_coeff.value();
        let mut envelope = self.envelope.value();

        // Convert sidechain level to dB
        let sidechain_db = amplitude_to_db(sidechain_level);

        // Calculate target envelope level
        let target_envelope = if sidechain_db > threshold {
            // Above threshold: apply compression
            sidechain_level
        } else {
            0.0
        };

        // Smooth envelope follower (attack/release)
        let coeff = if target_envelope > envelope {
            attack_coeff
        } else {
            release_coeff
        };

        envelope = target_envelope + coeff * (envelope - target_envelope);
        self.envelope.set_value(envelope);

        // Calculate gain reduction
        if envelope > 0.0 {
            let envelope_db = amplitude_to_db(envelope);
            if envelope_db > threshold {
                // Amount over threshold
                let over_db = envelope_db - threshold;
                // Apply ratio
                let gain_reduction_db = over_db * (1.0 - 1.0 / ratio);
                // Convert to linear gain
                db_to_amplitude(-gain_reduction_db)
            } else {
                1.0 // No reduction
            }
        } else {
            1.0 // No reduction
        }
    }
}

impl AudioUnit for SidechainCompressor {
    fn inputs(&self) -> usize {
        2
    }
    fn outputs(&self) -> usize {
        2
    }

    fn reset(&mut self) {
        self.envelope.set_value(0.0);
    }

    fn tick(&mut self, input: &[f32], output: &mut [f32]) {
        // Without sidechain, just pass through
        output[0] = input[0];
        output[1] = input[1];
    }

    fn process(&mut self, size: usize, input: &BufferRef, output: &mut BufferMut) {
        for i in 0..size {
            output.set(0, i, input.at(0, i));
            output.set(1, i, input.at(1, i));
        }
    }

    fn route(&mut self, input: &SignalFrame, _frequency: f64) -> SignalFrame {
        // Pass through routing - just copy input signals to output
        input.clone()
    }

    fn get_id(&self) -> u64 {
        const ID: &[u8] = b"sidechain_compressor";
        let mut hash = 0u64;
        for &byte in ID {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    fn footprint(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl SidechainAwareEffect for SidechainCompressor {
    fn process_with_sidechain(
        &mut self,
        input_left: f32,
        input_right: f32,
        sidechain_left: f32,
        sidechain_right: f32,
    ) -> (f32, f32) {
        // Detect sidechain level (using peak)
        let sidechain_level = sidechain_peak(sidechain_left, sidechain_right);

        // Calculate gain reduction
        let gain = self.calculate_gain_reduction(sidechain_level);

        // Apply gain reduction to input
        (input_left * gain, input_right * gain)
    }
}

/// Sidechain Gate - mutes audio when sidechain signal is below threshold
#[derive(Clone)]
pub struct SidechainGate {
    /// Threshold in dB (when sidechain is below this, gate closes)
    pub threshold: Shared,
    /// Attack time coefficient
    pub attack_coeff: Shared,
    /// Release time coefficient
    pub release_coeff: Shared,
    /// Current gate state (0.0 = closed, 1.0 = open)
    gate_state: Shared,
}

impl SidechainGate {
    /// Create a new sidechain gate
    pub fn new(threshold_db: f32, attack_sec: f32, release_sec: f32, sample_rate: f32) -> Self {
        // Calculate attack/release coefficients
        let attack_coeff = (-1.0 / (attack_sec * sample_rate)).exp();
        let release_coeff = (-1.0 / (release_sec * sample_rate)).exp();

        Self {
            threshold: shared(threshold_db),
            attack_coeff: shared(attack_coeff),
            release_coeff: shared(release_coeff),
            gate_state: shared(0.0),
        }
    }

    /// Calculate gate gain based on sidechain level
    #[inline]
    fn calculate_gate_gain(&mut self, sidechain_level: f32) -> f32 {
        let threshold = self.threshold.value();
        let attack_coeff = self.attack_coeff.value();
        let release_coeff = self.release_coeff.value();
        let mut gate_state = self.gate_state.value();

        // Convert sidechain level to dB
        let sidechain_db = amplitude_to_db(sidechain_level);

        // Calculate target gate state
        let target_state = if sidechain_db > threshold {
            1.0 // Gate open
        } else {
            0.0 // Gate closed
        };

        // Smooth gate state (attack/release)
        let coeff = if target_state > gate_state {
            attack_coeff // Opening
        } else {
            release_coeff // Closing
        };

        gate_state = target_state + coeff * (gate_state - target_state);
        self.gate_state.set_value(gate_state);

        gate_state
    }
}

impl AudioUnit for SidechainGate {
    fn inputs(&self) -> usize {
        2
    }
    fn outputs(&self) -> usize {
        2
    }

    fn reset(&mut self) {
        self.gate_state.set_value(0.0);
    }

    fn tick(&mut self, _input: &[f32], output: &mut [f32]) {
        // Without sidechain, gate is closed (muted)
        output[0] = 0.0;
        output[1] = 0.0;
    }

    fn process(&mut self, size: usize, input: &BufferRef, output: &mut BufferMut) {
        for i in 0..size {
            // Without sidechain, pass through (or optionally gate closed)
            output.set(0, i, input.at(0, i));
            output.set(1, i, input.at(1, i));
        }
    }

    fn route(&mut self, input: &SignalFrame, _frequency: f64) -> SignalFrame {
        // Pass through routing
        input.clone()
    }

    fn get_id(&self) -> u64 {
        const ID: &[u8] = b"sidechain_gate";
        let mut hash = 0u64;
        for &byte in ID {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    fn footprint(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl SidechainAwareEffect for SidechainGate {
    fn process_with_sidechain(
        &mut self,
        input_left: f32,
        input_right: f32,
        sidechain_left: f32,
        sidechain_right: f32,
    ) -> (f32, f32) {
        // Detect sidechain level (using peak)
        let sidechain_level = sidechain_peak(sidechain_left, sidechain_right);

        // Calculate gate gain (0.0 = closed, 1.0 = open)
        let gain = self.calculate_gate_gain(sidechain_level);

        // Apply gate to input
        (input_left * gain, input_right * gain)
    }
}

/// Helper function to build a sidechain effect by name
pub fn build_sidechain_effect(
    name: &str,
    params: &HashMap<String, f32>,
    sample_rate: f32,
) -> Option<Box<dyn SidechainAwareEffect>> {
    match name {
        "sidechain_compressor" => {
            let threshold = params.get("threshold").copied().unwrap_or(-20.0);
            let ratio = params.get("ratio").copied().unwrap_or(4.0);
            let attack = params.get("attack").copied().unwrap_or(0.01);
            let release = params.get("release").copied().unwrap_or(0.1);
            Some(Box::new(SidechainCompressor::new(
                threshold,
                ratio,
                attack,
                release,
                sample_rate,
            )))
        }
        "sidechain_gate" => {
            let threshold = params.get("threshold").copied().unwrap_or(-40.0);
            let attack = params.get("attack").copied().unwrap_or(0.001);
            let release = params.get("release").copied().unwrap_or(0.1);
            Some(Box::new(SidechainGate::new(
                threshold,
                attack,
                release,
                sample_rate,
            )))
        }
        _ => None,
    }
}
