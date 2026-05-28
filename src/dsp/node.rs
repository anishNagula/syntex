pub trait AudioNode: Send {
    fn next_sample(&mut self) -> f32;

    fn set_frequency(
        &mut self,
        _frequency: f32,
    ) {
    }
}
