#!/bin/bash

echo -e "Run with $0 N len iter\n"

source build.sh

fcpp="mc/cpp-$1-$2-$3"
frust="mc/rust-$1-$2-$3"

rm mc/time*

echo -e "\n==> Rust v1 <=="
{ time ./target/release/v1 $1 $2 $3 $frust-1; } 2> mc/time-rust-1
echo -e "\n==> Rust v2 <=="
{ time ./target/release/v2 $1 $2 $3 $frust-2; } 2> mc/time-rust-2
if diff $frust-2 $frust-1 &> /dev/null; then
    echo "Same output as Rust v1"
else
    echo -e "\nDIFFERENT! First 10 lines of output differences for $compiler:"
    diff $fcpp-$compiler $frust-1
fi
echo ""

for compiler in $compilers; do
  if which $compiler &> /dev/null; then
    echo "==> $compiler <=="
    { time ./monte-carlo-$compiler $1 $2 $3 $fcpp-$compiler; } 2> mc/time-$compiler
    if diff $fcpp-$compiler $frust-1 &> /dev/null; then
        echo "Same output as Rust v1"
    else
        echo -e "\nDIFFERENT! First 10 lines of output differences for $compiler:"
        diff $fcpp-$compiler $frust-1 | head -n10
    fi
    echo ""
 fi
done


tail -n3 mc/time*
