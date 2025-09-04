use std::sync::{Arc, Mutex};

const MAX_BUF_SIZE: usize = 2048;

pub type BufHandle = Arc<Mutex<triple_buffer::Output<WriteBuffer>>>;

#[derive(Debug, Clone)]
pub struct WriteBuffer {
    pointer: usize,
    buf_size: usize,
    pub buffer: [f32; MAX_BUF_SIZE],
}

impl Default for WriteBuffer {
    fn default() -> Self {
        Self::from_size(512)
    }
}

impl WriteBuffer {
    pub fn from_size(buf_size: usize) -> Self {
        Self {
            pointer: 0,
            buf_size,
            buffer: [0.0; MAX_BUF_SIZE],
        }
    }
}

impl WriteBuffer {
    pub fn write(&mut self, value: f32) {
        self.buffer[self.pointer] = value;
        self.pointer = (self.pointer + 1) % (self.buf_size);
    }

    pub fn resample_into<const N: usize>(&self) -> [f32; N] {
        let mut out = [0.0; N];

        let src_len = self.buf_size as f32;
        let dst_len = N as f32;

        for (i, item) in out.iter_mut().enumerate().take(N) {
            let t = i as f32 * (src_len - 1.0) / (dst_len - 1.0);
            let idx = t.floor() as usize;
            let frac = t - idx as f32;

            let next_idx = if idx + 1 < self.buf_size {
                idx + 1
            } else {
                idx
            };

            *item = self.buffer[idx] * (1.0 - frac) + self.buffer[next_idx] * frac;
        }

        out
    }
}
