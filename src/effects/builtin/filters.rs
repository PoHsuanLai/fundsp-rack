//! Filter effects (lowpass, highpass, bandpass, resonant variants)

use super::super::registry::{EffectBuilder, EffectControls, EffectMetadata};
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
        EffectMetadata::new("lpf", "Lowpass filter")
            .with_param("cutoff", 1000.0, 20.0, 20000.0)
            .with_param("res", 0.5, 0.0, 10.0)
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
        EffectMetadata::new("hpf", "Highpass filter")
            .with_param("cutoff", 1000.0, 20.0, 20000.0)
            .with_param("res", 0.5, 0.0, 10.0)
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
        EffectMetadata::new("bpf", "Bandpass filter")
            .with_param("center", 1000.0, 20.0, 20000.0)
            .with_param("res", 0.5, 0.0, 10.0)
    }
}

// Normalized versions (these are aliases with different default parameters)
pub struct NormalizedLowpassBuilder;

impl EffectBuilder for NormalizedLowpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        LowpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("nlpf", "Normalized lowpass filter")
            .with_param("cutoff", 1000.0, 20.0, 20000.0)
            .with_param("res", 0.5, 0.0, 10.0)
    }
}

pub struct NormalizedHighpassBuilder;

impl EffectBuilder for NormalizedHighpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        HighpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("nhpf", "Normalized highpass filter")
            .with_param("cutoff", 1000.0, 20.0, 20000.0)
            .with_param("res", 0.5, 0.0, 10.0)
    }
}

pub struct NormalizedBandpassBuilder;

impl EffectBuilder for NormalizedBandpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        BandpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("nbpf", "Normalized bandpass filter")
            .with_param("center", 1000.0, 20.0, 20000.0)
            .with_param("res", 0.5, 0.0, 10.0)
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
        EffectMetadata::new("rlpf", "Resonant lowpass filter")
            .with_param("cutoff", 1000.0, 20.0, 20000.0)
            .with_param("res", 5.0, 0.0, 10.0)
    }
}

pub struct NormalizedResonantLowpassBuilder;

impl EffectBuilder for NormalizedResonantLowpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        ResonantLowpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("nrlpf", "Normalized resonant lowpass filter")
            .with_param("cutoff", 1000.0, 20.0, 20000.0)
            .with_param("res", 5.0, 0.0, 10.0)
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
        EffectMetadata::new("rhpf", "Resonant highpass filter")
            .with_param("cutoff", 1000.0, 20.0, 20000.0)
            .with_param("res", 5.0, 0.0, 10.0)
    }
}

pub struct NormalizedResonantHighpassBuilder;

impl EffectBuilder for NormalizedResonantHighpassBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        ResonantHighpassBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("nrhpf", "Normalized resonant highpass filter")
            .with_param("cutoff", 1000.0, 20.0, 20000.0)
            .with_param("res", 5.0, 0.0, 10.0)
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
        EffectMetadata::new("parametric_eq", "Parametric EQ (single band)")
            .with_param("freq", 1000.0, 20.0, 20000.0)
            .with_param("q", 1.0, 0.1, 10.0)
            .with_param("gain", 0.0, -24.0, 24.0)
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
        EffectMetadata::new("dc_blocker", "DC Blocker (removes DC offset)")
            .with_param("cutoff", 10.0, 1.0, 50.0)
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
        EffectMetadata::new("notch", "Notch filter (removes specific frequency)")
            .with_param("freq", 1000.0, 20.0, 20000.0)
            .with_param("q", 2.0, 0.1, 100.0)
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
