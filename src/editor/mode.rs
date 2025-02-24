use nih_plug::prelude::*;
use nih_plug_vizia::{vizia::prelude::*, widgets::param_base::ParamWidgetBase};

use crate::FlipModes;

#[derive(Lens)]
pub struct Mode {
    param_base: ParamWidgetBase,
}

pub enum ModeEvents {
    Set(FlipModes),
}

impl Mode {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        let param_base = ParamWidgetBase::new(cx, params, params_to_param);

        Self { param_base }.build(
            cx,
            ParamWidgetBase::build_view(params, params_to_param, move |cx, _| {
                Binding::new(
                    cx,
                    ParamWidgetBase::make_lens(params, params_to_param, |p| {
                        p.modulated_normalized_value()
                    }),
                    move |cx, value| {
                        let helper_param = EnumParam::new("helper", FlipModes::Xor);

                        HStack::new(cx, |cx| {
                            [
                                (FlipModes::And, "&"),
                                (FlipModes::Not, "!"),
                                (FlipModes::Or, "|"),
                                (FlipModes::Xor, "^"),
                            ]
                            .map(|(mode, label)| {
                                let this = helper_param.preview_normalized(mode);

                                let is_selected = this == value.get(cx);

                                Label::new(cx, label)
                                    .font_size(20.0)
                                    .font_weight(if is_selected {
                                        FontWeightKeyword::Black
                                    } else {
                                        FontWeightKeyword::Thin
                                    })
                                    .width(Pixels(32.0))
                                    .height(Pixels(36.0))
                                    .text_align(TextAlign::Center)
                                    .border_width(Pixels(1.0))
                                    .border_color(Color::white())
                                    .border_radius(Pixels(8.0))
                                    .on_mouse_up(move |cx, _| cx.emit(ModeEvents::Set(mode)));
                            });
                        })
                        .child_left(Stretch(1.0))
                        .child_right(Stretch(1.0))
                        .height(Pixels(36.0))
                        .child_top(Pixels(20.0));
                    },
                );
            }),
        )
    }
}

impl View for Mode {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|state_change, _| match state_change {
            ModeEvents::Set(variant) => {
                let helper_param = EnumParam::new("helper", FlipModes::Xor);
                let value = helper_param.preview_normalized(*variant);

                self.param_base.begin_set_parameter(cx);
                self.param_base.set_normalized_value(cx, value);
                self.param_base.end_set_parameter(cx);
            }
        })
    }

    fn element(&self) -> Option<&'static str> {
        Some("mdoe-button")
    }
}
