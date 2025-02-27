use cyma::prelude::*;
use digit::Digit;
use mode::Mode;
use monitor::{Monitor, MonitorParams};
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::{ParamButton, ParamSlider};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::model::FlipModes;
use crate::BitFlipperParams;

mod digit;
pub mod mode;
mod monitor;

const BLUE: &str = "#51e2e0";
const GREEN: &str = "#51e273";
const RED: &str = "#e2517a";

pub enum StateUI {
    Update(Option<u8>, Option<FlipModes>),
}

#[derive(Lens)]
struct Data {
    params: Arc<BitFlipperParams>,
    monitor_params: MonitorParams,
}

impl Model for Data {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, meta| match event {
            StateUI::Update(bit, mode) => {
                let mut bits = self.params.bits.to_u32();
                let mode = mode.unwrap_or_else(|| self.params.mode.value());

                bit.map(|bit| bits ^= 1 << bit);

                self.monitor_params = MonitorParams::new(bits, mode);
                meta.consume();
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (544, 480))
}

pub(crate) fn create(
    bus: Arc<MonoBus>,
    params: Arc<BitFlipperParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        cx.add_font_mem(include_bytes!("../assets/font/joystix monospace.otf"));
        cx.add_stylesheet("*{font-family:Joystix}").unwrap();

        bus.subscribe(cx);

        let bits_state = params.bits.to_u32();
        let mode_state = params.mode.value();

        Data {
            params: params.clone(),
            monitor_params: MonitorParams::new(bits_state, mode_state),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Bit flipper")
                .width(Stretch(1.0))
                .text_align(TextAlign::Center)
                .font_size(30.0)
                .child_top(Pixels(16.0));

            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Binding::new(cx, Data::monitor_params, |cx, monitor| {
                        let bits = monitor.get(cx).bits;

                        Label::new(cx, &format!("u32: {}", bits)).font_size(12.0);
                        Label::new(cx, &format!("f32: {}", f32::from_bits(bits))).font_size(10.0);
                    });
                })
                .height(Pixels(48.0))
                .color(Color::white());

                HStack::new(cx, |cx| {
                    ParamButton::new(cx, Data::params, |p| &p.remove_dc_offset);
                    VStack::new(cx, |cx| {
                        Label::new(cx, "pre Gain").color(Color::white());
                        ParamSlider::new(cx, Data::params, |p| &p.pre_gain).color(Color::white());
                    });
                })
                .col_between(Stretch(1.0))
                .child_space(Stretch(1.0))
                .width(Stretch(1.0))
                .left(Pixels(16.0))
                .right(Pixels(16.0))
                .height(Pixels(96.0));

