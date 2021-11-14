use std::f32::consts::TAU;
use rand::Rng;
use super::{Hz, Sample};
use super::phasor::Phasor;

/// The various waveforms the `MonoSynth` can generate.
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    WhiteNoise,
    BrownNoise,
}

pub struct Osc {
    pub phasor: Phasor,
    pub waveform: Waveform,
    pub last_value: Sample,
}

impl Osc {
    pub fn new(frequency: Hz, sample_rate: f32) -> Self {
        Self {
            phasor: Phasor::new(frequency, sample_rate),
            waveform: Waveform::Sine,
            last_value: 0.1,
        }
    }

    pub fn get_frequency(&self) -> Hz {
        self.phasor.frequency
    }

    pub fn set_frequency(&mut self, frequency: Hz) {
        self.phasor.frequency = frequency;
    }

    pub fn sample(&mut self) -> Sample {
        match self.next() {
            Some(s) => { s },
            None => 0.0
        }
    }
}

impl Iterator for Osc {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.phasor.next().unwrap();
        let mut rng = rand::thread_rng();

        let sample = match self.waveform {
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
                (self.last_value + (rng.gen::<f32>()) * 0.2 - 0.1).clamp(-1.0, 1.0)
            },
        };

        Some(sample)
    }
}

