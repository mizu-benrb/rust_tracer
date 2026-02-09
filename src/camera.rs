use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use rand::rngs::ThreadRng;
use rayon::prelude::*;
use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::{sample_square};
use crate::vectors::{cross, random_in_unit_disk, random_on_hemisphere, random_unit_vector, unit_vector, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel100_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    // Public-facing methods and variables here
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, vfov: f64) -> Self { Camera {
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from: Point3::new(0.0, 0.0, 0.0),
        look_at: Point3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,

        image_height: 0,
        pixel_samples_scale: Default::default(),
        center: Default::default(),
        pixel100_loc: Default::default(),
        pixel_delta_u: Default::default(),
        pixel_delta_v: Default::default(),
        u: Default::default(),
        v: Default::default(),
        w: Default::default(),
        defocus_disk_v: Default::default(),
        defocus_disk_u: Default::default(),
    }}

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        let image_width = self.image_width;
        let image_height = self.image_height;

        // Change the parameter in num_threads() to allocate more threads.
        rayon::ThreadPoolBuilder::new().num_threads(8).build_global().unwrap();

        // Progress bar settings
        let progress_bar = ProgressBar::new(image_height as u64);
        progress_bar.set_style(ProgressStyle::with_template("{prefix} {bar:80.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("##-"));
        progress_bar.set_prefix("Scanlines completed...");

        let now = Instant::now();

        // Render
        // PPM file settings
        println!("P3\n{image_width} {image_height}\n255");

        let mut image_buffer = vec![Color::BLACK; (image_height * image_width) as usize];

        image_buffer.par_chunks_mut(image_width as usize).enumerate().for_each(|(y, row)| {
            let mut thread_rng = rand::rng();

           for x in 0..image_width {
               let mut pixel_color = Color::new(0.0, 0.0, 0.0);
               for _s in 0..self.samples_per_pixel {
                   let r = self.get_ray(x, y as u32, &mut Some(&mut thread_rng));
                   pixel_color += self.ray_color(&r, self.max_depth, world);
               }
               pixel_color *= self.pixel_samples_scale;
               row[x as usize] = pixel_color;
           }

           progress_bar.inc(1);
        });

        for c in image_buffer {
            write_color(&c);
        }

        eprintln!("{} ms", now.elapsed().as_millis());
    }

    // Private methods and variables here
    /// Initializes all internal variables required for rendering a ray traced image
    fn initialize(&mut self) {
        // Calculate image height, and ensure it's >=1
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.look_from;

        // Camera
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate camera's u, v, w unit basis vectors for camera coordinate frame
        self.w = unit_vector(self.look_from - self.look_at);
        self.u = unit_vector(cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        // Calculate vectors across horizontal, down vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v; // Viewport goes downwards; y=0 top-most

        // Calculate horizontal, vertical delta vectors from pixel-to-pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate location of upper-left-most pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel100_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    /// Construct a ray from defocus disk towards a randomly sampled point around pixel location i, j
    ///
    /// # Parameters
    /// i: Horizontal positioning of the pixel, with 0 being leftmost
    /// j: Vertical positioning of the pixel, with 0 being topmost
    ///
    /// # Returns
    /// A ray pointed at a spot in world space randomly sampled around pixel location i, j
    fn get_ray(&self, i: u32, j: u32, thread_rng: &mut Option<&mut ThreadRng>) -> Ray {
        let offset = sample_square(thread_rng);
        let pixel_sample = self.pixel100_loc
                                + ((i as f64 + offset.x()) * self.pixel_delta_u)
                                + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample(thread_rng) };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self, thread_rng: &mut Option<&mut ThreadRng>) -> Point3 {
        let p = random_in_unit_disk(thread_rng);
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(temp_rec) = world.hit(r, &Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scatter)) = temp_rec.mat.scatter(r, &temp_rec) {
                return attenuation * self.ray_color(&scatter, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = unit_vector(*r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}