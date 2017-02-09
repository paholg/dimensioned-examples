# Hard sphere fluid Monte Carlo simulation with units on the inside.
---

```rust
extern crate dimensioned as dim;
extern crate time;
```

Because we are using a much nicer, incredibly generic vector library, there is very little extra work
for us to do in this program to get units.

```rust
mod vector3d_generic;
use vector3d_generic::Vector3d;

use dim::si::{self, Meter, M};

const R: Meter<f64> = Meter {
    value_unsafe: 1.0,
    _marker: std::marker::PhantomData,
};

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
    let dz_density = 0.01 * M;
    let density_fname = &argv[4];
    let density_path = std::path::Path::new(density_fname);

    let density_bins: usize = *(len / dz_density + 0.5) as usize;

    let mut density_histogram: Vec<usize> = vec![0; density_bins];
    let mut spheres: Vec<Vector3d<Meter<f64>>> = Vec::with_capacity(n);

    let min_cell_width = 2.0 * 2.0f64.sqrt() * R;
    let cells = *(len / min_cell_width) as usize;
    let cell_w = len / (cells as f64);

    if cell_w < min_cell_width {
        panic!("Placement cell size too small");
    }
```

This is exactly the same code we had when we weren't using units at all. All that messy
`value_unsafe` stuff is gone!

```rust
    let offset = [Vector3d::new(0.0*M,  cell_w, cell_w) / 2.0,
                  Vector3d::new(cell_w, 0.0*M,  cell_w) / 2.0,
                  Vector3d::new(cell_w, cell_w, 0.0*M) / 2.0,
                  Vector3d::new(0.0*M,  0.0*M,  0.0*M) / 2.0];


    let mut b: usize = 0;
    'a: for i in 0..cells {
        for j in 0..cells {
            for k in 0..cells {
                for off in offset.iter() {
```

Here too. We can forget we even have units most of the time.

```rust
                    let x = (i as f64) * cell_w;
                    let y = (j as f64) * cell_w;
                    let z = (k as f64) * cell_w;

                    spheres.push(Vector3d::new(x, y, z) + off.clone());

                    b += 1;
                    if b >= n {
                        break 'a;
                    }
                }
            }
        }
    }
```

Let's verify that we didn't place any spheres overlapping eachother, as they would get
stuck like that and mess up the simulation results.

```rust
    for i in 0..n {
        for j in i+1..n {
            assert!(!overlap(spheres[i], spheres[j], len));
        }
    }

    println!("Placed spheres!");

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

            if !overlaps {
                spheres[i] = temp;
            }
        }

        for sphere in &spheres {
```

As our vectors are now on the outside, we have no problem calling `sphere.z`.

```rust
            let z_i = *(sphere.z / dz_density) as usize;
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

            use dim::Map;
            let seconds = (elapsed / si::S).map(|x| x as usize) % 60;
            let minutes = (elapsed / si::MIN).map(|x| x as usize) % 60;
            let hours = (elapsed / si::HR).map(|x| x as usize) % 24;
            let days = (elapsed / si::DAY).map(|x| x as usize);
            println!("(Rust) Saving data after {} days, {:02}:{:02}:{:02}, {} iterations \
                      complete.", days, hours, minutes, seconds, iteration);

            let mut densityout = std::fs::File::create(&density_path).expect("Couldn't make file!");
            let zbins: usize = *(len / dz_density) as usize;
            for z_i in 0..zbins {
                let z = (z_i as f64 + 0.5) * dz_density;
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
}

fn fix_periodic(mut v: Vector3d<Meter<f64>>, len: Meter<f64>) -> Vector3d<Meter<f64>> {
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

fn periodic_diff(a: Vector3d<Meter<f64>>, b: Vector3d<Meter<f64>>, len: Meter<f64>) -> Vector3d<Meter<f64>> {
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
```

Because of how generic this `Vector3d` is, each of its functions is in its own trait, so we
have to get that trait into scope before using it.

```rust
use vector3d_generic::Norm2;
fn overlap(a: Vector3d<Meter<f64>>, b: Vector3d<Meter<f64>>, len: Meter<f64>) -> bool {
    let d2 = periodic_diff(a, b, len).norm2();
    d2 < R * R
}

fn random_move(v: &Vector3d<Meter<f64>>, scale: f64, len: Meter<f64>) -> Vector3d<Meter<f64>> {
    fix_periodic(*v + Vector3d::ran(scale)*M, len)
}
```
