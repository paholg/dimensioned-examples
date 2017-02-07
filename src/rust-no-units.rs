//@ todo: introduction

//@ Not used for the simulation itself, but we want to save to file at fixed intervals and to know
//@ how long we've been running the simulation for.

extern crate time;
use time::precise_time_s;

//@ This is a very simple 3d vector over f64s.

mod vector3d;
use vector3d::Vector3d;

//@ All distances should be in terms of the sphere radius, `R`. We don't have units to enforce
//@ this, though, so if we ever change `R`, or decide to work in terms of diameter instead, we may
//@ get bugs where we've forgetten to divide by `R`.

const R: f64 = 1.0;

fn main() {

    //@ This is just a test program, so we'll do some very simple argument parsing.

    let argv: Vec<String> = std::env::args().collect();
    if argv.len() != 5 {
        println!("Call with {} N len iter
    where N is the number of spheres (must be a cube),
        len is the length of the cell sides,
        iter is the number of iterations to run for,
        fname is name to save the density file.", argv[0]);
        panic!("Arguments bad!");
    }

    //@ `n` is the number of spheres in our simulation.

    let n: usize = argv[1].parse().expect("Need integer for N");

    //@ The length of the sides of our cubical cell.

    let len: f64 = argv[2].parse().expect("Neat float for len") * R;

    //@ The number of iterations for which to run the simulation. Here, we are calling an iteration
    //@ an attempted move of every sphere. Others might call an iteration an attempted move of a
    //@ single sphere, making our terms differ by a factor of `n`.

    let iterations: usize = argv[3].parse().expect("Need integer for iterations");

    //@ Whenever we attempt to move a sphere, it will be by a random distancee from a Gaussian
    //@ distribution where `scale` is the width. Its value does not affect the
    //@ correctness of our results, but it will affect how many moves we accept and how quickly we
    //@ converge on the correct result. In a real simulation, we would want to adjust this to get a
    //@ reasonable acceptance rate.

    let scale: f64 = 0.05;

    //@ This is the width of the bins for the density histogram. We are using *e* to mean a general
    //@ coordinate direction (*x*, *y*, or *z*), so *de* is the delta of that coordinate.

    let de_density: f64 = 0.01;

    //@ Set up the file for saving output.

    let density_fname = &argv[4];
    let density_path = std::path::Path::new(density_fname);

    //@ Based on the size of the cell and the size of the bins, we can create the density
    //@ histogram. Note that it is one-dimenional, as we are only storing density information along
    //@ one axis.

    let density_bins: usize = (1.0 * len / de_density + 0.5) as usize;
    let mut density_histogram: Vec<usize> = vec![0; density_bins];

    //@ The most important variable we have. Our spheres!

    let mut spheres: Vec<Vector3d> = Vec::with_capacity(n);

    //@ For initial placement, we need a valid state. That means that all spheres must be in the
    //@ cell and none of them may be overlapping. One method would be to place them randomly and
    //@ then move them around until they no longer overlap, but that is time consuming.
    //@
    //@ Instead, we will place them on a face centered cubic (fcc) grid, which allows the highest
    //@ packing denisty possible for spheres. If we were running a real simulation, we would then
    //@ want to move them a bunch before taking data, so that they start in a higher entropy state.

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

    //@ Now that they've been placed, let's make sure that none of the spheres overlap. That should
    //@ only happen if our placement algorithm is wrong or if a user selects a ridiculously high
    //@ density.

    for i in 0..n {
        for j in i+1..n {
            if overlap(spheres[i], spheres[j], len) {
                panic!("Error in sphere placement!!!");
            }
        }
    }
    println!("Placed spheres!");

    //@ We want to time the simulation, both so that we can see how long it's been running for and
    //@ so that we can save our results at a regular frequency. We will start saving every second,
    //@ doubling the time between saves each time we save, up to a maximum of 30 minutes each save.

    let mut output_period: f64 = 1.0; // start with 1 s
    let max_output_period: f64 = 60.0 * 30.0; // top out at 30 mins

    //@ Let's start the clock!

    let start_time = precise_time_s();
    let mut last_output = start_time;

    //@ We start the main loop

    for iteration in 1..iterations + 1 {

        //@ First, we attempt to move each sphere once.

        for i in 0..n {
            let temp = random_move(&spheres[i], scale, len);
            let mut overlaps = false;

            //@ Unlike before, we have to check sphere *i* against all spheres *j*, not just the
            //@ ones with higher indices, because we're moving them as we go.

            for j in 0..n {
                if j != i && overlap(spheres[i], spheres[j], len) {
                    overlaps = true;
                    break;
                }
            }

            //@ We only want to keep the move if the sphere doesn't overlap with any others.

            if !overlaps {
                spheres[i] = temp;
            }
        }

        //@ Now, we will update the density histogram with the new locations of the spheres. We
        //@ could do this either more or less frequently, but doing it each time we move all the
        //@ spheres seems reasonable enough.

        for sphere in spheres {

            //@ Each bin in the histogram is actually a slice of the cell. Since we're only
            //@ tracking data along the *z*-axis, the *x* and *y* coordinates don't matter at all
            //@ for this purpose.
            //@
            //@ Where `sphere.z` is the *z* coordinate of the sphere in real space, *z_i* is the
            //@ corresponding index in the histogram.

            let z_i: usize = (sphere.z / de_density) as usize;
            density_histogram[z_i] += 1;
        }

        //@ Finally, if enough time has passed, let's save our density data to a file.

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

            let mut densityout = std::fs::File::create(&density_path).expect("Couldn't make file!");
            let zbins: usize = (len / de_density) as usize;
            for z_i in 0..zbins {
                let z = (z_i as f64 + 0.5) * de_density;
                let zhist = density_histogram[z_i];
                let data = format!("{:6.3}   {}\n", z, zhist);
                use std::io::Write;
                match densityout.write(data.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => println!("error writing {}", e),
                }
            }
        }
    }
}

//@ While we are operating in a cubical cell, we are simulating all of space being filled with
//@ copies of our cell. We do this by having periodic boundary conditions. Think of it like a Pacman
//@ level; if you exit on one side, you come back on the other side. That's what this function
//@ handles.

fn fix_periodic(v: Vector3d, len: f64) -> Vector3d {
    let mut v = v;
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

//@ This function finds the vector from sphere `a` to sphere `b`, even across the periodic
//@ boundary.

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

//@ Spheres `a` and `b`: Where are they? Do they overlap? Let's find out!

fn overlap(a: Vector3d, b: Vector3d, len: f64) -> bool {
    let d2 = periodic_diff(a, b, len).norm2();

    //@ We use the distance squared to avoid having to take an unnecessary square root.

    d2 < R * R
}

//@ Perform a random move using a Gaussian distribution of standard deviation of width
//@ `scale`. Then, move the result into the cell in case it's escaped.

fn random_move(v: &Vector3d, scale: f64, len: f64) -> Vector3d {
    fix_periodic(*v + Vector3d::ran(scale), len)
}
