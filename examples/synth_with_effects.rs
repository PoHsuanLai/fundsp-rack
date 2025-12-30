//! Example: Processing synths through effect chains
//!
//! This example demonstrates the full workflow of fundsp-rack:
//! 1. Create synths using the registry
//! 2. Process audio through an effect chain
//! 3. Render the result to WAV files

use anyhow::Result;
use fundsp::hacker32::*;
use fundsp_rack::prelude::*;
use hound::{WavSpec, WavWriter};
use std::sync::Arc;

const SAMPLE_RATE: f64 = 48000.0;

fn main() -> Result<()> {
    let synth_registry = SynthRegistry::with_builtin();
    let effect_registry = Arc::new(EffectRegistry::with_builtin());

    let spec = WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // === Example 1: Pad synth with reverb and chorus ===
    println!("Rendering pad with reverb and chorus...");
    {
        let (synth, _controls) = synth_registry
            .synth("pad")
            .freq(220.0)
            .amp(0.3)
            .build()?;

        let mut chain = EffectChain::with_shared_registry(Arc::clone(&effect_registry))
            .with_sample_rate(SAMPLE_RATE);
        chain
            .add("chorus", &[("rate", 0.5), ("depth", 0.3)])?
            .add("hall", &[("mix", 0.4)])?;

        render_with_effects("output_pad_reverb.wav", synth, &mut chain, &spec, 4.0)?;
    }

    // === Example 2: TB-303 with distortion and delay ===
    println!("Rendering TB-303 with distortion and delay...");
    {
        let (synth, _controls) = synth_registry
            .synth("tb303")
            .freq(55.0)
            .cutoff(1200.0)
            .res(0.8)
            .amp(0.4)
            .build()?;

        let mut chain = EffectChain::with_shared_registry(Arc::clone(&effect_registry))
            .with_sample_rate(SAMPLE_RATE);
        chain
            .add("distortion", &[("drive", 3.0), ("mix", 0.5)])?
            .add("ping_pong", &[("time", 0.3), ("feedback", 0.4), ("mix", 0.3)])?
            .add("lpf", &[("cutoff", 4000.0)])?;

        render_with_effects("output_tb303_fx.wav", synth, &mut chain, &spec, 4.0)?;
    }

    // === Example 3: Supersaw lead with full chain ===
    println!("Rendering supersaw lead with full effect chain...");
    {
        let (synth, _controls) = synth_registry
            .synth("supersaw")
            .freq(440.0)
            .detune(0.015)
            .amp(0.25)
            .build()?;

        // Classic trance lead chain
        let mut chain = EffectChain::with_shared_registry(Arc::clone(&effect_registry))
            .with_sample_rate(SAMPLE_RATE);
        chain
            .add("chorus", &[("rate", 0.8), ("depth", 0.2)])?
            .add("eq_3band", &[("low", -2.0), ("mid", 1.0), ("high", 2.0)])?
            .add("compressor", &[("threshold", 0.5), ("ratio", 4.0)])?
            .add("stereo_delay", &[("time_l", 0.25), ("time_r", 0.375), ("feedback", 0.3), ("mix", 0.25)])?
            .add("plate", &[("mix", 0.2)])?;

        render_with_effects("output_supersaw_lead.wav", synth, &mut chain, &spec, 4.0)?;
    }

    // === Example 4: Lo-fi piano ===
    println!("Rendering lo-fi piano...");
    {
        let (synth, _controls) = synth_registry
            .synth("electric_piano")
            .freq(261.63) // Middle C
            .amp(0.5)
            .build()?;

        let mut chain = EffectChain::with_shared_registry(Arc::clone(&effect_registry))
            .with_sample_rate(SAMPLE_RATE);
        chain
            .add("tape_saturation", &[("drive", 1.5), ("mix", 0.6)])?
            .add("lofi", &[("bitdepth", 12.0), ("sample_rate", 22050.0)])?
            .add("room", &[("mix", 0.3)])?;

        render_with_effects("output_lofi_piano.wav", synth, &mut chain, &spec, 4.0)?;
    }

    // === Example 5: Polyphonic strings with effects ===
    println!("Rendering polyphonic strings with effects...");
    {
        // Create a C major chord using multiple voices
        let (c4, _) = synth_registry.synth("strings").note(60).amp(0.2).build()?;
        let (e4, _) = synth_registry.synth("strings").note(64).amp(0.2).build()?;
        let (g4, _) = synth_registry.synth("strings").note(67).amp(0.2).build()?;

        // Mix the chord
        let chord = Box::new(Net::wrap(c4) + Net::wrap(e4) + Net::wrap(g4));

        let mut chain = EffectChain::with_shared_registry(Arc::clone(&effect_registry))
            .with_sample_rate(SAMPLE_RATE);
        chain
            .add("chorus", &[("rate", 0.3), ("depth", 0.4)])?
            .add("hall", &[("mix", 0.5)])?
            .add("stereo_width", &[("width", 1.5)])?;

        render_with_effects("output_strings_chord.wav", chord, &mut chain, &spec, 5.0)?;
    }

    println!("\nRendered files:");
    println!("  - output_pad_reverb.wav      (pad + chorus + hall reverb)");
    println!("  - output_tb303_fx.wav        (tb303 + distortion + ping-pong delay)");
    println!("  - output_supersaw_lead.wav   (supersaw + full lead chain)");
    println!("  - output_lofi_piano.wav      (electric piano + tape + lofi)");
    println!("  - output_strings_chord.wav   (strings chord + chorus + hall)");

    Ok(())
}

/// Render a synth through an effect chain to a WAV file
fn render_with_effects(
    filename: &str,
    mut synth: Box<dyn AudioUnit>,
    chain: &mut EffectChain,
    spec: &WavSpec,
    duration_secs: f32,
) -> Result<()> {
    synth.set_sample_rate(SAMPLE_RATE);
    synth.allocate();

    let mut writer = WavWriter::create(filename, *spec)?;
    let total_samples = (SAMPLE_RATE * duration_secs as f64) as usize;

    for _ in 0..total_samples {
        // Get audio from synth
        let (synth_l, synth_r) = synth.get_stereo();

        // Process through effect chain
        let (out_l, out_r) = chain.process(synth_l, synth_r);

        // Write to file
        writer.write_sample((out_l.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)?;
        writer.write_sample((out_r.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)?;
    }

    writer.finalize()?;
    Ok(())
}
