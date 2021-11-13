//! # Rust Synthesizer for the Godot game engine
//!
//! This crate contains logic for generating samples for audio wave forms which are then
//! used to fill Godot's `AudioStreamPlayback` buffers. Scripts using this code as a dynamic
//! library will be able to request a certain number of frames (represented as a `Vector2`)
//! generated with the current synthesis parameter settings.
//!
//! Because of how the Godot bindings work, the Synth will have a default
//! sample rate at 48kHz. You'll want to set it in your script's `_ready`
//! function to match the sample rate in Godot.

use gdnative::prelude::*;
use gdnative::core_types::TypedArray;

mod phasor;
use phasor::{Phase};

mod osc;
use osc::{Osc, Waveform};

pub mod adsr;
use adsr::Envelope;

/// Aliasing some types to distinguish various audio properties.
type Sample = f32;
type SamplesPerSecond = f32;
type Hz = f32;
type Amplitude = f32;
type Millisecond = u32;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct MonoSynth {
    pub osc: Osc,
    pub sample_rate: SamplesPerSecond,
    pub frequency: Hz,
    pub apply_bend: bool,
    pub phasor_bend: Vector2,
    pub continuous: bool,
    pub duration: Millisecond,
    // ADSR amplifier
    envelope: Envelope,
    // filter
    pub cutoff: Hz,
    // pub resonance:
    // pub modulator:
    pub frequency_modulation: bool,
    pub fm_frequency: Hz,
    pub fm_depth: Amplitude,
    fm_osc: Osc,
    current_envelope_position: usize,
}

pub struct Bender {}

impl Bender {
    // for (i = 0; i < nframes; ++i) {
    //     if (in[i] < x0)
    //       out[i] = (y0/x0)*in[i];
    //     else
    //       out[i] = ((1-y0)/(1-x0)) * (in[i] - x0) + y0;
    //   }
    fn bend(phase: Phase, phasor_bend: Vector2) -> f32 {
        if phase < phasor_bend.x {
            (phasor_bend.y / phasor_bend.x) * phase
        } else {
            ((1.0 - phasor_bend.y) / (1.0 - phasor_bend.x)) * (phase - phasor_bend.x) + phasor_bend.y
        }
    }
}

#[methods]
impl MonoSynth {
    /// # Examples
    ///
    /// ```gdscript
    /// var MonoSynth = preload("res://MonoSynth.gdns")
    /// var synth = MonoSynth.new()
    /// synth.set_sample_rate(24000.0)
    /// wave.square() # changes to a square wave
    /// ```
    pub fn new(_owner: &Node) -> Self {
        let freq = 440.0;
        let sprt = 48000.0;

        Self {
            osc: Osc::new(freq, sprt),
            sample_rate: sprt,
            frequency: freq,
            apply_bend: false,
            phasor_bend: Vector2::new(0.0, 0.0),
            continuous: true,
            duration: 0,
            envelope: Envelope::new(500, 1000, 0.5, 4000, sprt),
            cutoff: 0.0,
            frequency_modulation: false,
            fm_frequency: 10.0,
            fm_depth: 0.1,
            fm_osc: Osc::new(10.0, sprt),
            current_envelope_position: 0,
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("DAS IST KLANGFARBRS.")
    }

    #[export]
    fn sine(&mut self, _owner: &Node) {
        self.osc.waveform = Waveform::Sine
    }

    #[export]
    fn square(&mut self, _owner: &Node) {
        self.osc.waveform = Waveform::Square
    }

    #[export]
    fn triangle(&mut self, _owner: &Node) {
        self.osc.waveform = Waveform::Triangle
    }

    #[export]
    fn sawtooth(&mut self, _owner: &Node) {
        self.osc.waveform = Waveform::Sawtooth
    }

    #[export]
    fn white_noise(&mut self, _owner: &Node) {
        self.osc.waveform = Waveform::WhiteNoise
    }

    #[export]
    fn brown_noise(&mut self, _owner: &Node) {
        self.osc.waveform = Waveform::BrownNoise
    }

    #[export]
    fn frequency(&mut self, _owner: &Node, frequency: Hz) {
        self.osc.set_frequency(frequency)
    }

    #[export]
    fn continuous(&mut self, _owner: &Node, state: bool) {
        self.continuous = state;
    }

    #[export]
    fn phasor_bend(&mut self, _owner: &Node, phasor_bend: Vector2) {
        self.phasor_bend = phasor_bend
    }

    #[export]
    fn apply_bend(&mut self, _owner: &Node, apply_bend: bool) {
        self.apply_bend = apply_bend
    }

    #[export]
    pub fn set_sample_rate(&mut self, _owner: &Node, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    #[export]
    fn frequency_modulation(&mut self, _owner: &Node, frequency_modulation: bool) {
        self.frequency_modulation = frequency_modulation
    }

    #[export]
    fn fm_frequency(&mut self, _owner: &Node, fm_frequency: f32) {
        self.fm_osc.set_frequency(fm_frequency)
    }

    #[export]
    fn fm_depth(&mut self, _owner: &Node, fm_depth: f32) {
        self.fm_depth = fm_depth
    }

    #[export]
    fn envelope(
        &mut self, _owner: &Node,
        attack: Millisecond, decay: Millisecond, sustain: Amplitude, release: Millisecond
    ) {
        self.envelope = Envelope::new(attack, decay, sustain, release, self.sample_rate);
    }

    #[export]
    pub fn frames(&mut self, _owner: &Node, samples: i32) -> TypedArray<Vector2> {
        let mut frames = TypedArray::new();

        for _i in 0..samples {
            // let next_phase : f32;

            if self.frequency_modulation {
                let modulation_value =  self.fm_osc.next().unwrap() * self.fm_depth;
                self.osc.set_frequency(self.osc.get_frequency() + modulation_value);
            }

            let mut sample = self.osc.next().unwrap();
            self.osc.last_value = sample;

            // if self.apply_bend {
            //     self.phasor.phase = Bender::bend(next_phase, self.phasor_bend);
            // } else {
            //     self.phasor.phase = next_phase;
            // }

            if !self.continuous {
                let pos = self.current_envelope_position;
                let atk = self.envelope.attack.len();
                let atkdcy = atk + self.envelope.decay.len();

                if pos < atk {
                    sample = sample * self.envelope.attack[pos]
                } else if pos >= atk && pos < atkdcy  {
                    sample = sample * self.envelope.decay[pos - atk]
                } else if pos < self.envelope.len() {
                    sample = sample * self.envelope.release[pos - atkdcy]
                }

                self.current_envelope_position += 1;

                if self.current_envelope_position >= self.envelope.len() {
                    self.current_envelope_position = 0;
                }
            }

            frames.push(Vector2::new(sample, sample));
        }

        return frames
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<MonoSynth>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
