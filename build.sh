#!/bin/sh

g++ -o cpp-src/vector3d.o -c cpp-src/vector3d.cpp
g++ -o monte-carlo cpp-src/vector3d.o cpp-src/monte-carlo.cpp

cargo build
