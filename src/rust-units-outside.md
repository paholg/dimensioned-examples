We'll start by importing only the things that we'll be using a lot.

```rust
extern crate dimensioned as dim;
extern crate time;

mod vector3d;
use vector3d::Vector3d;

use dim::si::{self, Meter, M};
```

Once `const_fns` are stable, this can be replaced with a much nicer call to `Meter::new()`. In this
case, we could just do `const R: Meter<f64> = M;` but we may wish to change its value.

```rust
const R: Meter<f64> = Meter {
    value_unsafe: 1.0,
    _marker: std::marker::PhantomData,
};
```


We will need the `norm2()` function from `Vector3d`, so let's make it work on vectors with
units. First, we'll define a trait for it because we love member functions.

```rust
trait Norm2 {
    type Output;
    fn norm2(self) -> Self::Output;
}
```

Now, we'll implement it using the handy `MapUnsafe` trait from `dimensioned`. It is so named
because it gives us unlimited power over our value and its units, thereby disregarding all the
dimensional safety that this library provides. It's a lot like the `unsafe` keyword, only for
dimensional safety instead of memory safety. Once we've ensured that `norm2` is implemented
correctly, we can rest easy knowing it will be safe to use.

We could implement this a bit more simply if we only cared about it working for the `SI` unit
system, but it's not that much more trouble to make it general, and now it will work with *any*
unit system.

The `norm2` funtion is a particulary nice case study for this pattern, because it's rather simple
and yet involes changing both the value type and the units of our object.

```rust
use std::ops::Mul;
use dim::{Dimensioned, MapUnsafe};
use dim::typenum::{Prod, P2};
impl<D, U> Norm2 for D where
    U: Mul<P2>,
    D: Dimensioned<Value = Vector3d, Units = U> + MapUnsafe<f64, Prod<U, P2>>,
{
    type Output = <D as MapUnsafe<f64, Prod<U, P2>>>::Output;
    fn norm2(self) -> Self::Output {
        self.map_unsafe(Vector3d::norm2)
    }
}
```

Notice that `MapUnsafe` is parametrized by the output types of your value and units, like so:
`MapUnsafe<ValueOut, UnitsOut>`. The new units are defined just by the associated type, `Output`
(in this case, by the expression `Prod<U, P2>` which means the new units are whatever the old ones
were, only doubled), whereas the new value type (`f64` here) needs to be specified there and
altered in the function body (the call to `norm2` here).


Other vector operations could be similarly implemented, but they are not needed here, and so their
implementation is left as an exercise for the reader.


We'll define our own wrapper around `precise_time_s` so that it has units.

```rust
fn time() -> si::Second<f64> {
    time::precise_time_s() * si::S
}


fn main() {

    let argv: Vec<String> = std::env::args().collect();

    if argv.len() != 5 {
        println!("Call with {} N len iter
    where N is the number of spheres (must be a cube),
        len is the length of the cell sides,
        iter is the number of iterations to run for,
        fname is name to save the density file.", argv[0]);
        panic!("Arguments bad!");
    }

    let n: usize = argv[1].parse().expect("Need integer for N");
    let len = argv[2].parse::<f64>().expect("Neat float for len") * M;
    let iterations: usize = argv[3].parse().expect("Need integer for iterations");
    let scale: f64 = 0.05;

    let de_density = 0.01 * M;
    let density_fname = &argv[4];
    let density_path = std::path::Path::new(density_fname);
```

The `Deref` trait is implemented for `SI<V, A> -> V` only for dimensionless quantities, so
we can go from `len/de_density` (which both have units of `Meter`) to a primitive in a
convenient, yet dimensionally safe, manner.

```rust
    let density_bins: usize = *(len / de_density + 0.5) as usize;

    let mut density_histogram: Vec<usize> = vec![0; density_bins];
    let mut spheres: Vec<Meter<Vector3d>> = Vec::with_capacity(n);

    let min_cell_width = 2.0 * 2.0f64.sqrt() * R;
    let cells = *(len / min_cell_width) as usize;
    let cell_w = len / (cells as f64);

    if cell_w < min_cell_width {
        panic!("Placement cell size too small");
    }
```

Oops, we run into our first problem here. We need to make vectors from `cell_w`, but it
has dimensions so we can't do it directly. We have to pull out the value, put that in the vector,
and then wrap the whole vector in dimensions. This is essentially
dimensioned's version of an unsafe block, and it could be avoided by using a generic
vector with the dimensions on the inside.

```rust
    let offset = [Meter::new(Vector3d::new(0.0, cell_w.value_unsafe, cell_w.value_unsafe) / 2.0),
                  Meter::new(Vector3d::new(cell_w.value_unsafe, 0.0, cell_w.value_unsafe) / 2.0),
                  Meter::new(Vector3d::new(cell_w.value_unsafe, cell_w.value_unsafe, 0.0) / 2.0),
                  Meter::new(Vector3d::new(0.0, 0.0, 0.0) / 2.0)];
```

I would have liked to wrap these vectors in dimensions by simply multiplying by `M`.

