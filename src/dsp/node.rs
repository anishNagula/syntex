pub trait AudioSource: Send {
    fn next_sample(&mut self) ->f32;
}

pub trait AudioProcessor {
    fn process(&self, input: f32) -> f32;
}
