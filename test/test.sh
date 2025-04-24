#!/bin/bash

ycc=../target/debug/ycc
# set -e DO NOT add this 

try() {
    # rm -rf tmp.s
    
    expected="$1"
    input="$2"
    # $ycc "$input" > tmp.s
    $ycc "$input" > tmp.s
    clang -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" != "$expected" ];then
        echo "$input expected, but got $actual"
        exit 1
    fi
}

echo "=================== build ==================== "
cargo clean 
cargo build 

echo "===================  test =================== "

try 0 0
try 42 42

echo OK