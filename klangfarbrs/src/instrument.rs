use super::{ Partial, Millisecond, Sample, Hz, SamplesPerSecond };

pub struct Instrument {
    pub partials: Vec<Partial>,
}

impl Instrument {
    pub fn new(base_freq: Hz, partial_multipliers: Vec<f32>, sample_rate: SamplesPerSecond) -> Self {
        Self {
            partials: partial_multipliers.iter()
                .map(|&p| Partial::new(1.0, 1.0, p, 0.0, sample_rate, 2000, base_freq))
                .collect()
        }
    }

    pub fn bell(base_freq: Hz, duration: Millisecond, sample_rate: SamplesPerSecond) -> Self {
        let bell_partials = vec![
            (1.0, 1.0, 0.56, 0.0),
            (0.67, 0.9, 0.56, 1.0),
            (1.0, 0.65, 0.82, 0.0),
            (1.8, 0.55, 0.92, 1.7),
            (2.67, 0.325, 1.19, 0.0),
            (1.67, 0.35, 1.7, 0.0),
            (1.46, 0.25, 2.0, 0.0),
            (1.33, 0.2, 2.74, 0.0),
            (1.33, 0.15, 3.0, 0.0),
            (1.0, 0.1, 3.76, 0.0),
            (1.33, 0.075, 4.07, 0.0),
        ];

        Self {
            partials: bell_partials.iter()
                .map(|&p| Partial::new(p.0, p.1, p.2, p.3, sample_rate, duration, base_freq))
                .collect()
        }
    }

    pub fn sample(&mut self) -> Sample {
        match self.next() {
            Some(s) => { s },
            None => 0.0
        }
    }
}

impl Iterator for Instrument {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let partial_samps : Vec<Option<Sample>> = self.partials.iter_mut()
            .map(|o| o.next()).collect();

        let filtered : Vec<Sample> = partial_samps.iter().filter(|opt| opt.is_some())
            .map(|i| i.unwrap()).collect();

        if filtered.is_empty() {
            None
        } else {
            Some(filtered.iter().sum())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let sr = 44800.0;
        let mut inst = Instrument::new(220.0, vec![1.0, 2.0, 4.0], sr);

        assert_eq!(inst.next(), Some(0.0));
        assert_eq!(inst.last(), Some(0.0));
    }
}

// Partial::new(1.0, 1.0, p, 0.0, sample_rate, 2000, base_freq)
