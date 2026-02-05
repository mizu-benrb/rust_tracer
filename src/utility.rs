use rand::Rng;
use crate::vectors::Vec3;

// Utility functions for random number generation
#[inline]
pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    let value: f64 = rng.random();
    value
}
#[inline]
pub fn range_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    let value: f64 = min + (max-min) * rng.random::<f64>();
    value
}

pub fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
}