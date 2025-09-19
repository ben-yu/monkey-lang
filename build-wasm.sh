#!/bin/bash

echo "Building Monkey Language for WebAssembly..."

# Install wasm-bindgen-cli if not already installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Build the WASM target
echo "Compiling to WebAssembly..."
cargo build --target wasm32-unknown-unknown --release

if [ $? -eq 0 ]; then
    echo "Generating WASM bindings..."
    mkdir -p web/pkg
    wasm-bindgen --out-dir web/pkg --web target/wasm32-unknown-unknown/release/monkey_lang.wasm
    
    if [ $? -eq 0 ]; then
        echo "✅ Build successful!"
        echo ""
        echo "To serve the web app:"
        echo "  cd web"
        echo "  python3 -m http.server 8000"
        echo ""
        echo "Then open http://localhost:8000 in your browser"
    else
        echo "❌ WASM bindgen failed!"
        exit 1
    fi
else
    echo "❌ Build failed!"
    exit 1
fi