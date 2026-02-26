use std::sync::Arc;
use image::{open, RgbImage};
use crate::color::Color;
use crate::interval::Interval;
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

pub struct Image_Texture {
    image: RgbImage,
}

impl Image_Texture {
    pub fn new(image: RgbImage) -> Image_Texture { Image_Texture { image } }

    pub fn new_load(filename: &str) -> Image_Texture {
        let image_result = open("./images/".to_owned() + filename);
        let image = match image_result {
            Ok(i) => i.into_rgb8(),
            Err(e) => panic!("{}, {filename} is not a valid file location", e),
        };

        Image_Texture { image }
    }
}

impl Texture for Image_Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        // If there is no texture data, return magenta
        if self.image.height() <= 0 { return Color::new(1.0, 0.0, 1.0); }

        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as usize - 1;
        let j = (v * self.image.height() as f64) as usize - 1;

        let pixel = self.image.get_pixel(i as u32, j as u32);

        let color_scale = 1.0 / 255.0;
        Color::new(color_scale * pixel[0] as f64, color_scale * pixel[1] as f64, color_scale * pixel[2] as f64)
    }
}