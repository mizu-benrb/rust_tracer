use std::ops;
use std::fmt;
use crate::utility::{random_double, range_double};

// Struct definition for different dimension vectors
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub e: [f64; 2],
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

pub struct Vec4 {
    pub e: [f64; 4],
}

// Useful aliases
pub type Point2 = Vec2;
pub type Point3 = Vec3;

// Implement associated functions, overloaded operators for Vec2
impl Default for Vec2 {
    fn default() -> Self {
        Self { e : [0.0, 0.0] }
    }
}

impl ops::Index<usize> for Vec2 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self { Self { e: [x, y] } }

    pub fn x(&self) -> f64 { self[0] }

    pub fn y(&self) -> f64 { self[1] }

    pub fn length_squared(&self) -> f64 { self.x().powi(2) + self.y().powi(2) }

    pub fn length(&self) -> f64 { self.length_squared().sqrt() }
}


// Implement associated functions, overloaded operators for Vec3
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e : [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    
    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(range_double(min, max), range_double(min, max), range_double(min, max))
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e : [0.0, 0.0, 0.0] }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(
            self.x() + other.x(), self.y() + other.y(), self.z() + other.z()
        );
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self::new(
            self.x() * other, self.y() * other, self.z() * other
        )
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0 / other;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();

        Vec3::new(x, y, z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Mul<Vec3> for Vec3 { // Vector-by-vector multiplication
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl ops::Mul<f64> for Vec3 { // Vector-by-scalar multiplication
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<Vec3> for f64 { // Scalar-by-vector multiplication
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

#[inline]
pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

#[inline]
pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(
        lhs.y() * rhs.z() - lhs.z() * rhs.y(),
        lhs.z() * rhs.x() - lhs.x() * rhs.z(),
        lhs.x() * rhs.y() - lhs.y() * rhs.x()
    )
}

#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    let len: f64 = v.length();
    v / len
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    // EVISCERATE THIS AND REPLACE WITH SOMETHING BETTER
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let len_sq = p.length_squared();
        if 1e-160 < len_sq && len_sq <= 1.0 {
            return p / len_sq.sqrt();
        }
    }
}

#[inline]
pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if(dot(&on_unit_sphere, &normal) > 0.0) {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

// Implement associated functions, overloaded operators for Vec2
impl Default for Vec4 {
    fn default() -> Self {
        Self { e : [0.0, 0.0, 0.0, 0.0] }
    }
}

impl ops::Index<usize> for Vec4 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self { e: [x, y, z, w] } }

    pub fn x(&self) -> f64 { self[0] }
    pub fn y(&self) -> f64 { self[1] }
    pub fn z(&self) -> f64 { self[2] }
    pub fn w(&self) -> f64 { self[3] }

    pub fn length_squared(&self) -> f64 { self.x().powi(2) + self.y().powi(2) + self.z().powi(2) + self.w().powi(2) }

    pub fn length(&self) -> f64 { self.length_squared().sqrt() }
}

impl ops::Mul<Vec4> for Vec4 { // Vector-by-vector multiplication
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z(), self.w() * rhs.w())
    }
}

impl ops::Mul<f64> for Vec4 { // Vector-by-scalar multiplication
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: f64) -> Vec4 {
        Vec4::new(self.x() * rhs, self.y() * rhs, self.z() * rhs, self.w() * rhs)
    }
}

impl ops::Mul<Vec4> for f64 { // Scalar-by-vector multiplication
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        rhs * self
    }
}