---
title: "dao Module"
description: "! DAO module"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# dao Module

## Overview

! DAO module

This module contains 12 Rust source files implementing core functionality for the Anya Core system.

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
pub mod compat;
pub mod governance;
pub mod legal;
pub mod voting;
pub use governance::DaoLevel;
pub use governance::{DaoGovernance, ProposalStatus};
pub struct Proposal {
pub struct ProposalMetrics {
pub struct RiskMetrics {
pub struct DAOConfig {
```

## Components

The following files implement this module:

- **clarinet.rs** - ! Mock implementation of the clarinet module for test compatibility
- **mod.rs** - ! Clarity REPL compatibility module
- **mod.rs** - ! Stacks REPL compatibility layer
- **mod.rs** - ! Stacks VM types compatibility layer
- **mod.rs** - ! Compatibility modules for different protocols
- **governance.rs** - / DAO-3 Compliance Check
- **legal.rs** - / Legal wrapper integration for DAO-4 [AIS-3][BPC-3][DAO-3]
- **mod.rs** - ! DAO module
- **mod.rs** - ! DAO module tests
- **token_contract.rs** - Anya Governance Token (AGT) Contract Implementation
- **types.rs** - ! DAO Types
- **voting.rs** - / Quadratic Voting Implementation [AIS-3][BPC-3][DAO-3]

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::dao;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test dao::

# Run specific test
cargo test dao::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
