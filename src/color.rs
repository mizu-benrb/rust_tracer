use crate::vectors::Vec3;
use std::convert::From;

pub type Color = Vec3; // Change in future to Vec4 for alpha

pub struct RGBValue {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<RGBValue> for Color {
    fn from(rgb: RGBValue) -> Color {
        Color {
            e: [rgb.r as f64 / 255.0, rgb.g as f64 / 255.0, rgb.b as f64 / 255.0],
        }
    }
}

pub enum BlendMode {
    None,
    Add,
    Multiply,
    Subtract,
    AlphaBlend,
}

impl Color {
    pub fn color_lerp(&self, target_color: Color, mut t: f64) -> Color {
        if t < 0.0 {
            t = 0.0;
        } else if t > 1.0 {
            t = 1.0;
        }

        let new_r = self.x() + t * (target_color.x() - self.x());
        let new_g = self.y() + t * (target_color.y() - self.y());
        let new_b = self.z() + t * (target_color.z() - self.z());
        
        Color { e: [new_r, new_g, new_b] }
    }
}

pub fn write_color(pixel_color: &Color) {
    let r = if pixel_color.x() < 0.0 { 0.0 } else if pixel_color.x() > 1.0 { 1.0 } else { pixel_color.x() };
    let g = if pixel_color.y() < 0.0 { 0.0 } else if pixel_color.y() > 1.0 { 1.0 } else { pixel_color.y() };
    let b = if pixel_color.z() < 0.0 { 0.0 } else if pixel_color.z() > 1.0 { 1.0 } else { pixel_color.z() };

    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}

