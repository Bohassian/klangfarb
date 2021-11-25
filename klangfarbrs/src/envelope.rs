use super::{Millisecond, Amplitude, SamplesPerSecond, Line};

pub struct Envelope {
    pub attack: Line,
    pub decay: Line,
    pub release: Line,
}

impl Envelope {
    pub fn new(
        attack: Millisecond, decay: Millisecond, sustain: Amplitude, release: Millisecond, sample_rate: SamplesPerSecond
    ) -> Self {
        let attack = Line::new(0.0, 1.0, attack, sample_rate);
        let decay = Line::new(1.0, sustain, decay, sample_rate);
        let release = Line::new(sustain, 0.0, release, sample_rate);

        Self { attack, decay, release }
    }
}

impl Iterator for Envelope {
    type Item = Amplitude;

    fn next(&mut self) -> Option<Self::Item> {
        let mut val = self.attack.next();
        if val.is_none() {
            val = self.decay.next();
            if val.is_none() {
                self.release.next()
            } else {
                val
            }
        } else {
            val
        }
    }
}
