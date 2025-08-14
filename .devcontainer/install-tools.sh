#!/bin/bash
# Script to install required tools for Anya Core development

set -e # Stop on first error
echo "=== Installing Development Tools for Anya Core ==="

# Update system packages
echo "Updating system packages..."
if [ "$(id -u)" -ne 0 ]; then
    # Not running as root, use sudo
    sudo apt-get update
    sudo apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        wget \
        jq \
        libssl-dev \
        pkg-config \
        protobuf-compiler \
        libprotobuf-dev \
        libclang-dev
else
    # Running as root
    apt-get update
    apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        wget \
        jq \
        libssl-dev \
        pkg-config \
        protobuf-compiler \
        libprotobuf-dev \
        libclang-dev
fi

# Update rustup
echo "Updating Rust..."
rustup update
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

# Install Rust components
echo "Installing Rust components..."
rustup component add rustfmt clippy rls rust-src rust-analysis

# Install Cargo utilities
echo "Installing Cargo utilities..."
# First install cargo-binstall (fast binary installer)
cargo install cargo-binstall || echo "cargo-binstall installation failed, continuing..."

# Install tools preferring binary installation
echo "Installing cargo tools via binstall..."
cargo binstall -y --force \
    cargo-audit \
    cargo-update \
    cargo-outdated \
    cargo-edit \
    cargo-tarpaulin \
    cargo-insta \
    cargo-criterion \
    cargo-nextest \
    cargo-watch \
    cargo-expand \
    cargo-llvm-cov \
    cargo-deny \
    cargo-make \
    cargo-web \
    wasm-pack \
    cross \
    sccache ||
    echo "Some binary tools failed to install, falling back to cargo install"

# Fallback for tools that might have failed binary installation
echo "Installing any missing tools via cargo install..."
for tool in cargo-audit cargo-update cargo-outdated cargo-edit cargo-tarpaulin cargo-insta cargo-criterion cargo-nextest cargo-watch cargo-expand cargo-llvm-cov cargo-deny cargo-make cargo-web wasm-pack cross sccache; do
    if ! command -v "${tool}" &>/dev/null; then
        echo "Installing ${tool}..."
        cargo install "${tool}" --force || echo "Failed to install ${tool}, skipping"
    fi
done

# Final guard to ensure sccache is present
if ! command -v sccache &>/dev/null; then
    echo "Attempting explicit sccache install (guard)"
    cargo install sccache --force || echo "WARN: sccache installation failed"
fi

# Install Python ML dependencies
echo "Installing Python ML dependencies..."
python -m pip install --upgrade pip
python -m pip install numpy pandas matplotlib scikit-learn tensorflow torch torchvision jupyter jupyterlab

# Install Node.js if not already installed
if ! command -v node &>/dev/null; then
    echo "Installing Node.js..."
    if [ "$(id -u)" -ne 0 ]; then
        # Not running as root, use sudo
        curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo bash -
        sudo apt-get install -y nodejs
    else
        # Running as root
        curl -fsSL https://deb.nodesource.com/setup_lts.x | bash -
        apt-get install -y nodejs
    fi
fi

# Install Bitcoin Core development dependencies
echo "Installing Bitcoin Core development dependencies..."
if [ "$(id -u)" -ne 0 ]; then
    # Not running as root, use sudo
    sudo apt-get install -y --no-install-recommends \
        automake \
        autotools-dev \
        bsdmainutils \
        libevent-dev \
        libboost-dev \
        libboost-filesystem-dev \
        libboost-system-dev \
        libboost-test-dev \
        libminiupnpc-dev \
        libnatpmp-dev \
        libzmq3-dev
else
    # Running as root
    apt-get install -y --no-install-recommends \
        automake \
        autotools-dev \
        bsdmainutils \
        libevent-dev \
        libboost-dev \
        libboost-filesystem-dev \
        libboost-system-dev \
        libboost-test-dev \
        libminiupnpc-dev \
        libnatpmp-dev \
        libzmq3-dev
fi

# Install Docker compose plugin if needed
echo "Ensuring docker compose is installed..."
if ! command -v docker compose &>/dev/null; then
    echo "Installing Docker compose plugin..."
    mkdir -p ~/.docker/cli-plugins/
    curl -SL https://github.com/docker/compose/releases/download/v2.24.6/docker-compose-linux-x86_64 -o ~/.docker/cli-plugins/docker-compose
    chmod +x ~/.docker/cli-plugins/docker-compose
fi

# Setup Git configuration with improved defaults
echo "Setting up Git defaults..."
git config --global pull.rebase true
git config --global rebase.autoStash true
git config --global fetch.prune true

echo "Installation complete!"
echo "Run ./.devcontainer/verify-setup.sh to verify the installation"
echo "Tip: Use ./.devcontainer/prune-caches.sh to reclaim space when low."
