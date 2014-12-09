#!/bin/bash

echo "Run with $0 N len iter"

source build.sh

fcpp="mc/cpp-$1-$2-$3"
frust="mc/rust-$1-$2-$3"

echo Running with rust
{ time ./target/release/monte-carlo-test $1 $2 $3 $frust; } 2> mc/time-rust

for compiler in $compilers; do
  which $compiler
  if [[ $? -eq 0 ]]; then
    echo You DO have $compiler
    { time ./monte-carlo-$compiler $1 $2 $3 $fcpp-$compiler; } 2> mc/time-$compiler
    echo -e "\nFirst 10 lines of output differences:" $compiler
    diff $fcpp-$compiler $frust | head -n10
  else
    echo You do not have $compiler
 fi
done


tail -n3 mc/time*
