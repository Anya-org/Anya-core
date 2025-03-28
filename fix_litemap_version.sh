#!/bin/bash
set -e

echo "Fixing litemap version compatibility issue..."

# Create a simplified workspace with just the core components
cat > Cargo.toml << 'TOML'
[workspace]
resolver = "2"
members = [
    "core",
    "anya-core",
    "cli"
]

# Explicitly exclude all other crates to avoid conflicts
exclude = [
    "anya-bitcoin",
    "anyacore",
    "anya-enterprise", 
    "anya-extensions",
    "anya-mobile",
    "bitcoin-adapter",
    "dependencies",
    "enterprise",
    "installer",
    "mobile",
    "scripts",
    "src",
    "workspace"
]
TOML

# Update litemap to a compatible version as suggested by the error message
echo "Updating litemap to compatible version..."
cargo update litemap@0.7.5 --precise 0.5.0

# If that specific command fails, try a more general approach
if [ $? -ne 0 ]; then
    echo "Direct update failed, trying cargo update..."
    cargo update
fi

# Add a backup option - manually update core/Cargo.toml to avoid url dependency
if [ -f "core/Cargo.toml" ]; then
    echo "Updating core/Cargo.toml to avoid problematic dependencies..."
    
    # Remove url dependency which pulls in litemap
    sed -i '/url =/d' core/Cargo.toml
    
    # Use a basic url_parser instead
    cat >> core/Cargo.toml << 'URL_DEPS'
    
# Use a simpler URL parsing method to avoid the litemap dependency
[dependencies.url_parser]
version = "0.1.0"
optional = true
URL_DEPS
    
    # Update reqwest to use rustls explicitly
    sed -i 's/reqwest = { version = "[^"]*"/reqwest = { version = "0.11.11", default-features = false/g' core/Cargo.toml
    sed -i 's/features = \["json"\]/features = ["json", "rustls-tls"]/g' core/Cargo.toml
fi

echo "Attempting to build with simplified workspace..."
cargo build -p core -p anya-core

echo "You can now build the core components with:"
echo "cargo build --bin anya-core"
