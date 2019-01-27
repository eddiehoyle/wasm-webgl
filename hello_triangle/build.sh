#!/bin/bash

cd $(dirname $0)

if [ ! -d "dist" ]; then
  mkdir dist
fi

if [ $1 = "release" ]; then
    echo "Building release"
    cargo build --target wasm32-unknown-unknown --release
    wasm-bindgen ./target/wasm32-unknown-unknown/release/hello_triangle.wasm --out-dir dist --no-typescript --no-modules
    wasm-opt -O3 -o dist/optimized.wasm dist/hello_triangle_bg.wasm
    mv dist/optimized.wasm dist/hello_triangle_bg.wasm
else
    echo "Building debug"
    RUST_BACKTRACE=1
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen ./target/wasm32-unknown-unknown/debug/hello_triangle.wasm --out-dir dist --no-typescript --no-modules
fi