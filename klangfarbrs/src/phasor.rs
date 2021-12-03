use super::{SamplesPerSecond, Hz};

pub type Phase = f32; // Phase will always be between 0.0 and 1.0.

/// Phase stays between 0.0 and 1.0 and represents position on the axis of time
/// for a given wave form. Since audio signals are periodic, we can just calculate
/// the first cycle of a wave repeatedly. This also prevents pitch drift caused by
/// floating point errors over time.
pub struct Phasor {
    pub phase: Phase,
    pub frequency: Hz,
    pub sample_rate: SamplesPerSecond,
}

impl Phasor {
    pub fn new(frequency: Hz, sample_rate: SamplesPerSecond) -> Self {
        Self { phase: 0.0, frequency, sample_rate }
    }
}

impl Iterator for Phasor {
    type Item = Phase;

    fn next(&mut self) -> Option<Self::Item> {
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;

        Some(self.phase)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_produces_expected_next_value() {
        let mut phasor = Phasor::new(10.0, 100.0);
        assert_eq!(phasor.next(), Some(0.1))
    }

    #[test]
    fn it_wraps_around_as_expected() {
        let phasor = Phasor::new(10.0, 100.0);
        let last_val = phasor.take(11).last().unwrap();
        assert_eq!(last_val.floor(), 0.0)
    }
}


// TODO: fixup bender/modulator to allow phase modulation again
// pub struct Bender {}

// impl Bender {
//     // for (i = 0; i < nframes; ++i) {
//     //     if (in[i] < x0)
//     //       out[i] = (y0/x0)*in[i];
//     //     else
//     //       out[i] = ((1-y0)/(1-x0)) * (in[i] - x0) + y0;
//     //   }
//     fn bend(phase: Phase, phasor_bend: Vector2) -> f32 {
//         if phase < phasor_bend.x {
//             (phasor_bend.y / phasor_bend.x) * phase
//         } else {
//             ((1.0 - phasor_bend.y) / (1.0 - phasor_bend.x)) * (phase - phasor_bend.x) + phasor_bend.y
//         }
//     }
// }

