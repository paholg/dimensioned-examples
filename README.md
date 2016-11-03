# Monte Carlo Test

This repository contains multiple versions of the same Monte Carlo simulation. It began as a timing
test between C++ code and equivalent Rust code. Now it serves primarily as an example of using
[`dimensioned`](https://github.com/paholg/dimensioned/).

As a result of starting as essentially a Rust clone of C++ code, much of the Rust code isn't
exactly idiomatic.

# Usage

To run all versions of the same simulation, run

```
./run_both.sh N len iter
```

where `N` is the number of spheres, `len` is the cell dimension, and `iter` is the number of
iterations to run for.

Running with

```
./run_both.sh 30 8 1000000
```

gives a nice, quick timing comparison and has the same output for both Rust and C++ (with more
spheres, the outputs vary slightly due to floating point nonsense).

# About the simulation

This simulation creates a cubic cell and then places some spheres inside it. The cell has walls on
the *z*-axis, and periodic boundary conditions on the *x* and *y* axes. That means that if a sphere
moves outside the cell in the *x* direction, it pops in on the other side, much like Pacman.

Then, each iteration, a random move is attempted for each of the spheres. If it is moved into
another sphere or into a wall, then that move is rejected, and the sphere stays where it was.

As the simulation runs, a histogram stores counts of where spheres are seen. Since the *x* and *y*
axes are incredibly boring, the histogram only stores where spheres are seen along the *z*
axis. This histogram can then be used to calculate the density of spheres in the cell.

I did research for my undergraduate degree in physics with more involved versions of this
simulation, so it seemed a good place to start playing with Rust.

# Using dimensioned
