use indicatif::{ProgressBar, ProgressStyle};
use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::{random_double, sample_square};
use crate::vectors::{unit_vector, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel100_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    // Public-facing methods and variables here
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self { Camera {
        aspect_ratio,
        image_width,
        samples_per_pixel,
        image_height: 0,
        pixel_samples_scale: Default::default(),
        center: Default::default(),
        pixel100_loc: Default::default(),
        pixel_delta_u: Default::default(),
        pixel_delta_v: Default::default(),
    }}

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        let image_width = self.image_width;
        let image_height = self.image_height;

        // Progress bar settings
        let progress_bar = ProgressBar::new(image_height as u64);
        progress_bar.set_style(ProgressStyle::with_template("{prefix} {bar:80.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("##-"));
        progress_bar.set_prefix("Scanlines completed...");

        // Render
        // PPM file settings
        println!("P3\n{image_width} {image_height}\n255");

        for j in 0..image_height as u32 {
            progress_bar.inc(1);

            for i in 0..image_width as u32 {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for s in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color(&pixel_color);
            }
        }
    }

    // Private methods and variables here
    /// Initializes all internal variables required for rendering a ray traced image
    fn initialize(&mut self) {
        // Calculate image height, and ensure it's >=1
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = Point3::new(0.0, 0.0, 0.0);

        // Calculate vectors across horizontal, down vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate horizontal, vertical delta vectors from pixel-to-pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate location of upper-left-most pixel
        let viewport_upper_left = self.center
            - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel100_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    /// Construct a ray from camera origin towards a randomly sampled point around pixel location i, j
    ///
    /// # Parameters
    /// i: Horizontal positioning of the pixel, with 0 being leftmost
    /// j: Vertical positioning of the pixel, with 0 being topmost
    ///
    /// # Returns
    /// A ray pointed at a spot in world space randomly sampled around pixel location i, j
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel100_loc
                                + ((i as f64 + offset.x()) * self.pixel_delta_u)
                                + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn ray_color(&mut self, r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(temp_rec) = world.hit(r, &Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (temp_rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = unit_vector(*r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}