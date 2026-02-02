use crate::vectors::Vec3;
use crate::vectors::Point3;

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Default for Ray {
    fn default() -> Ray {
        Self { origin: Vec3::default(), dir: Vec3::default() }
    }
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Self { origin, dir }
    }

    pub fn origin(&self) -> &Point3 { &self.origin }
    pub fn direction(&self) -> &Vec3 { &self.dir }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}

