# rqrr-wasm

This project is still a work in progress. This project purposes is mostly to learn about WebAssembly. 
I was going to use [`quirs`](https://docs.rs/quirs/0.1.1/quirs/), but it depended on `libc` which 
can not compile to WebAssembly.

Live demo: https://jackyef.github.io/rqrr-wasm/

```
qr_rust.js 1.4 KB gzip
qr_rust.wasm 264 KB gzip
```

## Example
You can copy the qr_rust.js and qr_rust.wasm to your project, then follow the implementation in this [repo](https://github.com/jackyef/rqrr-wasm).
(Check the webpack config, and wasm/index.js)
I am still working on packing this as an npm module that can be easier to use.

## Requirements

1. Install rust [guide](https://doc.rust-lang.org/book/ch01-01-installation.html)
2. Add wasm as a compilation target to rust 
```
rustup target add wasm32-unknown-unknown
```
3. Install wasm-bindgen cli [guide](https://docs.rs/crate/wasm-bindgen/0.2.8)
4. Install cmake
5. Clone this [git repo](https://github.com/WebAssembly/binaryen), and build the binaries
6. Add the generated binaries to your `bin` directory (`/usr/local/bin`)

## Building

To build the rust source code to optimised .wasm for production
```
./build.sh
```

The rust source code is inside `rust-src` dir.

## Get the demo running locally

Easiest way is to use `http-server` npm module

```
npm install http-server -g
http-server ./docs -g 
```

Open the address, an `initialize` button will appear after the wasm module is loaded. Click on it and it will start detecting QR code from the media stream.
The output will be logged into console.

## Next goals
- [ ] Try to optimise the output and the code
- [ ] Make it easier to consume as npm package
