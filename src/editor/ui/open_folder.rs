use crate::editor::texture::{TextureError, UVSegment};
use crate::editor::ui::{StaticBox, StaticBoxPipeline, UiBox, UiElement, UiInteractive};
use std::sync::Arc;

pub struct OpenFolderBtn(StaticBox);

impl OpenFolderBtn {
    pub fn new(
        device: &wgpu::Device,
        uv_segment: &UVSegment,
        position: (u16, u16),
        pipeline: Arc<StaticBoxPipeline>,
    ) -> Result<Self, TextureError> {
        Ok(Self(StaticBox::new(
            device, uv_segment, position, pipeline,
        )?))
    }
}

impl UiElement for OpenFolderBtn {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn prerender(
        &mut self,
        queue: &wgpu::Queue,
        params: Arc<crate::BitFlipperParams>,
        buffer: &[f32],
    ) {
        self.0.prerender(queue, params, buffer);
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.0.render(render_pass);
    }
}

impl UiBox for OpenFolderBtn {
    fn width(&self) -> u16 {
        self.0.width()
    }

    fn height(&self) -> u16 {
        self.0.height()
    }

    fn position(&self) -> (u16, u16) {
        self.0.position()
    }
}

impl UiInteractive for OpenFolderBtn {}
