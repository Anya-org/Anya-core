#!/bin/bash
# Cross-platform installer for Anya Core

# Determine OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS_TYPE="macOS"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS_TYPE="Linux"
else
    OS_TYPE="Unknown"
fi

# Check for root
if [ "$EUID" -ne 0 ]; then
  echo "This installer requires root privileges. Please run with sudo."
  exit 1
fi

# Ensure Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust is required but not installed. Would you like to install it now? (y/n)"
    read -r install_rust
    if [[ "$install_rust" =~ ^[Yy]$ ]]; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo "Rust is required to continue. Exiting."
        exit 1
    fi
fi

# Run installer
echo "Installing Anya Core on $OS_TYPE..."
cargo run --release --bin anya-installer -- install
