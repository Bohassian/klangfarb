use super::{Millisecond, Amplitude, SamplesPerSecond};
use super::utils::*;

pub struct Line {
    index: u32,
    samples: u32,
    slope: f32,
    y_intercept: f32,
}

impl Line {
    pub fn new(
        start: Amplitude, end: Amplitude, duration: Millisecond, sample_rate: SamplesPerSecond
    ) -> Self {
        let number_of_samples = ms_to_samples(duration, sample_rate);

        Self {
            index: 0,
            slope: slope(start, end, number_of_samples),
            samples: number_of_samples,
            y_intercept: start,
        }
    }
}

impl Iterator for Line {
    type Item = Amplitude;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.index;
        let val = self.slope * idx as f32 + self.y_intercept;
        self.index += 1;

        if idx <= self.samples {
            Some(val)
        } else {
            None
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_a_slope() {
      let expected = 0.5;
      let slope = slope(0.0, 0.5, 1);
      assert_eq! (slope, expected)
    }

    #[test]
    fn it_calculates_the_next_values() {
      let mut line = Line::new(0.0, 0.5, 1, 5000.0);
      assert_eq!(line.next(), Some(0.0));
      assert_eq!(line.next(), Some(0.1));
      assert_eq!(line.next(), Some(0.2));
      assert_eq!(line.next(), Some(0.3));
      assert_eq!(line.next(), Some(0.4));
      assert_eq!(line.next(), Some(0.5));
      assert_eq!(line.next(), None);
    }
}
