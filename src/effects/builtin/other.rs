//! Other/special effects (slicer, wobble, ring_mod, octaver)

use super::super::registry::{
    EffectBuilder, EffectCategory, EffectControls, EffectMetadata, ParameterDef,
};
use fundsp::hacker32::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Slicer - Rhythmic gating/volume modulation
pub struct SlicerBuilder;

impl EffectBuilder for SlicerBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let rate = params.get("rate").copied().unwrap_or(8.0); // slices per second
        let _phase = params.get("phase").copied().unwrap_or(0.0);
        let width = params.get("width").copied().unwrap_or(0.5); // duty cycle of gate

        // Use square wave for rhythmic gating
        // square() outputs -1 to 1, we map it to 0 to 1 for amplitude gating
        let gate_lfo_left = sine_hz(rate).clone()
            >> map(move |x: &Frame<f32, U1>| {
                // Create pulse wave from sine: if > threshold, 1.0, else 0.0
                if x[0] > (1.0 - width * 2.0) {
                    1.0
                } else {
                    0.0
                }
            });

        let gate_lfo_right = sine_hz(rate)
            >> map(
                move |x: &Frame<f32, U1>| {
                    if x[0] > (1.0 - width * 2.0) {
                        1.0
                    } else {
                        0.0
                    }
                },
            );

        // Apply gating to left and right channels
        let effect = (pass() * gate_lfo_left) | (pass() * gate_lfo_right);

        (Box::new(effect), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "slicer".to_string(),
            description: "Rhythmic gating/volume modulation".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "rate".to_string(),
                    default: 8.0,
                    min: 0.1,
                    max: 100.0,
                },
                ParameterDef {
                    name: "phase".to_string(),
                    default: 0.0,
                    min: 0.0,
                    max: 1.0,
                },
                ParameterDef {
                    name: "width".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Modulation,
            latency_samples: 0,
        }
    }
}

/// Wobble - LFO filter sweep (dubstep-style)
pub struct WobbleBuilder;

impl EffectBuilder for WobbleBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let rate = params.get("rate").copied().unwrap_or(4.0); // wobble rate in Hz
        let min_cutoff = params.get("min_cutoff").copied().unwrap_or(200.0);
        let max_cutoff = params.get("max_cutoff").copied().unwrap_or(2000.0);
        let res = params.get("res").copied().unwrap_or(0.3);

        // Create LFO for cutoff modulation
        let lfo_left = sine_hz(rate).clone();
        let lfo_right = sine_hz(rate);

        // Map LFO (-1 to 1) to cutoff range (min to max)
        let cutoff_left = lfo_left
            >> map(move |x: &Frame<f32, U1>| fundsp::math::lerp11(min_cutoff, max_cutoff, x[0]));
        let cutoff_right = lfo_right
            >> map(move |x: &Frame<f32, U1>| fundsp::math::lerp11(min_cutoff, max_cutoff, x[0]));

        // Apply moog filter with modulated cutoff
        // moog_q takes: Input 0 = audio, Input 1 = cutoff frequency
        let left = (pass() | cutoff_left) >> moog_q(res);
        let right = (pass() | cutoff_right) >> moog_q(res);

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "wobble".to_string(),
            description: "LFO filter sweep (dubstep-style)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "rate".to_string(),
                    default: 4.0,
                    min: 0.1,
                    max: 20.0,
                },
                ParameterDef {
                    name: "min_cutoff".to_string(),
                    default: 200.0,
                    min: 50.0,
                    max: 5000.0,
                },
                ParameterDef {
                    name: "max_cutoff".to_string(),
                    default: 2000.0,
                    min: 100.0,
                    max: 10000.0,
                },
                ParameterDef {
                    name: "res".to_string(),
                    default: 0.3,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Filter,
            latency_samples: 0,
        }
    }
}

/// Ring Modulator - Multiplies signal with sine wave for metallic tones
pub struct RingModBuilder;

impl EffectBuilder for RingModBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let freq = params.get("freq").copied().unwrap_or(440.0);
        let mix = params.get("mix").copied().unwrap_or(0.5);

        // Ring modulation: multiply input by sine wave
        let carrier_left = sine_hz(freq).clone();
        let carrier_right = sine_hz(freq);

        let dry = 1.0 - mix;
        let wet = mix;

        // Mix dry and wet using & operator to branch and sum
        let left = (pass() * dry) & (pass() * carrier_left * wet);
        let right = (pass() * dry) & (pass() * carrier_right * wet);

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "ring_mod".to_string(),
            description: "Ring modulator for metallic tones".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "freq".to_string(),
                    default: 440.0,
                    min: 20.0,
                    max: 5000.0,
                },
                ParameterDef {
                    name: "mix".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Distortion,
            latency_samples: 0,
        }
    }
}

/// Octaver - Adds octave below (pitch-shifted)
pub struct OctaverBuilder;

impl EffectBuilder for OctaverBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let octave = params.get("octave").copied().unwrap_or(-1.0);
        let mix = params.get("mix").copied().unwrap_or(0.5);

        let dry = 1.0 - mix;
        let wet = mix;

        // Classic analog octaver approach:
        // - Octave down: Full-wave rectification + lowpass filter (creates subharmonics)
        // - Octave up: Full-wave rectification + highpass filter (creates harmonics)

        if octave < 0.0 {
            // Octave down effect
            // Full-wave rectify (creates 2x frequency components) then lowpass
            // This creates a suboctave effect similar to classic analog octavers
            let octave_down_left = pass()
                >> map(|x: &Frame<f32, U1>| x[0].abs())  // Full-wave rectify
                >> lowpass_hz(800.0, 1.0); // Lowpass to extract fundamental

            let octave_down_right =
                pass() >> map(|x: &Frame<f32, U1>| x[0].abs()) >> lowpass_hz(800.0, 1.0);

            // Mix dry and wet using & operator to branch and sum
            let left = (pass() * dry) & (octave_down_left * wet * 0.5); // Scale down rectified signal
            let right = (pass() * dry) & (octave_down_right * wet * 0.5);

            (Box::new(left | right), EffectControls::new())
        } else {
            // Octave up effect
            // Full-wave rectify then highpass to get upper harmonics
            let octave_up_left = pass()
                >> map(|x: &Frame<f32, U1>| x[0].abs())  // Full-wave rectify doubles frequency
                >> highpass_hz(200.0, 0.7); // Highpass to remove low freq artifacts

            let octave_up_right =
                pass() >> map(|x: &Frame<f32, U1>| x[0].abs()) >> highpass_hz(200.0, 0.7);

            // Mix dry and wet using & operator to branch and sum
            let left = (pass() * dry) & (octave_up_left * wet * 0.7);
            let right = (pass() * dry) & (octave_up_right * wet * 0.7);

            (Box::new(left | right), EffectControls::new())
        }
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "octaver".to_string(),
            description: "Adds octaves above or below".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "octave".to_string(),
                    default: -1.0,
                    min: -2.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "mix".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Other,
            latency_samples: 0,
        }
    }
}

/// Register all other/special effects  
pub fn register_all(registry: &mut super::super::registry::EffectRegistry) {
    registry.register("slicer", Arc::new(SlicerBuilder));
    registry.register("wobble", Arc::new(WobbleBuilder));
    registry.register("ring_mod", Arc::new(RingModBuilder));
    registry.register("octaver", Arc::new(OctaverBuilder));
}
