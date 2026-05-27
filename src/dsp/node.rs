pub trait AudioNode {
    fn sample_next(&mut self) ->f32;
}
