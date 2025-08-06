---
title: "Getting Started"
description: "Quick start guide for Anya Core"
last_updated: $(date +%Y-%m-%d)
---

[AIR-3][AIS-3][BPC-3][RES-3]

# Getting Started with Anya Core

## Overview

This guide helps you get started with Anya Core quickly and efficiently.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [First Steps](#first-steps)
- [Next Steps](#next-steps)

## Installation

### Prerequisites

- **Rust**: 1.70.0 or later
- **Git**: Latest stable version
- **System**: Linux, macOS, or Windows with WSL2

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Clone and Build

```bash
git clone https://github.com/anya-org/anya-core.git
cd anya-core
cargo build --release
```

## Quick Start

### 1. Verify Installation

```bash
# Check that everything built correctly
cargo test --lib

# Run specific module tests
cargo test bitcoin::
cargo test ml::
cargo test web5::
```

### 2. Basic Configuration

Create a basic configuration file:

```bash
cp anya.conf.example anya.conf
```

Basic configuration:

```toml
[network]
mode = "testnet"

[features]
bitcoin = true
web5 = false
ml = true

[logging]
level = "info"
```

### 3. Run Your First Program

```rust
use anya_core::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Anya Core!");

    let config = AnyaConfig::from_file("anya.conf")?;
    let core = AnyaCore::new(config).await?;

    println!("Anya Core initialized successfully!");

    Ok(())
}
```

## Configuration

Anya Core supports various configuration options:

### Features

Enable specific features based on your needs:

```toml
[features]
bitcoin = true       # Bitcoin protocol support
web5 = true         # Web5 integration
ml = true           # Machine learning capabilities
enterprise = false  # Enterprise features
```

### Network Settings

```toml
[network]
mode = "testnet"    # or "mainnet"
peers = ["localhost:8333"]
```

## First Steps

1. **Explore Modules**: Check the [module documentation](../README.md)
2. **Read Examples**: Look at implementation examples
3. **Run Tests**: Verify everything works in your environment
4. **Build Something**: Start with a simple integration

## Next Steps

- [Architecture Guide](../architecture/README.md)
- [API Reference](../api/README.md)
- [Module Documentation](../README.md)
- [Contributing Guide](../contributing/README.md)

## Support

- [GitHub Issues](https://github.com/anya-org/anya-core/issues)
- [Discussions](https://github.com/anya-org/anya-core/discussions)

*Last updated: $(date +%Y-%m-%d)*
