#!/bin/bash
set -e

echo "Checking current Rust version..."
rustc --version

echo "Updating Rust toolchain..."
rustup update stable

echo "Checking updated Rust version..."
rustc --version

echo "Rust toolchain updated. Now attempting to build..."
cargo check
