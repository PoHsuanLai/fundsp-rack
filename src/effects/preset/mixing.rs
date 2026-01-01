//! Mixing effect presets
//!
//! Pre-configured effect chains for common mixing scenarios (vocals, guitars, drums, etc.).

use super::{EffectPreset, EffectPresetBank};
use crate::effects::serialize::EffectState;

/// Built-in mixing presets
pub struct MixingPresets;

impl MixingPresets {
    // ========== VOCAL PRESETS ==========

    /// Clean vocal chain
    pub fn vocal_clean() -> EffectPreset {
        EffectPreset::new("Clean Vocal")
            .with_effect(
                EffectState::new("hpf")
                    .with_param("cutoff", 100.0)
                    .with_param("res", 0.5),
            )
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", -2.0)
                    .with_param("mid", 1.0)
                    .with_param("high", 1.5),
            )
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.01)
                    .with_param("release", 0.15),
            )
            .with_description("Clean vocal processing with clarity boost")
            .with_tag("vocal")
            .with_tag("clean")
    }

    /// Warm vocal chain with saturation
    pub fn vocal_warm() -> EffectPreset {
        EffectPreset::new("Warm Vocal")
            .with_effect(
                EffectState::new("hpf")
                    .with_param("cutoff", 80.0)
                    .with_param("res", 0.5),
            )
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 1.0)
                    .with_param("mid", 0.5)
                    .with_param("high", -0.5),
            )
            .with_effect(EffectState::new("tape").with_param("saturation", 0.2))
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.015)
                    .with_param("release", 0.2),
            )
            .with_effect(
                EffectState::new("reverb")
                    .with_param("room", 0.2)
                    .with_param("time", 1.0),
            )
            .with_description("Warm vocal with tape saturation and light reverb")
            .with_tag("vocal")
            .with_tag("warm")
    }

    /// Telephone/radio effect vocal
    pub fn vocal_telephone() -> EffectPreset {
        EffectPreset::new("Telephone Vocal")
            .with_effect(
                EffectState::new("hpf")
                    .with_param("cutoff", 500.0)
                    .with_param("res", 0.7),
            )
            .with_effect(
                EffectState::new("lpf")
                    .with_param("cutoff", 3000.0)
                    .with_param("res", 0.7),
            )
            .with_effect(EffectState::new("bitcrush").with_param("bits", 10.0))
            .with_effect(EffectState::new("soft_clip").with_param("amount", 0.4))
            .with_description("Telephone/radio effect for vocals")
            .with_tag("vocal")
            .with_tag("telephone")
            .with_tag("lofi")
    }

    // ========== GUITAR PRESETS ==========

    /// Clean guitar tone
    pub fn guitar_clean() -> EffectPreset {
        EffectPreset::new("Clean Guitar")
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", -1.0)
                    .with_param("mid", 0.5)
                    .with_param("high", 1.0),
            )
            .with_effect(
                EffectState::new("chorus")
                    .with_param("separation", 0.015)
                    .with_param("variation", 0.3),
            )
            .with_effect(
                EffectState::new("reverb")
                    .with_param("room", 0.3)
                    .with_param("time", 1.5),
            )
            .with_description("Clean guitar with chorus and reverb")
            .with_tag("guitar")
            .with_tag("clean")
    }

    /// Crunchy overdrive guitar
    pub fn guitar_crunch() -> EffectPreset {
        EffectPreset::new("Crunch Guitar")
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 1.0)
                    .with_param("mid", 2.0)
                    .with_param("high", 0.5),
            )
            .with_effect(EffectState::new("overdrive").with_param("amount", 0.5))
            .with_effect(
                EffectState::new("lpf")
                    .with_param("cutoff", 6000.0)
                    .with_param("res", 0.3),
            )
            .with_effect(
                EffectState::new("delay")
                    .with_param("time", 0.3)
                    .with_param("feedback", 0.2),
            )
            .with_description("Crunchy overdrive guitar tone")
            .with_tag("guitar")
            .with_tag("crunch")
            .with_tag("overdrive")
    }

    /// High-gain distorted guitar
    pub fn guitar_distorted() -> EffectPreset {
        EffectPreset::new("Distorted Guitar")
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 2.0)
                    .with_param("mid", 3.0)
                    .with_param("high", 1.0),
            )
            .with_effect(EffectState::new("distortion").with_param("amount", 0.7))
            .with_effect(
                EffectState::new("lpf")
                    .with_param("cutoff", 5000.0)
                    .with_param("res", 0.4),
            )
            .with_effect(EffectState::new("gate").with_param("threshold", -40.0))
            .with_description("High-gain distorted guitar with gate")
            .with_tag("guitar")
            .with_tag("distortion")
            .with_tag("metal")
    }

    // ========== DRUM PRESETS ==========

    /// Punchy drum bus
    pub fn drum_bus() -> EffectPreset {
        EffectPreset::new("Drum Bus")
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 2.0)
                    .with_param("mid", -1.0)
                    .with_param("high", 1.0),
            )
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.005)
                    .with_param("release", 0.08),
            )
            .with_effect(EffectState::new("soft_clip").with_param("amount", 0.15))
            .with_description("Punchy drum bus processing")
            .with_tag("drums")
            .with_tag("bus")
    }

    /// Parallel drum compression (NY compression style)
    pub fn drum_parallel() -> EffectPreset {
        EffectPreset::new("Parallel Drums")
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.001)
                    .with_param("release", 0.05),
            )
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 3.0)
                    .with_param("mid", 1.0)
                    .with_param("high", 2.0),
            )
            .with_effect(EffectState::new("soft_clip").with_param("amount", 0.2))
            .with_description("Heavy parallel compression for drums (NY style)")
            .with_tag("drums")
            .with_tag("parallel")
            .with_tag("compression")
    }

    // ========== SYNTH PRESETS ==========

    /// Wide synth pad processing
    pub fn synth_pad() -> EffectPreset {
        EffectPreset::new("Synth Pad")
            .with_effect(
                EffectState::new("chorus")
                    .with_param("separation", 0.025)
                    .with_param("variation", 0.5),
            )
            .with_effect(
                EffectState::new("reverb")
                    .with_param("room", 0.6)
                    .with_param("time", 3.0),
            )
            .with_effect(
                EffectState::new("delay")
                    .with_param("time", 0.4)
                    .with_param("feedback", 0.3),
            )
            .with_effect(
                EffectState::new("lpf")
                    .with_param("cutoff", 8000.0)
                    .with_param("res", 0.2),
            )
            .with_description("Wide, spacious synth pad processing")
            .with_tag("synth")
            .with_tag("pad")
            .with_tag("ambient")
    }

    /// Aggressive synth lead
    pub fn synth_lead() -> EffectPreset {
        EffectPreset::new("Synth Lead")
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", -2.0)
                    .with_param("mid", 2.0)
                    .with_param("high", 1.0),
            )
            .with_effect(EffectState::new("overdrive").with_param("amount", 0.3))
            .with_effect(
                EffectState::new("delay")
                    .with_param("time", 0.15)
                    .with_param("feedback", 0.25),
            )
            .with_effect(
                EffectState::new("reverb")
                    .with_param("room", 0.2)
                    .with_param("time", 0.8),
            )
            .with_description("Cutting synth lead with presence")
            .with_tag("synth")
            .with_tag("lead")
    }

    // ========== BASS PRESETS ==========

    /// Clean bass DI processing
    pub fn bass_clean() -> EffectPreset {
        EffectPreset::new("Clean Bass")
            .with_effect(
                EffectState::new("hpf")
                    .with_param("cutoff", 40.0)
                    .with_param("res", 0.5),
            )
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 2.0)
                    .with_param("mid", 0.0)
                    .with_param("high", -1.0),
            )
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.01)
                    .with_param("release", 0.1),
            )
            .with_description("Clean, tight bass processing")
            .with_tag("bass")
            .with_tag("clean")
    }

    /// Growly bass with distortion
    pub fn bass_growl() -> EffectPreset {
        EffectPreset::new("Growl Bass")
            .with_effect(
                EffectState::new("hpf")
                    .with_param("cutoff", 50.0)
                    .with_param("res", 0.5),
            )
            .with_effect(
                EffectState::new("eq_3band")
                    .with_param("low", 1.0)
                    .with_param("mid", 2.0)
                    .with_param("high", 0.0),
            )
            .with_effect(EffectState::new("overdrive").with_param("amount", 0.5))
            .with_effect(
                EffectState::new("lpf")
                    .with_param("cutoff", 4000.0)
                    .with_param("res", 0.4),
            )
            .with_effect(
                EffectState::new("compressor")
                    .with_param("attack", 0.008)
                    .with_param("release", 0.08),
            )
            .with_description("Aggressive growly bass with midrange bite")
            .with_tag("bass")
            .with_tag("growl")
            .with_tag("distortion")
    }
}

