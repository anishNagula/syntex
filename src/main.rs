use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

mod dsp;
use dsp::oscillator::SineOscillator;
use dsp::gain::Gain;
use dsp::node::{AudioSource, AudioProcessor};

fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("No output device found");

    let config = device.default_output_config().unwrap();

    let sample_rate = config.sample_rate() as f32;

    let frequency = 440.0;
    let gain_amount = 2.0;

    let mut osc = SineOscillator::new(sample_rate, frequency);

    let gain = Gain::new(gain_amount);

    let err_fn = |err| eprintln!("stream error: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device
            .build_output_stream(
                &config.into(),
                move |data: &mut [f32], _| {
                    for sample in data.iter_mut() {
                        let value = osc.next_sample();
                        let processed = gain.process(value);
                        *sample = processed * 0.2;
                    }
                },
                err_fn,
                None,
            )
            .unwrap(),

        _ => panic!("Unsupported sample format"),
    };

    stream.play().unwrap();

    println!("Playing 440Hz sine wave...");

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
