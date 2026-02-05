use crate::vectors::Vec3;
use std::convert::From;
use crate::interval::Interval;

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

    // Translate [0-1] component values to [0,255] range
    static intensity: Interval = Interval { min: 0.000, max: 0.999 };
    let r_byte = (256.0 * intensity.clamp(r)) as u8;
    let g_byte = (256.0 * intensity.clamp(g)) as u8;
    let b_byte = (256.0 * intensity.clamp(b)) as u8;

    println!("{} {} {}", r_byte, g_byte, b_byte);
}

