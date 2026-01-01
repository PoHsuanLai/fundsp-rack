//! Mastering effect chain presets
//!
//! Pre-configured effect chains for mastering and final mix processing.

use super::{EffectPreset, EffectPresetBank};
use crate::effects::serialize::EffectState;

/// Built-in mastering presets
pub struct MasteringPresets;

impl MasteringPresets {
    /// Transparent mastering chain - subtle enhancement
    pub fn transparent() -> EffectPreset {
        EffectPreset::new("Transparent Master")
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 0.5)
                    .with_param("mid", 0.0)
                    .with_param("high", 0.5),
            )
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.03)
                    .with_param("release", 0.2),
            )
            .with_effect(
                EffectState::new("limiter")
                    .with_param("attack", 0.001)
                    .with_param("release", 0.1),
            )
            .with_description("Subtle, transparent mastering chain")
            .with_tag("mastering")
            .with_tag("transparent")
    }

    /// Warm analog-style mastering
    pub fn warm() -> EffectPreset {
        EffectPreset::new("Warm Master")
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 1.5)
                    .with_param("mid", -0.5)
                    .with_param("high", -1.0),
            )
            .with_effect(EffectState::new("tape").with_param("saturation", 0.3))
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.02)
                    .with_param("release", 0.15),
            )
            .with_effect(
                EffectState::new("limiter")
                    .with_param("attack", 0.001)
                    .with_param("release", 0.1),
            )
            .with_description("Warm, analog-style mastering with tape saturation")
            .with_tag("mastering")
            .with_tag("warm")
            .with_tag("analog")
    }

    /// Loud and punchy mastering for EDM/electronic
    pub fn loud() -> EffectPreset {
        EffectPreset::new("Loud Master")
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 2.0)
                    .with_param("mid", 0.0)
                    .with_param("high", 1.5),
            )
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.005)
                    .with_param("release", 0.1),
            )
            .with_effect(EffectState::new("soft_clip").with_param("amount", 0.2))
            .with_effect(
                EffectState::new("limiter")
                    .with_param("attack", 0.0005)
                    .with_param("release", 0.05),
            )
            .with_description("Loud, punchy mastering for electronic music")
            .with_tag("mastering")
            .with_tag("loud")
            .with_tag("edm")
    }

    /// Broadcast-ready mastering (for podcasts, radio)
    pub fn broadcast() -> EffectPreset {
        EffectPreset::new("Broadcast Master")
            .with_effect(
                EffectState::new("hpf")
                    .with_param("cutoff", 80.0)
                    .with_param("res", 0.5),
            )
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", -1.0)
                    .with_param("mid", 1.0)
                    .with_param("high", 0.5),
            )
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.01)
                    .with_param("release", 0.15),
            )
            .with_effect(EffectState::new("normaliser"))
            .with_effect(
                EffectState::new("limiter")
                    .with_param("attack", 0.001)
                    .with_param("release", 0.1),
            )
            .with_description("Broadcast-ready mastering for podcasts and radio")
            .with_tag("mastering")
            .with_tag("broadcast")
            .with_tag("podcast")
    }

    /// Lo-fi mastering for vintage sound
    pub fn lofi() -> EffectPreset {
        EffectPreset::new("Lo-Fi Master")
            .with_effect(
                EffectState::new("lpf")
                    .with_param("cutoff", 8000.0)
                    .with_param("res", 0.3),
            )
            .with_effect(EffectState::new("bitcrush").with_param("bits", 12.0))
            .with_effect(EffectState::new("tape").with_param("saturation", 0.5))
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 1.0)
                    .with_param("mid", -1.0)
                    .with_param("high", -2.0),
            )
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.02)
                    .with_param("release", 0.2),
            )
            .with_description("Lo-fi, vintage-style mastering")
            .with_tag("mastering")
            .with_tag("lofi")
            .with_tag("vintage")
    }
}

/// Extension trait for adding mastering presets to a bank
pub trait PresetBankMasteringExt {
    /// Add all mastering presets to the bank
    fn add_mastering_presets(&mut self);
}

impl PresetBankMasteringExt for EffectPresetBank {
    fn add_mastering_presets(&mut self) {
        self.add_preset(MasteringPresets::transparent());
        self.add_preset(MasteringPresets::warm());
        self.add_preset(MasteringPresets::loud());
        self.add_preset(MasteringPresets::broadcast());
        self.add_preset(MasteringPresets::lofi());
    }
}

/// Create a preset bank with all mastering presets
pub fn mastering_bank() -> EffectPresetBank {
    let mut bank = EffectPresetBank::new("Mastering");
    bank.add_mastering_presets();
    bank
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mastering_presets() {
        let bank = mastering_bank();
        assert_eq!(bank.presets.len(), 5);

        // All should have mastering tag
        for preset in &bank.presets {
            assert!(preset.tags.contains(&"mastering".to_string()));
        }
    }

    #[test]
    fn test_transparent_preset() {
        let preset = MasteringPresets::transparent();
        assert_eq!(preset.name, "Transparent Master");
        assert_eq!(preset.effects.len(), 3);
        assert!(preset.effects.iter().any(|e| e.name == "limiter"));
    }
}
