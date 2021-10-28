// use gdnative::api::Resource;
use gdnative::prelude::*;
use gdnative::core_types::TypedArray;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct MonoBuffer {
    #[property]
    frames: TypedArray<f32>
}

pub fn fill_frames() -> TypedArray<f32> {
    let frequency = 440.0;
    let sample_rate = 44100.0;
    let duration = sample_rate * 3.0;
    let mut frames = TypedArray::new();

    for i in 0..duration as i32 {
        frames.push(f32::sin(std::f32::consts::TAU * frequency * i as f32/sample_rate));
    }

    return frames
}

#[methods]
impl MonoBuffer {
    fn new(_owner: &Node) -> Self {
        MonoBuffer { frames: fill_frames() }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Whatever, connected.")
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the `MonoBuffer` type we declared.
    handle.add_class::<MonoBuffer>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
