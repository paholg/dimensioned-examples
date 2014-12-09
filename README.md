Usage: `./run_both.sh N len iter` where `N` is the number of spheres, `len` is the cell dimension, and `iter` is the number of iterations to run for.

This will compile both the Rust and C++ Monte Carlo programs, run them both with timings
printed, and diff their output.
