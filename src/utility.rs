use rand::prelude::ThreadRng;
use rand::Rng;
use crate::vectors::Vec3;

// Utility functions for random number generation
#[inline]
pub fn random_double(thread_rng: &mut Option<&mut ThreadRng>) -> f64 {
    let rng = match thread_rng {
        None => { &mut rand::rng() }
        Some(value) => { value }
    };
    let value: f64 = rng.random();
    value
}
#[inline]
pub fn range_double(min: f64, max: f64, thread_rng: &mut Option<&mut ThreadRng>) -> f64 {
    let rng = match thread_rng {
        None => { &mut rand::rng() }
        Some(value) => { value }
    };
    let value: f64 = min + (max-min) * rng.random::<f64>();
    value
}

pub fn sample_square(thread_rng: &mut Option<&mut ThreadRng>) -> Vec3 {
    Vec3::new(random_double(thread_rng) - 0.5, random_double(thread_rng) - 0.5, 0.0)
}