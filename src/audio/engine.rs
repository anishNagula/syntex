use crate::dsp::oscillators::sine::SineOscillator;
use crate::dsp::oscillators::square::SquareOscillator;
use crate::dsp::gain::Gain;
use crate::dsp::node::{AudioSource, AudioProcessor};

pub struct AudioEngine {
    osc: Box<dyn AudioSource>,
    gain: Gain,
}

impl AudioEngine {
    pub fn new(sample_rate: f32, frequency: f32, gain_amount: f32) -> Self {
        let osc = Box::new(SineOscillator::new(sample_rate, frequency));

        let gain = Gain::new(gain_amount);

        Self {
            osc,
            gain,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let value = self.osc.next_sample();
        let processed = self.gain.process(value);
        processed * 0.2
    }
}
