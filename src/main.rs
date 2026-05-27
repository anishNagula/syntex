use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

mod dsp;
mod audio;

use audio::engine::AudioEngine;


fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("No output device found");

    let config = device.default_output_config().unwrap();

    let sample_rate = config.sample_rate() as f32;

    let mut frequency = 440.0;
    let gain_amount = 0.2;

    let mut engine = AudioEngine::new(sample_rate, frequency, gain_amount);

    let err_fn = |err| eprintln!("stream error: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device
            .build_output_stream(
                &config.into(),
                move |data: &mut [f32], _| {
                    for sample in data.iter_mut() {
                        *sample = engine.next_sample();
                        frequency += 0.01;
                        engine.set_frequency(frequency);
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
