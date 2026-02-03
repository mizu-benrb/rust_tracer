use std::sync::Arc;
use crate::ray::*;
use crate::vectors::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Sets hit record's normal vector
    // NOTE: outward_normal assumed to be of unit length
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal.clone() } else { -outward_normal.clone() };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> { None }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new_empty() -> HittableList { HittableList { objects: Vec::new() } }
    pub fn new(h: Arc<dyn Hittable>) -> HittableList { HittableList { objects: vec![h] } }

    pub fn add(&mut self, h: Arc<dyn Hittable>) { self.objects.push(h); }
    pub fn clear(&mut self) { self.objects.clear(); }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(temp_rec) =  object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }

        hit_record
    }
}