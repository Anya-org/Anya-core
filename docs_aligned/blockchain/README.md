---
title: "blockchain Module"
description: "! Blockchain module"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# blockchain Module

## Overview

! Blockchain module

This module contains 3 Rust source files implementing core functionality for the Anya Core system.

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
pub enum BlockchainError {
pub struct BlockchainMetrics {
pub struct BlockchainState {
pub struct PeerInfo {
pub struct MempoolStatus {
pub struct BlockInfo {
pub struct TransactionInfo {
pub trait NodePort {
pub struct UtxoInfo {
pub trait WalletPort {
```

## Components

The following files implement this module:

- **adapter.rs** - ! Bitcoin blockchain adapter implementation
- **adapter_test.rs** - ! Tests for the Bitcoin blockchain adapter
- **mod.rs** - ! Blockchain module

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::blockchain;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test blockchain::

# Run specific test
cargo test blockchain::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
