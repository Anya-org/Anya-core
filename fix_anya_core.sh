#!/bin/bash
set -e

echo "===== Starting Anya Core Repair Process ====="

# Step 1: Fix dependencies
echo "Fixing dependencies..."
./fix_dependencies.sh

# Step 2: Build import fixer
echo "Building import fixer..."
cargo build --bin fix_imports

# Step 3: Run import fixer
echo "Running import fixer..."
cargo run --bin fix_imports

# Step 4: Setup VSCode
echo "Setting up VSCode..."
./setup_vscode.sh

# Step 5: Try building the project
echo "Attempting to build anya-core..."
cargo build --bin unified_installer

echo "===== Anya Core Repair Process Complete ====="
echo "You can now open VSCode with: code ."
