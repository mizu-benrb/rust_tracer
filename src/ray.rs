use crate::vectors::Vec3;
use crate::vectors::Point3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
    tm: f64,
}

impl Default for Ray {
    fn default() -> Ray {
        Self { origin: Vec3::default(), dir: Vec3::default(), tm: 0.0 }
    }
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Self { origin, dir, tm: 0.0 }
    }
    pub fn new_t(origin: Point3, dir: Vec3, tm: f64) -> Ray { Self { origin, dir, tm } }

    pub fn origin(&self) -> &Point3 { &self.origin }
    pub fn direction(&self) -> &Vec3 { &self.dir }
    pub fn tm(&self) -> f64 { self.tm }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}

