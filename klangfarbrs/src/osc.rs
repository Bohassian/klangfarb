use std::f32::consts::TAU;
use rand::Rng;
use crate::{Waveform, Phase, Sample};

pub struct Osc {}

impl Osc {
    pub fn generate_sample(waveform: &Waveform, phase: Phase, last_value: Sample) -> Sample {
        let phase = phase;
        let mut rng = rand::thread_rng();

        match waveform {
            Waveform::Sine => {
                (TAU * phase).sin()
            },

            Waveform::Square => {
                if phase < 0.5 {
                    -1.0
                } else {
                    1.0
                }
            },

            Waveform::Triangle => {
                if phase < 0.5 {
                    4.0 * phase - 1.0
                } else {
                    4.0 * (1.0 - phase) - 1.0
                }
            },

            Waveform::Sawtooth => {
                2.0 * phase - 1.0
            },

            Waveform::WhiteNoise => {
                (rng.gen::<f32>()).sin()
            },

            Waveform::BrownNoise => {
                (last_value + (rng.gen::<f32>()) * 0.2 - 0.1).clamp(-1.0, 1.0)
            },
        }
    }
}

