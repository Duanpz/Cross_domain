#!/bin/bash

cd "$(dirname "$0")/.."

echo "Building project..."
cargo build --workspace --release

echo "Build completed!"