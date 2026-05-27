Syntex is a modular audio engine written in Rust.

It generates and processes real-time audio through small composable DSP modules like oscillators, gain stages, filters, and effects connected together in signal chains.

Instead of using graphical patch editors or piano-roll interfaces, Syntex focuses on building and controlling audio pipelines through code and text-based configuration.

<hr style="height:1px;border:none;color:#333;background-color:#333;" />

#### The project currently implements:

- realtime audio output
- a sine wave oscillator
- a basic DSP processing chain
- modular separation between audio output and DSP generation

#### Current signal flow:

```Oscillator -> Gain -> Speaker```

<hr style="height:1px;border:none;color:#333;background-color:#333;" />


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
 
<hr style="height:1px;border:none;color:#333;background-color:#333;" />


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
