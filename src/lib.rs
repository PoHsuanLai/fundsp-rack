//! # fundsp-rack
//!
//! A preset library for [FunDSP](https://github.com/SamiPerttu/fundsp) providing
//! ready-to-use synthesizers and effects with friendly APIs.
//!
//! ## Features
//!
//! - **30+ synthesizers** - From basic waveforms to complex pads and leads
//! - **50+ effects** - Filters, reverbs, delays, distortion, dynamics, and more
//! - **Polyphony** - Easy voice management for playing chords
//! - **Real-time control** - Lock-free parameter updates
//! - **Effect chains** - Combine multiple effects easily
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use fundsp_rack::prelude::*;
//!
//! // Create a synth
//! let registry = SynthRegistry::with_builtin();
//! let (synth, controls) = registry.synth("pad").freq(220.0).build().unwrap();
//!
//! // Control in real-time
//! controls.amp.set(0.5);
//!
//! // Create an effect chain
//! let mut chain = EffectChain::with_registry(EffectRegistry::with_builtin());
//! chain.add_effect("chorus", &Default::default()).unwrap();
//! chain.add_effect("hall", &Default::default()).unwrap();
//! ```
//!
//! ## Polyphonic Synths
//!
//! ```rust,no_run
//! use fundsp_rack::prelude::*;
//!
//! // Create a polyphonic synth with 8 voices
//! let mut poly = PolySynth::new("strings", 8);
//!
//! // Play a chord
//! poly.note_on(60, 0.8); // C4
//! poly.note_on(64, 0.8); // E4
//! poly.note_on(67, 0.8); // G4
//!
//! // Get audio
//! let (left, right) = poly.get_stereo();
//! ```

pub mod effects;
pub mod error;
pub mod metrics;
pub mod params;
pub mod synth;

// Re-export common types at crate root
pub use error::{Error, Result};
pub use metrics::{CpuMeter, MetricsAggregator, PerformanceMetrics};
pub use params::ParameterDef;

/// Prelude module - import everything you need
pub mod prelude {
    // Core
    pub use crate::error::{Error, Result};
    pub use crate::metrics::{CpuMeter, MetricsAggregator, PerformanceMetrics};
    pub use crate::params::ParameterDef;

    // Synth
    pub use crate::synth::{
        midi_to_freq, EnvelopeConfig, FluentSynthBuilder, LFOConfig, LFOTarget, LFOWaveform,
        PolySynth, PolySynthBuilder, Synth, SynthBuilder, SynthCategory, SynthMetadata,
        SynthRegistry, SynthRegistryExt, VoiceControls, ADSR, AHD, AR,
    };
    #[cfg(feature = "serde")]
    pub use crate::synth::{PresetBank, SynthId, SynthPreset, Uuid};

    // Effects
    pub use crate::effects::{
        Effect, EffectBuilder, EffectCategory, EffectChain, EffectControls, EffectId,
        EffectMetadata, EffectRegistry, EffectRegistryExt, FluentEffectBuilder, ParameterRange,
        SidechainAwareEffect, SmoothedParam, SmoothedParamBuilder,
    };
    #[cfg(feature = "serde")]
    pub use crate::effects::{ChainState, EffectState};
}
