//! Render synths to WAV files

use anyhow::Result;
use fundsp::hacker32::*;
use fundsp_rack::prelude::*;
use hound::{WavSpec, WavWriter};

const SAMPLE_RATE: f64 = 48000.0;

fn main() -> Result<()> {
    let registry = SynthRegistry::with_builtin();
    let spec = WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // Sine wave
    let (synth, _) = registry.synth("sine").freq(440.0).amp(0.3).build()?;
    render("output_sine.wav", synth, &spec, 3.0)?;

    // TB-303 bass
    let (synth, _) = registry
        .synth("tb303")
        .freq(55.0)
        .cutoff(800.0)
        .res(0.7)
        .amp(0.4)
        .build()?;
    render("output_tb303.wav", synth, &spec, 3.0)?;

    // FM chord (C major)
    let (c, _) = registry
        .synth("fm")
        .note(60)
        .ratio(3.5)
        .index(2.0)
        .amp(0.15)
        .build()?;
    let (e, _) = registry
        .synth("fm")
        .note(64)
        .ratio(3.5)
        .index(2.0)
        .amp(0.15)
        .build()?;
    let (g, _) = registry
        .synth("fm")
        .note(67)
        .ratio(3.5)
        .index(2.0)
        .amp(0.15)
        .build()?;
    let mixed = Box::new(Net::wrap(c) + Net::wrap(e) + Net::wrap(g));
    render("output_fm_chord.wav", mixed, &spec, 4.0)?;

    // Supersaw
    let (synth, _) = registry
        .synth("supersaw")
        .freq(220.0)
        .detune(0.02)
        .amp(0.2)
        .build()?;
    render("output_supersaw.wav", synth, &spec, 3.0)?;

    println!(
        "Rendered: output_sine.wav, output_tb303.wav, output_fm_chord.wav, output_supersaw.wav"
    );
    Ok(())
}

fn render(filename: &str, mut synth: Box<dyn AudioUnit>, spec: &WavSpec, secs: f32) -> Result<()> {
    synth.set_sample_rate(SAMPLE_RATE);
    synth.allocate();
    let mut writer = WavWriter::create(filename, *spec)?;
    for _ in 0..(SAMPLE_RATE * secs as f64) as usize {
        let (l, r) = synth.get_stereo();
        writer.write_sample((l.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)?;
        writer.write_sample((r.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)?;
    }
    writer.finalize()?;
    Ok(())
}
