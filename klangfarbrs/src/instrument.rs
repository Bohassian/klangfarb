use super::{ Partial, Sample, Hz, SamplesPerSecond };

pub struct Instrument {
    pub partials: Vec<Partial>,
    pub complete: bool,
}

impl Instrument {
    pub fn new(base_freq: Hz, partial_multipliers: Vec<f32>, sample_rate: SamplesPerSecond) -> Self {
        Self {
            partials: partial_multipliers.iter()
                .map(|&p| Partial::new(1.0, 1.0, p, 0.0, sample_rate, 2000, base_freq))
                .collect(),
            complete: false,
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
            self.complete = true;
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
