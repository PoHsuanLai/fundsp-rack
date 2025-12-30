# fundsp-rack

A preset library for [FunDSP](https://github.com/SamiPerttu/fundsp): ready-to-use synthesizers and effects with friendly APIs.

## What is this?

**fundsp-rack** provides:

- **30+ synthesizers**: Basic waveforms, analog emulations, pads, leads, keys, and more
- **50+ effects**: Reverbs, delays, filters, distortion, dynamics, modulation, EQ
- **Polyphony**: Easy voice management for playing chords
- **Real-time control**: Lock-free parameter updates via `Shared` variables
- **Effect chains**: Combine multiple effects with ordering and bypass

This is a **preset/convenience library** - it doesn't replace FunDSP, it extends it with ready-made instruments and effects.

## Quick Start

```toml
[dependencies]
fundsp-rack = "0.1"
```

### Synthesizers

```rust
use fundsp_rack::prelude::*;

let registry = SynthRegistry::with_builtin();

// Create a synth with fluent API
let (synth, controls) = registry.synth("tb303")
    .freq(55.0)
    .param("cutoff", 800.0)
    .param("res", 0.7)
    .build()?;

// Real-time control
controls.amp.set(0.5);
controls.pitch_bend.set(2.0);
```

### Polyphonic Synths

```rust
use fundsp_rack::prelude::*;

// Create an 8-voice polyphonic synth
let mut poly = PolySynth::new("strings", 8);

// Play a chord
poly.note_on(60, 0.8); // C4
poly.note_on(64, 0.8); // E4
poly.note_on(67, 0.8); // G4

// Get audio output
let (left, right) = poly.get_stereo();

// Release notes
poly.note_off(60);
poly.all_notes_off();
```

### Effects

```rust
use fundsp_rack::prelude::*;

let registry = EffectRegistry::with_builtin();

// Create an effect with fluent API
let (effect, controls) = registry.effect("hall")
    .param("mix", 0.4)
    .build()?;

// Real-time control
controls.set("mix", 0.5);
```

### Effect Chain

```rust
use fundsp_rack::prelude::*;

let mut chain = EffectChain::with_registry(EffectRegistry::with_builtin());

chain.add_effect("chorus", &Default::default())?;
chain.add_effect("hall", &Default::default())?;
chain.add_effect("tape", &Default::default())?;

// Process audio
let (out_l, out_r) = chain.process(in_l, in_r);

// Bypass/mute effects
chain.bypass_effect(0, true)?;
```

## Features

```toml
# Default (no extra features)
fundsp-rack = "0.1"

# With serialization support
fundsp-rack = { version = "0.1", features = ["serde"] }
```

## Built-in Synths

| Category  | Synths                                |
| --------- | ------------------------------------- |
| Basic     | sine, saw, square, tri, pulse         |
| Analog    | tb303, prophet, supersaw, hoover      |
| Keys      | organ, electric_piano (rhodes)        |
| Leads     | lead, sub, brass                      |
| Pads      | strings, pad                          |
| Modulated | mod_saw, mod_sine, mod_tri, mod_pulse |
| Detuned   | dsaw, dpulse, dtri                    |
| Digital   | fm, pretty_bell, dull_bell            |
| Physical  | piano, pluck                          |
| Bass      | bass_foundation, bass_highend         |
| Ambient   | dark_ambience, growl                  |
| Tech      | tech_saws, blade, zawa                |
| Noise     | noise                                 |

## Built-in Effects

| Category   | Effects                                        |
| ---------- | ---------------------------------------------- |
| Reverb     | reverb, room, hall, plate                      |
| Delay      | delay, stereo_delay, ping_pong, slapback, echo |
| Modulation | chorus, flanger, phaser, tremolo, vibrato      |
| Filter     | lpf, hpf, bpf, notch                           |
| EQ         | eq_3band, tilt_eq, low_shelf, high_shelf       |
| Dynamics   | compressor, limiter, gate, expander            |
| Distortion | distortion, bitcrusher, krush                  |
| Lo-Fi      | tape_saturation, lofi, vinyl                   |
| Spatial    | pan, stereo_width                              |
| Other      | gain, dc_block                                 |

## Custom Synths

```rust
use fundsp_rack::prelude::*;
use fundsp::hacker32::*;
use std::collections::HashMap;

struct MySynth;

impl SynthBuilder for MySynth {
    fn build(&self, freq: f32, _params: &HashMap<String, f32>)
        -> (Box<dyn AudioUnit>, VoiceControls)
    {
        let amp = shared(1.0);
        let pitch_bend = shared(1.0);

        let osc = var_fn(&pitch_bend, move |b| freq * b) >> sine();
        let synth = osc * var(&amp);

        (Box::new(synth | synth.clone()), VoiceControls {
            amp,
            pitch_bend,
            cutoff: None,
            resonance: None,
            pressure: shared(0.0),
        })
    }

    fn metadata(&self) -> SynthMetadata {
        SynthMetadata::new("my_synth", "Custom synth", SynthCategory::Basic)
    }
}

// Register and use
let mut registry = SynthRegistry::with_builtin();
registry.register("my_synth", Arc::new(MySynth));
```

## Custom Effects

```rust
use fundsp_rack::prelude::*;
use fundsp::hacker32::*;
use std::collections::HashMap;

struct MyEffect;

impl EffectBuilder for MyEffect {
    fn build(&self, params: &HashMap<String, f32>)
        -> (Box<dyn AudioUnit>, EffectControls)
    {
        let gain = shared(params.get("gain").copied().unwrap_or(1.0));
        let effect = (pass() * var(&gain)) | (pass() * var(&gain));

        let mut controls = EffectControls::new();
        controls.params.insert("gain".into(), gain);

        (Box::new(effect), controls)
    }

    fn metadata(&self) -> EffectMetadata {
        EffectMetadata::new("my_effect", "Custom gain", EffectCategory::Other)
            .with_param("gain", 1.0, 0.0, 2.0)
    }
}
```

## License

MIT OR Apache-2.0
