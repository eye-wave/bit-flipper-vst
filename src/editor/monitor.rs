use nih_plug_vizia::vizia::{
    prelude::*,
    vg::{self, Color},
};
use std::f32::consts::TAU;

use crate::FlipModes;

#[derive(Debug, Clone, Default, Data)]
pub struct MonitorParams {
    pub bits: u32,
    pub mode: FlipModes,
}

impl MonitorParams {
    pub fn new(bits: u32, mode: FlipModes) -> Self {
        Self { bits, mode }
    }
}

pub struct Monitor<L: Lens<Target = MonitorParams>> {
    params: L,
}

impl<L> Monitor<L>
where
    L: Lens<Target = MonitorParams>,
{
    pub fn new(cx: &mut Context, params: L) -> Handle<Self> {
        Self { params }.build(cx, |_cx| {})
    }

    fn sine_wave() -> [f32; 2048] {
        let mut wave = [0.0; 2048];
        for (i, sample) in wave.iter_mut().enumerate() {
            *sample = (i as f32 / 2048.0 * TAU).sin();
        }
        wave
    }
}

impl<L> View for Monitor<L>
where
    L: Lens<Target = MonitorParams>,
{
    fn element(&self) -> Option<&'static str> {
        Some("sine")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let mask = self.params.get(cx).bits;
        let mode = self.params.get(cx).mode;

        let mut display_buffer = Self::sine_wave();
        for s in display_buffer.iter_mut() {
            *s = mode.transform(*s, mask);
        }

        let mut path = vg::Path::new();

        for (i, sample) in display_buffer.iter().enumerate() {
            let x = (i as f32 / 2048.0) * bounds.w;
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
