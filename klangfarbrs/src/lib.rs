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
mod line;
use line::Line;

mod osc;
use osc::{Osc, Waveform};

pub mod envelope;
use envelope::Envelope;

mod instrument;
use instrument::Instrument;

mod utils;

/// Aliasing some types to distinguish various audio properties.
type Sample = f32;
type SamplesPerSecond = f32;
type Hz = f32;
type Amplitude = f32;
type Millisecond = u32;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct MonoSynth {
    pub osc: Instrument,
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
    pub attack: Millisecond,
    pub decay: Millisecond,
    pub sustain: Amplitude,
    pub release: Millisecond,
    fm_osc: Osc,
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
            envelope: Envelope::new(30, 500, 0.5, 1000, sprt),
            cutoff: 0.0,
            frequency_modulation: false,
            fm_frequency: 10.0,
            fm_depth: 0.1,
            fm_osc: Osc::new(freq * 2.0, sprt),
            attack: 10,
            decay: 100,
            sustain: 0.5,
            release: 500,
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
        self.frequency = frequency;
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
    fn fm_frequency(&mut self, _owner: &Node, fm_frequency: Hz) {
        self.fm_osc.set_frequency(fm_frequency)
    }

    #[export]
    fn fm_depth(&mut self, _owner: &Node, fm_depth: f32) {
        self.fm_depth = fm_depth
    }

    #[export]
    fn set_attack(&mut self, _owner: &Node, attack: Millisecond) {
        self.attack = attack
    }

    #[export]
    fn set_decay(&mut self, _owner: &Node, decay: Millisecond) {
        self.decay = decay
    }

    #[export]
    fn set_sustain(&mut self, _owner: &Node, sustain: Amplitude) {
        self.sustain = sustain
    }

    #[export]
    fn set_release(&mut self, _owner: &Node, release: Millisecond) {
        self.release = release
    }

    #[export]
    fn envelope(
        &mut self, _owner: &Node,
        attack: Millisecond, decay: Millisecond, sustain: Amplitude, release: Millisecond
    ) {
        self.envelope = Envelope::new(attack, decay, sustain, release, self.sample_rate);
    }

    #[export]
    fn trigger(&mut self, _owner: &Node,
    ) {
        self.envelope = Envelope::new(self.attack, self.decay, self.sustain, self.release, self.sample_rate);
    }

    #[export]
    pub fn frames(&mut self, _owner: &Node, samples: i32) -> TypedArray<Vector2> {
        let mut frames = TypedArray::new();

        for _i in 0..samples {
            // let next_phase : f32;

            if self.frequency_modulation {
                let modulation_value =  self.fm_osc.sample() * self.fm_depth;
                self.osc.set_frequency(self.osc.get_frequency() + modulation_value);
            }

            let mut sample = self.osc.sample();
            self.osc.last_value = sample;

            // TODO:
            // if self.apply_bend {
            //     self.phasor.phase = Bender::bend(next_phase, self.phasor_bend);
            // } else {
            //     self.phasor.phase = next_phase;
            // }

            if !self.continuous {
                sample *= match self.envelope.next() {
                    Some(a) => a,
                    None => 0.0,
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
