---
title: "types Module"
description: "Common types used across the Anya Core ecosystem"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# types Module

## Overview

Common types used across the Anya Core ecosystem

This module contains 2 Rust source files implementing core functionality for the Anya Core system.

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
pub mod compliance;
pub use compliance::{BipSupportLevel, MilestoneStatus, ProtocolCompliance, VerificationStatus};
```

## Components

The following files implement this module:

- **compliance.rs** - Missing types for BIP compliance and protocol validation
- **mod.rs** - Common types used across the Anya Core ecosystem

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::types;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test types::

# Run specific test
cargo test types::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
