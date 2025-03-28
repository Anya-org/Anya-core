#!/bin/bash
set -e

echo "Updating dependencies for testing..."

# Update core/Cargo.toml to add test dependencies
cat >> core/Cargo.toml << 'TOML'

[dev-dependencies]
tokio = { version = "1.28.1", features = ["full", "test-util", "macros"] }
rand = "0.8.5"
TOML

# Update anya-core/Cargo.toml to add tokio macros
sed -i 's/tokio = { version = "1.28.1", features = \["full"\] }/tokio = { version = "1.28.1", features = ["full", "macros"] }/' anya-core/Cargo.toml

echo "Dependencies updated for testing"
