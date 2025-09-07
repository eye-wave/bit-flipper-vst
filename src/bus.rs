use crossbeam_channel::{Receiver, Sender, bounded};
use nih_plug::buffer::Buffer;

pub const BUFFER_SIZE: usize = 2048;

#[derive(Debug, Clone)]
pub struct Bus {
    channel: (Sender<f32>, Receiver<f32>),
}

impl Default for Bus {
    fn default() -> Self {
        Self::new(BUFFER_SIZE)
    }
}

impl Bus {
    pub fn new(size: usize) -> Self {
        let channel = bounded(size);
        Self { channel }
    }

    pub fn read(&self) -> Vec<f32> {
        self.channel.1.try_iter().collect()
    }

    pub fn send(&self, value: f32) {
        self.channel.0.try_send(value).ok();
    }

    pub fn send_buffer_summing(&self, buffer: &mut Buffer) {
        let channels = buffer.channels();

        if channels == 1 {
            for mut x in buffer.iter_samples() {
                self.send(*x.get_mut(0).unwrap());
            }
        } else {
            for mut x in buffer.iter_samples() {
                self.send(x.iter_mut().map(|x| *x).sum::<f32>() / channels as f32);
            }
        }
    }
}
