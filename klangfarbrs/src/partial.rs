use super::{ Amplitude, Hz, Line, Millisecond, Osc, Sample, SamplesPerSecond };

pub struct Partial {
    amplitude: Amplitude,
    relative_duration: f32,
    relative_frequency: f32,
    detune: Hz,
    osc: Osc,
    attack: Line,
    decay: Line,
}

impl Partial {
    pub fn new(
        amplitude: Amplitude,
        relative_duration: f32,
        relative_frequency: f32,
        detune: Hz,
        sample_rate: SamplesPerSecond,
        base_duration: Millisecond,
        base_frequency: Hz,
    ) -> Self {
        let freq = base_frequency * relative_frequency + detune;
        let decay_duration = base_duration as f32 * relative_duration;

        Self {
            amplitude, relative_duration, relative_frequency, detune,
            osc: Osc::new(freq, sample_rate),
            attack: Line::new(0.0, amplitude, 5, sample_rate),
            decay: Line::new(amplitude, 0.0, decay_duration as u32, sample_rate),
        }
    }
}

impl Iterator for Partial {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let amplitude = match self.attack.next() {
            Some(amp) => Some(amp),
            None => self.decay.next()
        };

        match amplitude {
            Some(amp) => Some(self.osc.next().unwrap() * amp),
            None => None
        }
    }
}
