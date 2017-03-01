# Generic 3d Vectors

```rust
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
```

Now we're generic over any type `T`.

```rust
#[derive(Clone, Copy)]
pub struct Vector3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
```

Pretty much all of our functions have to be in their own traits, making the struct `impl`
rather small.

```rust
impl<T> Vector3d<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3d<T> {
        Vector3d { x: x, y: y, z: z }
    }
}
```

Generating random vectors in a Gaussian distribution pretty much requires that we're working
with floats, so we'll only define it for vectors over `f64`.

```rust
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
```

These three operators (`Add`, `Sub`, and `Neg`) do not change units, and so we can implement
them expecting type `T` to not change. We could be more generic, and implement them similarly to
how we will do `Mul`, but I'm not sure that anything would require that.

```rust
use std::ops::Add;
impl<T> Add<Vector3d<T>> for Vector3d<T>
    where T: Add<T, Output = T>
{
    type Output = Vector3d<T>;
    fn add(self, rhs: Vector3d<T>) -> Self::Output {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

use std::ops::Sub;
impl<T> Sub<Vector3d<T>> for Vector3d<T>
    where T: Sub<T, Output = T>
{
    type Output = Vector3d<T>;
    fn sub(self, rhs: Vector3d<T>) -> Self::Output {
        Vector3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

use std::ops::Neg;
impl<T> Neg for Vector3d<T>
    where T: Neg<Output = T>
{
    type Output = Vector3d<T>;
    fn neg(self) -> Self::Output {
        Vector3d::new(-self.x, -self.y, -self.z)
    }
}
```

For `Mul` and `Div`, we need type to be able to change. For example, if we multiply
`Vector3d<Newton<f64>>` by `Meter<f64>`, we will end up with `Vector3d<Joule<f64>>`.

```rust
use std::ops::Mul;
use dim::typenum::Prod;
impl<T, U> Mul<U> for Vector3d<T>
    where T: Mul<U>,
          U: Clone
{
    type Output = Vector3d<Prod<T, U>>;
    fn mul(self, rhs: U) -> Self::Output {
        Vector3d::new(self.x * rhs.clone(), self.y * rhs.clone(), self.z * rhs)
    }
}

use std::ops::Div;
use dim::typenum::Quot;
impl<T, U> Div<U> for Vector3d<T>
    where T: Div<U>,
          U: Clone
{
    type Output = Vector3d<Quot<T, U>>;
    fn div(self, rhs: U) -> Self::Output {
        Vector3d::new(self.x / rhs.clone(), self.y / rhs.clone(), self.z / rhs)
    }
}
```

The first of our custom operations. We create a trait with an associated type.

```rust
pub trait Dot<Rhs = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}
```

And then we implement it. Again, we are assuming that our vectors are over some type that does
not change over addition; if we weren't making that assumption, this would get a good deal
messier.

```rust
impl<T, U> Dot<Vector3d<U>> for Vector3d<T>
    where T: Mul<U>,
          Prod<T, U>: Add<Output = Prod<T, U>>
{
    type Output = Prod<T, U>;
    fn dot(self, rhs: Vector3d<U>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
```

The cross product follows the same pattern.

```rust
pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl<T, U> Cross<Vector3d<U>> for Vector3d<T>
    where T: Mul<U> + Copy,
          U: Copy,
          Prod<T, U>: Sub<Output = Prod<T, U>>
{
    type Output = Vector3d<Prod<T, U>>;
    fn cross(self, rhs: Vector3d<U>) -> Self::Output {
        Vector3d::new(self.y * rhs.z - self.z * rhs.y,
                      self.z * rhs.x - self.x * rhs.z,
                      self.x * rhs.y - self.y * rhs.x)
    }
}
```



```rust
pub trait Norm2 {
    type Output;
    fn norm2(self) -> Self::Output;
}
```

To implement norm-squared, we can just call out to `Dot` that we've already defined.

```rust
impl<T> Norm2 for Vector3d<T>
    where Vector3d<T>: Copy + Dot
{
    type Output = <Vector3d<T> as Dot>::Output;
    fn norm2(self) -> Self::Output {
        self.dot(self)
    }
}
```



```rust
pub trait Norm {
    type Output;
    fn norm(self) -> Self::Output;
}
```

Implementing `Norm` is a bit trickier. For this, we need to take a square root. We have a
couple options.

1. We could just implement it for primitives and leave it to users to make a norm for anything else they want.
2. We could use the `Float` trait from the *num* crate. This is more flexible, but still leaves out *dimensioned*.
3. We could use the `Sqrt` trait from *dimensioned*. This gives us support for *dimensioned*
   and primitives, but requires our vector library be aware of *dimensioned*.

We will go with option 3.

```rust
use dim::Sqrt;
impl<T> Norm for Vector3d<T>
    where Vector3d<T>: Norm2,
          <Vector3d<T> as Norm2>::Output: Sqrt
{
    type Output = <<Vector3d<T> as Norm2>::Output as Sqrt>::Output;
    fn norm(self) -> Self::Output {
        self.norm2().sqrt()
    }
}
```



```rust
pub trait Normalized {
    type Output;
    fn normalized(self) -> Self::Output;
}

impl<T> Normalized for Vector3d<T>
    where Vector3d<T>: Clone + Norm + Div<<Vector3d<T> as Norm>::Output>
{
    type Output = Quot<Self, <Self as Norm>::Output>;
    fn normalized(self) -> Self::Output {
        let n = self.clone().norm();
        self / n
    }
}
```



```rust
use std::ops::Index;
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

use std::ops::IndexMut;
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

use std::fmt;
impl<T> fmt::Display for Vector3d<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
```
