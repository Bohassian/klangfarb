use crate::{Millisecond, Amplitude};

pub struct Envelope {
    attack: Vec<Amplitude>,
    decay: Vec<Amplitude>,
    release: Vec<Amplitude>,
}

impl Envelope {
    fn new(attack: Millisecond, decay: Millisecond, sustain: Amplitude, release: Millisecond) -> Self {
        let attack = vec![sustain; attack as usize];
        let decay = vec![sustain; decay as usize];
        let release = vec![sustain; release as usize];

        Self{attack, decay, release}
    }

    pub fn len(&self) -> usize {
        self.attack.len() + self.decay.len() + self.release.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_expected_total_length() {
        let total = Envelope::new(10, 10, 1.0, 10).len();
        assert_eq! (30, total)
    }
}
