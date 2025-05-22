mod background;

pub mod pipeline;
pub use background::*;

pub trait UiElement {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}
