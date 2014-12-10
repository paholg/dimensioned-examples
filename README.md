A project to compare Rust efficiency to C++ for a simple Monte Carlo simulation.

Usage: `./run_both.sh N len iter` where `N` is the number of spheres, `len` is the cell dimension, and `iter` is the number of iterations to run for.

Running with `./run_both.sh 30 8 1000000` gives a nice timing comparison and has the same output for both Rust and C++ (with more spheres, the outputs vary slightly).

This will compile both the Rust and C++ Monte Carlo programs, run them both with timings
printed, and diff their output.
