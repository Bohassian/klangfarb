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
    fn it_produces_expected_next_values() {
        let phasor = Phasor::new(10.0, 100.0);
        let taken_iterator  = phasor.take(11);
        assert_eq!(taken_iterator.last(), Some(0.1))
        // assert_eq!([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.1], next)
    }
}
