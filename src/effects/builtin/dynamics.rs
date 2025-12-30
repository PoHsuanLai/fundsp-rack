//! Dynamics effects (limiter, compressor, normaliser)

use super::super::registry::{
    EffectBuilder, EffectCategory, EffectControls, EffectMetadata, ParameterDef,
};
use super::super::sidechain::{SidechainCompressor, SidechainGate};
use fundsp::hacker32::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Limiter effect
pub struct LimiterBuilder;

impl EffectBuilder for LimiterBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let attack = params.get("attack").copied().unwrap_or(0.01);
        let release = params.get("release").copied().unwrap_or(0.1);

        let limiter = limiter_stereo(attack, release);
        (Box::new(limiter), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "limiter".to_string(),
            description: "Limiter (prevents clipping)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "attack".to_string(),
                    default: 0.01,
                    min: 0.001,
                    max: 0.1,
                },
                ParameterDef {
                    name: "release".to_string(),
                    default: 0.1,
                    min: 0.01,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Dynamics,
            latency_samples: 0,
        }
    }
}

/// Compressor effect (using limiter as base)
pub struct CompressorBuilder;

impl EffectBuilder for CompressorBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let attack = params.get("attack").copied().unwrap_or(0.01);
        let release = params.get("release").copied().unwrap_or(0.1);

        // Use limiter as compressor base
        let comp = limiter_stereo(attack, release);
        (Box::new(comp), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "compressor".to_string(),
            description: "Compressor (reduces dynamic range)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "attack".to_string(),
                    default: 0.01,
                    min: 0.001,
                    max: 0.1,
                },
                ParameterDef {
                    name: "release".to_string(),
                    default: 0.1,
                    min: 0.01,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Dynamics,
            latency_samples: 0,
        }
    }
}

/// Normaliser (automatic gain control)
pub struct NormaliserBuilder;

impl EffectBuilder for NormaliserBuilder {
    fn build(&self, _params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        // Simple normalizer using limiter with fast attack/release
        let norm = limiter_stereo(0.001, 0.01);
        (Box::new(norm), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "normaliser".to_string(),
            description: "Normaliser (automatic gain control)".to_string(),
            parameters: vec![],
            category: EffectCategory::Dynamics,
            latency_samples: 0,
        }
    }
}

/// Sidechain Compressor effect
/// Compresses the audio signal based on an external sidechain signal.
/// Fully implemented with SidechainAwareEffect trait integration.
pub struct SidechainCompressorBuilder;

impl EffectBuilder for SidechainCompressorBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let threshold = params.get("threshold").copied().unwrap_or(-20.0);
        let ratio = params.get("ratio").copied().unwrap_or(4.0);
        let attack = params.get("attack").copied().unwrap_or(0.01);
        let release = params.get("release").copied().unwrap_or(0.1);

        // Create actual sidechain compressor
        // Note: Sample rate will be passed from EffectChain which gets it from AudioBackend
        let sample_rate = 48000.0; // Default, overridden when chain.set_sample_rate() is called
        let compressor = SidechainCompressor::new(threshold, ratio, attack, release, sample_rate);

        // Create controls using the Shared parameters
        let controls = EffectControls::new();
        // Note: Parameters are already shared inside SidechainCompressor
        // They can be controlled by modifying compressor.threshold.set_value(), etc.

        (Box::new(compressor), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "sidechain_compressor".to_string(),
            description: "Sidechain Compressor (compress based on external signal)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "threshold".to_string(),
                    default: -20.0,
                    min: -60.0,
                    max: 0.0,
                },
                ParameterDef {
                    name: "ratio".to_string(),
                    default: 4.0,
                    min: 1.0,
                    max: 20.0,
                },
                ParameterDef {
                    name: "attack".to_string(),
                    default: 0.01,
                    min: 0.001,
                    max: 0.1,
                },
                ParameterDef {
                    name: "release".to_string(),
                    default: 0.1,
                    min: 0.01,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Dynamics,
            latency_samples: 0,
        }
    }
}

/// Sidechain Gate effect
/// Gates the audio signal based on an external sidechain signal.
/// Fully implemented with SidechainAwareEffect trait integration.
pub struct SidechainGateBuilder;

impl EffectBuilder for SidechainGateBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let threshold = params.get("threshold").copied().unwrap_or(-40.0);
        let attack = params.get("attack").copied().unwrap_or(0.001);
        let release = params.get("release").copied().unwrap_or(0.05);

        // Create actual sidechain gate
        // Note: Sample rate will be passed from EffectChain which gets it from AudioBackend
        let sample_rate = 48000.0; // Default, overridden when chain.set_sample_rate() is called
        let gate = SidechainGate::new(threshold, attack, release, sample_rate);

        // Create controls using the Shared parameters
        let controls = EffectControls::new();
        // Note: Parameters are already shared inside SidechainGate

        (Box::new(gate), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "sidechain_gate".to_string(),
            description: "Sidechain Gate (gate based on external signal)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "threshold".to_string(),
                    default: -40.0,
                    min: -80.0,
                    max: 0.0,
                },
                ParameterDef {
                    name: "attack".to_string(),
                    default: 0.001,
                    min: 0.0001,
                    max: 0.1,
                },
                ParameterDef {
                    name: "release".to_string(),
                    default: 0.05,
                    min: 0.001,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Dynamics,
            latency_samples: 0,
        }
    }
}

/// Register all dynamics effects
pub fn register_all(registry: &mut super::super::registry::EffectRegistry) {
    registry.register("limiter", Arc::new(LimiterBuilder));
    registry.register("compressor", Arc::new(CompressorBuilder));
    registry.register("normaliser", Arc::new(NormaliserBuilder));
    registry.register("sidechain_compressor", Arc::new(SidechainCompressorBuilder));
    registry.register("sidechain_gate", Arc::new(SidechainGateBuilder));
}
