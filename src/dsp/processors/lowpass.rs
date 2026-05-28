use crate::dsp::node::AudioNode;

pub struct LowPassFilterNode {
    input: Box<dyn AudioNode>,

    alpha: f32,

    previous_output: f32,
}

impl LowPassFilterNode {
    pub fn new(
        input: Box<dyn AudioNode>,
        alpha: f32,
    ) -> Self {
        Self {
            input,
            alpha,
            previous_output: 0.0,
        }
    }
}

impl AudioNode for LowPassFilterNode {

    fn next_sample(&mut self) -> f32 {

        // Get sample from child node
        let input_sample =
            self.input.next_sample();

        // One-pole lowpass filter
        let output =
            self.previous_output
            + (
                self.alpha
                * (
                    input_sample
                    - self.previous_output
                )
            );

        // Store filter history
        self.previous_output = output;

        output
    }

    fn set_frequency(
        &mut self,
        frequency: f32,
    ) {
        self.input
            .set_frequency(frequency);
    }
}
