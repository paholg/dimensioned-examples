This is a very basic, no-frills 3d vector library. It provides only vectors over `f64`.

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

/// Basic 3d vector class. It currently only support 64 bit floats
#[derive(Clone, Copy)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3d {
        Vector3d { x: x, y: y, z: z }
    }

    pub fn dot(self, v: Vector3d) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: Vector3d) -> Vector3d {
        Vector3d {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn norm2(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f64 {
        self.norm2().sqrt()
    }

    pub fn normalized(self) -> Vector3d {
        let n = self.norm();
        Vector3d {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }

    pub fn ran(scale: f64) -> Vector3d {
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

We'll want to be able to do arithmetic with our vectors, so let's define the basic operators.

```rust
use std::ops::Add;
impl Add<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn add(self, v: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}
```



```rust
use std::ops::Sub;
impl Sub<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn sub(self, v: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}
```



```rust
use std::ops::Neg;
impl Neg for Vector3d {
    type Output = Vector3d;
    fn neg(self) -> Vector3d {
        Vector3d {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
```



```rust
use std::ops::Mul;
impl Mul<f64> for Vector3d {
    type Output = Vector3d;
    fn mul(self, s: f64) -> Vector3d {
        Vector3d {
            x: s * self.x,
            y: s * self.y,
            z: s * self.z,
        }
    }
}
```



```rust
impl Mul<Vector3d> for f64 {
    type Output = Vector3d;
    fn mul(self, v: Vector3d) -> Vector3d {
        Vector3d {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}
```

We'll want to be able to divide vectors by scalars.

```rust
use std::ops::Div;
impl Div<f64> for Vector3d {
    type Output = Vector3d;
    fn div(self, s: f64) -> Vector3d {
        Vector3d {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}
```

It may be nice to index our vectors, so let's implement it.

```rust
use std::ops::Index;
impl Index<usize> for Vector3d {
    type Output = f64;
    fn index<'a>(&'a self, index: usize) -> &'a f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}

use std::ops::IndexMut;
impl IndexMut<usize> for Vector3d {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index"),
        }
    }
}
```

For debugging, it may be useful to display our vectors as an ordered triple, so let's implement it.

```rust
use std::fmt;
impl fmt::Display for Vector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
```
