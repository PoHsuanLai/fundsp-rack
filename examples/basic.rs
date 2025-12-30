//! Basic usage example for fundsp-rack
//!
//! Demonstrates creating synths and effects with the fluent API.

use fundsp_rack::prelude::*;

fn main() -> anyhow::Result<()> {
    // === Synths ===
    let synth_registry = SynthRegistry::with_builtin();
    println!("Registered {} synths", synth_registry.list_synths().len());

    // Create synths with fluent API
    let (_sine, sine_ctl) = synth_registry.synth("sine").freq(440.0).build()?;
    let (_tb303, tb303_ctl) = synth_registry
        .synth("tb303")
        .freq(55.0)
        .cutoff(800.0)
        .res(0.7)
        .build()?;

    // Real-time control
    sine_ctl.amp.set(0.5);
    sine_ctl.pitch_bend.set(2.0);
    if let Some(cutoff) = &tb303_ctl.cutoff {
        cutoff.set(1200.0);
    }

    // === Effects ===
    let mut chain = EffectChain::with_registry(EffectRegistry::with_builtin())
        .with_sample_rate(48000.0);

    chain.add_effect("lpf", &[("cutoff".into(), 2000.0)].into())?;
    chain.add_effect("reverb", &Default::default())?;

    println!("Created chain with {} effects", chain.len());

    // Process audio
    for _ in 0..1000 {
        let _ = chain.process(0.5, 0.5);
    }

    // Real-time control
    chain.set_param(0, "cutoff", 5000.0);
    chain.bypass_effect(1, true)?;

    println!("Total CPU: {:.2}%", chain.total_cpu_percent());

    Ok(())
}
