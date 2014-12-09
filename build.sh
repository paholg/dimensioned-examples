#!/bin/bash

# g++ -o cpp-src/monte-carlo.o -c -std=c++11 -fdata-sections -ffunction-sections -Werror -Wall -O3 cpp-src/monte-carlo.cpp

# g++ -o monte-carlo -Wl,-gc-sections cpp-src/monte-carlo.o

compilers="g++-4.9 g++-4.7 g++-4.4 g++-4.6 clang++"

for compiler in $compilers; do
  which $compiler
  if [[ $? -eq 0 ]]; then
    echo You DO have $compiler
    echo $compiler -o monte-carlo-$compiler -Werror -Wall -O3 cpp-src/monte-carlo.cpp
    $compiler -o monte-carlo-$compiler -Werror -Wall -O3 cpp-src/monte-carlo.cpp
  else
    echo You do not have $compiler
  fi
done

cargo build --release

