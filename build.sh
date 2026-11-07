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

# Check if frontend source changed
FRONTEND_CHANGED=false
NEW_HASH=$(find frontend/src frontend/index.html frontend/Cargo.toml -type f -exec cat {} + | md5sum | awk '{print $1}')
if [ ! -f target/.frontend_hash ]; then
    FRONTEND_CHANGED=true
else
    OLD_HASH=$(cat target/.frontend_hash)
    if [ "$OLD_HASH" != "$NEW_HASH" ]; then
        FRONTEND_CHANGED=true
    fi
fi

# Build frontend only if source changed
if [ "$FRONTEND_CHANGED" = true ]; then
    # Build frontend -> target/dist/ (configured in Trunk.toml)
    echo "[1/2] Building frontend..."
    pushd frontend
    trunk build $TRUNK_FLAGS
    popd
    echo "$NEW_HASH" > target/.frontend_hash
else
    echo "[1/2] Frontend unchanged, skipping..."
fi

# Build backend -> target/$BUILD_DIR/
echo "[2/2] Building backend..."
cargo build $CARGO_FLAGS

echo "=== Done ==="
echo "Run: ./target/$BUILD_DIR/mole-tls"
echo "Open: http://localhost:7999/"
