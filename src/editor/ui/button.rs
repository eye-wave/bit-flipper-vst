use super::{StaticBox, StaticBoxPipeline, UiBox, UiElement, UiInteractive};
use std::sync::Arc;

pub struct Button {
    static_box: StaticBox,
    is_on: bool,
}

impl Button {
    pub fn new(
        device: &wgpu::Device,
        uv_segment: &str,
        position: (u16, u16),
        pipeline: Arc<StaticBoxPipeline>,
    ) -> Option<Self> {
        if let Some(static_box) = StaticBox::new(device, uv_segment, position, pipeline) {
            return Some(Self {
                is_on: false,
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

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, queue: &wgpu::Queue) {
        if self.is_on {
            self.static_box.render(render_pass, queue);
        }
    }
}
