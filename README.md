<div align="center">
  <img width="600" height="314" alt="Syntex(1) " src="https://github.com/user-attachments/assets/ea0a12e3-bf5e-4d5d-a0ba-2b7f25774ad6" />
</div>
<br>

Syntex is a modular audio engine written in Rust.

It generates and processes real-time audio through small composable DSP modules like oscillators, gain stages, filters, and effects connected together in signal chains.

Instead of using graphical patch editors or piano-roll interfaces, Syntex focuses on building and controlling audio pipelines through code and text-based configuration.

---

#### The project currently implements:

- realtime audio output
- a sine wave oscillator
- a basic DSP processing chain
- modular separation between audio output and DSP generation

#### Current signal flow:

```Oscillator -> Gain -> Speaker```

---

#### The audio backend is responsible for:

- opening the output device
- streaming audio buffers
- communicating with the operating system audio layer

#### DSP modules are responsible for:
- generating audio samples
- processing signals
- maintaining DSP state

#### Current DSP components include:
- SineOscillator
    - Generates continuous sine-wave audio using phase accumulation.
- Gain
    - Scales incoming sample amplitudes before output.
 
---

#### DSP traits currently implemented:
- `AudioSource`
    - Shared interface for modules capable of generating audio samples.
- `AudioProcessor`
    - Shared interface for modules that process incoming audio samples.

---

#### Audio Engine

Syntex currently uses a dedicated `AudioEngine` layer to separate DSP execution from the audio backend.

The engine is responsible for:

- owning DSP modules
- executing the processing chain
- generating final output samples

This keeps the audio backend independent from DSP implementation details.

#### Current execution model:
```
main.rs
    ↓
AudioEngine
    ↓
Oscillator -> Gain
    ↓
Output Sample
```

---

#### Realtime DSP Notes

Audio generation in Syntex is stateful.

Modules like oscillators maintain internal state across audio callbacks to ensure continuous waveform generation without discontinuities or audible artifacts.

The current sine oscillator uses phase accumulation:

```phase -> sine -> sample```

where phase advances continuously based on:
- frequency
- sample rate

---


### Current project structure:
```
src/
├── audio/
├── dsp/
│   ├── mod.rs
│   ├── oscillator.rs
│   └── gain.rs
└── main.rs
```

The project is currently experimental and heavily focused on learning and architecture exploration.

