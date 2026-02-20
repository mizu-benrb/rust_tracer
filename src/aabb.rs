use crate::interval::Interval;
use crate::ray::Ray;
use crate::vectors::Point3;

#[derive(Default, Clone, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    // Calculates the bounding intervals of the box using given extrema points
    pub fn new_ab(a: Point3, b: Point3) -> Self {
        Self {
            x: if a.x() < b.x() { Interval::new(a.x(), b.x()) } else { Interval::new(b.x(), a.x()) },
            y: if a.y() < b.y() { Interval::new(a.y(), b.y()) } else { Interval::new(b.y(), a.y()) },
            z: if a.z() < b.z() { Interval::new(a.z(), b.z()) } else { Interval::new(b.z(), a.z()) }
        }
    }

    pub fn combine(&self, other: &AABB) -> AABB {
        let x = self.x.combine(&other.x);
        let y = self.y.combine(&other.y);
        let z = self.z.combine(&other.z);
        AABB { x, y, z }
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        if n == 1 {
            return self.y
        } else if n == 2 {
            return self.z
        }
        self.x
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Interval> {
        let ray_orig = ray.origin();
        let ray_dir = ray.direction();
        let mut new_interval = ray_t;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis as usize];

            let t0 = (ax.min - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis as usize]) * adinv;

            if t0 < t1 {
                if (t0 > ray_t.min) { new_interval.min = t0; }
                if (t1 < ray_t.max) { new_interval.max = t1; }
            } else {
                if (t1 > ray_t.min) { new_interval.min = t1; }
                if (t0 < ray_t.max) { new_interval.max = t0; }
            }

            if (new_interval.max <= new_interval.min) {
                return None;
            }
        }
        Some(new_interval)
    }

    pub fn is_default(&self) -> bool {
        self.x.is_default() && self.y.is_default() && self.z.is_default()
    }

}