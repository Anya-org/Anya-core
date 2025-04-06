#!/bin/bash
# Anya Core Unix Installer

# OS detection
if [[ "msys" == "darwin"* ]]; then
  PLATFORM="macos"
else
  PLATFORM="linux"
fi

# System checks
MEM_GB=7.90494
CPU_CORES=4

# Configure based on system resources
if [[  -ge 8 ]]; then
  FEATURES="--features=full-stack"
else
  FEATURES="--features=minimal"
fi

# Build
cargo build --release 

# Install configuration
mkdir -p ~/.anya/config
cp ./config/default.yaml ~/.anya/config/
