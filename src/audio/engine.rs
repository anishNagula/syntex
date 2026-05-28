use crate::dsp::generators::lfo::LFOOscillator;
use crate::dsp::generators::sine::SineOscillator;
use crate::dsp::generators::square::SquareOscillator;
use crate::dsp::processors::lowpass::LowPassFilterNode;

use crate::dsp::node::AudioNode;

use crate::dsp::processors::gain::GainNode;

use crate::dsp::routing::mixer::MixerNode;

pub struct AudioEngine {
    root: Box<dyn AudioNode>,

    lfo: LFOOscillator,

    base_frequency: f32,
}

impl AudioEngine {
    pub fn new(
        sample_rate: f32,
        frequency: f32,
        gain_amount: f32,
    ) -> Self {

        // Generator nodes
        let sine: Box<dyn AudioNode> =
            Box::new(
                SineOscillator::new(
                    sample_rate,
                    frequency,
                ),
            );

        let square: Box<dyn AudioNode> =
            Box::new(
                SquareOscillator::new(
                    sample_rate,
                    frequency,
                ),
            );

        let filtered_square: Box<dyn AudioNode> =
            Box::new(
                LowPassFilterNode::new(
                    square,
                    0.05,
                ),
            );

        // Mixer node
        let mixer: Box<dyn AudioNode> =
            Box::new(
                MixerNode::new(vec![
                    sine,
                    filtered_square,
                ]),
            );

        // Gain node
        let gain: Box<dyn AudioNode> =
            Box::new(
                GainNode::new(
                    mixer,
                    gain_amount,
                ),
            );

        // LFO modulation source
        let lfo = LFOOscillator::new(
            sample_rate,
            5.0,
            20.0,
        );

        Self {
            root: gain,
            lfo,
            base_frequency: frequency,
        }
    }

    pub fn next_sample(
        &mut self,
    ) -> f32 {

        // Generate modulation signal
        let lfo_sample =
            self.lfo.next_sample();

        // Compute modulated frequency
        let modulated_frequency =
            self.base_frequency
            + (
                lfo_sample
                * self.lfo.modulation_depth()
            );

        // Propagate modulation
        self.root.set_frequency(
            modulated_frequency,
        );

        // Recursively evaluate DSP tree
        self.root.next_sample()
    }

    pub fn set_frequency(
        &mut self,
        frequency: f32,
    ) {
        self.base_frequency =
            frequency;
    }
}
