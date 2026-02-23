use std::sync::Arc;
use crate::color::Color;
use crate::vectors::Point3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color { Color::default() }
}

pub struct Solid_Color {
    albedo: Color,
}

impl Solid_Color {
    pub fn new(albedo: &Color) -> Solid_Color { Solid_Color { albedo: *albedo } }

    pub fn new_color(red: f64, green: f64, blue: f64) -> Solid_Color { Solid_Color { albedo: Color::new(red, green, blue) } }
}

impl Texture for Solid_Color {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.albedo
    }
}

pub struct Checker_Texture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checker_Texture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Checker_Texture {
        Checker_Texture { inv_scale: 1.0 / scale, even, odd }
    }

    pub fn new_solids(scale: f64, c1: &Color, c2: &Color) -> Checker_Texture {
        Checker_Texture { inv_scale: 1.0 / scale, even: Arc::new(Solid_Color::new(c1)), odd: Arc::new(Solid_Color::new(c2))}
    }
}

impl Texture for Checker_Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let xInteger = (self.inv_scale * p.x()).floor() as i32;
        let yInteger = (self.inv_scale * p.y()).floor() as i32;
        let zInteger = (self.inv_scale * p.z()).floor() as i32;

        let isEven = (xInteger + yInteger + zInteger) % 2 == 0;

        return if isEven { self.even.value(u, v, p) } else { self.odd.value(u, v, p) }
    }
}