use nih_plug::prelude::*;
use nih_plug_vizia::{vizia::prelude::*, widgets::param_base::ParamWidgetBase};

use crate::FlipModes;

#[derive(Lens)]
pub struct Mode {
    param_base: ParamWidgetBase,
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
        Self {
            param_base: ParamWidgetBase::new(cx, params, params_to_param),
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, "&");
                Label::new(cx, "|");
                Label::new(cx, "!");
                Label::new(cx, "^");
            });
        })
    }

    fn switch_mode(&self, mode: FlipModes, cx: &mut EventContext) {
        let param = EnumParam::new("enum-helper", mode);
        let value = param.unmodulated_normalized_value();

        self.param_base.begin_set_parameter(cx);
        self.param_base.set_normalized_value(cx, value);
        self.param_base.end_set_parameter(cx);
    }
}

impl View for Mode {
    fn element(&self) -> Option<&'static str> {
        Some("mdoe-button")
    }
}
