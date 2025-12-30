//! Time-based effects (reverb, delay, echo)

use super::super::registry::{
    EffectBuilder, EffectCategory, EffectControls, EffectMetadata, ParameterDef,
};
use fundsp::hacker32::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Reverb effect
pub struct ReverbBuilder;

impl EffectBuilder for ReverbBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let room_size = params.get("room").copied().unwrap_or(0.5);
        let time = params.get("time").copied().unwrap_or(1.0);

        let effect = reverb4_stereo(room_size, time);
        (Box::new(effect), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "reverb".to_string(),
            description: "Reverb effect".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "room".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 1.0,
                },
                ParameterDef {
                    name: "time".to_string(),
                    default: 1.0,
                    min: 0.1,
                    max: 10.0,
                },
            ],
            category: EffectCategory::Time,
            latency_samples: 0,
        }
    }
}

/// Room reverb - small room preset
pub struct RoomReverbBuilder;

impl EffectBuilder for RoomReverbBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let mix = params.get("mix").copied().unwrap_or(0.3);
        // Small room: short time, small size
        // Use & operator to branch input to dry/wet paths and sum outputs
        let effect = (pass() | pass()) * (1.0 - mix) & reverb4_stereo(0.3, 0.5) * mix;
        (Box::new(effect), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "room".to_string(),
            description: "Small room reverb".to_string(),
            parameters: vec![ParameterDef {
                name: "mix".to_string(),
                default: 0.3,
                min: 0.0,
                max: 1.0,
            }],
            category: EffectCategory::Time,
            latency_samples: 0,
        }
    }
}

/// Hall reverb - large hall preset
pub struct HallReverbBuilder;

impl EffectBuilder for HallReverbBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let mix = params.get("mix").copied().unwrap_or(0.4);
        // Large hall: long time, large size
        // Use & operator to branch input to dry/wet paths and sum outputs
        let effect = (pass() | pass()) * (1.0 - mix) & reverb4_stereo(0.8, 3.0) * mix;
        (Box::new(effect), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "hall".to_string(),
            description: "Large hall reverb".to_string(),
            parameters: vec![ParameterDef {
                name: "mix".to_string(),
                default: 0.4,
                min: 0.0,
                max: 1.0,
            }],
            category: EffectCategory::Time,
            latency_samples: 0,
        }
    }
}

/// Plate reverb - bright metallic reverb
pub struct PlateReverbBuilder;

impl EffectBuilder for PlateReverbBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let mix = params.get("mix").copied().unwrap_or(0.35);
        let decay = params.get("decay").copied().unwrap_or(2.0);
        // Plate: medium size, longer decay, bright character
        // Use & operator to branch input to dry/wet paths and sum outputs
        let effect = (pass() | pass()) * (1.0 - mix) & reverb4_stereo(0.5, decay) * mix;
        (Box::new(effect), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "plate".to_string(),
            description: "Plate reverb (bright, metallic)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "mix".to_string(),
                    default: 0.35,
                    min: 0.0,
                    max: 1.0,
                },
                ParameterDef {
                    name: "decay".to_string(),
                    default: 2.0,
                    min: 0.5,
                    max: 5.0,
                },
            ],
            category: EffectCategory::Time,
            latency_samples: 0,
        }
    }
}

/// Delay effect
pub struct DelayBuilder;

impl EffectBuilder for DelayBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let time = params.get("time").copied().unwrap_or(0.5);
        let mix = params.get("mix").copied().unwrap_or(0.5);

        // Use & operator to branch input to dry/wet paths and sum outputs
        let delay_left = pass() >> fundsp::prelude::delay(time as f64);
        let delay_right = pass() >> fundsp::prelude::delay(time as f64);

        let left = pass() * (1.0 - mix) & delay_left * mix;
        let right = pass() * (1.0 - mix) & delay_right * mix;

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "delay".to_string(),
            description: "Delay effect".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "time".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "mix".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Time,
            latency_samples: 0,
        }
    }
}

/// Stereo delay - ping-pong style
pub struct StereoDelayBuilder;

