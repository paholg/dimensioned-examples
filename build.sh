#!/bin/bash

# g++ -o cpp-src/monte-carlo.o -c -std=c++11 -fdata-sections -ffunction-sections -Werror -Wall -O3 cpp-src/monte-carlo.cpp

# g++ -o monte-carlo -Wl,-gc-sections cpp-src/monte-carlo.o

compilers="g++ g++-4.8 g++-4.7 g++-4.4 g++-4.6 clang++"

for compiler in $compilers; do
  if which $compiler &> /dev/null; then
    #echo You DO have $compiler
    echo -e $compiler -o monte-carlo-$compiler -Werror -Wall -O3 cpp-src/monte-carlo.cpp "\n"
    $compiler -o monte-carlo-$compiler -Werror -Wall -O3 cpp-src/monte-carlo.cpp
  else
    echo -e You do not have $compiler "\n"
  fi
done

cargo build --release

