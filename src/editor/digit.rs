use nih_plug::params::Param;
use nih_plug_vizia::{vizia::prelude::*, widgets::param_base::ParamWidgetBase};

use super::StateUI;

#[derive(Lens)]
pub struct Digit {
    param_base: ParamWidgetBase,
    bit: u8,
}

impl Digit {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        bit: u8,
        color: String,
        params: L,
        params_to_param: FMap,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        Self {
            param_base: ParamWidgetBase::new(cx, params, params_to_param),
            bit,
        }
        .build(
            cx,
            ParamWidgetBase::build_view(params, params_to_param, move |cx, _| {
                Binding::new(
                    cx,
                    ParamWidgetBase::make_lens(params, params_to_param, |p| {
                        p.modulated_normalized_value()
                    }),
                    move |cx, value| {
                        Label::new(cx, value)
                            .text_align(TextAlign::Center)
                            .color(Color::black())
                            .font_size(16.0)
                            .width(Pixels(16.0))
                            .background_color(Color::from(color.as_str()));
                    },
                )
            }),
        )
    }

    /// Set the parameter's normalized value to either 0.0 or 1.0 depending on its current value.
    fn toggle_value(&self, cx: &mut EventContext) {
        let current_value = self.param_base.unmodulated_normalized_value();
        let new_value = if current_value >= 0.5 { 0.0 } else { 1.0 };

        self.param_base.begin_set_parameter(cx);
        self.param_base.set_normalized_value(cx, new_value);
        self.param_base.end_set_parameter(cx);
    }
}

impl View for Digit {
    fn element(&self) -> Option<&'static str> {
        Some("digit-button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left)
            | WindowEvent::MouseDoubleClick(MouseButton::Left)
            | WindowEvent::MouseTripleClick(MouseButton::Left) => {
                self.toggle_value(cx);

                cx.emit(StateUI::Update(Some(self.bit), None));
            }
            _ => {}
        })
    }
}
