use std::fs;

use crate::dsp::generators::lfo::LFOOscillator;

use crate::dsp::node::AudioNode;

use crate::patch::builder::PatchBuilder;
use crate::patch::definition::PatchNode;

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

        // Load external patch file
        let patch_json =
            fs::read_to_string(
                "patches/basic.json",
            )
            .expect(
                "failed to read patch file",
            );

        // Deserialize patch definition
        let patch: PatchNode =
            serde_json::from_str(
                &patch_json,
            )
            .expect(
                "invalid patch json",
            );

        // Build executable DSP tree
        let root =
            PatchBuilder::build(
                patch,
                sample_rate,
            );

        // LFO modulation source
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

        // Evaluate recursive DSP tree
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