/// Extension trait for adding mixing presets to a bank
pub trait PresetBankMixingExt {
    /// Add all mixing presets to the bank
    fn add_mixing_presets(&mut self);
}

impl PresetBankMixingExt for EffectPresetBank {
    fn add_mixing_presets(&mut self) {
        // Vocals
        self.add_preset(MixingPresets::vocal_clean());
        self.add_preset(MixingPresets::vocal_warm());
        self.add_preset(MixingPresets::vocal_telephone());

        // Guitars
        self.add_preset(MixingPresets::guitar_clean());
        self.add_preset(MixingPresets::guitar_crunch());
        self.add_preset(MixingPresets::guitar_distorted());

        // Drums
        self.add_preset(MixingPresets::drum_bus());
        self.add_preset(MixingPresets::drum_parallel());

        // Synths
        self.add_preset(MixingPresets::synth_pad());
        self.add_preset(MixingPresets::synth_lead());

        // Bass
        self.add_preset(MixingPresets::bass_clean());
        self.add_preset(MixingPresets::bass_growl());
    }
}

/// Create a preset bank with all mixing presets
pub fn mixing_bank() -> EffectPresetBank {
    let mut bank = EffectPresetBank::new("Mixing");
    bank.add_mixing_presets();
    bank
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixing_presets() {
        let bank = mixing_bank();
        assert_eq!(bank.presets.len(), 12);
    }

    #[test]
    fn test_vocal_presets() {
        let bank = mixing_bank();
        let vocals = bank.get_by_tag("vocal");
        assert_eq!(vocals.len(), 3);
    }

    #[test]
    fn test_guitar_presets() {
        let bank = mixing_bank();
        let guitars = bank.get_by_tag("guitar");
        assert_eq!(guitars.len(), 3);
    }
}
