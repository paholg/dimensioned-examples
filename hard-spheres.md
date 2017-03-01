# Monte Carlo Simulation of a Homogeneous Hard Sphere Fluid

This repository contains four versions of the same simulation; a C++ version, a Rust version with
no units, and two Rust versions with units that demonstrate different ways to use
[*dimensioned*](https://github.com/paholg/dimensioned/).

## Use

To run all versions of the simulation, run

```
./run_all.sh N len iter
```

where `N` is the number of spheres, `len` is the cell dimension, and `iter` is the number of
iterations for which to run.

Running with

```
./run_all.sh 100 10 1000000
```

gives a nice, quick timing comparison.

## About the simulation

A hard sphere fluid is a fluid made of spherical particles that have only one interaction; they
can't be in the same place at the same time. Imagine a box in space with a bunch of billiard balls
bouncing around in it -- that's essentially what it is. That is also one way to simulate it, called
molecular dynamics. The simulation method invoked here, Monte Carlo, does not do that. We'll get to
that in a minute.

We are simulating a homogeneous fluid. That means that it's the same everywhere---there are no
interfaces, or particles that are different from eachother, or really anything interesting at
all. Basically, instead of a box of billiard balls in space, all of space is filled with billiard
balls and only billiard balls. We simulate this by having a finite box with periodic boundary
conditions. If a sphere moves out of the box, then it moves back in on the other side, much like a
Pacman level. This way, we are simulating all of space being filled with repeating copies of our
box. As long as our box isn't too small, this will give the same results as having a box of
infinite size.

In a Monte Carlo simulation, we make a random move of each sphere. If a move is valid, we keep
it. If not, we reject the move, returning the system to its state before the move. A move is valid
if it does not cause any of the spheres to overlap. Once we've attempted moving each sphere, we
repeat, ad infinitum.

As the simulation runs, a histogram stores counts of where spheres are seen. This allows us to
calculate the density of spheres across space. It will be the same everywhere, and could be
calculated just from the simulation inputs, but this gives us an output to ensure the different
versions are all getting the exact same results.

I did research for my undergraduate degree in physics with more involved versions of this
simulation, so it seemed a good place to start playing with Rust.

## The Simulation Versions

### 0. C++

The C++ version of this simulation is located in `cpp-src`. It is not the focus of this
documentation and will not be mentioned again.

### 1. Rust with no units

Let's start with the basic simulation. For simplicity, we use a very basic, non-generic 3d vector
library, which you can [check out here](src/vector3d.md).

The only thing of note is the random number generation, which was done that way only because we
wanted identical results for the Rust and C++ code, so they both use the same, basic random number
generator.

Consider this the reference version; it explains what's happening in the simulation, whereas the
other versions will only explain differences. If you don't care about the simulation and are only here to look at
units, feel free to skim over this or skip it entirely.

[Check out the code here](src/no-units.md).

### 2. Rust with units outside

In this first version with units, we use the same non-generic vector library. So, we are forced to
wrap the vectors in units. This allows the most flexibility in what other libraries can be used
with *dimensioned*, but isn't quite as convenient to work with as the next version.

[Check out the code here](src/units-outside.md).

### 3. Rust with units inside

In this final version, we treat primitives with units are just primitives, resulting in code that
is very similar to the version with no units. It is the ideal way to use *dimensioned*.

Of course, there's a catch. We need a much more flexibile vector library, which you can view
[here](src/vector3d_generic.md). Note that it is not enough for the vector library to be generic,
it also has to have no contraints on how types change under operations. E.g. When you multiply two
vectors over `Meter<f64>`, you'll end up with a vector over `Meter2<f64>`, and the library has to
allow this.

So, this version has less flexibility in terms of what libraries it can be used with, but allows
treating primitives with units as just primitives, which turns out to be *really* nice.

[Check out the code here](src/units-inside.md).
