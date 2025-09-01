use std::sync::{Arc, Mutex};

pub type BusHandle = Arc<Mutex<triple_buffer::Output<Bus>>>;

#[derive(Debug, Clone)]
pub struct Bus {
    pointer: usize,
    pub buffer: [f32; 1024],
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            pointer: 0,
            buffer: [0.0; 1024],
        }
    }
}

impl Bus {
    pub fn write(&mut self, value: f32) {
        self.buffer[self.pointer] = value;
        self.pointer = (self.pointer + 1) & 1023;
    }

    pub fn resample_into<const N: usize>(&self) -> [f32; N] {
        let mut out = [0.0; N];

        let src_len = self.buffer.len() as f32;
        let dst_len = N as f32;

        for (i, item) in out.iter_mut().enumerate().take(N) {
            let t = i as f32 * (src_len - 1.0) / (dst_len - 1.0);
            let idx = t.floor() as usize;
            let frac = t - idx as f32;

            let next_idx = if idx + 1 < self.buffer.len() {
                idx + 1
            } else {
                idx
            };

            *item = self.buffer[idx] * (1.0 - frac) + self.buffer[next_idx] * frac;
        }

        out
    }
}
