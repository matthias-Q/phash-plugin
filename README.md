# Rust pHash Processor

This Firefox extension computes a perceptual hash of an image using Rust compiled to WebAssembly. Right-click an image to compute its hash.

## Installation

1. Build the Rust WASM module

Navigate to the wasm folder and run:

```bash
wasm-pack build --target web --out-dir pkg
```

Ensure the generated pkg folder stays inside the extension folder.

2. Load the extension in Firefox
Open `about:debugging#/runtime/this-firefox`
Click Load Temporary Add-on and select `manifest.json` from the extension root.

## Usage

* Right-click any image
* Select "Compute Image Hash"
* An alert will display the computed hash

## Build Requirements
* Rust toolchain with the `wasm32-unknown-unknown` target installed
* `wasm-pack` for compiling Rust to WebAssembly

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --force
```
