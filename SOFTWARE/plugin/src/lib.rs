// Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
//! Ward-Beck M464 High Pass / Low Pass Filter (1976).
//!
//! `dsp` is the signal path on its own (used by the Ward-Beck Audio Tap). With the default feature `vst`
//! this crate is also a full VST3 / CLAP plugin whose editor is the M464 front panel (drawing 464-04-003).

pub mod dsp;
pub mod filter;

#[cfg(feature = "vst")]
mod editor;
#[cfg(feature = "vst")]
pub use plugin::*;

#[cfg(feature = "vst")]
mod plugin {
    use crate::dsp::{Settings, HF_HZ, LF_HZ, M464};
    use nih_plug::prelude::*;
    use std::sync::Arc;

    fn hz(v: f32) -> String {
        if v >= 1000.0 {
            format!("{} kHz", v / 1000.0)
        } else {
            format!("{v} Hz")
        }
    }

    #[derive(Params)]
    pub struct Params464 {
        #[persist = "editor-state"]
        pub editor_state: Arc<wbs_panel::nih_plug_egui::EguiState>,
        #[id = "hf_in"]
        pub hf_in: BoolParam,
        #[id = "hf"]
        pub hf: IntParam,
        #[id = "lf_in"]
        pub lf_in: BoolParam,
        #[id = "lf"]
        pub lf: IntParam,
    }

    impl Default for Params464 {
        fn default() -> Self {
            let d = Settings::default();
            Params464 {
                editor_state: crate::editor::default_state(),
                hf_in: BoolParam::new("HF CUT IN", d.hf_in),
                hf: IntParam::new("HF CUT", d.hf as i32, IntRange::Linear { min: 0, max: 10 })
                    .with_value_to_string(Arc::new(|i| hz(HF_HZ[i.clamp(0, 10) as usize]))),
                lf_in: BoolParam::new("LF CUT IN", d.lf_in),
                lf: IntParam::new("LF CUT", d.lf as i32, IntRange::Linear { min: 0, max: 10 })
                    .with_value_to_string(Arc::new(|i| hz(LF_HZ[i.clamp(0, 10) as usize]))),
            }
        }
    }

    impl Params464 {
        pub fn settings(&self) -> Settings {
            Settings { lf_in: self.lf_in.value(), lf: self.lf.value() as usize, hf_in: self.hf_in.value(), hf: self.hf.value() as usize }
        }

        /// The same settings from plain parameter values looked up by parameter id (bool 0 / 1, int and enum as
        /// their index): how a host outside nih-plug (the Pro Tools build) drives the DSP.
        pub fn settings_from(v: &dyn Fn(&str) -> f32) -> Settings {
            Settings { lf_in: (v("lf_in") > 0.5), lf: (v("lf").round() as i32) as usize, hf_in: (v("hf_in") > 0.5), hf: (v("hf").round() as i32) as usize }
        }
    }

    pub struct Plugin {
        params: Arc<Params464>,
        dsp: M464,
    }

    impl Default for Plugin {
        fn default() -> Self {
            Plugin { params: Arc::new(Params464::default()), dsp: M464::new(48000.0) }
        }
    }

    impl nih_plug::prelude::Plugin for Plugin {
        const NAME: &'static str = "WBS M464 Hi Pass Lo Pass Filter";
        const VENDOR: &'static str = "Ward-Beck Systems";
        const URL: &'static str = "https://ward-beck.ca";
        const EMAIL: &'static str = "info@ward-beck.ca";
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");

        const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
            AudioIOLayout { main_input_channels: NonZeroU32::new(2), main_output_channels: NonZeroU32::new(2), ..AudioIOLayout::const_default() },
            AudioIOLayout { main_input_channels: NonZeroU32::new(1), main_output_channels: NonZeroU32::new(1), ..AudioIOLayout::const_default() },
        ];
        const SAMPLE_ACCURATE_AUTOMATION: bool = false;
        type SysExMessage = ();
        type BackgroundTask = ();

        fn params(&self) -> Arc<dyn Params> {
            self.params.clone()
        }

        fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
            crate::editor::create(self.params.clone())
        }

        fn initialize(&mut self, _layout: &AudioIOLayout, config: &BufferConfig, _context: &mut impl InitContext<Self>) -> bool {
            self.dsp.set_sample_rate(config.sample_rate);
            true
        }

        fn reset(&mut self) {
            self.dsp.reset();
        }

        fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, _context: &mut impl ProcessContext<Self>) -> ProcessStatus {
            self.dsp.set(self.params.settings());
            self.dsp.process(buffer.as_slice());
            ProcessStatus::Normal
        }
    }

    impl ClapPlugin for Plugin {
        const CLAP_ID: &'static str = "ca.ward-beck.m464";
        const CLAP_DESCRIPTION: Option<&'static str> = Some("Ward-Beck M464 High Pass / Low Pass Filter (1976): 11-step LF and HF cuts, 18 dB/octave");
        const CLAP_MANUAL_URL: Option<&'static str> = Some("https://wardbecksystems.github.io/WBSps.ca/PAGES/M464.html");
        const CLAP_SUPPORT_URL: Option<&'static str> = None;
        const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo, ClapFeature::Mono, ClapFeature::Filter];
    }

    impl Vst3Plugin for Plugin {
        const VST3_CLASS_ID: [u8; 16] = *b"WBS-M464-HpLpFlt";
        const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx, Vst3SubCategory::Filter];
    }

    #[cfg(test)]
    mod settings_from_tests {
        use super::*;

        #[test]
        fn settings_from_matches_settings() {
            let p = Params464::default();
            let map = p.param_map();
            let v = |id: &str| {
                let (_, ptr, _) = map.iter().find(|(i, _, _)| i == id).expect(id);
                unsafe { ptr.unmodulated_plain_value() }
            };
            assert_eq!(format!("{:?}", Params464::settings_from(&v)), format!("{:?}", p.settings()));
        }
    }

    nih_export_clap!(Plugin);
    nih_export_vst3!(Plugin);
}
