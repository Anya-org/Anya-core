#!/bin/bash

# This script checks for unused dependencies in the workspace.

set -e

echo "Checking for unused dependencies..."

# Install cargo-udeps if it's not already installed
if ! command -v cargo-udeps &> /dev/null
then
    echo "cargo-udeps could not be found, installing..."
    cargo install cargo-udeps
fi

cargo udeps --all-targets --all-features --workspace
