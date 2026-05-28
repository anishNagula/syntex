use crate::dsp::generators::lfo::LFOOscillator;

use crate::dsp::node::AudioNode;

use crate::patch::builder::PatchBuilder;

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

        let root =
            PatchBuilder::basic_patch(
                sample_rate,
                frequency,
                gain_amount,
            );

        let lfo = LFOOscillator::new(
            sample_rate,
            5.0,
            20.0,
        );

        Self {
            root,
            lfo,
            base_frequency: frequency,
        }
    }

    pub fn next_sample(
        &mut self,
    ) -> f32 {

        let lfo_sample =
            self.lfo.next_sample();

        let modulated_frequency =
            self.base_frequency
            + (
                lfo_sample
                * self.lfo.modulation_depth()
            );

        self.root.set_frequency(
            modulated_frequency,
        );

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
