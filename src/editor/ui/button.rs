use super::{StaticBox, StaticBoxPipeline, UiBox, UiElement, UiInteractive};
use crate::model::FlipModes;
use std::sync::Arc;

pub struct Button {
    static_box: StaticBox,
    state: FlipModes,
    is_on: bool,
}

impl Button {
    pub fn new(
        device: &wgpu::Device,
        flip_mode: FlipModes,
        uv_segment: &str,
        position: (u16, u16),
        pipeline: Arc<StaticBoxPipeline>,
    ) -> Option<Self> {
        if let Some(static_box) = StaticBox::new(device, uv_segment, position, pipeline) {
            return Some(Self {
                is_on: false,
                state: flip_mode,
                static_box,
            });
        }

        None
    }

    pub fn onclick(&mut self) {
        self.is_on = !self.is_on
    }
}

impl UiBox for Button {
    fn width(&self) -> u16 {
        16
    }

    fn height(&self) -> u16 {
        16
    }

    fn position(&self) -> (u16, u16) {
        self.static_box.position()
    }
}

impl UiInteractive for Button {}

impl UiElement for Button {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn prerender(&mut self, params: Arc<crate::BitFlipperParams>) {
        self.is_on = self.state == params.mode.value()
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, queue: &wgpu::Queue) {
        if self.is_on {
            self.static_box.render(render_pass, queue);
        }
    }
}

pub struct ButtonBuilder<'a> {
    d: &'a wgpu::Device,
    p: Arc<StaticBoxPipeline>,
}

impl<'a> ButtonBuilder<'a> {
    pub fn new(device: &'a wgpu::Device, pipeline: Arc<StaticBoxPipeline>) -> Self {
        Self {
            d: device,
            p: pipeline.clone(),
        }
    }

    pub fn mode(&self, mode: FlipModes) -> Button {
        match mode {
            FlipModes::Xor => Button::new(self.d, mode, "btn_xor", (46, 51), self.p.clone()),
            FlipModes::Or => Button::new(self.d, mode, "btn_or", (46, 68), self.p.clone()),
            FlipModes::And => Button::new(self.d, mode, "btn_and", (46, 85), self.p.clone()),
            FlipModes::Not => Button::new(self.d, mode, "btn_not", (46, 102), self.p.clone()),
        }
        .unwrap()
    }
}
