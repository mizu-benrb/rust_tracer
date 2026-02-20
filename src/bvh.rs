use std::sync::Arc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::range_int_unseeded;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if let Some(interval) = self.bbox.hit(ray, ray_t) {
            let hit_left = self.left.hit(ray, interval);
            let hit_right = self.right.hit(ray, interval);

            hit_left.or(hit_right)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB { self.bbox.clone() }
}

impl BvhNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let axis = range_int_unseeded(0, 3);
        let object_span = objects.len();

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match object_span {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => (objects[0].clone(), objects[1].clone()),
            _ => {
                objects.sort_by(|a, b| {
                    let a_axis_interval = a.bounding_box().axis_interval(axis);
                    let b_axis_interval = b.bounding_box().axis_interval(axis);
                    a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
                });

                let mid = object_span / 2;
                let (left_half, right_half) = objects.split_at_mut(mid);
                (
                    Arc::new(BvhNode::new(left_half)) as Arc<dyn Hittable>,
                    Arc::new(BvhNode::new(right_half)) as Arc<dyn Hittable>
                )
            }
        };

        let bbox = left.bounding_box().combine(&right.bounding_box());

        BvhNode { left, right, bbox }
    }

    pub fn new_from_hittable_list(hittable_list: &mut HittableList) -> Self {
        BvhNode::new(&mut hittable_list.objects[..])
    }
}