use crate::editor::texture::{
    TextureError,
    UVSegment::{self, *},
};
use crate::editor::ui::{StaticBox, StaticBoxPipeline, UiElement};
use std::sync::Arc;

pub struct Text<const N: usize> {
    boxes: [StaticBox; N],
    content: [char; N],
}

const ALLOWED_CHARS: [char; 14] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '-', 'd', 'B',
];

impl From<char> for UVSegment {
    fn from(value: char) -> Self {
        match value {
            '0' => UV_num_0,
            '1' => UV_num_1,
            '2' => UV_num_2,
            '3' => UV_num_3,
            '4' => UV_num_4,
            '5' => UV_num_5,
            '6' => UV_num_6,
            '7' => UV_num_7,
            '8' => UV_num_8,
            '9' => UV_num_9,
            'd' => UV_num_d,
            'B' => UV_num_B,
            '-' => UV_num_minus,
            _ => UV_num_dot,
        }
    }
}

impl<const N: usize> Text<N> {
    pub fn new(
        device: &wgpu::Device,
        position: (u16, u16),
        pipeline: Arc<StaticBoxPipeline>,
    ) -> Result<Self, TextureError> {
        let boxes: [StaticBox; N] = std::array::try_from_fn(|n| {
            let pos = (position.0 + n as u16 * 7, position.1);

            StaticBox::new(device, &UV_num_0, pos, pipeline.clone())
        })?;

        Ok(Self {
            content: ['0'; N],
            boxes,
        })
    }

    pub fn change_text(&mut self, text: &str) {
        for (ch, t_ch) in self
            .content
            .iter_mut()
            .zip(text.chars().chain(std::iter::repeat(' ')))
        {
            if ALLOWED_CHARS.contains(&t_ch) {
                *ch = t_ch;
            } else {
                *ch = ' ';
            }
        }
    }
}

impl<const N: usize> UiElement for Text<N> {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn prerender(
        &mut self,
        queue: &wgpu::Queue,
        _params: Arc<crate::BitFlipperParams>,
        _buffer: &[f32],
    ) {
        for (b, ch) in self.boxes.iter_mut().zip(self.content.iter()) {
            b.swap_uv(queue, &(*ch).into()).ok();
        }
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        for (b, ch) in self.boxes.iter().zip(self.content.iter()) {
            if *ch != ' ' {
                b.render(render_pass)
            }
        }
    }
}

pub struct VolumeText {
    text: Text<8>,
}

impl VolumeText {
    pub fn new(
        device: &wgpu::Device,
        position: (u16, u16),
        pipeline: Arc<StaticBoxPipeline>,
    ) -> Result<Self, TextureError> {
        Ok(Self {
            text: Text::new(device, position, pipeline)?,
        })
    }
}

impl UiElement for VolumeText {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn prerender(
        &mut self,
        queue: &wgpu::Queue,
        params: Arc<crate::BitFlipperParams>,
        buffer: &[f32],
    ) {
        let mut text = params.pre_gain.to_string();
        if !text.starts_with("-") {
            text = " ".to_string() + &text;
        }

        self.text.change_text(&text);
        self.text.prerender(queue, params, buffer);
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.text.render(render_pass);
    }
}
