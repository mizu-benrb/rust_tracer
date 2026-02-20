use std::sync::Arc;
use crate::ray::*;
use crate::vectors::*;
use crate::interval::*;
use crate::material::Material;
use crate::aabb::AABB;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
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

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> { None }
    fn bounding_box(&self) -> AABB { AABB::default() }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>, // Bounded objects
    bbox: AABB,
}

impl HittableList {
    pub fn new_empty() -> HittableList { HittableList { objects: Vec::new(), bbox: AABB::default() } }
    pub fn new(h: Arc<dyn Hittable>) -> HittableList {
        let temp_box = h.bounding_box();
        HittableList {
            objects: vec![h],
            bbox: temp_box,
        }
    }

    pub fn add(&mut self, h: Arc<dyn Hittable>) {
        self.bbox = self.bbox.combine(&h.bounding_box());
        self.objects.push(h);
    }
    pub fn clear(&mut self) { self.objects.clear(); }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(temp_rec) =  object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> AABB { self.bbox.clone() }
}