                HStack::new(cx, |cx| {
                    Digit::new(cx, 31, BLUE.into(), Data::params, |p| &p.bits.mask_bit_32);

                    Digit::new(cx, 30, GREEN.into(), Data::params, |p| &p.bits.mask_bit_31);
                    Digit::new(cx, 29, GREEN.into(), Data::params, |p| &p.bits.mask_bit_30);
                    Digit::new(cx, 28, GREEN.into(), Data::params, |p| &p.bits.mask_bit_29);
                    Digit::new(cx, 27, GREEN.into(), Data::params, |p| &p.bits.mask_bit_28);
                    Digit::new(cx, 26, GREEN.into(), Data::params, |p| &p.bits.mask_bit_27);
                    Digit::new(cx, 25, GREEN.into(), Data::params, |p| &p.bits.mask_bit_26);
                    Digit::new(cx, 24, GREEN.into(), Data::params, |p| &p.bits.mask_bit_25);
                    Digit::new(cx, 23, GREEN.into(), Data::params, |p| &p.bits.mask_bit_24);

                    Digit::new(cx, 22, RED.into(), Data::params, |p| &p.bits.mask_bit_23);
                    Digit::new(cx, 21, RED.into(), Data::params, |p| &p.bits.mask_bit_22);
                    Digit::new(cx, 20, RED.into(), Data::params, |p| &p.bits.mask_bit_21);
                    Digit::new(cx, 19, RED.into(), Data::params, |p| &p.bits.mask_bit_20);
                    Digit::new(cx, 18, RED.into(), Data::params, |p| &p.bits.mask_bit_19);
                    Digit::new(cx, 17, RED.into(), Data::params, |p| &p.bits.mask_bit_18);
                    Digit::new(cx, 16, RED.into(), Data::params, |p| &p.bits.mask_bit_17);
                    Digit::new(cx, 15, RED.into(), Data::params, |p| &p.bits.mask_bit_16);
                    Digit::new(cx, 14, RED.into(), Data::params, |p| &p.bits.mask_bit_15);
                    Digit::new(cx, 13, RED.into(), Data::params, |p| &p.bits.mask_bit_14);
                    Digit::new(cx, 12, RED.into(), Data::params, |p| &p.bits.mask_bit_13);
                    Digit::new(cx, 11, RED.into(), Data::params, |p| &p.bits.mask_bit_12);
                    Digit::new(cx, 10, RED.into(), Data::params, |p| &p.bits.mask_bit_11);
                    Digit::new(cx, 9, RED.into(), Data::params, |p| &p.bits.mask_bit_10);
                    Digit::new(cx, 8, RED.into(), Data::params, |p| &p.bits.mask_bit_9);
                    Digit::new(cx, 7, RED.into(), Data::params, |p| &p.bits.mask_bit_8);
                    Digit::new(cx, 6, RED.into(), Data::params, |p| &p.bits.mask_bit_7);
                    Digit::new(cx, 5, RED.into(), Data::params, |p| &p.bits.mask_bit_6);
                    Digit::new(cx, 4, RED.into(), Data::params, |p| &p.bits.mask_bit_5);
                    Digit::new(cx, 3, RED.into(), Data::params, |p| &p.bits.mask_bit_4);
                    Digit::new(cx, 2, RED.into(), Data::params, |p| &p.bits.mask_bit_3);
                    Digit::new(cx, 1, RED.into(), Data::params, |p| &p.bits.mask_bit_2);
                    Digit::new(cx, 0, RED.into(), Data::params, |p| &p.bits.mask_bit_1);
                })
                .left(Pixels(16.0))
                .right(Pixels(16.0))
                .height(Pixels(16.0))
                .min_width(Pixels(512.0));

                HStack::new(cx, |cx| {
                    Label::new(cx, "Sign")
                        .width(Pixels(80.0))
                        .text_align(TextAlign::Center)
                        .background_color(BLUE);

                    Label::new(cx, "Exponent")
                        .width(Pixels(128.0))
                        .text_align(TextAlign::Center)
                        .background_color(GREEN);

                    Label::new(cx, "Fraction")
                        .width(Stretch(1.0))
                        .text_align(TextAlign::Center)
                        .background_color(RED);
                })
                .left(Pixels(16.0))
                .right(Pixels(16.0))
                .height(Pixels(24.0))
                .min_width(Pixels(512.0));
            })
            .color(Color::black())
            .height(Pixels(80.0))
            .width(Pixels(512.0));

            VStack::new(cx, |cx| {
                Mode::new(cx, Data::params, |p| &p.mode);

                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Monitor::new(cx, Data::monitor_params);
                    })
                    .width(Pixels(128.0))
                    .height(Pixels(128.0))
                    .background_color(Color::from("#202020"));

                    Oscilloscope::new(cx, bus.clone(), 0.128, (-1.0, 1.0), ValueScaling::Linear)
                        .color(Color::white())
                        .width(Pixels(128.0))
                        .height(Pixels(128.0));
                });
            })
            .height(Pixels(192.0))
            .top(Pixels(96.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));
        })
        .row_between(Pixels(0.0))
        .background_color(Color::from("#222"))
        .color(Color::from("#eee"));
    })
}
