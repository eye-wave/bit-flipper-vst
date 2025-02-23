use nih_plug_vizia::vizia::{
    prelude::*,
    vg::{self, Color},
};

pub struct Monitor<L: Lens<Target = [f32; 256]>> {
    buffer: L,
}

impl<L> Monitor<L>
where
    L: Lens<Target = [f32; 256]>,
{
    pub fn new(cx: &mut Context, buffer: L) {
        Self { buffer }.build(cx, |_cx| {});
    }
}

impl<L> View for Monitor<L>
where
    L: Lens<Target = [f32; 256]>,
{
    fn element(&self) -> Option<&'static str> {
        Some("sine")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let mut path = vg::Path::new();
        let buffer = self.buffer.get(cx);

        for (i, sample) in buffer.iter().enumerate() {
            let x = (i as f32 / 256.0) * bounds.w;
            let y = (sample / 2.0 + 0.5) * bounds.h;

            if i == 0 {
                path.move_to(bounds.x + x, bounds.y + y);
            } else {
                path.line_to(bounds.x + x, bounds.y + y);
            }
        }

        let paint = vg::Paint::color(Color::white());

        canvas.stroke_path(&path, &paint);
    }
}
