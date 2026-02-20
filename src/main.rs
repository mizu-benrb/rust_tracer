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
mod image_io;
mod aabb;
mod bvh;

use std::sync::Arc;
use std::f64::consts::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::rngs::Xoshiro256PlusPlus;
use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::vectors::*;
use crate::color::*;
use crate::ray::*;
use crate::raster::*;
use crate::polyhedrons::*;
use crate::hittable::*;
use crate::interval::Interval;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::utility::{random_double, random_double_unseeded, range_double, range_double_unseeded};

const PHI: f64 = 1.618033988749894;

fn main() {
    // HW 2
    render_ray_image();
}

fn render_ray_image() {

    let mut world: HittableList = HittableList::new_empty();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    //world.add(Arc::new(
    //    Plane::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), material_ground)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, material_ground)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_double_unseeded();
            let center = Point3::new(a as f64 + 0.9 * random_double_unseeded(), 0.2, b as f64 + 0.9 * random_double_unseeded());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> =
                    if choose_material < 0.8 {
                        let albedo = Color::random_unseeded() * Color::random_unseeded();
                        Arc::new(Lambertian::new(albedo))
                    } else if choose_material < 0.95 {
                        let albedo = Color::random_range_unseeded(0.5, 1.0);
                        let fuzz = range_double_unseeded(0.0, 0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    } else {
                        Arc::new(Dielectric::new(1.5))
                    };
                let center2 =
                    if choose_material < 0.8 {
                        center + Vec3::new(0.0, range_double_unseeded(0.0, 0.0), 0.0)
                    } else {
                        center
                    };

                world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, sphere_material)));
            }

        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2)));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_3)));

    let bvh_test = HittableList::new(Arc::new(BvhNode::new_from_hittable_list(&mut world)));

    let mut cam: Camera = Camera::new( 16.0 / 9.0, 1920, 512, 32, 20.0);

    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&bvh_test);
}

fn render_ray_image_2() {

    let mut world: HittableList = HittableList::new_empty();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.9, 0.3, 0.5)));
    world.add(Arc::new(
        Plane::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), material_ground)));

    let material_glass = Arc::new(Dielectric::new(1.00/1.33));
    let one_third_sqrt = (1.0f64 / 3.0f64).sqrt();
    let starting_point = Point3::new(1.0, 0.2, -2.0);

    for i in 0..10 {
        let size_interval = 1.0 + i as f64;
        let center = starting_point + Point3::new(one_third_sqrt * size_interval, one_third_sqrt * size_interval,-one_third_sqrt * size_interval);
        world.add(Arc::new(
            Sphere::new(center, size_interval / 2.2, material_glass.clone())));
    }

    let mut cam: Camera = Camera::new( 16.0 / 9.0, 1920, 512, 32, 60.0);

    cam.look_from = Point3::new(0.3, 0.2, 0.0);
    cam.look_at = Point3::new(0.8, 0.7, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 5.0;

    cam.render(&world);
}

fn render_ray_image_3() {

    let mut world: HittableList = HittableList::new_empty();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.70, 0.70, 0.9)));
    world.add(Arc::new(
        Plane::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), material_ground)));

    let material_snow = Arc::new(Lambertian::new(Color::WHITE));
    let material_coal = Arc::new(Metal::new(Color::new(0.05, 0.05, 0.05), 0.6));
    let material_glass = Arc::new(Dielectric::new(1.5));

    // Snowballs
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 6.0, 0.0), 6.0, material_snow.clone())));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 13.0, 0.0), 4.5, material_snow.clone())));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 19.5, 0.0), 3.5, material_snow.clone())));

    // Eyes
    world.add(Arc::new(
        Sphere::new(Point3::new(1.0, 19.5, -3.3), 0.5, material_coal.clone())));
    world.add(Arc::new(
        Sphere::new(Point3::new(1.0, 19.5, -3.3), 1.0, material_glass.clone())));
    world.add(Arc::new(
        Sphere::new(Point3::new(-1.0, 19.5, -3.3), 0.5, material_coal.clone())));
    world.add(Arc::new(
        Sphere::new(Point3::new(-1.0, 19.5, -3.3), 1.0, material_glass.clone())));

    // Mouth
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 17.6, -3.3), 0.3, material_coal.clone())));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.6, 18.1, -3.3), 0.3, material_coal.clone())));
    world.add(Arc::new(
        Sphere::new(Point3::new(-0.6, 18.1, -3.3), 0.3, material_coal.clone())));

    // Suit
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 12.0, -4.3), 0.3, material_coal.clone())));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 13.0, -4.4), 0.3, material_coal.clone())));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 14.0, -4.3), 0.3, material_coal.clone())));

    let mut cam: Camera = Camera::new( 16.0 / 9.0, 960, 128, 32, 70.0);

    cam.look_from = Point3::new(-15.0, 10.5, -15.0);
    cam.look_at = Point3::new(1.0, 16.0, 1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 8.0;

    cam.render(&world);
}

fn render_ray_image_4() {

    let mut world: HittableList = HittableList::new_empty();
    //let material_ground = Arc::new(Lambertian::new(Color::new(0.70, 0.70, 0.9)));
    //world.add(Arc::new(
    //    Plane::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), material_ground)));

    let material_1 = Arc::new(Metal::new(Color::RED, 0.1));
    let material_2 = Arc::new(Metal::new(Color::GREEN, 0.1));
    let material_3 = Arc::new(Metal::new(Color::BLUE, 0.1));
    let material_glass = Arc::new(Dielectric::new(1.3));

    world.add(Arc::new(
        Plane::new(Point3::new(-0.25, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), material_1)
    ));
    world.add(Arc::new(
        Plane::new(Point3::new(0.25, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), material_2)
    ));
    world.add(Arc::new(
        Plane::new(Point3::new(0.0, 1.0, -0.25), Vec3::new(0.0, 0.0, 0.25), material_3)
    ));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 5.0, 0.0), 3.0, material_glass.clone())
    ));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 5.0, -7.0), 3.0, material_glass.clone())
    ));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 5.0, -14.0), 3.0, material_glass.clone())
    ));

    let mut cam: Camera = Camera::new( 16.0 / 9.0, 1920, 512, 32, 70.0);

    cam.look_from = Point3::new(-10.0, 10.0, -10.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

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
