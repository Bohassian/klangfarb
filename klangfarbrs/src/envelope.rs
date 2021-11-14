use super::{Millisecond, Amplitude, SamplesPerSecond};

pub struct Envelope {
    pub attack: Vec<Amplitude>,
    pub decay: Vec<Amplitude>,
    pub release: Vec<Amplitude>,
    pub index: usize,
}

impl Envelope {
    pub fn new(
        attack: Millisecond, decay: Millisecond, sustain: Amplitude, release: Millisecond, sample_rate: SamplesPerSecond
    ) -> Self {
        let attack = interpolate(0.0, 1.0, ms_to_samples(attack, sample_rate));
        let decay = interpolate(1.0, sustain, ms_to_samples(decay, sample_rate));
        let release = interpolate(sustain, 0.0, ms_to_samples(release, sample_rate));

        Self { attack, decay, release, index: 0 }
    }

    pub fn len(&self) -> usize {
        self.attack.len() + self.decay.len() + self.release.len()
    }
}

impl Iterator for Envelope {
    type Item = Amplitude;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.index;
        let atk = self.attack.len();
        let atkdcy = atk + self.decay.len();

        self.index += 1;

        if idx < self.len() {
            let val = if idx < atk {
                self.attack[idx]
            } else if idx >= atk && idx < atkdcy  {
                self.decay[idx - atk]
            } else {
                self.release[idx - atkdcy]
            };

            Some(val)
        } else {
            None
        }
    }
}

fn interpolate(start: Amplitude, end: Amplitude, milliseconds: Millisecond) -> Vec<Amplitude> {
    let step_size = (end - start) / milliseconds as f32;
    let mut amps = vec!();
    let mut current_val = start + step_size;

    for i in 0..=milliseconds {
        if i == 0 {
            amps.push(start)
        } else if i == milliseconds {
            amps.push(end)
        } else {
            amps.push(current_val);
            current_val += step_size;
        }
    }

    amps
}

fn ms_to_samples(ms: Millisecond, sample_rate: SamplesPerSecond) -> u32 {
    let multiplier = sample_rate as u32 / 1000;
    multiplier * ms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_expected_total_length() {
        let expected = 3 + ms_to_samples(30, 48000.0) as usize;
        let total = Envelope::new(10, 10, 1.0, 10, 48000.0).len();
        assert_eq! (expected, total)
    }

    #[test]
    fn interpolate_works() {
        let expected = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        let result = interpolate(0.0, 1.0, 5);
        assert_eq!(expected, result)
    }
}
