#!/bin/bash
set -e

echo "Scanning and fixing all Cargo.toml files in the project..."

# Function to fix a single Cargo.toml file
fix_cargo_file() {
    local file=$1
    echo "Examining $file..."
    
    # Make a backup
    cp "$file" "$file.bak"
    
    # Replace workspace inheritance with explicit values
    sed -i 's/version.workspace = true/version = "0.1.0"/' "$file"
    sed -i 's/edition.workspace = true/edition = "2021"/' "$file"
    sed -i 's/authors.workspace = true/authors = ["Anya Development Team"]/' "$file"
    sed -i 's/license.workspace = true/license = "Apache-2.0"/' "$file"
    
    # Fix web5 dependency if present
    if grep -q "web5 =" "$file"; then
        sed -i '/web5 =/d' "$file"
        sed -i '/\[dependencies\]/a web5 = { git = "https://github.com/TBD54566975/web5-rs", branch = "main" }' "$file"
    fi
    
    echo "Fixed $file"
}

# Find all Cargo.toml files and fix them
find . -name "Cargo.toml" | while read -r file; do
    fix_cargo_file "$file"
done

# Create a simple workspace in root Cargo.toml
cat > Cargo.toml << 'TOML'
[workspace]
members = [
    "core",
    "cli"
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Anya Development Team"]
description = "Bitcoin Layer 4 Protocol Implementation"
license = "Apache-2.0"
repository = "https://github.com/anya-im/anya-core"

[workspace.dependencies]
bitcoin = { version = "0.29.2", features = ["rand", "serde"] }
secp256k1 = { version = "0.24.0", features = ["rand", "serde"] }
tokio = { version = "1.28.1", features = ["full"] }
serde = { version = "1.0.160", features = ["derive"] }
TOML

echo "All Cargo.toml files fixed. Now attempting to build..."
cargo check
