#!/bin/bash

g++ -o cpp-src/monte-carlo.o -c -std=c++11 -fdata-sections -ffunction-sections -Werror -Waddress -Warray-bounds -Wc++11-compat -Wchar-subscripts -Wenum-compare -Wcomment -Wformat -Wmaybe-uninitialized -Wmissing-braces -Wnonnull -Wparentheses -Wreorder -Wreturn-type -Wsequence-point -Wsign-compare -Wstrict-aliasing -Wstrict-overflow=1 -Wswitch -Wtrigraphs -Wuninitialized -Wunknown-pragmas -Wunused-function -Wunused-label -Wunused-value -Wunused-variable -O3 -Isrc -Iinclude -Itests cpp-src/monte-carlo.cpp

g++ -o monte-carlo -Wl,-gc-sections cpp-src/monte-carlo.o


cargo build --release
