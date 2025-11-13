# Build the Rust WASM module
build:
    @echo "Building Rust WASM..."
    cd wasm && wasm-pack build --target web --out-dir pkg

local:
    @echo "Building Rust WASM..."
    cd wasm && wasm-pack build --target nodejs --out-dir pkg_node
# Clean Rust build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cd wasm && cargo clean
    rm -rf wasm/pkg

