use std::f32::consts::PI;

use crate::dsp::node::AudioNode;

pub struct SineOscillator {
    sample_rate: f32,
    frequency: f32,
    phase: f32,
}

impl SineOscillator {
    pub fn new(sample_rate: f32, frequency: f32) -> Self {
        Self {
            sample_rate,
            frequency,
            phase: 0.0,
        }
    }
}

impl AudioNode for SineOscillator {
    fn next_sample(&mut self) -> f32 {
        let sample = self.phase.sin();

        let phase_increment =
            (2.0 * PI * self.frequency) / self.sample_rate;

        self.phase =
            (self.phase + phase_increment) % (2.0 * PI);

        sample
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}
