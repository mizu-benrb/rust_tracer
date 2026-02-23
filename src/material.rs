use std::sync::Arc;
use rand::rngs::Xoshiro256PlusPlus;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{Solid_Color, Texture};
use crate::utility::random_double;
use crate::vectors::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3};

pub trait Material: Send + Sync {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord, rng: &mut Xoshiro256PlusPlus) -> Option<(Color, Ray)>;
}

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian { Lambertian { tex: Arc::new(Solid_Color::new(&albedo)) } }

    pub fn new_texture(tex: Arc<dyn Texture>) -> Lambertian { Lambertian { tex } }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Xoshiro256PlusPlus) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector(rng);

        // Catch degenerate scatter directions
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new_t(rec.p, scatter_direction, r_in.tm());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal { Metal { albedo, fuzz }}
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Xoshiro256PlusPlus) -> Option<(Color, Ray)> {
        let mut reflect_direction = reflect(r_in.direction(), &rec.normal);
        reflect_direction = unit_vector(reflect_direction) + (self.fuzz * random_unit_vector(rng));
        let scattered = Ray::new_t(rec.p, reflect_direction, r_in.tm());
        if dot(scattered.direction(), &rec.normal) > 0.0 {
            return Some((self.albedo, scattered))
        }
        None
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric { Dielectric { refractive_index } }

    pub fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut Xoshiro256PlusPlus) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face { 1.0 / self.refractive_index } else { self.refractive_index };

        let unit_direction = unit_vector(*r_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_double(rng) {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, ri);
        }

        let scattered = Ray::new_t(rec.p, direction, r_in.tm());
        Some((attenuation, scattered))
    }
}