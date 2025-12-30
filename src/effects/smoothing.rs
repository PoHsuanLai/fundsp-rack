//! Parameter smoothing to prevent audio clicks
//!
//! When parameters change suddenly, they can cause audible clicks/pops.
//! This module provides smoothing to gradually transition between values.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// A smoothed parameter that gradually transitions to target values
pub struct SmoothedParam {
    /// Current smoothed value
    current: f32,
    /// Target value we're smoothing toward
    target: Arc<AtomicU64>,
    /// Smoothing time constant (samples to reach ~63% of target)
    time_constant: f32,
    /// Coefficient calculated from time constant
    coefficient: f32,
}

impl SmoothedParam {
    /// Create a new smoothed parameter
    ///
    /// # Arguments
    /// * `initial_value` - Starting value
    /// * `smoothing_ms` - Smoothing time in milliseconds
    /// * `sample_rate` - Sample rate in Hz
    pub fn new(initial_value: f32, smoothing_ms: f32, sample_rate: f32) -> Self {
        let time_constant = smoothing_ms * sample_rate / 1000.0;
        let coefficient = (-1.0 / time_constant).exp();

        Self {
            current: initial_value,
            target: Arc::new(AtomicU64::from(initial_value.to_bits() as u64)),
            time_constant,
            coefficient,
        }
    }

    /// Set a new target value (will smoothly transition)
    pub fn set_target(&self, value: f32) {
        self.target.store(value.to_bits() as u64, Ordering::Relaxed);
    }

    /// Get the target value handle for external control
    pub fn target_handle(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.target)
    }

    /// Get next smoothed value (call once per sample)
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> f32 {
        let target_bits = self.target.load(Ordering::Relaxed);
        let target = f32::from_bits(target_bits as u32);

        // Exponential smoothing: current += (target - current) * (1 - coefficient)
        self.current += (target - self.current) * (1.0 - self.coefficient);

        self.current
    }

    /// Get current value without advancing
    #[inline]
    pub fn current(&self) -> f32 {
        self.current
    }

    /// Check if we've reached the target (within threshold)
    #[inline]
    pub fn is_settled(&self, threshold: f32) -> bool {
        let target_bits = self.target.load(Ordering::Relaxed);
        let target = f32::from_bits(target_bits as u32);
        (self.current - target).abs() < threshold
    }

    /// Force current value to target (no smoothing)
    pub fn snap_to_target(&mut self) {
        let target_bits = self.target.load(Ordering::Relaxed);
        self.current = f32::from_bits(target_bits as u32);
    }

    /// Update sample rate (recalculates coefficient)
    pub fn set_sample_rate(&mut self, sample_rate: f32, smoothing_ms: f32) {
        self.time_constant = smoothing_ms * sample_rate / 1000.0;
        self.coefficient = (-1.0 / self.time_constant).exp();
    }
}

/// Builder for SmoothedParam with a fluent API
pub struct SmoothedParamBuilder {
    initial_value: f32,
    smoothing_ms: f32,
    sample_rate: f32,
}

impl SmoothedParamBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            initial_value: 0.0,
            smoothing_ms: 10.0, // 10ms default smoothing
            sample_rate: 48000.0,
        }
    }

    /// Set initial value
    pub fn with_initial(mut self, value: f32) -> Self {
        self.initial_value = value;
        self
    }

    /// Set smoothing time in milliseconds
    pub fn with_smoothing_ms(mut self, ms: f32) -> Self {
        self.smoothing_ms = ms;
        self
    }

    /// Set sample rate
    pub fn with_sample_rate(mut self, rate: f32) -> Self {
        self.sample_rate = rate;
        self
    }

    /// Build the SmoothedParam
    pub fn build(self) -> SmoothedParam {
        SmoothedParam::new(self.initial_value, self.smoothing_ms, self.sample_rate)
    }
}

impl Default for SmoothedParamBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoothing_basic() {
        let mut param = SmoothedParam::new(0.0, 10.0, 48000.0);

        // Set target
        param.set_target(1.0);

        // First sample should be between 0 and 1
        let val1 = param.next();
        assert!(val1 > 0.0 && val1 < 1.0);

        // Should gradually approach target
        let val2 = param.next();
        assert!(val2 > val1);
        assert!(val2 < 1.0);

        // After many samples, should be very close to target
        for _ in 0..10000 {
            param.next();
        }
        assert!((param.current() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_builder_pattern() {
        let param = SmoothedParamBuilder::new()
            .with_initial(0.5)
            .with_smoothing_ms(5.0)
            .with_sample_rate(44100.0)
            .build();

        assert_eq!(param.current(), 0.5);
    }

    #[test]
    fn test_snap_to_target() {
        let mut param = SmoothedParam::new(0.0, 10.0, 48000.0);
        param.set_target(1.0);

        // Without snap, would be gradual
        let val1 = param.next();
        assert!(val1 < 0.5);

        // Snap immediately
        param.snap_to_target();
        assert_eq!(param.current(), 1.0);
    }

    #[test]
    fn test_is_settled() {
        let mut param = SmoothedParam::new(0.0, 10.0, 48000.0);
        param.set_target(1.0);

        // Not settled initially
        assert!(!param.is_settled(0.01));

        // After many samples, should be settled
        for _ in 0..10000 {
            param.next();
        }
        assert!(param.is_settled(0.01));
    }
}
