#!/bin/bash
set -e

echo "Fixing web5 dependency..."

# Remove the incorrect web5 dependency
sed -i '/web5 = /d' Cargo.toml

# Add the correct web5 dependency from GitHub
sed -i '/\[dependencies\]/a web5 = { git = "https://github.com/TBD54566975/web5-rs", features = ["did", "dwn"] }' Cargo.toml

# Fix any other problematic dependencies
if grep -q "bincode =" Cargo.toml; then
    sed -i 's/bincode = .*/bincode = "1.3.3"/' Cargo.toml
fi

# Fix any workspace issues
if ! grep -q '\[workspace\]' Cargo.toml; then
    echo "
[workspace]
members = [
    \"core\",
    \"cli\"
]
" >> Cargo.toml
fi

echo "Dependencies fixed. Now attempting to build..."
cargo check
