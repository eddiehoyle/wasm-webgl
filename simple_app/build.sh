#!/bin/bash

cd $(dirname $0)

if [ ! -d "dist" ]; then
  mkdir dist
fi

PROJECT=${PWD##*/}
  
if [ "$1" == "release" ]; then
    echo "Building release"
    cargo build --target wasm32-unknown-unknown --release
    wasm-bindgen ./target/wasm32-unknown-unknown/release/$PROJECT.wasm --out-dir dist --no-typescript --no-modules
    wasm-opt -O3 -o dist/optimized.wasm dist/$PROJECT_bg.wasm
    mv dist/optimized.wasm dist/$PROJECT_bg.wasm
else
    echo "Building debug"
    RUST_BACKTRACE=1
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen ./target/wasm32-unknown-unknown/debug/$PROJECT.wasm --out-dir dist --no-typescript --no-modules
fi