We could multiply with `M` on the right, but then we would have to implement `Mul<SI<f64, A>>
for Vector3d`. That's fine in this example, but if `Vector3d` were defined in a different crate, then
we'd be out of luck.

We could multiply with `M` on the left if we were using the `oibit` feature of dimensioned, but
that currently requires a nightly version of the compiler, so we won't do that either for
this example.

So, we were forced to call the constructor `Meter::new()`.

Once our variables are wrapped in dimensions, though, this stops being an issue.

As we iterate over cells in the lattice, `offset` will give us the adjustments to make to
place our spheres.

```rust
    let mut b: usize = 0;
    'a: for i in 0..cells {
        for j in 0..cells {
            for k in 0..cells {
                for off in offset.iter() {
```

We have to do that same dimensionally unsafe trick here.

```rust
                    let x = (i as f64) * cell_w.value_unsafe;
                    let y = (j as f64) * cell_w.value_unsafe;
                    let z = (k as f64) * cell_w.value_unsafe;
```

At least we get the benefit of our dimensions for this addition.

```rust
                    spheres.push(Meter::new(Vector3d::new(x, y, z)) + off.clone());

                    b += 1;
                    if b >= n {
                        break 'a;
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in i+1..n {
            if overlap(spheres[i], spheres[j], len) {
                panic!("Error in sphere placement!!!");
            }
        }
    }
    println!("Placed spheres!");
```

No comments needed to note that these times are 1 s and 30 min respectively!

```rust
    let mut output_period = 1.0 * si::S;
    let max_output_period = 30.0 * si::MIN;

    let start_time = time();
    let mut last_output = start_time;

    for iteration in 1..iterations + 1 {
        for i in 0..n {
            let temp = random_move(&spheres[i], scale, len);
            let mut overlaps = false;

            for j in 0..n {
                if j != i && overlap(spheres[i], spheres[j], len) {
                    overlaps = true;
                    break;
                }
            }
```

We only want to keep the move if it was valid. We can't be moving our spheres into
eachother!

```rust
            if !overlaps {
                spheres[i] = temp;
            }
        }

        for sphere in &spheres {
```

We get to use that dereference trick again to go from `Unitless<f64>` to
`f64` safely.
fixme: Note we're doing sphere[2] not sphere.z

```rust
            let z_i: usize = *(sphere[2] / de_density) as usize;
            density_histogram[z_i] += 1;
        }

        let now = time();
        if (now - last_output > output_period) || iteration == iterations {
            last_output = now;
            output_period = if output_period * 2.0 < max_output_period {
                output_period * 2.0
            } else {
                max_output_period
            };
            let elapsed = now - start_time;
```

Note that, like `Deref`, this `map` function is only defined for `Unitless`
quantities. There is also a `map_unsafe()` function that works on quantities with
units, but its use should be avoided if possible as it circumvents all the unit
safety that dimensioned provides.

```rust
            use dim::Map;
            let seconds = (elapsed / si::S).map(|x| x as usize) % 60;
            let minutes = (elapsed / si::MIN).map(|x| x as usize) % 60;
            let hours = (elapsed / si::HR).map(|x| x as usize) % 24;
            let days = (elapsed / si::DAY).map(|x| x as usize);

            println!("(Rust) Saving data after {} days, {:02}:{:02}:{:02}, {} iterations \
                      complete.", days, hours, minutes, seconds, iteration);
```

Saving density

```rust
            let mut densityout = std::fs::File::create(&density_path).expect("Couldn't make file!");
```

Handy `Deref` yet again.

```rust
            let zbins: usize = *(len / de_density) as usize;

            for z_i in 0..zbins {
                let z = (z_i as f64 + 0.5) * de_density;
                let zhist = density_histogram[z_i];
                let data = format!("{:6.3}   {}\n", z / R, zhist);
                use std::io::Write;
                match densityout.write(data.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => println!("error writing {}", e),
                }
            }
        }
    }
    // ---------------------------------------------------------------------------
    // END OF MAIN PROGRAM LOOP
    // ---------------------------------------------------------------------------
}

fn fix_periodic(mut v: Meter<Vector3d>, len: Meter<f64>) -> Meter<Vector3d> {
    for i in 0..3 {
        if v[i] > len {
            v[i] -= len;
        }
        if v[i] < 0.0 * M {
            v[i] += len;
        }
    }
    v
}

fn periodic_diff(a: Meter<Vector3d>, b: Meter<Vector3d>, len: Meter<f64>) -> Meter<Vector3d> {
    let mut v = b - a;
    for i in 0..3 {
        if v[i] > 0.5 * len {
            v[i] -= len;
        }
        if v[i] < -0.5 * len {
            v[i] += len;
        }
    }
    v
}

fn overlap(a: Meter<Vector3d>, b: Meter<Vector3d>, len: Meter<f64>) -> bool {
    let d2 = periodic_diff(a, b, len).norm2();
    d2 < R * R
}

fn random_move(v: &Meter<Vector3d>, scale: f64, len: Meter<f64>) -> Meter<Vector3d> {
    fix_periodic(*v + Meter::new(Vector3d::ran(scale)), len)
}
```
