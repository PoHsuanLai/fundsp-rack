//! EQ effects (3-band EQ, tilt EQ, etc.)

use super::super::registry::{EffectBuilder, EffectControls, EffectMetadata};
use fundsp::hacker32::*;
use std::collections::HashMap;
use std::sync::Arc;

/// 3-band EQ (low, mid, high)
pub struct EQ3BandBuilder;

impl EffectBuilder for EQ3BandBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let low_gain_db = params.get("low").copied().unwrap_or(0.0); // dB
        let mid_gain_db = params.get("mid").copied().unwrap_or(0.0); // dB
        let high_gain_db = params.get("high").copied().unwrap_or(0.0); // dB
        let low_freq = params.get("low_freq").copied().unwrap_or(200.0);
        let high_freq = params.get("high_freq").copied().unwrap_or(3000.0);

        // Convert dB to linear amplitude (fundsp filters expect linear gain)
        let low_gain = db_amp(low_gain_db);
        let mid_gain = db_amp(mid_gain_db);
        let high_gain = db_amp(high_gain_db);

        let low_shared = shared(low_gain_db);
        let mid_shared = shared(mid_gain_db);
        let high_shared = shared(high_gain_db);

        let mut controls = EffectControls::new();
        controls
            .params
            .insert("low".to_string(), low_shared.clone());
        controls
            .params
            .insert("mid".to_string(), mid_shared.clone());
        controls
            .params
            .insert("high".to_string(), high_shared.clone());

        // Use shelf filters for low and high, bell for mid
        // Low shelf at low_freq, High shelf at high_freq, Bell at geometric mean
        let mid_freq = (low_freq * high_freq).sqrt();

        let left = pass()
            >> lowshelf_hz(low_freq, 0.7, low_gain)
            >> bell_hz(mid_freq, 1.0, mid_gain)
            >> highshelf_hz(high_freq, 0.7, high_gain);

        let right = pass()
            >> lowshelf_hz(low_freq, 0.7, low_gain)
            >> bell_hz(mid_freq, 1.0, mid_gain)
            >> highshelf_hz(high_freq, 0.7, high_gain);

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("eq_3band", "3-band EQ (low/mid/high)")
            .with_param("low", 0.0, -12.0, 12.0)
            .with_param("mid", 0.0, -12.0, 12.0)
            .with_param("high", 0.0, -12.0, 12.0)
            .with_param("low_freq", 200.0, 50.0, 500.0)
            .with_param("high_freq", 3000.0, 1000.0, 10000.0)
    }
}

/// Tilt EQ - simple bass/treble balance
pub struct TiltEQBuilder;

impl EffectBuilder for TiltEQBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let tilt = params.get("tilt").copied().unwrap_or(0.0); // -1 = bass, +1 = treble
        let freq = params.get("freq").copied().unwrap_or(1000.0); // Pivot frequency

        let tilt_shared = shared(tilt);

        let mut controls = EffectControls::new();
        controls
            .params
            .insert("tilt".to_string(), tilt_shared.clone());

        // Convert tilt (-1 to 1) to dB gains, then to linear amplitude
        let tilt_db = tilt * 6.0; // ±6dB range
        let low_gain = db_amp(-tilt_db);
        let high_gain = db_amp(tilt_db);

        // Low shelf boost/cut and high shelf opposite
        let left = pass() >> lowshelf_hz(freq, 0.7, low_gain) >> highshelf_hz(freq, 0.7, high_gain);

        let right =
            pass() >> lowshelf_hz(freq, 0.7, low_gain) >> highshelf_hz(freq, 0.7, high_gain);

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("tilt_eq", "Tilt EQ (bass/treble balance)")
            .with_param("tilt", 0.0, -1.0, 1.0)
            .with_param("freq", 1000.0, 200.0, 5000.0)
    }
}

/// Low shelf EQ
pub struct LowShelfBuilder;

impl EffectBuilder for LowShelfBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let freq = params.get("freq").copied().unwrap_or(200.0);
        let gain_db = params.get("gain").copied().unwrap_or(0.0);
        let q = params.get("q").copied().unwrap_or(0.7);

        // Convert dB to linear amplitude
        let gain = db_amp(gain_db);

        let freq_shared = shared(freq);
        let gain_shared = shared(gain_db);

        let mut controls = EffectControls::new();
        controls
            .params
            .insert("freq".to_string(), freq_shared.clone());
        controls
            .params
            .insert("gain".to_string(), gain_shared.clone());

        let left = pass() >> lowshelf_hz(freq, q, gain);
        let right = pass() >> lowshelf_hz(freq, q, gain);

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("low_shelf", "Low shelf EQ")
            .with_param("freq", 200.0, 20.0, 1000.0)
            .with_param("gain", 0.0, -12.0, 12.0)
            .with_param("q", 0.7, 0.1, 2.0)
    }
}

/// High shelf EQ
pub struct HighShelfBuilder;

impl EffectBuilder for HighShelfBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let freq = params.get("freq").copied().unwrap_or(3000.0);
        let gain_db = params.get("gain").copied().unwrap_or(0.0);
        let q = params.get("q").copied().unwrap_or(0.7);

        // Convert dB to linear amplitude
        let gain = db_amp(gain_db);

        let freq_shared = shared(freq);
        let gain_shared = shared(gain_db);

        let mut controls = EffectControls::new();
        controls
            .params
            .insert("freq".to_string(), freq_shared.clone());
        controls
            .params
            .insert("gain".to_string(), gain_shared.clone());

        let left = pass() >> highshelf_hz(freq, q, gain);
        let right = pass() >> highshelf_hz(freq, q, gain);

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("high_shelf", "High shelf EQ")
            .with_param("freq", 3000.0, 500.0, 15000.0)
            .with_param("gain", 0.0, -12.0, 12.0)
            .with_param("q", 0.7, 0.1, 2.0)
    }
}

/// Register all EQ effects
pub fn register_all(registry: &mut super::super::registry::EffectRegistry) {
    registry.register("eq_3band", Arc::new(EQ3BandBuilder));
    registry.register("eq3", Arc::new(EQ3BandBuilder)); // alias
    registry.register("tilt_eq", Arc::new(TiltEQBuilder));
    registry.register("tilt", Arc::new(TiltEQBuilder)); // alias
    registry.register("low_shelf", Arc::new(LowShelfBuilder));
    registry.register("lowshelf", Arc::new(LowShelfBuilder)); // alias
    registry.register("high_shelf", Arc::new(HighShelfBuilder));
    registry.register("highshelf", Arc::new(HighShelfBuilder)); // alias
}
