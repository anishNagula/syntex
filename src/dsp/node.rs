pub trait AudioSource: Send {
    fn next_sample(&mut self) ->f32;
    fn set_frequency(&mut self, frequency: f32);
}

pub trait AudioProcessor {
    fn process(&self, input: f32) -> f32;
}
