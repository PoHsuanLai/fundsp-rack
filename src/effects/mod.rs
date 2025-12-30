//! Audio effects module
//!
//! This module provides:
//! - **Effect plugin trait** - `EffectBuilder` for creating custom effects
//! - **Effect registry** - Register and manage effects by name
//! - **Effect chain** - Chain multiple effects together with UUID tracking
//! - **50+ built-in effects** - Filters, distortion, dynamics, reverb, delay, modulation, and more
//! - **Real-time parameter control** - Lock-free parameter updates via `fundsp::shared::Shared`
//! - **Sidechain support** - Effects that respond to external audio signals

pub mod builder;
pub mod builtin;
pub mod chain;
pub mod registry;
#[cfg(feature = "serde")]
pub mod serialize;
pub mod sidechain;
pub mod smoothing;

pub use builder::{Effect, EffectBuilder as FluentEffectBuilder, EffectRegistryExt};
pub use chain::EffectChain;
pub use registry::{
    EffectBuilder, EffectCategory, EffectControls, EffectMetadata, EffectRegistry, ParameterRange,
};
#[cfg(feature = "serde")]
pub use serialize::{ChainState, EffectState};
pub use sidechain::SidechainAwareEffect;
pub use smoothing::{SmoothedParam, SmoothedParamBuilder};

// Re-export UUID for effect IDs
pub use uuid::Uuid;

/// Unique identifier for an effect instance
pub type EffectId = Uuid;
