use super::{StaticBox, StaticBoxPipeline, UiBox, UiElement, UiInteractive, texture::TextureError};
use crate::editor::texture::UVSegment::{self, *};
use rand::Rng;
use std::sync::Arc;

pub struct Digit {
    id: u8,
    static_box: StaticBox,
    position: (u16, u16),
    is_on: bool,
}

enum DigitType {
    One,
    Zero,
}

impl From<bool> for DigitType {
    fn from(value: bool) -> Self {
        match value {
            true => Self::One,
            false => Self::Zero,
        }
    }
}

impl Digit {
    pub fn new(
        device: &wgpu::Device,
        id: u8,
        position: (u16, u16),
        pipeline: Arc<StaticBoxPipeline>,
    ) -> Result<Self, TextureError> {
        let static_box = StaticBox::new(device, &UV_digi_1_0, position, Some(2.0 / 6.0), pipeline)?;

        Ok(Self {
            id,
            static_box,
            position,
            is_on: true,
        })
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    fn random_uv(&self, d_type: DigitType) -> UVSegment {
        let mut rng = rand::rng();
        let n: u8 = rng.random_range(0..=8);

        match d_type {
            DigitType::One => match n {
                1 => UV_digi_1_1,
                2 => UV_digi_1_2,
                3 => UV_digi_1_3,
                4 => UV_digi_1_4,
                5 => UV_digi_1_5,
                6 => UV_digi_1_6,
                7 => UV_digi_1_7,
                _ => UV_digi_1_0,
            },
            DigitType::Zero => match n {
                1 => UV_digi_0_1,
                2 => UV_digi_0_2,
                3 => UV_digi_0_3,
                4 => UV_digi_0_4,
                5 => UV_digi_0_5,
                6 => UV_digi_0_6,
                7 => UV_digi_0_7,
                _ => UV_digi_0_0,
            },
        }
    }
}

impl UiElement for Digit {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn prerender(
        &mut self,
        queue: &wgpu::Queue,
        params: Arc<crate::BitFlipperParams>,
        _buffer: &crate::Bus,
    ) {
        let val = params
            .bits
            .get_bit_param(self.id)
            .map(|param| param.value())
            .unwrap_or_default();

        if self.is_on != val {
            self.is_on = val;

            let uv_id = self.random_uv(val.into());
            self.static_box.swap_uv(queue, &uv_id).ok();
        }
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.static_box.render(render_pass);
    }
}

impl UiBox for Digit {
    fn width(&self) -> u16 {
        11
    }

    fn height(&self) -> u16 {
        11
    }

    fn position(&self) -> (u16, u16) {
        let (x, y) = self.position;
        (x - 1, y)
    }
}

impl UiInteractive for Digit {}

pub struct DigitCluster {
    pub digits: Vec<Digit>,
}

impl DigitCluster {
    pub fn new(device: &wgpu::Device, pipeline: Arc<StaticBoxPipeline>) -> Self {
        let mut digits = Vec::<_>::new();

        digits.push(Digit::new(device, 32, (74, 30), pipeline.clone()).unwrap());

        for i in 0..8usize {
            let x = 75 + (i & 3) * 11;
            let y = 45 + (i / 4) * 11;

            digits.push(
                Digit::new(device, 31 - i as u8, (x as u16, y as u16), pipeline.clone()).unwrap(),
            );
        }

        for i in 0..23usize {
            let x = 75 + (i & 3) * 11;
            let y = 71 + (i / 4) * 11;

            digits.push(
                Digit::new(device, 23 - i as u8, (x as u16, y as u16), pipeline.clone()).unwrap(),
            );
        }

        Self { digits }
    }
}

impl UiElement for DigitCluster {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn prerender(
        &mut self,
        queue: &wgpu::Queue,
        params: Arc<crate::BitFlipperParams>,
        _buffer: &crate::Bus,
    ) {
        for digi in self.digits.iter_mut() {
            digi.prerender(queue, params.clone(), _buffer);
        }
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for digi in self.digits.iter() {
            digi.render(render_pass);
        }
    }
}

#[rustfmt::skip]
impl UiBox for DigitCluster {
    fn width(&self) -> u16 { 0 }
    fn height(&self) -> u16 { 0 }
    fn position(&self) -> (u16, u16) { (0, 0) }
}

impl UiInteractive for DigitCluster {}
