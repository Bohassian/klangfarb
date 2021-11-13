use super::{SamplesPerSecond, Hz};

type Phase = f32; // Phase will always be between 0.0 and 1.0.

pub struct PhasorIter {
    pub phase: Phase,
    pub frequency: Hz,
    sample_rate: SamplesPerSecond,
}

impl PhasorIter {
    fn new(frequency: Hz, sample_rate: SamplesPerSecond) -> Self {
        Self { phase: 0.0, frequency, sample_rate }
    }
}

impl Iterator for PhasorIter {
    type Item = Phase;

    fn next(&mut self) -> Option<Self::Item> {
        self.phase += (self.frequency / self.sample_rate);

        if self.phase > 1.0 { self.phase -= 1.0; }

        Some(self.phase)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_produces_expected_next_value() {
        let mut phasor = PhasorIter::new(10.0, 100.0);
        assert_eq!(phasor.next(), Some(0.1))
    }

    #[test]
    fn it_produces_expected_next_values() {
        let phasor = PhasorIter::new(10.0, 100.0);
        let next  = phasor.take(11);
        assert_eq!(0.1, next.last().unwrap())
        // assert_eq!([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.1], next)
    }
}
