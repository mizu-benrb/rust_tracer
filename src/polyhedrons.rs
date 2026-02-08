use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vectors::*;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self { Self { center, radius: radius.max(0.0), mat } }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let center = self.center;
        let radius = self.radius;

        let oc = center - *r.origin(); // Potential bug: Flip center and r.origin if encountered
        let a = r.direction().length_squared();
        let h = dot(&oc, &r.direction());
        let c = oc.length_squared() - radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find nearest root within acceptable range
        let mut root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut hit_record: HitRecord = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: (r.at(root) - center) / radius,
            front_face: true,
        };
        let outward_normal = hit_record.normal;
        hit_record.set_face_normal(r, &outward_normal);

        Some(hit_record)
    }
}

pub struct Plane {
    normal: Vec3,
    origin: Point3,
    mat: Arc<dyn Material>,
}

impl Plane {
    // Ensure normal is unit length
    pub fn new(normal: Vec3, origin: Point3, mat: Arc<dyn Material>) -> Self { Self { normal: normal / normal.length(), origin, mat }}
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Assume all vectors involved are normalized
        let denom = dot(&self.normal, r.direction());
        if denom.abs() <= 1e-8 {
            return None;
        }

        let o_r0 = self.origin - *r.origin();
        let t = dot(&o_r0, &self.normal) / denom;
        if !ray_t.surrounds(t) {
            return None;
        }
        let mut rec = HitRecord {
            t,
            p: r.at(t),
            mat: self.mat.clone(),
            normal: self.normal,
            front_face: true,
        };
        let outward_normal = rec.normal;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}