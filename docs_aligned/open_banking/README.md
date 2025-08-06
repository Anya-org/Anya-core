---
title: "open_banking Module"
description: "Core functionality"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# open_banking Module

## Overview

Core functionality

This module contains 4 Rust source files implementing core functionality for the Anya Core system.

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
pub struct OpenBankingEngine {
```

## Components

The following files implement this module:

- **compliance.rs** - Implementation file
- **enterprise_graph.rs** - Implementation file
- **enterprise.rs** - Enterprise Banking Module
- **mod.rs** - Implementation file

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::open_banking;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test open_banking::

# Run specific test
cargo test open_banking::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
