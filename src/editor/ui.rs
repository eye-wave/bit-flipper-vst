use std::any::Any;
use std::sync::Arc;

mod background;
mod button;

pub(super) mod pipeline;
pub(super) mod texture;

pub use background::*;
pub use button::*;

pub trait UiElement {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, queue: &wgpu::Queue);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub(super) fn create_vertices(pos: (u16, u16), dim: (u16, u16), viewport: (u16, u16)) -> [f32; 24] {
    let x = pos.0 as f32;
    let y = pos.1 as f32;
    let w = dim.0 as f32;
    let h = dim.1 as f32;
    let screen_w = viewport.0 as f32;
    let screen_h = viewport.1 as f32;

    let left = (x / screen_w) * 2.0 - 1.0;
    let right = ((x + w) / screen_w) * 2.0 - 1.0;
    let top = 1.0 - (y / screen_h) * 2.0;
    let bottom = 1.0 - ((y + h) / screen_h) * 2.0;

    #[rustfmt::skip]
    let vertices: [f32; 24] = [
        left, top, 0.0, 1.0,
        right, top, 1.0, 1.0,
        left, bottom, 0.0, 0.0,

        right, top, 1.0, 1.0,
        right, bottom, 1.0, 0.0,
        left, bottom, 0.0, 0.0
    ];

    vertices
}

pub trait UiBox {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn position(&self) -> (u16, u16);

    fn is_mouse_over(&self, mouse_pos: (u16, u16)) -> bool {
        let (x, y) = self.position();
        let w = self.width();
        let h = self.height();

        let (mx, my) = mouse_pos;

        if mx <= x + w && mx >= x && my <= y + h && my >= y {
            return true;
        }

        false
    }

    fn on_click(&mut self) {}
}

pub fn create_editor_buttons(
    btn_pipeline: Arc<ButtonPipeline>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Vec<Button> {
    let base_pos = (46, 51);
    let buffers: &[&[u8]] = &[
        include_bytes!("../../assets/textures/__btn_xor__.png"),
        include_bytes!("../../assets/textures/__btn_or__.png"),
        include_bytes!("../../assets/textures/__btn_and__.png"),
        include_bytes!("../../assets/textures/__btn_not__.png"),
    ];

    buffers
        .iter()
        .enumerate()
        .map(|(i, buf)| {
            let img = image::load_from_memory(buf).unwrap().to_rgba8();
            let (x, mut y) = base_pos;

            y += i as u16 * 17;

            Button::new(btn_pipeline.clone(), device, queue, img, (x, y))
        })
        .collect()
}
