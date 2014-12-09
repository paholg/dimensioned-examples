extern crate time;

use std::os;
use std::num::Float;
use time::precise_time_s;
use std::io::File;

use vector3d::Vector3d;

mod vector3d;

const R: f64 = 1.0;

fn main() {
    let argv = os::args();
    // println!("Call with {} N len iter\n  where N is the number of spheres (must be a cube),\n        len is the length of the cell sides,\n        and iter is the number of iterations to run for,\n       fname is name to save the density file.\n", argv[0]);
    if argv.len() != 5 { panic!("Arguments bad!"); }
    // -----------------------------------------------------------------------------
    // Define Constants
    // -----------------------------------------------------------------------------
    let n: uint = from_str(argv[1].as_slice()).expect("Need integer for N");
    let len: f64 = from_str(argv[2].as_slice()).expect("Neat float for len");
    let iterations: uint = from_str(argv[3].as_slice()).expect("Need integer for iterations");
    let scale: f64 = 0.05;
    let de_density: f64 = 0.01;
    let density_fname = argv[4].as_slice();
    let density_path = Path::new(density_fname);

    // ---------------------------------------------------------------------------
    // Define variables
    // ---------------------------------------------------------------------------
    let density_bins: uint = (1.0*len/de_density + 0.5) as uint;
    let mut density_histogram: Vec<uint> = Vec::from_elem(density_bins, 0);
    let mut spheres: Vec<Vector3d> = Vec::with_capacity(n);

    // ---------------------------------------------------------------------------
    // Set up the initial grid
    // ---------------------------------------------------------------------------
    // Balls will be initially placed on a face centered cubic (fcc) grid
    // Note that the unit cells need not be actually "cubic", but the fcc grid will
    //   be stretched to cell dimensions
    let min_cell_width = 2.0*2.0.sqrt()*R;
    let cells = (len/min_cell_width) as uint;
    let cell_w = len/(cells as f64);

    if cell_w < min_cell_width {
        panic!("Placement cell size too small");
    }
    let offset: [Vector3d,..4] = [
        Vector3d{x: 0.0, y: cell_w, z: cell_w}/2.0,
        Vector3d{x: cell_w, y: 0.0, z: cell_w}/2.0,
        Vector3d{x: cell_w, y: cell_w, z: 0.0}/2.0,
        Vector3d{x: 0.0, y: 0.0, z: 0.0}
        ];
    let mut b = 0u;
    'a: for i in range(0u, cells) {
        for j in range(0u, cells) {
            for k in range(0u, cells) {
                for &off in offset.iter() {
                    spheres.push(Vector3d{x: (i as f64)*cell_w, y: (j as f64)*cell_w, z: (k as f64)*cell_w} + off);
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
    for i in range(0u, n) {
        for j in range(0u, n) {
            if i != j && overlap(&spheres[i], &spheres[j], len) {
                panic!("Error in sphere placement!!!");
            }
        }
    }
    println!("Placed spheres!");
    // ---------------------------------------------------------------------------
    // MAIN PROGRAM LOOP
    // ---------------------------------------------------------------------------
    let mut output_period: f64 = 1.0; // start with 1 s
    let max_output_period: f64 = 60.0*30.0; // top out at 30 mins
    let start_time = precise_time_s();
    let mut last_output = start_time;

    let mut totalmoves = 0u;
    //let mut workingmoves = 0u;
    for iteration in range(1u, iterations+1) {
        // ---------------------------------------------------------------
        // Move each sphere once
        // ---------------------------------------------------------------
        for i in range(0u, n) {
            let temp = random_move(&spheres[i], scale, len);
            let mut overlaps = false;
            for j in range(0u, n) {
                if j != i && overlap(&spheres[i], &spheres[j], len) {
                    overlaps = true;
                    break;
                }
            }
            if !overlaps {
                spheres[i] = temp;
                //workingmoves += 1;
            }
            totalmoves += 1;
        }
        // ---------------------------------------------------------------
        // Add data to density historam
        // ---------------------------------------------------------------
        for i in range(0u, n) {
            let z_i: uint = (spheres[i][2]/de_density) as uint;
            density_histogram[z_i] += 1;
        }
        // ---------------------------------------------------------------
        // Save to file
        // ---------------------------------------------------------------
        let now = precise_time_s();
        if (now - last_output > output_period) || iteration == iterations {
            last_output = now;
            output_period =
                if output_period*2.0 < max_output_period { output_period*2.0 }
            else { max_output_period };
            let elapsed = now - start_time;
            let seconds = (elapsed as uint) % 60;
            let minutes = ((elapsed/60.0) as uint) % 60;
            let hours = ((elapsed/3600.0) as uint) % 24;
            let days = (elapsed/86400.0) as uint;
            println!("(Rust) Saving data after {} days, {:02}:{:02}:{:02}, {} iterations complete.",
                     days, hours, minutes, seconds, iteration);
            // Saving density
            let mut densityout = File::create(&density_path);
            let zbins: uint = (len/de_density) as uint;
            for z_i in range(0u, zbins) {
                let z = (z_i as f64 + 0.5)*de_density;
                //let zshell_volume = len*len*de_density;
                let zhist = density_histogram[z_i];
                //let zdensity = ((zhist*n) as f64)/((totalmoves as f64)*zshell_volume);
                let data = format!("{:6.3}   {}\n", z, zhist);
                match densityout.write(data.as_bytes()) {
                   Ok(()) => (),
                   Err(e) => println!("error writing {}", e)
                }
            }
        }
    }
    // ---------------------------------------------------------------------------
    // END OF MAIN PROGRAM LOOP
    // ---------------------------------------------------------------------------
}

fn fix_periodic(mut v: Vector3d, len: f64) -> Vector3d {
    for i in range(0u, 3) {
        while v[i] > len { v[i] -= len; }
        while v[i] < 0.0 { v[i] += len; }
    }
    v
}

fn periodic_diff(a: &Vector3d, b: &Vector3d, len: f64) -> Vector3d {
    let mut v = *b - *a;
    for i in range(0u, 3) {
        while v[i] > len/2.0 { v[i] -= len; }
        while v[i] < -len/2.0 { v[i] += len; }
    }
    v
}

fn overlap(a: &Vector3d, b: &Vector3d, len: f64) -> bool {
    let d2 = periodic_diff(a, b, len).norm2();
    d2 < R*R
}

fn random_move(v: &Vector3d, scale: f64, len: f64) -> Vector3d {
    fix_periodic(*v + Vector3d::ran(scale), len)
}
