//! EQ effects (3-band EQ, tilt EQ, etc.)

use super::super::registry::{
    EffectBuilder, EffectCategory, EffectControls, EffectMetadata, ParameterDef,
};
use fundsp::hacker32::*;
use std::collections::HashMap;
use std::sync::Arc;

/// 3-band EQ (low, mid, high)
pub struct EQ3BandBuilder;

impl EffectBuilder for EQ3BandBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let low_gain = params.get("low").copied().unwrap_or(0.0);    // dB
        let mid_gain = params.get("mid").copied().unwrap_or(0.0);    // dB
        let high_gain = params.get("high").copied().unwrap_or(0.0);  // dB
        let low_freq = params.get("low_freq").copied().unwrap_or(200.0);
        let high_freq = params.get("high_freq").copied().unwrap_or(3000.0);

        let low_shared = shared(low_gain);
        let mid_shared = shared(mid_gain);
        let high_shared = shared(high_gain);

        let mut controls = EffectControls::new();
        controls.params.insert("low".to_string(), low_shared.clone());
        controls.params.insert("mid".to_string(), mid_shared.clone());
        controls.params.insert("high".to_string(), high_shared.clone());

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
        EffectMetadata {
            name: "eq_3band".to_string(),
            description: "3-band EQ (low/mid/high)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "low".to_string(),
                    default: 0.0,
                    min: -12.0,
                    max: 12.0,
                },
                ParameterDef {
                    name: "mid".to_string(),
                    default: 0.0,
                    min: -12.0,
                    max: 12.0,
                },
                ParameterDef {
                    name: "high".to_string(),
                    default: 0.0,
                    min: -12.0,
                    max: 12.0,
                },
                ParameterDef {
                    name: "low_freq".to_string(),
                    default: 200.0,
                    min: 50.0,
                    max: 500.0,
                },
                ParameterDef {
                    name: "high_freq".to_string(),
                    default: 3000.0,
                    min: 1000.0,
                    max: 10000.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
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
        controls.params.insert("tilt".to_string(), tilt_shared.clone());

        // Convert tilt (-1 to 1) to dB gains
        let tilt_db = tilt * 6.0; // ±6dB range

        // Low shelf boost/cut and high shelf opposite
        let left = pass()
            >> lowshelf_hz(freq, 0.7, -tilt_db)
            >> highshelf_hz(freq, 0.7, tilt_db);

        let right = pass()
            >> lowshelf_hz(freq, 0.7, -tilt_db)
            >> highshelf_hz(freq, 0.7, tilt_db);

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "tilt_eq".to_string(),
            description: "Tilt EQ (bass/treble balance)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "tilt".to_string(),
                    default: 0.0,
                    min: -1.0,
                    max: 1.0,
                },
                ParameterDef {
                    name: "freq".to_string(),
                    default: 1000.0,
                    min: 200.0,
                    max: 5000.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

/// Low shelf EQ
pub struct LowShelfBuilder;

impl EffectBuilder for LowShelfBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let freq = params.get("freq").copied().unwrap_or(200.0);
        let gain = params.get("gain").copied().unwrap_or(0.0);
        let q = params.get("q").copied().unwrap_or(0.7);

        let freq_shared = shared(freq);
        let gain_shared = shared(gain);

        let mut controls = EffectControls::new();
        controls.params.insert("freq".to_string(), freq_shared.clone());
        controls.params.insert("gain".to_string(), gain_shared.clone());

        let left = pass() >> lowshelf_hz(freq, q, gain);
        let right = pass() >> lowshelf_hz(freq, q, gain);

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "low_shelf".to_string(),
            description: "Low shelf EQ".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "freq".to_string(),
                    default: 200.0,
                    min: 20.0,
                    max: 1000.0,
                },
                ParameterDef {
                    name: "gain".to_string(),
                    default: 0.0,
                    min: -12.0,
                    max: 12.0,
                },
                ParameterDef {
                    name: "q".to_string(),
                    default: 0.7,
                    min: 0.1,
                    max: 2.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

/// High shelf EQ
pub struct HighShelfBuilder;

impl EffectBuilder for HighShelfBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let freq = params.get("freq").copied().unwrap_or(3000.0);
        let gain = params.get("gain").copied().unwrap_or(0.0);
        let q = params.get("q").copied().unwrap_or(0.7);

        let freq_shared = shared(freq);
        let gain_shared = shared(gain);

        let mut controls = EffectControls::new();
        controls.params.insert("freq".to_string(), freq_shared.clone());
        controls.params.insert("gain".to_string(), gain_shared.clone());

        let left = pass() >> highshelf_hz(freq, q, gain);
        let right = pass() >> highshelf_hz(freq, q, gain);

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "high_shelf".to_string(),
            description: "High shelf EQ".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "freq".to_string(),
                    default: 3000.0,
                    min: 500.0,
                    max: 15000.0,
                },
                ParameterDef {
                    name: "gain".to_string(),
                    default: 0.0,
                    min: -12.0,
                    max: 12.0,
                },
                ParameterDef {
                    name: "q".to_string(),
                    default: 0.7,
                    min: 0.1,
                    max: 2.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
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
