use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;

#[derive(Enum, Debug, PartialEq, Clone, Copy, Data)]
pub enum FlipModes {
    And,
    Or,
    Not,
    Xor,
}

impl Default for FlipModes {
    fn default() -> Self {
        Self::Xor
    }
}

impl FlipModes {
    pub fn transform(&self, sample: f32, mask: u32) -> f32 {
        let bits = sample.to_bits();
        let flipped = match self {
            Self::And => bits & mask,
            Self::Or => bits | mask,
            Self::Not => !bits,
            Self::Xor => bits ^ mask,
        };

        f32::from_bits(flipped).clamp(-1.0, 1.0)
    }
}
