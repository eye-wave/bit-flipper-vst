use std::sync::Arc;

use crate::editor::texture::{TextureError, UVSegment::*};
use crate::editor::ui::{
    ColorBox, ColorBoxPipeline, StaticBox, StaticBoxPipeline, UiBox, UiElement, UiInteractive,
};
use crate::editor::{VIEW_HEIGHT, VIEW_WIDTH};

pub struct Warning {
    position: (u16, u16),
    tex_box: StaticBox,
    color_box: ColorBox,
}

impl Warning {
    pub fn new(
        device: &wgpu::Device,
        position: (u16, u16),
        b_pipeline: Arc<StaticBoxPipeline>,
        c_pipeline: Arc<ColorBoxPipeline>,
    ) -> Result<Self, TextureError> {
        Ok(Self {
            position,
            tex_box: StaticBox::new(device, &UV_warning, position, b_pipeline)?,
            color_box: ColorBox::new(
                device,
                (0, 0),
                VIEW_WIDTH,
                VIEW_HEIGHT,
                [0.0, 0.0, 0.0, 0.8],
                c_pipeline,
            ),
        })
    }
}

impl UiElement for Warning {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.color_box.render(render_pass);
        self.tex_box.render(render_pass);
    }
}

impl UiBox for Warning {
    fn width(&self) -> u16 {
        9
    }

    fn height(&self) -> u16 {
        9
    }

    fn position(&self) -> (u16, u16) {
        let (x, y) = self.position;

        (x + 104, y + 75)
    }
}

impl UiInteractive for Warning {}
