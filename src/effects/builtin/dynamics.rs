//! Dynamics effects (limiter, compressor, normaliser)

use super::super::registry::{EffectBuilder, EffectControls, EffectMetadata};
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
        EffectMetadata::new("limiter", "Limiter (prevents clipping)")
            .with_param("attack", 0.01, 0.001, 0.1)
            .with_param("release", 0.1, 0.01, 1.0)
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
        EffectMetadata::new("compressor", "Compressor (reduces dynamic range)")
            .with_param("attack", 0.01, 0.001, 0.1)
            .with_param("release", 0.1, 0.01, 1.0)
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
        EffectMetadata::new("normaliser", "Normaliser (automatic gain control)")
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
        EffectMetadata::new("sidechain_compressor", "Sidechain Compressor (compress based on external signal)")
            .with_param("threshold", -20.0, -60.0, 0.0)
            .with_param("ratio", 4.0, 1.0, 20.0)
            .with_param("attack", 0.01, 0.001, 0.1)
            .with_param("release", 0.1, 0.01, 1.0)
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
        EffectMetadata::new("sidechain_gate", "Sidechain Gate (gate based on external signal)")
            .with_param("threshold", -40.0, -80.0, 0.0)
            .with_param("attack", 0.001, 0.0001, 0.1)
            .with_param("release", 0.05, 0.001, 1.0)
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
