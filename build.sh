#!/bin/sh

set -ex

# Build the `hello_world.wasm` file using Cargo/rustc
cargo build --target wasm32-unknown-unknown --release

mkdir -p dist
wasm-bindgen target/wasm32-unknown-unknown/release/qr_rust.wasm --out-dir ./dist --no-modules --no-typescript

wasm-opt -Os dist/qr_rust_bg.wasm -o dist/qr_rust.wasm