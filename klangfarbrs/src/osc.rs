use std::f32::consts::TAU;
use crate::{Waveform, Phase, Sample};

pub struct Osc {}

impl Osc {
    pub fn generate_sample(waveform: &Waveform, phase: Phase) -> Sample {
        let phase = phase;

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
            }
        }
    }
}

