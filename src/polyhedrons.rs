use std::sync::Arc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::PHI;
use crate::ray::Ray;
use crate::vectors::*;

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(static_center: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(static_center, Vec3::ZERO),
            radius: radius.max(0.0),
            mat,
            bbox: {
                let r_vec = Vec3::new(radius, radius, radius);
                AABB::new_ab(static_center - r_vec, static_center + r_vec)
            },
        }
    }

    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let center = Ray::new(center1, center2 - center1);
        Self {
            center,
            radius: radius.max(0.0),
            mat,
            bbox: {
                let r_vec = Vec3::new(radius, radius, radius);
                let box1 = AABB::new_ab(center.at(0.0) - r_vec, center.at(0.0) + r_vec);
                let box2 = AABB::new_ab(center.at(1.0) - r_vec, center.at(1.0) + r_vec);
                box1.combine(&box2)
            }
        }
    }

    // p: given point on sphere of radius one, centered at origin
    // u: returned value [0,1] of angle around the Y axis from X=-1
    // v: return value [0,1] of angle from Y=-1 to Y=+1
    pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.tm());
        let radius = self.radius;

        let oc = current_center - *r.origin(); // Potential bug: Flip center and r.origin if encountered
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
            u: 0.0,
            v: 0.0,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: (r.at(root) - current_center) / radius,
            front_face: true,
        };
        let outward_normal = hit_record.normal;
        hit_record.set_face_normal(r, &outward_normal);
        (hit_record.u, hit_record.v) = Sphere::get_sphere_uv(&outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
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
            u: 0.0,
            v: 0.0,
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