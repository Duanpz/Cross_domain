#!/bin/bash
set -e
echo "Building project..."

cargo clean
cargo build --workspace

echo "Build completed successfully!"
