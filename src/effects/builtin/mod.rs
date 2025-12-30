//! Built-in audio effects

pub mod distortion;
pub mod dynamics;
pub mod eq;
pub mod filters;
pub mod lofi;
pub mod modulation;
pub mod other;
pub mod spatial;
pub mod time;

/// Register all built-in effects with the registry
pub fn register_all(registry: &mut super::registry::EffectRegistry) {
    distortion::register_all(registry);
    dynamics::register_all(registry);
    eq::register_all(registry);
    filters::register_all(registry);
    lofi::register_all(registry);
    modulation::register_all(registry);
    other::register_all(registry);
    spatial::register_all(registry);
    time::register_all(registry);
}
