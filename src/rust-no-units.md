```rust
extern crate time;

use std::path::Path;
// use std::num::Float;
use time::precise_time_s;
use std::fs::File;
use std::io::Write;

use vector3d::Vector3d;

mod vector3d;

const R: f64 = 1.0;

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
    // -----------------------------------------------------------------------------
    // Define Constants
    // -----------------------------------------------------------------------------
    let n: usize = argv[1].parse().expect("Need integer for N");
    let len: f64 = argv[2].parse().expect("Neat float for len");
    let iterations: usize = argv[3].parse().expect("Need integer for iterations");
    let scale: f64 = 0.05;
    let de_density: f64 = 0.01;
    let density_fname = &argv[4];
    let density_path = Path::new(density_fname);

    // ---------------------------------------------------------------------------
    // Define variables
    // ---------------------------------------------------------------------------
    let density_bins: usize = (1.0 * len / de_density + 0.5) as usize;
    let mut density_histogram: Vec<usize> = vec![0; density_bins];
    let mut spheres: Vec<Vector3d> = Vec::with_capacity(n);

    // ---------------------------------------------------------------------------
    // Set up the initial grid
    // ---------------------------------------------------------------------------
    // Balls will be initially placed on a face centered cubic (fcc) grid
    // Note that the unit cells need not be actually "cubic", but the fcc grid will
    //   be stretched to cell dimensions
    let min_cell_width = 2.0 * 2.0f64.sqrt() * R;
    let cells = (len / min_cell_width) as usize;
    let cell_w = len / (cells as f64);

    if cell_w < min_cell_width {
        panic!("Placement cell size too small");
    }
    let offset: [Vector3d; 4] = [Vector3d::new(0.0,    cell_w, cell_w) / 2.0,
                                 Vector3d::new(cell_w, 0.0,    cell_w) / 2.0,
                                 Vector3d::new(cell_w, cell_w, 0.0   ) / 2.0,
                                 Vector3d::new(0.0,    0.0,    0.0   )];
    let mut b: usize = 0;
    'a: for i in 0..cells {
        for j in 0..cells {
            for k in 0..cells {
                for off in offset.iter() {
                    spheres.push(
                        Vector3d::new((i as f64) * cell_w, (j as f64) * cell_w, (k as f64) * cell_w)
                            + off.clone()
                    );
                    b += 1;
                    if b >= n {
                        break 'a;
                    }
                }
            }
        }
    }

    // ---------------------------------------------------------------------------
    // Ensure spheres don't overlap
    // ---------------------------------------------------------------------------

    for i in 0..n {
        for j in i+1..n {
            if overlap(spheres[i], spheres[j], len) {
                panic!("Error in sphere placement!!!");
            }
        }
    }
    println!("Placed spheres!");

    // ---------------------------------------------------------------------------
    // MAIN PROGRAM LOOP
    // ---------------------------------------------------------------------------

    let mut output_period: f64 = 1.0; // start with 1 s
    let max_output_period: f64 = 60.0 * 30.0; // top out at 30 mins
    let start_time = precise_time_s();
    let mut last_output = start_time;

    // let mut totalmoves = 0;
    // let mut workingmoves = 0;
    for iteration in 1..iterations + 1 {
        // ---------------------------------------------------------------
        // Move each sphere once
        // ---------------------------------------------------------------
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
                // workingmoves += 1;
            }
            // totalmoves += 1;
        }

        // ---------------------------------------------------------------
        // Add data to density historam
        // ---------------------------------------------------------------
        for i in 0..n {
            let z_i: usize = (spheres[i][2] / de_density) as usize;
            density_histogram[z_i] += 1;
        }

        // ---------------------------------------------------------------
        // Save to file
        // ---------------------------------------------------------------
        let now = precise_time_s();
        if (now - last_output > output_period) || iteration == iterations {
            last_output = now;
            output_period = if output_period * 2.0 < max_output_period {
                output_period * 2.0
            } else {
                max_output_period
            };
            let elapsed = now - start_time;
            let seconds = (elapsed as usize) % 60;
            let minutes = ((elapsed / 60.0) as usize) % 60;
            let hours = ((elapsed / 3600.0) as usize) % 24;
            let days = (elapsed / 86400.0) as usize;
            println!("(Rust) Saving data after {} days, {:02}:{:02}:{:02}, {} iterations \
                      complete.", days, hours, minutes, seconds, iteration);
            // Saving density
            let mut densityout = File::create(&density_path).expect("Couldn't make file!");
            let zbins: usize = (len / de_density) as usize;
            for z_i in 0..zbins {
                let z = (z_i as f64 + 0.5) * de_density;
                // let zshell_volume = len*len*de_density;
                let zhist = density_histogram[z_i];
                // let zdensity = ((zhist*n) as f64)/((totalmoves as f64)*zshell_volume);
                let data = format!("{:6.3}   {}\n", z, zhist);
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

fn fix_periodic(mut v: Vector3d, len: f64) -> Vector3d {
    for i in 0..3 {
        if v[i] > len {
            v[i] -= len;
        }
        if v[i] < 0.0 {
            v[i] += len;
        }
    }
    v
}

fn periodic_diff(a: Vector3d, b: Vector3d, len: f64) -> Vector3d {
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

fn overlap(a: Vector3d, b: Vector3d, len: f64) -> bool {
    let d2 = periodic_diff(a, b, len).norm2();
    d2 < R * R
}

fn random_move(v: &Vector3d, scale: f64, len: f64) -> Vector3d {
    fix_periodic(*v + Vector3d::ran(scale), len)
}
```