impl EffectBuilder for StereoDelayBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let time_l = params.get("time_l").copied().unwrap_or(0.25);
        let time_r = params.get("time_r").copied().unwrap_or(0.375); // Offset for stereo
        let mix = params.get("mix").copied().unwrap_or(0.4);

        // Different delay times for left and right create stereo width
        // Use & operator to branch input to dry/wet paths and sum outputs
        let delay_left = pass() >> fundsp::prelude::delay(time_l as f64);
        let delay_right = pass() >> fundsp::prelude::delay(time_r as f64);

        let left = pass() * (1.0 - mix) & delay_left * mix;
        let right = pass() * (1.0 - mix) & delay_right * mix;

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "stereo_delay".to_string(),
            description: "Stereo delay with independent L/R times".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "time_l".to_string(),
                    default: 0.25,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "time_r".to_string(),
                    default: 0.375,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "mix".to_string(),
                    default: 0.4,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Time,
            latency_samples: 0,
        }
    }
}

/// Ping-pong delay
pub struct PingPongDelayBuilder;

impl EffectBuilder for PingPongDelayBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let time = params.get("time").copied().unwrap_or(0.25);
        let mix = params.get("mix").copied().unwrap_or(0.4);

        // Left and right alternate: L gets delay, R gets 2x delay
        // Use & operator to branch input to dry/wet paths and sum outputs
        let delay_left = pass() >> fundsp::prelude::delay(time as f64);
        let delay_right = pass() >> fundsp::prelude::delay((time * 2.0) as f64);

        let left = pass() * (1.0 - mix) & delay_left * mix;
        let right = pass() * (1.0 - mix) & delay_right * mix;

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "ping_pong".to_string(),
            description: "Ping-pong delay (bounces L-R)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "time".to_string(),
                    default: 0.25,
                    min: 0.05,
                    max: 1.0,
                },
                ParameterDef {
                    name: "mix".to_string(),
                    default: 0.4,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Time,
            latency_samples: 0,
        }
    }
}

/// Slapback delay - short rock/rockabilly delay
pub struct SlapbackDelayBuilder;

impl EffectBuilder for SlapbackDelayBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let time = params.get("time").copied().unwrap_or(0.08); // ~80ms
        let mix = params.get("mix").copied().unwrap_or(0.3);

        // Use & operator to branch input to dry/wet paths and sum outputs
        let delay_left = pass() >> fundsp::prelude::delay(time as f64);
        let delay_right = pass() >> fundsp::prelude::delay(time as f64);

        let left = pass() * (1.0 - mix) & delay_left * mix;
        let right = pass() * (1.0 - mix) & delay_right * mix;

        (Box::new(left | right), EffectControls::new())
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "slapback".to_string(),
            description: "Slapback delay (short, punchy)".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "time".to_string(),
                    default: 0.08,
                    min: 0.03,
                    max: 0.15,
                },
                ParameterDef {
                    name: "mix".to_string(),
                    default: 0.3,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Time,
            latency_samples: 0,
        }
    }
}

/// Echo (alias for delay with feedback)
pub struct EchoBuilder;

impl EffectBuilder for EchoBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        DelayBuilder.build(params)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata {
            name: "echo".to_string(),
            description: "Echo effect".to_string(),
            parameters: vec![
                ParameterDef {
                    name: "time".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 2.0,
                },
                ParameterDef {
                    name: "mix".to_string(),
                    default: 0.5,
                    min: 0.0,
                    max: 1.0,
                },
            ],
            category: EffectCategory::Time,
            latency_samples: 0,
        }
    }
}

/// Register all time-based effects
pub fn register_all(registry: &mut super::super::registry::EffectRegistry) {
    // Reverbs
    registry.register("reverb", Arc::new(ReverbBuilder));
    registry.register("room", Arc::new(RoomReverbBuilder));
    registry.register("room_reverb", Arc::new(RoomReverbBuilder)); // alias
    registry.register("hall", Arc::new(HallReverbBuilder));
    registry.register("hall_reverb", Arc::new(HallReverbBuilder)); // alias
    registry.register("plate", Arc::new(PlateReverbBuilder));
    registry.register("plate_reverb", Arc::new(PlateReverbBuilder)); // alias

    // Delays
    registry.register("delay", Arc::new(DelayBuilder));
    registry.register("stereo_delay", Arc::new(StereoDelayBuilder));
    registry.register("ping_pong", Arc::new(PingPongDelayBuilder));
    registry.register("pingpong", Arc::new(PingPongDelayBuilder)); // alias
    registry.register("slapback", Arc::new(SlapbackDelayBuilder));
    registry.register("echo", Arc::new(EchoBuilder));
}
