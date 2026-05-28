use crate::dsp::node::AudioNode;

use crate::dsp::generators::sine::SineOscillator;
use crate::dsp::generators::square::SquareOscillator;

use crate::dsp::processors::gain::GainNode;
use crate::dsp::processors::lowpass::LowPassFilterNode;

use crate::dsp::routing::mixer::MixerNode;

use crate::patch::definition::PatchNode;

pub struct PatchBuilder;

impl PatchBuilder {

    pub fn build(
        node: PatchNode,
        sample_rate: f32,
    ) -> Box<dyn AudioNode> {

        match node {

            PatchNode::Sine {
                frequency,
            } => {

                Box::new(
                    SineOscillator::new(
                        sample_rate,
                        frequency,
                    ),
                )
            }

            PatchNode::Square {
                frequency,
            } => {

                Box::new(
                    SquareOscillator::new(
                        sample_rate,
                        frequency,
                    ),
                )
            }

            PatchNode::LowPass {
                input,
                alpha,
            } => {

                let input_node =
                    Self::build(
                        *input,
                        sample_rate,
                    );

                Box::new(
                    LowPassFilterNode::new(
                        input_node,
                        alpha,
                    ),
                )
            }

            PatchNode::Mixer {
                inputs,
            } => {

                let built_inputs =
                    inputs
                        .into_iter()
                        .map(|input| {
                            Self::build(
                                input,
                                sample_rate,
                            )
                        })
                        .collect();

                Box::new(
                    MixerNode::new(
                        built_inputs,
                    ),
                )
            }

            PatchNode::Gain {
                input,
                gain,
            } => {

                let input_node =
                    Self::build(
                        *input,
                        sample_rate,
                    );

                Box::new(
                    GainNode::new(
                        input_node,
                        gain,
                    ),
                )
            }
        }
    }
}
