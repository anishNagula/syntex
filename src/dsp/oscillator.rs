use std::f32::consts::PI;

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

    pub fn next_sample(&mut self) -> f32 {
        let sample = self.phase.sin();

        let phase_increment = (2.0 * PI * self.frequency) / self.sample_rate;
        self.phase = (self.phase + phase_increment) % (2.0 * PI);

        sample
    }
}
