use super::{ Osc, Envelope, Sample };

pub struct Instrument {
    pub osc_bank: Vec<Osc>,
    pub envelope: Envelope,
}

impl Instrument {
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
        let goo : f32 = self.osc_bank.iter_mut().map(|o| o.sample()).sum();
        let scaled = goo / self.osc_bank.len() as f32;

        match self.envelope.next() {
            Some(a) => Some(scaled * a),
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let sr = 44800.0;
        let mut inst = Instrument {
            osc_bank: vec![Osc::new(220.0, sr), Osc::new(440.0, sr), Osc::new(880.0, sr)],
            envelope: Envelope::new(10, 200, 0.7, 1000, sr),
        };

        assert_eq!(inst.next(), Some(0.0));
    }
}
