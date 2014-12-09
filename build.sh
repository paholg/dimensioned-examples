#!/bin/bash

set -ev

# g++ -o cpp-src/monte-carlo.o -c -std=c++11 -fdata-sections -ffunction-sections -Werror -Wall -O3 cpp-src/monte-carlo.cpp

# g++ -o monte-carlo -Wl,-gc-sections cpp-src/monte-carlo.o

g++ -o monte-carlo -std=c++11 -Werror -Wall -O3 cpp-src/monte-carlo.cpp


cargo build --release
