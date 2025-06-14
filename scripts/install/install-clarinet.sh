#!/usr/bin/env bash
# Install Clarinet for Linux/macOS by building from source
set -e

CLARINET_REPO="https://github.com/hirosystems/clarinet.git"
CLARINET_VERSION="2.3.0"

# Check if clarinet is already installed
if command -v clarinet >/dev/null 2>&1; then
  echo "Clarinet is already installed: $(clarinet --version)"
  exit 0
fi

# Ensure Rust toolchain is available
if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust toolchain not found, installing via rustup..."
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source "$HOME/.cargo/env"
fi

# Clone and build Clarinet
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

echo "Cloning Clarinet v$CLARINET_VERSION..."
git clone --depth 1 --branch v$CLARINET_VERSION "$CLARINET_REPO" "$TMPDIR"
cd "$TMPDIR"

echo "Building Clarinet... (this may take a few minutes)"
cargo build --release

# Install binary
echo "Installing Clarinet to /usr/local/bin..."
sudo cp target/release/clarinet /usr/local/bin/clarinet
sudo chmod +x /usr/local/bin/clarinet

# Verify installation
echo "Installed Clarinet version: $(clarinet --version)"
