---
title: "Getting Started with Anya Core"
description: "Quick start guide for Bitcoin infrastructure and Layer2 development with Anya Core"
---

# Getting Started with Anya Core [AIR-3][AIS-3][BPC-3][RES-3]

## Overview

Anya Core is a comprehensive Bitcoin infrastructure platform providing Layer2 protocol implementations, Nostr integration, and enterprise-grade security. This guide helps you set up and start developing with Anya Core quickly.

## Table of Contents

- [Quick Setup](#quick-setup)
- [System Requirements](#system-requirements)
- [Installation](#installation)
- [First Steps](#first-steps)
- [Development Workflow](#development-workflow)
- [Next Steps](#next-steps)

## Quick Setup

### Prerequisites Check

```bash
# Check Rust installation
rustc --version  # Should be 1.70+

# Check Git
git --version

# Check system resources
free -h  # Should have 4GB+ RAM available
df -h    # Should have 10GB+ free space
```

### 5-Minute Setup

```bash
# 1. Clone the repository
git clone https://github.com/anya-org/anya-core.git
cd anya-core

# 2. Run setup script
./scripts/setup.sh

# 3. Build the project
cargo build

# 4. Run tests to verify
cargo test --lib

# 5. Check status
./scripts/health-check.sh
```

## System Requirements

### Minimum Requirements

- **OS**: Linux (Ubuntu 20.04+), macOS (Big Sur+)
- **CPU**: 2 cores, 2.0GHz
- **RAM**: 4GB
- **Storage**: 10GB free space
- **Network**: Stable internet connection

### Recommended for Development

- **CPU**: 4+ cores, 3.0GHz+
- **RAM**: 8GB+
- **Storage**: 50GB+ SSD
- **Network**: Broadband connection

### For Production

- **CPU**: 8+ cores
- **RAM**: 16GB+
- **Storage**: 100GB+ NVMe SSD
- **Network**: 1Gbps+ connection

## Installation

### Option 1: Quick Install (Recommended)

```bash
# Download and run the installer
curl -sSL https://install.anya-core.dev | bash

# Or clone and install
git clone https://github.com/anya-org/anya-core.git
cd anya-core
./install.sh --type=standard --network=testnet
```

### Option 2: Manual Build

```bash
# 1. Install dependencies
sudo apt update && sudo apt install -y \
    build-essential curl git pkg-config \
    libssl-dev libpq-dev

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 3. Clone and build
git clone https://github.com/anya-org/anya-core.git
cd anya-core
cargo build --release

# 4. Run setup
./scripts/post-install.sh
```

### Option 3: Docker

```bash
# Pull and run
docker pull anya-core:latest
docker run -d --name anya-core \
  -p 8080:8080 \
  -v ~/.anya:/data \
  anya-core:latest

# Or build from source
docker build -t anya-core .
```

## First Steps

### 1. Verify Installation

```bash
# Check installation
anya-core --version

# Verify components
cargo test --bin health-check

# Check Layer2 protocols
cargo test layer2::test_protocols
```

### 2. Configuration

Create your configuration file:

```bash
# Copy example config
cp anya.conf.example anya.conf

# Edit configuration
nano anya.conf
```

Basic configuration:

```toml
[network]
mode = "testnet"
peers = ["localhost:8333"]

[layer2]
lightning = true
rgb = true
dlc = false

[security]
mfa_enabled = false  # Enable for production
audit_level = "basic"

[development]
debug = true
log_level = "info"
```

### 3. Start Development Server

```bash
# Start all services
cargo run --bin anya-server

# Or use the task runner
cargo run --bin dev-server
```

Your development server will be available at:

- **API**: <http://localhost:8080>
- **Dashboard**: <http://localhost:8080/dashboard>
- **Layer2 Status**: <http://localhost:8080/layer2/status>

## Development Workflow

### Running Tests

```bash
# Run all tests
cargo test

# Run specific component tests
cargo test layer2::
cargo test nostr::
cargo test bitcoin::

# Run integration tests
cargo test --test integration_comprehensive
```

### Code Examples

#### Lightning Network Payment

```rust
use anya_core::layer2::{LightningProtocol, LightningConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = LightningConfig::default_testnet();
    let lightning = LightningProtocol::new(config)?;
    
    lightning.initialize().await?;
    lightning.connect().await?;
    
    // Send payment
    let payment_request = "lnbc..."; // Lightning invoice
    let result = lightning.pay_invoice(payment_request).await?;
    println!("Payment sent: {:?}", result);
    
    Ok(())
}
```

#### Nostr Message

```rust
use anya_core::enterprise::{NostrClient, NostrConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = NostrConfig {
        relays: vec!["wss://relay.damus.io".to_string()],
        ..Default::default()
    };
    
    let client = NostrClient::new(config).await?;
    
    // Publish note
    client.publish_note("Hello, Nostr!").await?;
    
    Ok(())
}
```

### Building Features

1. **Fork and clone** the repository
2. **Create a feature branch**: `git checkout -b feature/my-feature`
3. **Write tests first** (TDD approach)
4. **Implement your feature**
5. **Run tests**: `cargo test`
6. **Submit a pull request**

## Next Steps

### Learn Core Concepts

- [Bitcoin Integration](../bitcoin/README.md) - Bitcoin protocol implementation
- [Layer2 Protocols](../layer2/README.md) - Lightning, RGB, DLC protocols
- [Nostr Integration](../nostr/README.md) - Decentralized messaging
- [Security Framework](../security/README.md) - Enterprise security

### Explore Examples

- [Payment Examples](../../examples/payments/) - Bitcoin and Lightning payments
- [Nostr Examples](../../examples/nostr/) - Messaging and social
- [Layer2 Examples](../../examples/layer2/) - Protocol implementations
- [Integration Examples](../../examples/integration/) - Full stack applications

### Development Resources

- [API Documentation](../api/README.md) - Complete API reference
- [Architecture Guide](../architecture/README.md) - System design and patterns
- [Contributing Guide](../contributing/README.md) - How to contribute
- [Testing Guide](../testing/README.md) - Testing strategies and tools

### Production Deployment

- [Deployment Guide](../deployment/README.md) - Production setup
- [Security Hardening](../security/hardening.md) - Security best practices
- [Monitoring Setup](../performance/README.md) - Observability and metrics
- [Backup & Recovery](../operations/backup.md) - Data protection

### Community

- **Discord**: Join our developer community
- **GitHub**: Report issues and contribute code
- **Documentation**: Help improve our docs
- **Blog**: Read technical deep-dives

---

**Last Updated:** June 20, 2025  
**Version:** 1.2.0  
**Status:** Active Development

