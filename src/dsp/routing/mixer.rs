use crate::dsp::node::AudioNode;

pub struct MixerNode {
    inputs: Vec<Box<dyn AudioNode>>,
}

impl MixerNode {
    pub fn new(
        inputs: Vec<Box<dyn AudioNode>>,
    ) -> Self {
        Self { inputs }
    }
}

impl AudioNode for MixerNode {
    fn next_sample(&mut self) -> f32 {

        let mut mixed = 0.0;

        for input in self.inputs.iter_mut() {
            mixed += input.next_sample();
        }

        // primitive normalization
        mixed / self.inputs.len() as f32
    }

    fn set_frequency(
        &mut self,
        frequency: f32,
    ) {
        for input in self.inputs.iter_mut() {
            input.set_frequency(frequency);
        }
    }
}
