use rand::prelude::ThreadRng;
use rand::rngs::Xoshiro256PlusPlus;
use rand::{Rng, RngExt};
use crate::vectors::Vec3;

// Utility functions for random number generation
#[inline]
pub fn random_double(rng: &mut Xoshiro256PlusPlus) -> f64 {
    let value: f64 = rng.random();
    value
}
#[inline]
pub fn random_double_unseeded() -> f64 {
    let value: f64 = rand::rng().random();
    value
}
#[inline]
pub fn range_double(min: f64, max: f64, rng: &mut Xoshiro256PlusPlus) -> f64 {
    let value: f64 = min + (max-min) * rng.random::<f64>();
    value
}
#[inline]
pub fn range_double_unseeded(min: f64, max: f64) -> f64 {
    let value: f64 = min + (max-min) * rand::rng().random::<f64>();
    value
}

#[inline]
pub fn range_int(min: i32, max: i32, rng: &mut Xoshiro256PlusPlus) -> i32 {
    rng.random_range(min..max)
}
#[inline]
pub fn range_int_unseeded(min: i32, max: i32) -> i32 {
    rand::rng().random_range(min..max)
}

#[inline]
pub fn sample_square(rng: &mut Xoshiro256PlusPlus) -> Vec3 {
    Vec3::new(random_double(rng) - 0.5, random_double(rng) - 0.5, 0.0)
}