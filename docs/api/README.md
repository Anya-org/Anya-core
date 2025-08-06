---
title: "API Reference"
description: "API reference for Anya Core"
status: "active"
last_updated: "2025-08-06"
---

# API Reference

This section contains the comprehensive API reference for the Anya Core system.

## Table of Contents

- [Overview](#overview)
- [Modules](#modules)
- [Usage](#usage)
- [Examples](#examples)

## Overview

The Anya Core API provides comprehensive interfaces for Bitcoin infrastructure, Layer2 protocol integration, and decentralized systems.

## Modules

The API documentation covers the following modules:

- **Bitcoin** - Bitcoin protocol operations and wallet management
- **Layer2** - Layer2 protocol integration (Lightning, RGB, DLC)
- **Web5** - Decentralized identity and data management
- **Tools** - Development and documentation tools
- **Enterprise** - Enterprise features and integrations

## Usage

API documentation is automatically generated from Rust source code using `cargo doc`.

### Viewing Documentation

```bash
cargo doc --open
```

## Examples

Here's a basic example of using the Anya Core API:

```rust
use anya_core::{AnyaCore, AnyaConfig};
use anya_core::bitcoin::BitcoinAdapter;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the system
    let config = AnyaConfig::default();
    let anya = AnyaCore::new(config)?;

    // Access Bitcoin functionality
    let bitcoin_adapter = anya.get_bitcoin_adapter();

    // Perform operations
    let address = bitcoin_adapter.generate_address()?;
    println!("Generated address: {}", address);

    Ok(())
}
```

*Last updated: 2025-08-06*
