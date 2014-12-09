#!/bin/bash

echo "Run with $0 N len iter"

./build.sh

fcpp="mc/cpp-$1-$2-$3" frust="mc/rust-$1-$2-$3"

time { ./target/release/monte-carlo-test $1 $2 $3 $frust; } 2> mc/rusttime &
time { ./monte-carlo $1 $2 $3 $fcpp; } 2> mc/cpptime &

FAIL=0

for job in `jobs -p`; do
    wait $job || let "FAIL+=1"
done

if [ "$FAIL" == "0" ]; then
    echo -e "\nFirst 10 lines of output differences:"
    diff $fcpp $frust | head -n10
    echo -e "\nThe C++ Program took:"
    cat mc/cpptime | tail -n3
    echo -e "\nThe Rust Program took:"
    cat mc/rusttime | tail -n3
else
    echo "Failed!"
    echo -e $rustout
    echo -e $cout
    exit 1
fi

