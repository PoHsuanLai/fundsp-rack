//! Spatial effects (pan)

use super::super::registry::{EffectBuilder, EffectControls, EffectMetadata};
use fundsp::hacker32::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Pan effect
pub struct PanBuilder;

impl EffectBuilder for PanBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let pan_val = params.get("pan").copied().unwrap_or(0.0); // -1.0 = left, 0.0 = center, 1.0 = right

        let pan_shared = shared(pan_val);
        let mut controls = EffectControls::new();
        controls
            .params
            .insert("pan".to_string(), pan_shared.clone());

        // Use FunDSP's built-in pan function
        let stereo_pan = fundsp::hacker32::pan(pan_val);

        (Box::new(stereo_pan), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("pan", "Pan (stereo positioning)")
            .with_param("pan", 0.0, -1.0, 1.0)
    }
}

/// Stereo Widener - widens or narrows stereo image
pub struct StereoWidenerBuilder;

impl EffectBuilder for StereoWidenerBuilder {
    fn build(&self, params: &HashMap<String, f32>) -> (Box<dyn AudioUnit>, EffectControls) {
        let width = params.get("width").copied().unwrap_or(1.0); // 0.0 = mono, 1.0 = normal, 2.0 = wide

        let width_shared = shared(width);
        let mut controls = EffectControls::new();
        controls
            .params
            .insert("width".to_string(), width_shared.clone());

        // Mid-Side (M/S) stereo widening technique:
        // 1. Convert L/R to Mid/Side: Mid = (L+R)/2, Side = (L-R)/2
        // 2. Scale Side by width factor
        // 3. Convert back to L/R: L = Mid + Side, R = Mid - Side
        //
        // Width values:
        // 0.0 = mono (no side signal)
        // 1.0 = normal stereo (unchanged)
        // 2.0 = extra wide (side doubled)

        use fundsp::hacker32::*;
        use fundsp::signal::Routing;

        // Create the width processing graph
        // Input is stereo (2 channels)
        let effect = An(MultiPass::<U2>::new())
            >> An(Map::new(
                move |input: &Frame<f32, U2>| {
                    let left = input[0];
                    let right = input[1];

                    // Convert to Mid/Side
                    let mid = (left + right) * 0.5;
                    let side = (left - right) * 0.5;

                    // Scale side by width
                    let scaled_side = side * width;

                    // Convert back to L/R
                    let new_left = mid + scaled_side;
                    let new_right = mid - scaled_side;

                    Frame::<f32, U2>::from([new_left, new_right])
                },
                Routing::Arbitrary(0.0),
            ));

        (Box::new(effect), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("stereo_widener", "Stereo Widener (adjusts stereo width)")
            .with_param("width", 1.0, 0.0, 2.0)
    }
}

// ============================================================================
// Additional Sonic Pi Effects
// ============================================================================

/// Register all spatial effects
pub fn register_all(registry: &mut super::super::registry::EffectRegistry) {
    registry.register("pan", Arc::new(PanBuilder));
    registry.register("stereo_widener", Arc::new(StereoWidenerBuilder));
    registry.register("stereo_width", Arc::new(StereoWidenerBuilder)); // alias
    registry.register("width", Arc::new(StereoWidenerBuilder)); // alias
}
