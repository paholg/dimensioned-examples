use std::fmt;
use std::num::Float;


#[allow(dead_code)]
static mut ran_x: u32 = 123456789;
#[allow(dead_code)]
static mut ran_y: u32 = 362436069;
#[allow(dead_code)]
static mut ran_z: u32 = 521288629;
#[allow(dead_code)]
static mut ran_w: u32 = 88675123;

#[allow(dead_code)]
struct Random;
impl Random {
    fn xorshift() -> u32 {
        let t: u32;
        unsafe {
            t = ran_x ^ (ran_x << 11);
            ran_x = ran_y;
            ran_y = ran_z;
            ran_z = ran_w;
            ran_w = ran_w ^ (ran_w >> 19) ^ (t ^ (t >> 8));
            ran_w
        }
    }
    fn ran() -> f64 {
        (Random::xorshift() as f64) * (1.0/4294967295.0)
    }
}

/// Basic 3d vector class. It currently only support 64 bit floats
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


#[allow(dead_code)]
impl Vector3d {
    /// Standard dot product.
    pub fn dot(&self, v: Vector3d) -> f64 { self.x*v.x + self.y*v.y + self.z*v.z }
    /// Standard cross product.
    pub fn cross(&self, v: Vector3d) -> Vector3d {
        Vector3d{x: self.y*v.z - self.z*v.y, y: self.z*v.x - self.x*v.z, z: self.x*v.y - self.y*v.x}
    }
    /// Returns the squared norm.
    pub fn norm2(&self) -> f64 { self.x*self.x + self.y*self.y + self.z*self.z }
    /// Returns the norm.
    pub fn norm(&self) -> f64 { self.norm2().sqrt() }
    /// Returns a normalized 2d vector parallel to self.
    pub fn normalized(&self) -> Vector3d {
        let n = self.norm();
        Vector3d{x: self.x/n, y: self.y/n, z: self.z/n }
    }
    pub fn ran(scale: f64) -> Vector3d {
        let mut x = 2.0*Random::ran() - 1.0;
        let mut y = 2.0*Random::ran() - 1.0;
        let mut r2 = x*x + y*y;
        while r2 >= 1.0 || r2 == 0.0 {
            x = 2.0*Random::ran() - 1.0;
            y = 2.0*Random::ran() - 1.0;
            r2 = x*x + y*y;
        }
        let mut fac = scale * (-2.0*r2.ln()/r2).sqrt();
        let mut out = Vector3d{x: x*fac, y: y*fac, z: 0.0};

        x = 2.0*Random::ran() - 1.0;
        y = 2.0*Random::ran() - 1.0;
        r2 = x*x + y*y;
        while r2 >= 1.0 || r2 == 0.0 {
            x = 2.0*Random::ran() - 1.0;
            y = 2.0*Random::ran() - 1.0;
            r2 = x*x + y*y;
        };
        fac = scale * (-2.0*r2.ln()/r2).sqrt();
        out[2] = x*fac;
        out
    }
}
/// Addition operator.
impl Add<Vector3d, Vector3d> for Vector3d {
    fn add(&self, v: &Vector3d) -> Vector3d { Vector3d{x: self.x + v.x, y: self.y + v.y, z: self.z + v.z} }
}
/// Subtration operator.
impl Sub<Vector3d, Vector3d> for Vector3d {
    fn sub(&self, v: &Vector3d) -> Vector3d { Vector3d{x: self.x - v.x, y: self.y - v.y, z: self.z - v.z} }
}
/// Gives the opposite of a vector.
impl Neg<Vector3d> for Vector3d {
    fn neg(&self) -> Vector3d { Vector3d{x: -self.x, y: -self.y, z: -self.z} }
}
/// Scalar multiplication, for scalars on the right.
impl Mul<f64, Vector3d> for Vector3d {
    fn mul(&self, s: &f64) -> Vector3d { Vector3d{x: (*s)*self.x, y: (*s)*self.y, z: (*s)*self.z} }
}
// Attempt at scalar multiplication for scalars on the left. Does not work.
// fixme: fix it or strap it
impl Mul<Vector3d, Vector3d> for f64 {
    fn mul(&self, v: &Vector3d) -> Vector3d { Vector3d{x: *self*v.x, y: *self*v.y, z: *self*v.z} }
}
/// Division by a scalar.
impl Div<f64, Vector3d> for Vector3d {
    fn div(&self, s: &f64) -> Vector3d { Vector3d{x: self.x/(*s), y: self.y/(*s), z: self.z/(*s)} }
}
impl Index<uint, f64> for Vector3d {
    fn index<'a>(&'a self, index: &uint) -> &'a f64 {
        match *index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}
impl IndexMut<uint, f64> for Vector3d {
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut f64 {
        match *index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index"),
        }
    }
}
impl fmt::Show for Vector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({}, {}, {})", self.x, self.y, self.z) }
}
