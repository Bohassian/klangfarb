// use gdnative::api::Resource;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Synth {
    frequency: f32
}

#[methods]
impl Synth {
    fn new(_owner: &Node) -> Self {
        Synth { frequency: 220.0 }
    }

    #[export]
    fn set_freq(
        &mut self,
        _owner: &Node,
        freq: f32
    ) {
        self.frequency = freq;
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        match test::test() {
            Ok(()) => godot_print!("POOOP"),
            Err(error) => godot_print!("SHITE! {:?}", error),
        };
    }
}

pub mod test {
    use cpal;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use dasp::{Sample};
    use dasp_signal::{self as signal, Signal};
    use std::sync::mpsc;

    pub fn test() -> Result<(), anyhow::Error> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("failed to find a default output device");
        let config = device.default_output_config()?;

        match config.sample_format() {
            cpal::SampleFormat::F32 => run::<f32>(&device, &config.into())?,
            cpal::SampleFormat::I16 => run::<i16>(&device, &config.into())?,
            cpal::SampleFormat::U16 => run::<u16>(&device, &config.into())?,
        }

        Ok(())
    }

    fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
    where
        T: cpal::Sample,
    {
        // Create a signal chain to play back 1 second of each oscillator at A4.
        let base_freq = 440.0;
        let multipliers = [0.8, 1.0, 1.2, 1.7, 2.9, 4.5, 8.8, 1.9, 3.6];
        let freqs = multipliers.map(|m| signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * m));
        let _sines = freqs.map(|f| f.clone().sine());

        // Shitty naive implementation until Rust clicks.
        let hz_0 = signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * 0.8);
        let hz_1 = signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * 1.0);
        let hz_2 = signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * 1.2);
        let hz_3 = signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * 1.7);
        let hz_4 = signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * 2.9);
        let hz_5 = signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * 4.5);
        let hz_6 = signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * 8.8);
        let hz_7 = signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * 1.9);
        let hz_8 = signal::rate(config.sample_rate.0 as f64).const_hz(base_freq * 3.6);

        let sine_0 = hz_0.clone().sine().scale_amp(0.03);
        let sine_1 = hz_1.clone().sine();
        let sine_2 = hz_2.clone().sine().scale_amp(0.8);
        let sine_3 = hz_3.clone().sine().scale_amp(0.004);
        let sine_4 = hz_4.clone().sine().scale_amp(0.03);
        let sine_5 = hz_5.clone().sine().scale_amp(0.02);
        let sine_6 = hz_6.clone().sine().scale_amp(0.001);
        let sine_7 = hz_7.clone().sine().scale_amp(0.006);
        let sine_8 = hz_8.clone().sine().scale_amp(0.05);


        // let hz = signal::rate(config.sample_rate.0 as f64).const_hz(440.0);
        let one_sec = config.sample_rate.0 as usize;

        // let mut sin_a = hz
        //     .clone()
        //     .sine();

        // let mut sin_b = hz_b
        //     .clone()
        //     .sine();

        let mut synth = sine_0
            .add_amp(sine_1)
            .add_amp(sine_2)
            .add_amp(sine_3)
            .add_amp(sine_4)
            .add_amp(sine_5)
            .add_amp(sine_6)
            .add_amp(sine_7)
            .add_amp(sine_8)
            .take(one_sec)
            .map(|s| s.to_sample::<f32>() * 0.2);

        // let mut synth = sines[0]
        //     .add_amp(sines[1])
        //     .add_amp(sines[2])
        //     .add_amp(sines[3])
        //     .add_amp(sines[4])
        //     .add_amp(sines[5])
        //     .add_amp(sines[6])
        //     .add_amp(sines[7])
        //     .add_amp(sines[8])
        //     .take(one_sec)
        //     .map(|s| s.to_sample::<f32>() * 0.2);
        // let mut synth = sin_a.add_amp(sin_b).take(one_sec).map(|s| s.to_sample::<f32>() * 0.2);
        // let mut synth = hz
        //     .clone()
        //     .sine()
        //     .take(one_sec)
        //     .chain(hz.clone().saw().take(one_sec))
        //     .chain(hz.clone().square().take(one_sec))
        //     .chain(hz.clone().noise_simplex().take(one_sec))
        //     .chain(signal::noise(0).take(one_sec))
        //     .map(|s| s.to_sample::<f32>() * 0.2);

        // A channel for indicating when playback has completed.
        let (complete_tx, complete_rx) = mpsc::sync_channel(1);

        // Create and run the stream.
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let channels = config.channels as usize;
        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &complete_tx, &mut synth)
            },
            err_fn,
        )?;
        stream.play()?;

        // Wait for playback to complete.
        complete_rx.recv().unwrap();
        stream.pause()?;

        Ok(())
    }

    fn write_data<T>(
        output: &mut [T],
        channels: usize,
        complete_tx: &mpsc::SyncSender<()>,
        signal: &mut dyn Iterator<Item = f32>,
    ) where
        T: cpal::Sample,
    {
        for frame in output.chunks_mut(channels) {
            let sample = match signal.next() {
                None => {
                    complete_tx.try_send(()).ok();
                    0.0
                }
                Some(sample) => sample,
            };
            let value: T = cpal::Sample::from::<f32>(&sample);
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }

}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the `Synth` type we declared.
    handle.add_class::<Synth>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
