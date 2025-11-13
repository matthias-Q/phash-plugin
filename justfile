# Build the Rust WASM module
build:
    @echo "Building Rust WASM..."
    cd wasm && wasm-pack build --target web --out-dir pkg

# Clean Rust build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cd wasm && cargo clean
    rm -rf wasm/pkg

