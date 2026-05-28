use crate::dsp::node::AudioNode;

pub struct GainNode {
    input: Box<dyn AudioNode>,
    gain: f32,
}

impl GainNode {
    pub fn new(
        input: Box<dyn AudioNode>,
        gain: f32,
    ) -> Self {
        Self {
            input,
            gain,
        }
    }
}

impl AudioNode for GainNode {
    fn next_sample(&mut self) -> f32 {
        let sample =
            self.input.next_sample();

        sample * self.gain
    }

    fn set_frequency(
        &mut self,
        frequency: f32,
    ) {
        self.input
            .set_frequency(frequency);
    }
}
