use super::{ Amplitude, Millisecond, SamplesPerSecond };

pub fn slope(start: Amplitude, end: Amplitude, duration: Millisecond) -> f32 {
    return (end - start) / duration as f32 ;
}

pub fn ms_to_samples(ms: Millisecond, sample_rate: SamplesPerSecond) -> u32 {
    let multiplier = sample_rate as u32 / 1000;
    multiplier * ms
}
