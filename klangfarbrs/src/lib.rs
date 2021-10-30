// use gdnative::api::Resource;
use gdnative::prelude::*;
use gdnative::core_types::TypedArray;
use std::f32::consts::TAU;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct SineWave {
    sample_rate: f32,
    phase: f32
}

#[methods]
impl SineWave {
    fn new(_owner: &Node) -> Self {
        SineWave { sample_rate: 44100.0, phase: 0.0 }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Whatever, connected.")
    }

    #[export]
    fn set_sample_rate(&mut self, _owner: &Node, sample_rate: f32) {
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
