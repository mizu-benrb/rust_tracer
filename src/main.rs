pub mod vectors;
pub mod color;
pub mod ray;
pub mod raster;
pub mod hittable;
mod polyhedrons;
mod interval;
mod camera;
mod utility;
mod material;

use std::sync::Arc;
use std::f64::consts::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::camera::Camera;
use crate::vectors::*;
use crate::color::*;
use crate::ray::*;
use crate::raster::*;
use crate::polyhedrons::*;
use crate::hittable::*;
use crate::interval::Interval;
use crate::material::{Dielectric, Lambertian, Metal};

const PHI: f64 = 1.618033988749894;

fn main() {
    // HW 2
    render_ray_image();
}

fn render_ray_image() {

    let mut world: HittableList = HittableList::new_empty();
    let r = (PI / 4.0).cos();

    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Arc::new(
        Plane::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -0.5, 0.0), material_ground)));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(
        Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(
        Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Arc::new(
        Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let mut cam: Camera = Camera::new( 16.0 / 9.0, 400, 32, 64, 20.0);

    cam.look_from = Point3::new(-2.0, 2.0, 1.0);
    cam.look_at = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}

fn render_creative_image() {
    let canvas = Canvas {
        width: 512,
        height: 512,
        default_color: Color::from(RGBValue { r: 0, g: 10, b: 60 }),
    };
    let mut frame_buffer: Vec<Color> = vec![Color::new(0.0, 0.0, 0.0); canvas.width as usize * canvas.height as usize];

    let pivot = Pos2::new(255, 255);
    let initial_size = 10;
    let main_color = Color::from(RGBValue { r: 255, g: 0, b: 0 });

    for i in (1..21).rev() {
        let quad = Quad {
            vertices: [
                Pos2::new(-initial_size * i, -initial_size * i),
                Pos2::new(initial_size * i, -initial_size * i),
                Pos2::new(initial_size * i, initial_size * i),
                Pos2::new(-initial_size * i, initial_size * i)
            ],
            color: main_color.color_lerp(canvas.default_color, i as f64 / 20.0)
        };
        let rotated_quad = quad.rotate(i as f64 * PHI * 10.0, Pos2::new(0, 0));
        let translated_quad = Quad {
            vertices: [
                Pos2::new(rotated_quad.vertices[0].x() + pivot.x(), rotated_quad.vertices[0].y() + pivot.y()),
                Pos2::new(rotated_quad.vertices[1].x() + pivot.x(), rotated_quad.vertices[1].y() + pivot.y()),
                Pos2::new(rotated_quad.vertices[2].x() + pivot.x(), rotated_quad.vertices[2].y() + pivot.y()),
                Pos2::new(rotated_quad.vertices[3].x() + pivot.x(), rotated_quad.vertices[3].y() + pivot.y()),
            ],
            color: rotated_quad.color,
        };
        translated_quad.draw_fill(&canvas, &mut frame_buffer);
    }

    output_ppm(&canvas, &frame_buffer);
}

fn render_test_image() {
    // Image settings
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    // Progress bar settings
    let progress_bar = ProgressBar::new(image_height as u64);
    progress_bar.set_style(ProgressStyle::with_template("{prefix} {bar:80.cyan/blue} {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("##-"));
    progress_bar.set_prefix("Scanlines completed...");

    for j in 0..image_height as u32 {
        progress_bar.inc(1);

        for i in 0..image_width as u32 {
            let pixel_color = Color::new(
                (i as f64) / (image_width - 1) as f64,
                (j as f64) / (image_height - 1) as f64,
                0.0,
            );
            write_color(&pixel_color);
        }
    }
}

fn render_raster_image() {
    let canvas = Canvas {
        width: 256,
        height: 256,
        default_color: Color::new(0.0, 0.0, 0.0),
    };
    let base_color = Color::new(1.0, 0.0, 0.0);
    let color2 = Color::new(0.0, 1.0, 0.0);
    let color3 = Color::new(0.0, 0.0, 1.0);

    let mut frame_buffer: Vec<Color> = vec![Color::new(0.0, 0.0, 0.0); canvas.width as usize * canvas.height as usize];

    let tri_1 = Triangle {
        vertices: [(130, 25), (170, 178), (67, 158)],
        color: base_color,
    };
    let tri_2 = Triangle {
        vertices: [(15, 30), (192, 8), (118, 97)],
        color: color2,
    };
    let tri_3 = Triangle {
        vertices: [(87, 47), (224, 100), (198, 176)],
        color: color3,
    };
    let cir_1 = Circle {
        center: (128, 128),
        radius: 32,
        color: base_color,
    };
    let quad1 = Quad {
        vertices: [Pos2::new(15, 15), Pos2::new(75, 20), Pos2::new(80, 165), Pos2::new(11, 145)],
        color: color2,
    };
    let square1 = Square {
        center: Pos2::new(200, 200),
        side_length: 50,
        color: color3,
    };

    //tri_1.draw_fill(&canvas, &mut frame_buffer);
    //tri_2.draw_fill(&canvas, &mut frame_buffer);
    //tri_3.draw_fill(&canvas, &mut frame_buffer);
    //cir_1.draw_fill(&canvas, &mut frame_buffer);
    quad1.draw_fill(&canvas, &mut frame_buffer);
    square1.draw_fill(&canvas, &mut frame_buffer);
    
    output_ppm(&canvas, &frame_buffer);
}
