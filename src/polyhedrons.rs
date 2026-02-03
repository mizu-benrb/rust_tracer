use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vectors::*;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self { Self { center, radius: radius.max(0.0) } }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        if (root <= t_min || t_max < root) {
            root = (h + sqrt_d) / a;
            if (root <= t_min || t_max < root) {
                return None;
            }
        }

        let mut hit_record: HitRecord = HitRecord {
            t: root,
            p: r.at(root),
            normal: (r.at(root) - center) / radius,
            front_face: true,
        };
        let outward_normal = hit_record.normal;
        hit_record.set_face_normal(r, &outward_normal);

        Some(hit_record)
    }
}

struct Plane {
    normal: Vec3,
    origin: Point3,
}

impl Plane {
    pub fn new(origin: Point3, normal: Vec3) -> Self { Self { origin, normal }}
}