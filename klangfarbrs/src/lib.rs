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

#[derive(NativeClass)]
#[inherit(Node)]
pub struct SineWave {
    pub sample_rate: f32,
    pub phase: f32
}

/// # Examples
///
/// ```
/// use klangfarbrs::SineWave;
/// let mut wave = SineWave { sample_rate: 24000.0, phase: 0.0 };
/// assert_eq!(wave.sample_rate, 24000.0);
/// ```

#[methods]
impl SineWave {
    pub fn new(_owner: &Node) -> Self {
        Self { sample_rate: 48000.0, phase: 0.0 }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("DAS IST KLANGFARBRS.")
    }

    #[export]
    pub fn set_sample_rate(&mut self, _owner: &Node, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    #[export]
    pub fn frames(&mut self, _owner: &Node, frequency: f32, duration: i32) -> TypedArray<Vector2> {
        let mut frames = TypedArray::new();

        for _i in 0..duration {
            let sample = (TAU * self.phase).sin().clamp(-1.0, 1.0);
            frames.push(Vector2::new(sample, sample));
            self.phase = (self.phase + (frequency / self.sample_rate)) % 1.0;
        }

        return frames
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the `Sine` type we declared.
    handle.add_class::<SineWave>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
