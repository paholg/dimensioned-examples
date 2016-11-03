extern crate typenum;

use std::fmt;
use std::ops::{Add, Sub, Neg, Mul, Div, Index, IndexMut};
use self::typenum::{Sum, Diff, Prod, Quot, Negate};


static mut RAN: Random = Random {
    ran_x: 123456789,
    ran_y: 362436069,
    ran_z: 521288629,
    ran_w: 88675123,
};

struct Random {
    ran_x: u32,
    ran_y: u32,
    ran_z: u32,
    ran_w: u32,
}

impl Random {
    fn xorshift(&mut self) -> u32 {
        let t = self.ran_x ^ (self.ran_x << 11);
        self.ran_x = self.ran_y;
        self.ran_y = self.ran_z;
        self.ran_z = self.ran_w;
        self.ran_w = self.ran_w ^ (self.ran_w >> 19) ^ (t ^ (t >> 8));
        self.ran_w
    }

    fn ran(&mut self) -> f64 {
        (self.xorshift() as f64) * (1.0 / 4294967295.0)
    }
}

#[derive(Clone, Copy)]
pub struct Vector3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


impl<T> Vector3d<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3d<T> {
        Vector3d { x: x, y: y, z: z }
    }
}

impl Vector3d<f64> {
    pub fn ran(scale: f64) -> Vector3d<f64> {
        unsafe {
            let mut x = 2.0 * RAN.ran() - 1.0;
            let mut y = 2.0 * RAN.ran() - 1.0;
            let mut r2 = x * x + y * y;
            while r2 >= 1.0 || r2 == 0.0 {
                x = 2.0 * RAN.ran() - 1.0;
                y = 2.0 * RAN.ran() - 1.0;
                r2 = x * x + y * y;
            }
            let mut fac = scale * (-2.0 * r2.ln() / r2).sqrt();
            let mut out = Vector3d {
                x: x * fac,
                y: y * fac,
                z: 0.0,
            };

            x = 2.0 * RAN.ran() - 1.0;
            y = 2.0 * RAN.ran() - 1.0;
            r2 = x * x + y * y;
            while r2 >= 1.0 || r2 == 0.0 {
                x = 2.0 * RAN.ran() - 1.0;
                y = 2.0 * RAN.ran() - 1.0;
                r2 = x * x + y * y;
            }
            fac = scale * (-2.0 * r2.ln() / r2).sqrt();
            out[2] = x * fac;
            out
        }
    }
}

pub trait Dot<Rhs> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}

impl<T, U> Dot<Vector3d<U>> for Vector3d<T>
    where T: Mul<U> + Copy, U: Copy,
          Prod<T, U>: Add,
          Sum<Prod<T, U>, Prod<T, U>>: Add<Prod<T, U>>,
{
    type Output = Sum<Sum<Prod<T, U>, Prod<T, U>>, Prod<T, U>>;
    fn dot(self, rhs: Vector3d<U>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

pub trait Cross<Rhs> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl<T, U> Cross<Vector3d<U>> for Vector3d<T>
    where T: Copy + Mul<U>, U: Copy,
          Prod<T, U>: Sub,
{
    type Output = Vector3d<Diff<Prod<T, U>, Prod<T, U>>>;
    fn cross(self, rhs: Vector3d<U>) -> Self::Output {
        Vector3d::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

pub trait Norm2 {
    type Output;
    fn norm2(self) -> Self::Output;
}

impl<T> Norm2 for Vector3d<T>
    where Vector3d<T>: Copy + Dot<Vector3d<T>>,
{
    type Output = <Vector3d<T> as Dot<Vector3d<T>>>::Output;
    fn norm2(self) -> Self::Output {
        self.dot(self)
    }
}

pub trait Norm {
    type Output;
    fn norm(self) -> Self::Output;
}

// impl<T> Norm for Vector3d<T>
//     where Vector3d<T>: Norm2,
//     <Vector3d<T> as Norm2>::Output: Sqrt,
// {
//     type Output = <<Vector3d<T> as Norm2>::Output as Sqrt>::Output;
//     fn norm(self) -> Self::Output {
//         self.norm2().sqrt()
//     }
// }

// pub fn normalized(self) -> Vector3d {
//         let n = self.norm();
//         Vector3d {
//             x: self.x / n,
//             y: self.y / n,
//             z: self.z / n,
//         }
//     }


impl<T, U> Add<Vector3d<U>> for Vector3d<T> where T: Add<U> {
    type Output = Vector3d<Sum<T, U>>;
    fn add(self, rhs: Vector3d<U>) -> Self::Output {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T, U> Sub<Vector3d<U>> for Vector3d<T> where T: Sub<U> {
    type Output = Vector3d<Diff<T, U>>;
    fn sub(self, rhs: Vector3d<U>) -> Self::Output {
        Vector3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Neg for Vector3d<T> where T: Neg {
    type Output = Vector3d<Negate<T>>;
    fn neg(self) -> Self::Output {
        Vector3d::new(-self.x, -self.y, -self.z)
    }
}


impl<T, U> Mul<U> for Vector3d<T> where T: Mul<U>, U: Copy {
    type Output = Vector3d<Prod<T, U>>;
    fn mul(self, rhs: U) -> Self::Output {
        Vector3d::new(self.x * rhs, self.y * rhs,  self.z * rhs)
    }
}

// impl Mul<Vector3d> for f64 {
//     type Output = Vector3d;
//     fn mul(self, v: Vector3d) -> Vector3d {
//         Vector3d {
//             x: self * v.x,
//             y: self * v.y,
//             z: self * v.z,
//         }
//     }
// }

impl<T, U> Div<U> for Vector3d<T> where T: Div<U>, U: Copy {
    type Output = Vector3d<Quot<T, U>>;
    fn div(self, rhs: U) -> Self::Output {
        Vector3d::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T> Index<usize> for Vector3d<T> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}

impl<T> IndexMut<usize> for Vector3d<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index"),
        }
    }
}

impl<T> fmt::Display for Vector3d<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
