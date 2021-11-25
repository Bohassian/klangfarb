use super::{Millisecond, Amplitude, SamplesPerSecond};

pub struct Line {
    pub start: Amplitude,
    pub end: Amplitude,
    pub duration: Millisecond,
    pub index: u32,
    slope: f32,
    samples: u32
}

impl Line {
    pub fn new(
        start: Amplitude, end: Amplitude, duration: Millisecond, sample_rate: SamplesPerSecond
    ) -> Self {
        Self { start, end, duration, index: 0, slope: slope(start, end, ms_to_samples(duration, sample_rate)), samples: ms_to_samples(duration, sample_rate) }
    }
}

impl Iterator for Line {
    type Item = Amplitude;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.index;
        let val = self.slope * idx as f32 + self.start;
        self.index += 1;
 
        if idx <= self.samples {
            Some(val)
        } else {
            None
        }
          
    }
}

fn slope(start: Amplitude, end: Amplitude, duration: Millisecond) -> f32 {
  return (end - start) / duration as f32 ;
}

fn ms_to_samples(ms: Millisecond, sample_rate: SamplesPerSecond) -> u32 {
    let multiplier = sample_rate as u32 / 1000;
    multiplier * ms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_a_slope() {
      let expected = 0.5;
      let slope = slope(0.0, 0.5, 1);
      assert_eq! (expected, slope)
    }

    #[test]
    fn it_calculates_the_next_values() {
      let mut line = Line::new(0.0, 0.5, 1, 5000.0);
      assert_eq!(0.0, line.next().unwrap());
      assert_eq!(0.1, line.next().unwrap());
      assert_eq!(0.2, line.next().unwrap());
      assert_eq!(0.3, line.next().unwrap());
      assert_eq!(0.4, line.next().unwrap());
      assert_eq!(0.5, line.next().unwrap());
      assert_eq!(None, line.next());
    }
}
