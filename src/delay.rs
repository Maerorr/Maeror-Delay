use nih_plug::prelude::Enum;

use crate::delay_enums::{DelayTime, DelayTiming};

pub struct Delay {
    buffer: Vec<f32>,
    max_delay_samples: usize,
    current_sample: usize,
    delay_samples: usize,
    delay_time: DelayTime,
    delay_timing: DelayTiming,
    sample_rate: f32,
    bpm: f32,
}

impl Delay {
    pub fn new() -> Self {
        Delay {
            buffer: vec![0.0; 10],
            max_delay_samples: 10,
            current_sample: 0,
            delay_samples: 0,
            delay_time: DelayTime::_1_4,
            delay_timing: DelayTiming::Straight,
            sample_rate: 44100.0,
            bpm: 120.0,
        }
    }

    pub fn process(&mut self, x: f32) -> f32 {
        let read_index: usize;
        if (self.current_sample as i32 - self.delay_samples as i32) < 0 {
            read_index = self.max_delay_samples - (self.delay_samples - self.current_sample);
        } else {
            read_index = self.current_sample - self.delay_samples;
        }
        let delayed_sample = self.buffer[read_index];

        // Store the input sample in the buffer
        self.buffer[self.current_sample] = x;


        // Update the current sample index in a circular manner
        self.current_sample = (self.current_sample + 1) % self.max_delay_samples;

        delayed_sample
    }

    pub fn resize_buffers(&mut self, sample_rate: f32, bpm: f32) {
        // the longest possible delay
        let new_len = ((DelayTime::get_max_in_beats() * 1.5 * 60.0 * sample_rate) / bpm as f32) as usize;
        self.max_delay_samples = new_len;
        self.buffer.resize(new_len, 0.0)
    }


    pub fn set_delay(&mut self, delay_time: DelayTime, delay_timing: DelayTiming, sample_rate: f32, bpm: f32) {
        // dynamically resize the buffers if needed
        if sample_rate != self.sample_rate || bpm != self.bpm {
            self.resize_buffers(sample_rate, bpm);
            self.sample_rate = sample_rate;
            self.bpm = bpm;
        }
        self.delay_time = delay_time;
        self.delay_timing = delay_timing;
        //Delay (in samples) = (Delay Time (in beats) * 60 * Sample Rate) / BPM
        // ((delay_time.get_delay_time() * 60.0 * sample_rate) / bpm as f32) as usize;
        self.delay_samples = match delay_timing {
            DelayTiming::Dotted => ((delay_time.get_delay_time_in_beats() * 1.5 * 60.0 * sample_rate) / bpm as f32) as usize,
            DelayTiming::Triplet => ((delay_time.get_delay_time_in_beats() * 0.75 * 60.0 * sample_rate) / bpm as f32) as usize,
            DelayTiming::Straight => ((delay_time.get_delay_time_in_beats() * 60.0 * sample_rate) / bpm as f32) as usize,
        }
    }

    pub fn get_delay_samples(&self) -> usize {
        self.delay_samples
    }
}