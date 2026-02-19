use rand::prelude::ThreadRng;
use rand::rngs::Xoshiro256PlusPlus;
use rand::{RngExt, SeedableRng};
use crate::vectors::Vec3;

// Utility functions for random number generation
#[inline]
pub fn random_double(rng: &mut Option<Xoshiro256PlusPlus>) -> f64 {
    let rng = match rng {
        None => { &mut Xoshiro256PlusPlus::from_rng(&mut rand::rng()) }
        Some(value) => { value }
    };
    let value: f64 = rng.random();
    value
}
#[inline]
pub fn range_double(min: f64, max: f64, rng: &mut Option<Xoshiro256PlusPlus>) -> f64 {
    let rng = match rng {
        None => { &mut Xoshiro256PlusPlus::from_rng(&mut rand::rng()) }
        Some(value) => { value }
    };
    let value: f64 = min + (max-min) * rng.random::<f64>();
    value
}

#[inline]
pub fn sample_square(rng: &mut Option<Xoshiro256PlusPlus>) -> Vec3 {
    Vec3::new(random_double(rng) - 0.5, random_double(rng) - 0.5, 0.0)
}