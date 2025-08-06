---
title: "tools Module"
description: "! Tools Module [AIR-3][AIS-3][BPC-3][AIT-3]"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# tools Module

## Overview

! Tools Module [AIR-3][AIS-3][BPC-3][AIT-3]

This module contains 10 Rust source files implementing core functionality for the Anya Core system.

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
pub mod commit_tracker;
pub mod doc_duplication_scanner;
pub mod doc_duplication_scanner_cli;
pub mod markdown;
pub mod source_of_truth_registry;
pub use commit_tracker::{update_ai_labelling_file, CommitInfo};
pub use markdown::{DocError, DocumentationValidator};
pub use source_of_truth_registry::{
```

## Components

The following files implement this module:

- **commit_tracker.rs** - [AIR-3][AIS-3][BPC-3][AIT-3] AI Commit Tracking Module
- **doc_duplication_scanner_cli.rs** - Implementation file
- **doc_duplication_scanner.rs** - Implementation file
- **markdown.rs** - [AIR-3][AIS-3][BPC-3][AIT-3] Markdown Documentation Validation Module
- **mod.rs** - ! Tools Module [AIR-3][AIS-3][BPC-3][AIT-3]
- **source_of_truth_registry.rs** - Canonical Source of Truth Registry Implementation
- **source_of_truth_registry_tests.rs** - Implementation file
- **mod.rs** - Integration tests for Source of Truth Registry
- **test_cli.rs** - Implementation file
- **test_doc_duplication.rs** - Implementation file

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::tools;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test tools::

# Run specific test
cargo test tools::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
