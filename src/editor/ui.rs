use super::{VIEW_HEIGHT, VIEW_WIDTH};
use std::{any::Any, sync::Arc};

mod background;
mod button;
mod color_box;
mod digit;
mod monitor;
mod postprocess;
mod slider;
mod static_box;
mod text;
mod warning;

pub(super) mod pipeline;
pub(super) mod texture;

pub use background::*;
pub use button::*;
pub use color_box::*;
pub use digit::*;
pub use monitor::*;
pub use postprocess::*;
pub use slider::*;
pub use static_box::*;
pub use text::*;
pub use warning::*;

pub trait UiElement {
    fn prerender(
        &mut self,
        _queue: &wgpu::Queue,
        _params: Arc<crate::BitFlipperParams>,
        _buffer: &[f32],
    ) {
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass);
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
}

pub trait UiBox {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn position(&self) -> (u16, u16);

    fn get_vertices(&self) -> [f32; 12] {
        let view_w = VIEW_WIDTH as f32;
        let view_h = VIEW_HEIGHT as f32;

        let (x, y) = self.position();
        let x1 = (x as f32 / view_w) * 2.0 - 1.0;
        let y1 = 1.0 - (y as f32 / view_h) * 2.0;

        let x2 = (x + self.width()) as f32 / view_w * 2.0 - 1.0;
        let y2 = 1.0 - (y + self.height()) as f32 / view_h * 2.0;

        [
            x1, y1, x2, y1, x1, y2, //
            x1, y2, x2, y1, x2, y2,
        ]
    }
}

impl UiBox for [u16; 4] {
    fn width(&self) -> u16 {
        self[2] - self[0]
    }

    fn height(&self) -> u16 {
        self[3] - self[1]
    }

    fn position(&self) -> (u16, u16) {
        (self[0], self[1])
    }
}

pub trait UiInteractive: UiElement + UiBox {
    fn is_mouse_over(&self, mouse_pos: (i16, i16)) -> bool {
        let (x, y) = self.position();
        let (mouse_x, mouse_y) = mouse_pos;
        let mouse_x = mouse_x as u16;
        let mouse_y = mouse_y as u16;

        mouse_x >= x && mouse_x < x + self.width() && mouse_y >= y && mouse_y < y + self.height()
    }
}
