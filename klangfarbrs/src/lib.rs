// use gdnative::api::Resource;
use gdnative::prelude::*;
use gdnative::core_types::TypedArray;
use std::f32;
use std::f32::consts::TAU;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct SineWave {}

#[methods]
impl SineWave {
    fn new(_owner: &Node) -> Self {
        SineWave {}
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Whatever, connected.")
    }

    #[export]
    pub fn frames(&self, _owner: &Node, frequency: f32, sample_rate: f32, duration: i32) -> TypedArray<f32> {
        let mut frames = TypedArray::new();
        let calculated_duration = sample_rate * duration as f32;

        for i in 0..calculated_duration as i32 {
            frames.push((TAU * frequency * i as f32/sample_rate).sin());
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
