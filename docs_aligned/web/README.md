---
title: "web Module"
description: "Core functionality"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# web Module

## Overview

Core functionality

This module contains 5 Rust source files implementing core functionality for the Anya Core system.

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
pub mod web5_adapter;
```

## Components

The following files implement this module:

- **did_handler.rs** - ...existing code...
- **mod.rs** - Implementation file
- **psbt_web5.rs** - ... existing code ...
- **web5_adapter_new.rs** - Web5Adapter: MIT-compliant isolates all web5-rust logic for DID, DWN, and VC operations
- **web5_adapter.rs** - Web5Adapter: MIT-compliant isolates all web5-rust logic for DID, DWN, and VC operations

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::web;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test web::

# Run specific test
cargo test web::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
