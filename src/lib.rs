use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, EguiState, widgets, egui::{self, Button}};
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;

use std::{sync::{Arc, mpsc::channel}, collections::VecDeque, env};

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use delay::Delay;
use delay_enums::{DelayTime, DelayTiming};
use egui_editor::*;
use filter::*;
mod editor;
mod delay;
mod delay_enums;
mod egui_editor;
mod filter;

mod egui_my_widgets {
    pub mod my_slider;
}

pub struct EffectPlugin {
    params: Arc<PluginParams>,
    left_delay: Delay,
    right_delay: Delay,
    left_feedback_buffer: Box<VecDeque<f32>>,
    right_feedback_buffer: Box<VecDeque<f32>>,
    lpf: BiquadFilter,
    sample_rate: f32,
    bpm: f32,
}

#[derive(Params)]
pub struct PluginParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "feedback"]
    feedback: FloatParam,

    #[id = "delay-time"]
    delay_time: EnumParam<DelayTime>,

    #[id = "delay-timing"]
    delay_timing: EnumParam<DelayTiming>,

    #[id = "cutoff"]
    cutoff: FloatParam,

    #[id = "resonance"]
    resonance: FloatParam,

    #[id = "filter_type"]
    filter_type: EnumParam<FilterType>,

    #[id = "dry"]
    dry: FloatParam,

    #[id = "wet"]
    wet: FloatParam,
}

impl Default for EffectPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
            left_delay: Delay::new(),
            right_delay: Delay::new(),
            left_feedback_buffer: Box::new(VecDeque::new()),
            right_feedback_buffer: Box::new(VecDeque::new()),
            sample_rate: 44100.0,
            bpm: 120.0,
            lpf: BiquadFilter::new(),
        }
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            feedback: FloatParam::new("Feedback", 0.625, FloatRange::Linear { min: 0.0, max: 1.0 })
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            delay_time: EnumParam::new("Delay Time", DelayTime::_1_4),

            delay_timing: EnumParam::new("Delay Timing", DelayTiming::Straight),

            cutoff: FloatParam::new("Cutoff", 15000.0, FloatRange::Skewed { min: 20.0, max: 20000.0, factor: 0.2 })
            .with_unit("")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),

            resonance: FloatParam::new("Resonance", 0.707, FloatRange::Linear { min: 0.5, max: 3.0 })
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            filter_type: EnumParam::new("Filter Type", FilterType::LowPass2),

            dry: FloatParam::new("Dry", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 })
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            wet: FloatParam::new("Wet", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 })
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

impl Plugin for EffectPlugin {
    const NAME: &'static str = "Maeror's Delay";
    const VENDOR: &'static str = "Maeror";
    const URL: &'static str = "";
    const EMAIL: &'static str = "none";
    const VERSION: &'static str = "0.0.1";

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        self.sample_rate = _buffer_config.sample_rate;

        self.left_delay.resize_buffers(self.sample_rate, 120.0);
        self.right_delay.resize_buffers(self.sample_rate, 120.0);
        self.lpf.set_sample_rate(self.sample_rate);
        self.lpf.second_order_lpf_coefficients(self.sample_rate, 20000.0, 0.707);
        //self.left_feedback_buffer = Box::new(VecDeque::new());
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // In current configuration this function iterates as follows:
        // 1. outer loop iterates block-size times
        // 2. inner loop iterates channel-size times. 
        let bpm = _context.transport().tempo.unwrap();
        if self.bpm != bpm as f32 {
            self.left_feedback_buffer = Box::new(VecDeque::with_capacity(((DelayTime::get_max_in_beats() * 1.5 * 60.0 * self.sample_rate) / bpm as f32) as usize));
            self.right_feedback_buffer = Box::new(VecDeque::with_capacity(((DelayTime::get_max_in_beats() * 1.5 * 60.0 * self.sample_rate) / bpm as f32) as usize));
            self.bpm = bpm as f32;
            for i in 0..self.left_feedback_buffer.capacity() {
                self.left_feedback_buffer.push_back(0.0);
                self.right_feedback_buffer.push_back(0.0);
            }
        }

        for (i, channel_samples) in buffer.iter_samples().enumerate() {
            // Smoothing is optionally built into the parameters themselves

            let feedback = self.params.feedback.smoothed.next();
            let delay_time = self.params.delay_time.value();
            let delay_timing = self.params.delay_timing.value();
            let cutoff = self.params.cutoff.smoothed.next();
            let resonance = self.params.resonance.smoothed.next();
            let dry = self.params.dry.smoothed.next();
            let wet = self.params.wet.smoothed.next();

            self.left_delay.set_delay(delay_time, delay_timing, self.sample_rate, self.bpm);
            self.right_delay.set_delay(delay_time, delay_timing, self.sample_rate, self.bpm);
            self.lpf.second_order_lpf_coefficients(self.sample_rate, cutoff, resonance);


            for (num, sample) in channel_samples.into_iter().enumerate() {
                // processing
                if num == 0 {
                    let x = self.lpf.process_left(*sample + feedback * self.left_feedback_buffer.get(0).unwrap());
                    let delayed_signal = self.left_delay.process(x);
                    *sample = dry * *sample + wet * delayed_signal;
                    self.left_feedback_buffer.rotate_right(1);
                    self.left_feedback_buffer[0] = delayed_signal;
                } else {
                    let x = self.lpf.process_right(*sample + feedback * self.right_feedback_buffer.get(0).unwrap());
                    let delayed_signal = self.right_delay.process(x);
                    *sample = dry * *sample + wet * delayed_signal;
                    self.right_feedback_buffer.rotate_right(1);
                    self.right_feedback_buffer[0] = delayed_signal;
                }
            }
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }
}

impl ClapPlugin for EffectPlugin {
    const CLAP_ID: &'static str = "{{ cookiecutter.clap_id }}";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("{{ cookiecutter.description }}");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for EffectPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"maeror-delay-vst";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Delay];
}


nih_export_vst3!(EffectPlugin);
