use std::fs::File;
use std::io::Write;
use crate::color::{format_color, Color};


pub fn create_ppm(image_buffer: &Vec<Color>, width: u32, height: u32, path: &str) -> std::io::Result<()> {
    let mut new_ppm = File::create(format!("{}.ppm", path))?;
    let mut file_str = format!("P3\n{} {}\n255\n", width, height);

    for c in image_buffer {
        file_str.push_str(&format_color(c));
    }

    new_ppm.write_all(file_str.as_bytes())?;

    Ok(())
}