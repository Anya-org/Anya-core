---
title: "layer2 Module"
description: "! Layer2 protocols module for Bitcoin scaling solutions"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# layer2 Module

## Overview

! Layer2 protocols module for Bitcoin scaling solutions

This module contains 19 Rust source files implementing core functionality for the Anya Core system.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Components](#components)
- [API](#api)
- [Examples](#examples)
- [Testing](#testing)
- [See Also](#see-also)

## Architecture

### Module Structure

This module exports the following public interfaces:

```rust
pub mod async_coordinator;
pub mod bob;
pub mod dlc;
pub mod lightning;
pub mod liquid;
pub mod manager;
pub mod mock; // Kept for backward compatibility and testing
pub mod production; // New production implementation
pub mod rgb;
pub mod rsk;
```

## Components

The following files implement this module:

- **async_coordinator.rs** - [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
- **mod.rs** - ! BOB protocol implementation for Layer2 Bitcoin scaling
- **comprehensive_tests.rs** - Implementation file
- **mod.rs** - [AIR-3][AIS-3][BPC-3][RES-3]
- **mod.rs** - [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
- **mod.rs** - ! Liquid protocol implementation for Layer2 Bitcoin scaling
- **manager.rs** - [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
- **mod.rs** - Implementation file
- **mod.rs** - ! Layer2 protocols module for Bitcoin scaling solutions
- **production_adapters.rs** - ! Production Layer2 Protocol Adapters
- **production.rs** - ! Production Layer2 Protocol Implementation
- **mod.rs** - [AIR-3][AIS-3][BPC-3][RES-3]
- **software_hsm.rs** - ! Real Software-based HSM Security Implementation
- **mod.rs** - ! RSK protocol implementation for Layer2 Bitcoin scaling
- **mod.rs** - ! Stacks protocol implementation for Layer2 Bitcoin scaling
- **protocol_trait.rs** - / Stacks configuration
- **mod.rs** - [AIR-3][AIS-3][BPC-3][RES-3]
- **mod.rs** - ! Taproot Assets protocol implementation for Layer2 Bitcoin scaling
- **taproot_asset_types.rs** - Implementation file

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::layer2;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test layer2::

# Run specific test
cargo test layer2::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
