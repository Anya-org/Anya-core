---
title: "ml Module"
description: "! Machine Learning module"
last_updated: 2025-08-06
---

[AIR-3][AIS-3][BPC-3][RES-3]

# ml Module

## Overview

! Machine Learning module

This module contains 25 Rust source files implementing core functionality for the Anya Core system.

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
pub use crate::dao::{Proposal, ProposalMetrics, RiskMetrics};
pub mod production;
pub use production::{
pub use service::MLModel;
pub mod real_inference;
pub use real_inference::{
pub mod agent_system;
pub use agent_system::MLAgentSystem;
pub mod adapters;
pub use adapters::{AdapterFactory, MLAdapterRegistry, MLModelAdapter};
```

## Components

The following files implement this module:

- **burn_adapter.rs** - ! Burn Framework Adapter
- **candle_adapter.rs** - ! Candle Framework Adapter
- **huggingface_adapter.rs** - ! HuggingFace Model Hub Adapter
- **mod.rs** - ! ML Model Adapters for External Framework Integration
- **ollama_adapter.rs** - ! Ollama Local LLM Adapter
- **torch_adapter.rs** - ! PyTorch Framework Adapter
- **agent_checker.rs** - [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::error::Error
- **agent.rs** - ML Agent System implementation
- **communication.rs** - ! Enhanced Agent Communication System
- **dao_agent.rs** - DAO Agent - Machine Learning Governance Agent for Anya Core
- **federated_agent.rs** - Federated Learning Agent
- **mod.rs** - Machine Learning Agents Module
- **system_map.rs** - System Map and Index for Agent Operations
- **web5_agent.rs** - Implementation file
- **agent_system.rs** - ML Agent System Implementation
- **management.rs** - Implementation file
- **models.rs** - Models module stub for ML agents
- **mod.rs** - ! Machine Learning module
- **mod.rs** - ! Advanced Agent Orchestration System
- **workflow_builder.rs** - ! Workflow Builder
- **mod.rs** - ! Planning & Reasoning Engine
- **production.rs** - ! Production ML Service Implementation
- **real_inference.rs** - ! Real ML Inference Engine
- **service.rs** - Machine Learning Service Implementation
- **mod.rs** - ! Tool Integration Framework

## API

Detailed API documentation is generated from source:

```bash
# View API docs for this module
cargo doc --open --package anya-core
```

## Examples

### Basic Usage

```rust
use anya_core::ml;

// Example usage (update based on actual API)
fn main() {
    // Implementation examples will be added
}
```

## Testing

Run tests for this module:

```bash
# Run all tests
cargo test ml::

# Run specific test
cargo test ml::test_name
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Architecture Guide](../architecture/README.md)

*Last updated: 2025-08-06*
