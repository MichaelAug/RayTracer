use rand::{thread_rng, Rng};
use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// random number in range [0, 1)
pub fn random_f64() -> f64 {
    thread_rng().gen()
}

// random number in range [min, max]
pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..=max)
}
