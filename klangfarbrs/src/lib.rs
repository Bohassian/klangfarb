//! # Rust audio oscillator for the Godot game engine
//!
//! This crate contains logic for generating samples for audio wave forms which are then
//! used to fill Godot's `AudioStreamPlayback` buffers. Scripts using this code as a dynamic
//! library will be able to request a certain number of frames (represented as a `Vector2`)
//! at a specific frequency. Because of how the Godot bindings work, the wave structs will
//! have a default sample rate at 48kHz. You'll want to set it in your script's `_ready`
//! function to match the sample rate in Godot.

use gdnative::prelude::*;
use gdnative::core_types::TypedArray;
use std::f32::consts::TAU;

/// This struct is used as a class in Godot. It is a "numerically controlled oscillator"
/// which is driven by a phasor. The sample rate and waveform should be set after you
/// create a new instance in GDScript.
#[derive(NativeClass)]
#[inherit(Node)]
pub struct Osc {
    pub waveform: Waveform,
    pub sample_rate: f32,
    phase: f32,
}

/// The various waveforms the `Osc` can generate.
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    // Noise,
}

/// Generates the next sample for an oscillator based on its waveform.
fn generate_sample(osc: &Osc) -> f32 {
    let phase = osc.phase;

    match osc.waveform {
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

/// Phase stays between 0.0 and 1.0 and represents position on the axis of time
/// for a given wave form. Since audio signals are periodic, we can just calculate
/// the first cycle of a wave repeatedly. This also prevents pitch drift caused by
/// floating point errors over time.
fn next_phase(osc: &Osc, frequency: f32) -> f32 {
    (osc.phase + (frequency / osc.sample_rate)) % 1.0
}

// for (i = 0; i < nframes; ++i) {
//     if (in[i] < x0)
//       out[i] = (y0/x0)*in[i];
//     else
//       out[i] = ((1-y0)/(1-x0)) * (in[i] - x0) + y0;
//   }
fn bend_phase(osc: &Osc, frequency: f32, bend_factor: f32) -> f32 {
    let current_phase = osc.phase;
    let next_phase = next_phase(osc, frequency);
    let step = next_phase - current_phase;

    if osc.phase < bend_factor {
        step * 2.0
    } else {
        step / 2.0
    }
}

/// # Examples
///
/// It is more work than benefit to figure out how to instantiate a Godot object (Node)
/// that does not behave as typical Rust. However, I wanted to try out the feature of
/// examples in the documentation that run as automated tests. :galaxy-brain:
///
/// ```
/// use klangfarbrs::Osc;
/// let mut wave = Osc { sample_rate: 24000.0, phase: 0.0 };
/// assert_eq!(wave.sample_rate, 24000.0);
/// ```
#[methods]
impl Osc {
    /// # Examples
    ///
    /// ```gdscript
    /// var Osc = preload("res://Osc.gdns")
    /// var wave = Osc.new()
    /// wave.set_sample_rate(24000.0)
    /// wave.square() # changes to a square wave
    /// ```
    pub fn new(_owner: &Node) -> Self {
        Self { waveform: Waveform::Sine, sample_rate: 48000.0, phase: 0.0 }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("DAS IST KLANGFARBRS.")
    }

    #[export]
    fn sine(&mut self, _owner: &Node) {
        self.waveform = Waveform::Sine
    }

    #[export]
    fn square(&mut self, _owner: &Node) {
        self.waveform = Waveform::Square
    }

    #[export]
    fn triangle(&mut self, _owner: &Node) {
        self.waveform = Waveform::Triangle
    }

    #[export]
    fn sawtooth(&mut self, _owner: &Node) {
        self.waveform = Waveform::Sawtooth
    }

    #[export]
    pub fn set_sample_rate(&mut self, _owner: &Node, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    #[export]
    pub fn frames(&mut self, _owner: &Node, frequency: f32, duration: i32, bend_factor: f32) -> TypedArray<Vector2> {
        let mut frames = TypedArray::new();

        for _i in 0..duration {
            let sample = generate_sample(&self);
            frames.push(Vector2::new(sample, sample));
            // self.phase = next_phase(&self, frequency);
            self.phase = self.phase + bend_phase(&self, frequency, bend_factor)
        }

        return frames
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the `Osc` type we declared.
    handle.add_class::<Osc>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
