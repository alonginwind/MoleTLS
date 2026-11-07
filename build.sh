#!/bin/bash
set -e

# Default to release mode
MODE="${1:-release}"

if [ "$MODE" = "debug" ]; then
    TRUNK_FLAGS=""
    CARGO_FLAGS=""
    BUILD_DIR="debug"
else
    TRUNK_FLAGS="--release"
    CARGO_FLAGS="--release"
    BUILD_DIR="release"
fi

echo "=== Building MoleTLS ($MODE) ==="

# Build frontend -> target/frontend/
echo "[1/2] Building frontend..."
cd frontend
trunk build $TRUNK_FLAGS --dist ../target/$BUILD_DIR/dist
cd ..

# Build backend -> target/$BUILD_DIR/
echo "[2/2] Building backend..."
cargo build $CARGO_FLAGS

echo "=== Done ==="
echo "target/"
echo "├── $BUILD_DIR/dist/  <- Frontend (HTML + WASM)"
echo "└── $BUILD_DIR/       <- Backend binary"
