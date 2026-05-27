use crate::dsp::oscillators::sine::SineOscillator;
use crate::dsp::oscillators::square::SquareOscillator;
use crate::dsp::oscillators::lfo::LFOOscillator;
use crate::dsp::gain::Gain;
use crate::dsp::node::{AudioSource, AudioProcessor};

pub struct AudioEngine {
    sources: Vec<Box<dyn AudioSource>>,
    gain: Gain,
    lfo: LFOOscillator,
    base_frequency: f32,
}

impl AudioEngine {
    pub fn new(sample_rate: f32, frequency: f32, gain_amount: f32) -> Self {

        let mut sources: Vec<Box<dyn AudioSource>> = Vec::new();
        sources.push(Box::new(SineOscillator::new(sample_rate, frequency)));
        sources.push(Box::new(SquareOscillator::new(sample_rate, frequency)));

        let gain = Gain::new(gain_amount);

        let lfo = LFOOscillator::new(sample_rate, 5.0, 20.0);

        Self {
            sources,
            gain,
            lfo,
            base_frequency: frequency,
        }
    }

    pub fn next_sample(&mut self) -> f32 {

        let lfo_sample = self.lfo.next_sample();

        let modulated_frequency = self.base_frequency + (lfo_sample * self.lfo.modulation_depth());

        for source in self.sources.iter_mut() {
            source.set_frequency(modulated_frequency);
        }

        let mut mixed = 0.0;

        for source in self.sources.iter_mut() {
            mixed += source.next_sample();
        }

        mixed /= self.sources.len() as f32;

        let processed = self.gain.process(mixed);
        
        processed * 0.2
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        for source in self.sources.iter_mut() {
            self.base_frequency = frequency;
        }
    }
}
