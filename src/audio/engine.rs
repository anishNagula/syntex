use crate::dsp::oscillators::sine::SineOscillator;
use crate::dsp::oscillators::square::SquareOscillator;
use crate::dsp::gain::Gain;
use crate::dsp::node::{AudioSource, AudioProcessor};

pub struct AudioEngine {
    sources: Vec<Box<dyn AudioSource>>,
    gain: Gain,
}

impl AudioEngine {
    pub fn new(sample_rate: f32, frequency: f32, gain_amount: f32) -> Self {

        let mut sources: Vec<Box<dyn AudioSource>> = Vec::new();
        sources.push(Box::new(SineOscillator::new(sample_rate, frequency)));
        sources.push(Box::new(SquareOscillator::new(sample_rate, frequency)));

        let gain = Gain::new(gain_amount);

        Self {
            sources,
            gain,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut mixed = 0.0;

        for source in self.sources.iter_mut() {
            mixed += source.next_sample();
        }
        let processed = self.gain.process(mixed);
        processed * 0.2
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        for source in self.sources.iter_mut() {
            source.set_frequency(frequency);
        }
    }
}
