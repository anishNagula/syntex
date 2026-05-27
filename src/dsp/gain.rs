pub struct Gain {
    amount: f32,
}

impl Gain {
    pub fn new(amount: f32) -> Self {
        Self {
            amount,
        }
    }

    pub fn process(&self, sample: f32) -> f32 {
        let processed_sample = sample * self.amount;

        processed_sample
    }
}
