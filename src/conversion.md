fixme: intro

We will be making a unit system, which requires that we get the macros from dimensioned when we
import it.

```rust
#[macro_use]
extern crate dimensioned as dim;
```

Let's make a new unit system. It's just a demo, so we'll make a simple one. If you want the
details on how this works, check out the dimensioned documentation on `make_units!`, but
basically we're making a unit system with only two base units, foot and minute. We create the
type aliases `Foot<V>` and `Minute<V>` for our base units, with associated constants `FT` and
`MIN` that each have a value of `1.0` of their respective types. We also create some more
constants that might be useful.

```rust
mod fm {
    make_units! {
        FM;
        ONE: Unitless;

        base {
            FT: Foot, "ft", Length;
            MIN: Minute, "min", Time;
        }
        derived {
        }
        constants {
            IN: Foot = 1.0 / 12.0;
            YD: Foot = 3.0;
            M: Foot = 1.0 / 0.3048;

            S: Minute = 1.0 / 60.0;
            HR: Minute = 60.0;
            DAY: Minute = 24.0 * 60.0;
        }
        fmt = true;
    }
    pub use self::f64consts::*;
}
```

We would like to be able to convert from the `SI` unit system to our `FM` unit system, so let's
implement `From`:

The `SI` system has 7 base units. Since our system only has 2, we will define the conversion
only when the other 5 are not present. Note that the order of the units is important, and
`SI`'s base units are in the following order: meter, kilogram, second, ampere, kelvin, candela,
mole. So, the first and third are the only ones we care about, mapping to our foot and minute,
respectively.

```rust
use std::ops::Mul;
use dim::typenum::{Integer, Prod, Z0};
use std::convert::From;
use dim::si;
impl<V, Length, Time> From<
        si::SI<V, tarr![Length, Z0, Time, Z0, Z0, Z0, Z0]>>
    for fm::FM<Prod<V, f64>, tarr![Length, Time]>
    where V: Mul<f64>, Length: Integer, Time: Integer,
{
    fn from(other: si::SI<V, tarr![Length, Z0, Time, Z0, Z0, Z0, Z0]>) -> Self {
```

Because we defined constants for the meter and second in our unit system already, we
can just use them for the conversion factors. Note that the type-level integers in the
type array represent the powers of each unit present (`tarr![P2, 0]` represents an area
for our `FM` unit system), so we need to raise each conversion factor to that power.

```rust
        let length_fac = fm::M.value_unsafe.powi(Length::to_i32());
        let time_fac = fm::S.value_unsafe.powi(Time::to_i32());
        let fac = length_fac * time_fac;

        fm::FM::new( other.value_unsafe * fac )
    }
}
```

We would also like to convert from our `FM` system to `SI`. Unfortunately, since `SI` was not
defined in this crate, we cannot implement `From`. That leaves us with three options:

1. Submit an issue on GitHub for dimensioned to add your unit system. Please feel free to do
   this if you think your unit system would be useful for others.
2. Create our own `From`-like trait and implement it.
3. Simply implement `Into` instead. The `std` documentation recommends against this, but I
   don't see a problem with it here.

Of course, you could choose options 2 and 3. In this example, we will go with 3:

```rust
impl<V, Length, Time> Into<
        si::SI<Prod<V, f64>, tarr![Length, Z0, Time, Z0, Z0, Z0, Z0]>>
    for fm::FM<V, tarr![Length, Time]>
    where V: Mul<f64>, Length: Integer, Time: Integer,
{
    fn into(self) -> si::SI<Prod<V, f64>, tarr![Length, Z0, Time, Z0, Z0, Z0, Z0]> {
        let length_fac = si::FT.value_unsafe.powi(Length::to_i32());
        let time_fac = si::MIN.value_unsafe.powi(Time::to_i32());
        let fac = length_fac * time_fac;

        si::SI::new( self.value_unsafe * fac )
    }
}
```

That's all. We might as well play around with it a bit.

```rust
fn main() {
```

Note that `1.2 * si::M` and `si::Meter::new(1.2)` are equivalent.

```rust
    let x1 = 3.0 * si::FT + si::Meter::new(2.0);
    let x2 = 3.0 * fm::FT + 2.0 * fm::M;

    assert_eq!(x1, x2.into());
    assert_eq!(x2, x1.into());

    println!("x1 = {:.2}, x2 = {:.2}", x1, x2);
```
prints: `x1 = 2.91 m, x2 = 9.56 ft`

```rust
    let x3 = x1 + x2.into();

    println!("x3 = {}", x3);
```
prints: `x3 = 5.8288 m`
```rust
}
```
