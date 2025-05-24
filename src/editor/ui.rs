mod background;
mod postprocess;

pub(super) mod pipeline;
pub(super) mod texture;

pub use background::*;
pub use postprocess::*;

pub trait UiElement {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, queue: &wgpu::Queue);
    // fn as_any_mut(&mut self) -> &mut dyn Any;
}

// pub trait UiBox {
//     fn width(&self) -> u16;
//     fn height(&self) -> u16;
//     fn position(&self) -> (u16, u16);

//     fn is_mouse_over(&self, mouse_pos: (u16, u16)) -> bool {
//         let (x, y) = self.position();
//         let w = self.width();
//         let h = self.height();

//         let (mx, my) = mouse_pos;

//         if mx <= x + w && mx >= x && my <= y + h && my >= y {
//             return true;
//         }

//         false
//     }

//     fn on_click(&mut self) {}
// }
