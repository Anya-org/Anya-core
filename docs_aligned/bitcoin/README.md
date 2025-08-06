---
title: "bitcoin Module"
description: "[AIR-3][AIS-3][BPC-3][RES-3] Fixed import to use the correct BitcoinAdapter"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# bitcoin Module

## Overview

[AIR-3][AIS-3][BPC-3][RES-3] Fixed import to use the correct BitcoinAdapter

This module contains 78 Rust source files implementing core functionality for the Anya Core system.

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
pub mod adapters;
pub mod bip341;
pub mod compat; // Compatibility module for older import patterns
pub mod config;
pub mod error;
pub mod interface;
pub mod layer2; // Export layer2 module for Layer2Protocol trait
pub mod lightning;
pub mod manager;
pub mod node; // Bitcoin node management
```

## Components

The following files implement this module:

- **mod.rs** - [AIR-3][AIS-3][BPC-3][RES-3] Bitcoin adapters module implementation
- **mod.rs** - Bitcoin Protocol Adapters
- **mod.rs** - Bitcoin RPC Adapter
- **rust.rs** - Migrated from OPSource to anya-core
- **mod.rs** - Bitcoin Storage Adapter
- **bip340.rs** - ! BIP-340 (Schnorr Signatures) Implementation
- **bip341.rs** - ! BIP-341 (Taproot) Implementation
- **anya_bitcoin.rs** - ! Shim module to remap anya_bitcoin imports to anya_core::bitcoin
- **mod.rs** - ! Compatibility module for older import patterns
- **config.rs** - Bitcoin configuration module
- **differential_fuzzer.rs** - Implementation file
- **invariant_checker.rs** - Implementation file
- **mod.rs** - ! Bitcoin Consensus Module
- **bitcoin_verification.rs** - [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
- **mod.rs** - [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
- **atomic_swaps.rs** - src/bitcoin/cross_chain/atomic_swaps.rs
- **bridge.rs** - src/bitcoin/cross_chain/bridge.rs
- **liquid.rs** - Migrated from OPSource to anya-core
- **mod.rs** - Migrated from OPSource to anya-core
- **routing.rs** - src/bitcoin/cross_chain/routing.rs
- **rsk.rs** - Migrated from OPSource to anya-core
- **adaptor_fixed.rs** - Implementation file
- **adaptor.rs** - src/bitcoin/dlc/adaptor.rs
- **batch_verification.rs** - ! DLC Oracle Batch Verification [AIR-3][AIS-3][BPC-3][PFM-3][RES-3]
- **contract.rs** - src/bitcoin/dlc/contract.rs
- **execution.rs** - src/bitcoin/dlc/execution.rs
- **mod.rs** - Migrated from OPSource to anya-core
- **oracle.rs** - src/bitcoin/dlc/oracle.rs
- **error.rs** - [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::error::Error
- **block.rs** - Bitcoin Block Interface Types
- **mod.rs** - Bitcoin Interface Module
- **network.rs** - Bitcoin Network Interface Types
- **transaction.rs** - Bitcoin Transaction Interface Types
- **mod.rs** - ! Layer 2 Bitcoin Protocol Implementations [AIR-3][AIS-3][BPC-3][RES-3]
- **client.rs** - RGB Client implementation
- **contract.rs** - RGB Contract implementation
- **mod_new.rs** - Implementation file
- **mod.rs** - RGB Layer 2 implementation
- **node.rs** - RGB Node implementation
- **schema.rs** - RGB Schema implementation
- **state.rs** - RGB State implementation
- **wallet.rs** - RGB Wallet implementation
- **lightning.rs** - Lightning Network Implementation for Bitcoin Module
- **manager.rs** - Bitcoin Manager Implementation
- **merkle.rs** - [AIS-3][BPC-3] Constant-time Merkle verification
- **mod.rs** - [AIR-3][AIS-3][BPC-3][RES-3] Fixed import to use the correct BitcoinAdapter
- **node.rs** - [AIR-3][AIS-3][BPC-3][AIT-3] Bitcoin Node Implementation
- **address.rs** - Bitcoin Address Utilities Module
- **mod.rs** - ! Bitcoin Protocol Implementation [AIR-3][AIS-3][BPC-3][AIT-3]
- **script.rs** - Bitcoin Script Execution Module
- **testing.rs** - ! Bitcoin Protocol Testing Module [AIR-3][AIS-3][BPC-3][AIT-3]
- **validation.rs** - Bitcoin Transaction Validation Module
- **psbt.rs** - [BPC-3][BIP-370] PSBTv2 Implementation
- **mod.rs** - [AIR-3][AIS-3][BPC-3][RES-3]
- **bridge.rs** - Implementation file
- **client.rs** - src/bitcoin/sidechains/liquid/client.rs
- **mod.rs** - src/bitcoin/sidechains/liquid/mod.rs
- **mod.rs** - Migrated from OPSource to anya-core
- **adaptor.rs** - src/bitcoin/dlc/adaptor.rs
- **bitcoin_verification.rs** - [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
- **bridge.rs** - src/bitcoin/sidechains/rsk/bridge.rs
- **client.rs** - src/bitcoin/sidechains/rsk/client.rs
- **mod.rs** - src/bitcoin/sidechains/liquid/mod.rs
- **oracle.rs** - src/bitcoin/dlc/oracle.rs
- **spv.rs** - ! Bitcoin SPV (Simplified Payment Verification) Implementation
- **mod.rs** - ! Taproot implementation for Bitcoin
- **script.rs** - src/bitcoin/taproot/script.rs
- **tests.rs** - Implementation file
- **tree.rs** - src/bitcoin/taproot/tree.rs
- **bip341_tests.rs** - ! Tests for the BIP-341 (Taproot) implementation
- **mod.rs** - ! Bitcoin module tests
- **validation.rs** - ! Bitcoin transaction validation [AIS-3][BPC-3][DAO-3][PFM-3]
- **bip32.rs** - [AIR-3][AIS-3][BPC-3][AIT-3] BIP32 HD Wallet Implementation
- **bip370.rs** - ! BIP-370 Implementation for improved PSBT handling [BPC-3]
- **coin_selection.rs** - Coin selection algorithms for Bitcoin wallet
- **implementation.rs** - Bitcoin wallet implementation details
- **mod.rs** - Bitcoin Wallet Module
- **transactions.rs** - / Options for transaction creation

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::bitcoin;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test bitcoin::

# Run specific test
cargo test bitcoin::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
