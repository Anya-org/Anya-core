#!/bin/bash
set -e

echo "Fixing dependency version issues..."

# 1. Add resolver = "2" to the workspace
sed -i '/\[workspace\]/a resolver = "2"' Cargo.toml

# 2. Pin native-tls to a compatible version
cat >> Cargo.toml << 'PATCH'

# Use specific versions compatible with current Rust
[patch.crates-io]
native-tls = "=0.2.11"
PATCH

# 3. Create a .cargo/config.toml file to set dependency overrides
mkdir -p .cargo
cat > .cargo/config.toml << 'CONFIG'
[dependencies]
native-tls = { version = "0.2.11" }

[build]
rustflags = ["-C", "target-cpu=native"]
CONFIG

# 4. Fix reqwest dependency which pulls in native-tls
sed -i 's/reqwest = { version = "0.11.18", features = \["json"\] }/reqwest = { version = "0.11.18", features = ["json"], default-features = false, features = ["rustls-tls"] }/' anya-core/Cargo.toml

# 5. Update core Cargo.toml too if it exists and uses reqwest
if [ -f "core/Cargo.toml" ]; then
    if grep -q "reqwest" "core/Cargo.toml"; then
        sed -i 's/reqwest = { version = "[^"]*", features = \["json"\] }/reqwest = { version = "0.11.18", features = ["json"], default-features = false, features = ["rustls-tls"] }/' core/Cargo.toml
    fi
fi

echo "Dependency versions fixed. Now updating Cargo.lock..."
cargo update -p native-tls --precise 0.2.11

echo "Now attempting to build..."
cargo check
