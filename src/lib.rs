use cyma::prelude::*;
use dc_block::DCBlocker;
use model::{BitParams, FlipModes};
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

mod dc_block;
mod editor;
pub mod model;

/// This is mostly identical to the gain example, minus some fluff, and with a GUI.
pub struct BitFlipper {
    params: Arc<BitFlipperParams>,
    dc_blocker: DCBlocker,
    bus: Arc<MonoBus>,
}

#[derive(Params)]
struct BitFlipperParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[nested(group = "bits")]
    pub bits: BitParams,

    #[id = "mode"]
    pub mode: EnumParam<FlipModes>,

    #[id = "remove_dc_offset"]
    pub remove_dc_offset: BoolParam,

    #[id = "pre_gain"]
    pub pre_gain: FloatParam,
}

impl Default for BitFlipper {
    fn default() -> Self {
        Self {
            bus: Arc::new(MonoBus::default()),
            params: Arc::new(BitFlipperParams::default()),
            dc_blocker: DCBlocker::new(0.995),
        }
    }
}

impl Default for BitFlipperParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            mode: EnumParam::new("mode", FlipModes::default()),
            bits: BitParams::default(),
            remove_dc_offset: BoolParam::new("Remove DC", true),
            pre_gain: FloatParam::new(
                "pre_gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_unit("dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2)),
        }
    }
}

impl Plugin for BitFlipper {
    const NAME: &'static str = "Bit-Flipper";
    const VENDOR: &'static str = "Software by _eyewave";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.bus.clone(),
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.bus.set_sample_rate(buffer_config.sample_rate);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let mask = self.params.bits.to_u32();
            let mode = self.params.mode.value();
            let gain = self.params.pre_gain.smoothed.next();
            let remove_dc = self.params.remove_dc_offset.value();

            for sample in channel_samples {
                *sample *= gain;
                *sample = mode.transform(*sample, mask);

                if remove_dc {
                    *sample = self.dc_blocker.process(*sample)
                }
            }
        }

        if self.params.editor_state.is_open() {
            self.bus.send_buffer_summing(buffer);
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for BitFlipper {
    const CLAP_ID: &'static str = "software-by-eyewave-bit-flipper.vst";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Bit manipulation distortion plugin.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for BitFlipper {
    const VST3_CLASS_ID: [u8; 16] = *b"bit-flipper-lmao";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(BitFlipper);
nih_export_vst3!(BitFlipper);
