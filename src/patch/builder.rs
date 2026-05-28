use crate::dsp::node::AudioNode;

use crate::dsp::generators::sine::SineOscillator;
use crate::dsp::generators::square::SquareOscillator;

use crate::dsp::processors::gain::GainNode;
use crate::dsp::processors::lowpass::LowPassFilterNode;

use crate::dsp::routing::mixer::MixerNode;

pub struct PatchBuilder;

impl PatchBuilder {

    pub fn basic_patch(
        sample_rate: f32,
        frequency: f32,
        gain_amount: f32,
    ) -> Box<dyn AudioNode> {

        // Sine oscillator
        let sine: Box<dyn AudioNode> =
            Box::new(
                SineOscillator::new(
                    sample_rate,
                    frequency,
                ),
            );

        // Square oscillator
        let square: Box<dyn AudioNode> =
            Box::new(
                SquareOscillator::new(
                    sample_rate,
                    frequency,
                ),
            );

        // Filter square oscillator
        let filtered_square: Box<dyn AudioNode> =
            Box::new(
                LowPassFilterNode::new(
                    square,
                    0.05,
                ),
            );

        // Mix sine + filtered square
        let mixer: Box<dyn AudioNode> =
            Box::new(
                MixerNode::new(vec![
                    sine,
                    filtered_square,
                ]),
            );

        // Final gain stage
        let gain: Box<dyn AudioNode> =
            Box::new(
                GainNode::new(
                    mixer,
                    gain_amount,
                ),
            );

        gain
    }
}
