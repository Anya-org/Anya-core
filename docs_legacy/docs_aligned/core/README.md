---
title: "core Module"
description: "Modules"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# core Module

## Overview

Modules

This module contains 7 Rust source files implementing core functionality for the Anya Core system.

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
pub mod metrics;
pub mod performance_optimization;
pub mod reliability;
pub use metrics::PrometheusMetrics;
pub use performance_optimization::{OptimizationStatus, PerformanceOptimizer, ResourceType};
pub use reliability::{
pub struct CoreSystem {
pub mod performance;
pub use performance::Metrics;
pub mod ports {
```

## Components

The following files implement this module:

- **config_management.rs** - / AIR-012: Unified Configuration Management System
- **metrics.rs** - Core metrics implementation using Prometheus
- **mod.rs** - Modules
- **performance_optimization.rs** - [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::error::Error
- **performance.rs** - Core performance module
- **reliability.rs** - ! [AIR-3][AIS-3][BPC-3][RES-3] Reliability and monitoring components for Anya Core
- **system_awareness.rs** - [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::core;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test core::

# Run specific test
cargo test core::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
