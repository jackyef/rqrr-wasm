# Rust playground

This project is still a work in progress

## Requirements

1. Install rust, [this is a good guide for it](https://doc.rust-lang.org/book/ch01-01-installation.html)
2. Add wasm as a compilation target to rust 
```
rustup target add wasm32-unknown-unknown
```
3. Install wasm-bindgen cli [guide](https://docs.rs/crate/wasm-bindgen/0.2.8)
4. Install cmake
5. Clone this [git repo](https://github.com/WebAssembly/binaryen), and build the binaries
6. Add the generated binaries to your `bin` directory (`/usr/local/bin`)

## Building

To build optimised .wasm for production
```
./build.sh
```

To check if the source code can be compiled correctly
```
cargo check
```

To build + run the binary in the same command
```
cargo run
```