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

# Build frontend -> target/dist/ (fixed path for rust-embed)
echo "[1/2] Building frontend..."
cd frontend
trunk build $TRUNK_FLAGS --dist ../target/dist
cd ..

# Build backend -> target/$BUILD_DIR/
echo "[2/2] Building backend..."
cargo build $CARGO_FLAGS

echo "=== Done ==="
echo "Run: ./target/$BUILD_DIR/mole-tls"
echo "Open: http://localhost:7999/"
