//! Filter effects (lowpass, highpass, bandpass, resonant variants)

use super::super::registry::{
    EffectBuilder, EffectCategory, EffectControls, EffectMetadata, ParameterDef,
};
use fundsp::hacker32::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Lowpass filter
pub struct LowpassBuilder;

impl EffectBuilder for LowpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(1000.0);
        let initial_res = params.get("res").copied().unwrap_or(0.5);

        let cutoff_shared = shared(initial_cutoff);
        let res_shared = shared(initial_res);

        let mut controls = EffectControls::new();
        controls
            .params
            .insert("cutoff".to_string(), cutoff_shared.clone());
        controls
            .params
            .insert("res".to_string(), res_shared.clone());

        let left = (pass() | var(&cutoff_shared) | var(&res_shared)) >> lowpass();
        let right = (pass() | var(&cutoff_shared) | var(&res_shared)) >> lowpass();

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "lpf".to_string(),
            description: "Lowpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

/// Highpass filter
pub struct HighpassBuilder;

impl EffectBuilder for HighpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let initial_cutoff = params.get("cutoff").copied().unwrap_or(1000.0);
        let initial_res = params.get("res").copied().unwrap_or(0.5);

        let cutoff_shared = shared(initial_cutoff);
        let res_shared = shared(initial_res);

        let mut controls = EffectControls::new();
        controls
            .params
            .insert("cutoff".to_string(), cutoff_shared.clone());
        controls
            .params
            .insert("res".to_string(), res_shared.clone());

        let left = (pass() | var(&cutoff_shared) | var(&res_shared)) >> highpass();
        let right = (pass() | var(&cutoff_shared) | var(&res_shared)) >> highpass();

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "hpf".to_string(),
            description: "Highpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

/// Bandpass filter
pub struct BandpassBuilder;

impl EffectBuilder for BandpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let initial_center = params.get("center").copied().unwrap_or(1000.0);
        let initial_res = params.get("res").copied().unwrap_or(0.5);

        let center_shared = shared(initial_center);
        let res_shared = shared(initial_res);

        let mut controls = EffectControls::new();
        controls
            .params
            .insert("center".to_string(), center_shared.clone());
        controls
            .params
            .insert("res".to_string(), res_shared.clone());

        let left = (pass() | var(&center_shared) | var(&res_shared)) >> bandpass();
        let right = (pass() | var(&center_shared) | var(&res_shared)) >> bandpass();

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "bpf".to_string(),
            description: "Bandpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "center".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

// Normalized versions (these are aliases with different default parameters)
pub struct NormalizedLowpassBuilder;

impl EffectBuilder for NormalizedLowpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        LowpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "nlpf".to_string(),
            description: "Normalized lowpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

pub struct NormalizedHighpassBuilder;

impl EffectBuilder for NormalizedHighpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        HighpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "nhpf".to_string(),
            description: "Normalized highpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

pub struct NormalizedBandpassBuilder;

impl EffectBuilder for NormalizedBandpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        BandpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "nbpf".to_string(),
            description: "Normalized bandpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "center".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

// Resonant filters (high Q versions)
pub struct ResonantLowpassBuilder;

