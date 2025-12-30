//! Performance and CPU metrics for audio processing
//!
//! Provides real-time CPU usage tracking for synths and effects.

#[cfg(test)]
use std::time::Duration;
use std::time::Instant;

/// Performance metrics for audio processing
#[derive(Debug, Clone, Copy)]
pub struct PerformanceMetrics {
    /// Average processing time per sample (nanoseconds)
    pub avg_sample_time_ns: f64,
    /// Peak processing time per sample (nanoseconds)
    pub peak_sample_time_ns: u64,
    /// CPU usage as percentage of available time
    /// (0.0 = no CPU, 1.0 = 100% CPU, >1.0 = overload)
    pub cpu_usage: f64,
    /// Number of samples processed
    pub samples_processed: u64,
    /// Total processing time
    pub total_time_ns: u64,
}

impl PerformanceMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self {
            avg_sample_time_ns: 0.0,
            peak_sample_time_ns: 0,
            cpu_usage: 0.0,
            samples_processed: 0,
            total_time_ns: 0,
        }
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Get average processing time in microseconds
    pub fn avg_time_us(&self) -> f64 {
        self.avg_sample_time_ns / 1000.0
    }

    /// Get peak processing time in microseconds
    pub fn peak_time_us(&self) -> f64 {
        self.peak_sample_time_ns as f64 / 1000.0
    }

    /// Get CPU usage as percentage (0-100%)
    pub fn cpu_percent(&self) -> f64 {
        self.cpu_usage * 100.0
    }

    /// Check if we're in danger of dropouts (>80% CPU)
    pub fn is_overloaded(&self) -> bool {
        self.cpu_usage > 0.8
    }

    /// Check if CPU usage is moderate (40-80%)
    pub fn is_moderate(&self) -> bool {
        self.cpu_usage >= 0.4 && self.cpu_usage <= 0.8
    }

    /// Check if CPU usage is low (<40%)
    pub fn is_low(&self) -> bool {
        self.cpu_usage < 0.4
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// CPU meter for tracking audio processing performance
pub struct CpuMeter {
    metrics: PerformanceMetrics,
    sample_rate: f64,
    time_per_sample_ns: f64,
    smoothing_factor: f64,
}

impl CpuMeter {
    /// Create a new CPU meter
    pub fn new(sample_rate: f64) -> Self {
        Self {
            metrics: PerformanceMetrics::new(),
            sample_rate,
            time_per_sample_ns: 1_000_000_000.0 / sample_rate,
            smoothing_factor: 0.99,
        }
    }

    /// Start timing a processing block
    #[inline]
    pub fn start_timing(&self) -> Instant {
        Instant::now()
    }

    /// Stop timing and update metrics
    #[inline]
    pub fn stop_timing(&mut self, start: Instant, num_samples: usize) {
        let elapsed = start.elapsed().as_nanos() as u64;

        if num_samples == 0 {
            return;
        }

        self.metrics.samples_processed += num_samples as u64;
        self.metrics.total_time_ns += elapsed;

        let time_per_sample = elapsed / num_samples as u64;

        if time_per_sample > self.metrics.peak_sample_time_ns {
            self.metrics.peak_sample_time_ns = time_per_sample;
        }

        if self.metrics.avg_sample_time_ns == 0.0 {
            self.metrics.avg_sample_time_ns = time_per_sample as f64;
        } else {
            self.metrics.avg_sample_time_ns = self.metrics.avg_sample_time_ns
                * self.smoothing_factor
                + time_per_sample as f64 * (1.0 - self.smoothing_factor);
        }

        self.metrics.cpu_usage = self.metrics.avg_sample_time_ns / self.time_per_sample_ns;
    }

    /// Time a closure
    pub fn measure<F>(&mut self, num_samples: usize, f: F)
    where
        F: FnOnce(),
    {
        let start = self.start_timing();
        f();
        self.stop_timing(start, num_samples);
    }

    /// Get current metrics
    pub fn metrics(&self) -> PerformanceMetrics {
        self.metrics
    }

    /// Reset metrics
    pub fn reset(&mut self) {
        self.metrics.reset();
    }

    /// Set sample rate
    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        self.time_per_sample_ns = 1_000_000_000.0 / sample_rate;
    }

    /// Set smoothing factor (0.0 = no smoothing, 0.99 = heavy smoothing)
    pub fn set_smoothing(&mut self, factor: f64) {
        self.smoothing_factor = factor.clamp(0.0, 0.999);
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> f64 {
        self.sample_rate
    }
}

impl Default for CpuMeter {
    fn default() -> Self {
        Self::new(48000.0)
    }
}

/// Aggregate metrics for multiple meters
pub struct MetricsAggregator {
    meters: Vec<CpuMeter>,
}

impl MetricsAggregator {
    pub fn new() -> Self {
        Self { meters: Vec::new() }
    }

    pub fn add_meter(&mut self, meter: CpuMeter) {
        self.meters.push(meter);
    }

    pub fn total_cpu_usage(&self) -> f64 {
        self.meters.iter().map(|m| m.metrics().cpu_usage).sum()
    }

    pub fn total_cpu_percent(&self) -> f64 {
        self.total_cpu_usage() * 100.0
    }

    pub fn meter_metrics(&self, index: usize) -> Option<PerformanceMetrics> {
        self.meters.get(index).map(|m| m.metrics())
    }

    pub fn all_metrics(&self) -> Vec<PerformanceMetrics> {
        self.meters.iter().map(|m| m.metrics()).collect()
    }

    pub fn reset_all(&mut self) {
        for meter in &mut self.meters {
            meter.reset();
        }
    }

    pub fn len(&self) -> usize {
        self.meters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.meters.is_empty()
    }
}

impl Default for MetricsAggregator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_metrics_creation() {
        let metrics = PerformanceMetrics::new();
        assert_eq!(metrics.samples_processed, 0);
        assert_eq!(metrics.cpu_usage, 0.0);
    }

    #[test]
    fn test_cpu_meter_basic() {
        let mut meter = CpuMeter::new(48000.0);
        let start = meter.start_timing();
        thread::sleep(Duration::from_micros(10));
        meter.stop_timing(start, 1);

        let metrics = meter.metrics();
        assert!(metrics.cpu_usage > 0.0);
        assert_eq!(metrics.samples_processed, 1);
    }

    #[test]
    fn test_measure_closure() {
        let mut meter = CpuMeter::new(48000.0);
        meter.measure(100, || {
            thread::sleep(Duration::from_micros(10));
        });

        let metrics = meter.metrics();
        assert_eq!(metrics.samples_processed, 100);
    }

    #[test]
    fn test_overload_detection() {
        let mut metrics = PerformanceMetrics::new();
        metrics.cpu_usage = 0.85;
        assert!(metrics.is_overloaded());
        assert!(!metrics.is_moderate());
        assert!(!metrics.is_low());
    }

    #[test]
    fn test_aggregator() {
        let mut agg = MetricsAggregator::new();
        let meter1 = CpuMeter::new(48000.0);
        let meter2 = CpuMeter::new(48000.0);

        agg.add_meter(meter1);
        agg.add_meter(meter2);

        assert_eq!(agg.len(), 2);
    }
}
