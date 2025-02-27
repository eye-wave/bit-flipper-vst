pub struct DCBlocker {
    alpha: f32,
    prev_input: f32,
    prev_output: f32,
}

impl DCBlocker {
    pub fn new(alpha: f32) -> Self {
        Self {
            alpha,
            prev_input: 0.0,
            prev_output: 0.0,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let output = input - self.prev_input + self.alpha * self.prev_output;
        self.prev_input = input;
        self.prev_output = output;
        output
    }
}