impl EffectBuilder for ResonantLowpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let mut resonant_params = params.clone();
        resonant_params.entry("res".to_string()).or_insert(5.0); // Higher default resonance
        LowpassBuilder.build(&resonant_params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "rlpf".to_string(),
            description: "Resonant lowpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 5.0,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

pub struct NormalizedResonantLowpassBuilder;

impl EffectBuilder for NormalizedResonantLowpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        ResonantLowpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "nrlpf".to_string(),
            description: "Normalized resonant lowpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 5.0,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

pub struct ResonantHighpassBuilder;

impl EffectBuilder for ResonantHighpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let mut resonant_params = params.clone();
        resonant_params.entry("res".to_string()).or_insert(5.0);
        HighpassBuilder.build(&resonant_params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "rhpf".to_string(),
            description: "Resonant highpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 5.0,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

pub struct NormalizedResonantHighpassBuilder;

impl EffectBuilder for NormalizedResonantHighpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        ResonantHighpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "nrhpf".to_string(),
            description: "Normalized resonant highpass filter".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "cutoff".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 5.0,
                    min: 0.0,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

/// Parametric EQ (bell filter)
pub struct ParametricEQBuilder;

impl EffectBuilder for ParametricEQBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let initial_freq = params.get("freq").copied().unwrap_or(1000.0);
        let initial_q = params.get("q").copied().unwrap_or(1.0);
        let initial_gain = params.get("gain").copied().unwrap_or(0.0); // dB

        let freq_shared = shared(initial_freq);
        let q_shared = shared(initial_q);
        let gain_shared = shared(initial_gain);

        let mut controls = EffectControls::new();
        controls
            .params
            .insert("freq".to_string(), freq_shared.clone());
        controls.params.insert("q".to_string(), q_shared.clone());
        controls
            .params
            .insert("gain".to_string(), gain_shared.clone());

        // Use FunDSP's bell filter (parametric EQ band)
        // bell takes: input, frequency, q, gain_db
        let left = (pass() | var(&freq_shared) | var(&q_shared) | var(&gain_shared)) >> bell();
        let right = (pass() | var(&freq_shared) | var(&q_shared) | var(&gain_shared)) >> bell();

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "parametric_eq".to_string(),
            description: "Parametric EQ (single band)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "freq".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "q".to_string(),
                    default: 1.0,
                    min: 0.1,
                    max: 10.0,
                },
                ParameterDef {
                    name: "gain".to_string(),
                    default: 0.0,
                    min: -24.0,
                    max: 24.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

/// DC Blocker - removes DC offset
pub struct DCBlockerBuilder;

impl EffectBuilder for DCBlockerBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let cutoff = params.get("cutoff").copied().unwrap_or(10.0); // Hz

        // DC blocker is a highpass filter at very low frequency
        let left = dcblock_hz(cutoff);
        let right = dcblock_hz(cutoff);

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "dc_blocker".to_string(),
            description: "DC Blocker (removes DC offset)".to_string(),
            parameters: vec![ParameterDef {
                name: "cutoff".to_string(),
                default: 10.0,
                min: 1.0,
                max: 50.0,
            }],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

/// Notch filter - removes specific frequency
pub struct NotchBuilder;

impl EffectBuilder for NotchBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let initial_freq = params.get("freq").copied().unwrap_or(1000.0);
        let initial_q = params.get("q").copied().unwrap_or(2.0);

        let freq_shared = shared(initial_freq);
        let q_shared = shared(initial_q);

        let mut controls = EffectControls::new();
        controls
            .params
            .insert("freq".to_string(), freq_shared.clone());
        controls.params.insert("q".to_string(), q_shared.clone());

        let left = (pass() | var(&freq_shared) | var(&q_shared)) >> notch();
        let right = (pass() | var(&freq_shared) | var(&q_shared)) >> notch();

        (Box::new(left | right), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "notch".to_string(),
            description: "Notch filter (removes specific frequency)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "freq".to_string(),
                    default: 1000.0,
                    min: 20.0,
                    max: 20000.0,
                },
                ParameterDef {
                    name: "q".to_string(),
                    default: 2.0,
                    min: 0.1,
                    max: 100.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

/// Register all filter effects
pub fn register_all(registry: &mut super::super::registry::EffectRegistry) {
    // Basic filters
    registry.register("lpf", Arc::new(LowpassBuilder));
    registry.register("lowpass", Arc::new(LowpassBuilder)); // alias
    registry.register("hpf", Arc::new(HighpassBuilder));
    registry.register("highpass", Arc::new(HighpassBuilder)); // alias
    registry.register("bpf", Arc::new(BandpassBuilder));
    registry.register("bandpass", Arc::new(BandpassBuilder)); // alias

    // Normalized filters
    registry.register("nlpf", Arc::new(NormalizedLowpassBuilder));
    registry.register("nhpf", Arc::new(NormalizedHighpassBuilder));
    registry.register("nbpf", Arc::new(NormalizedBandpassBuilder));

    // Resonant filters
    registry.register("rlpf", Arc::new(ResonantLowpassBuilder));
    registry.register("nrlpf", Arc::new(NormalizedResonantLowpassBuilder));
    registry.register("rhpf", Arc::new(ResonantHighpassBuilder));
    registry.register("nrhpf", Arc::new(NormalizedResonantHighpassBuilder));

    // Utility filters
    registry.register("parametric_eq", Arc::new(ParametricEQBuilder));
    registry.register("peq", Arc::new(ParametricEQBuilder)); // alias
    registry.register("dc_blocker", Arc::new(DCBlockerBuilder));
    registry.register("notch", Arc::new(NotchBuilder));
}
