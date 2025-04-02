# Anya Core Consolidated Modules Guide

## Overview

This document provides guidelines for using the consolidated modules created during the Anya Core reorganization. These modules reduce code duplication, enhance maintainability, and ensure consistent implementation of critical features across the codebase.

## Consolidated Module Directory

| Module | Path | Replaces | Purpose |
|--------|------|----------|---------|
| **Bitcoin Validation** | `packages/protocol-adapters/src/bitcoin/validation.rs` | Multiple validation implementations | Unified BIP-342 compliant validation |
| **PSBT Handler** | `packages/protocol-adapters/src/bitcoin/psbt.rs` | Multiple PSBT implementations | Consolidated PSBT creation and signing |
| **Metrics Service** | `packages/metrics/src/lib.rs` | Various metrics collectors | Unified metrics collection system |
| **MCP Server** | `packages/mcp-interface/src/http.rs` | Legacy HTTP handlers | Fixed MCP server implementation |

## Using the Consolidated Modules

### Bitcoin Validation Module

The validation module provides a unified approach to Bitcoin transaction and block validation with full BIP-342 support.

```rust
use anya_core_protocol_adapters::bitcoin::validation::BitcoinValidator;
use bitcoin::Network;

// Create a validator with BIP-342 support
let validator = BitcoinValidator::new(Network::Bitcoin, true);

// Validate a transaction
let result = validator.validate_transaction(&tx, None)?;

// Validate a block
let result = validator.validate_block(&block)?;
```

### PSBT Handler

The PSBT Handler consolidates all PSBT-related functionality into a single module, supporting both legacy and BIP-342 (Taproot) PSBTs.

```rust
use anya_core_protocol_adapters::bitcoin::psbt::PsbtHandler;
use bitcoin::Network;

// Create a PSBT handler with Tapscript support
let handler = PsbtHandler::new(Network::Bitcoin, true);

// Create and sign a PSBT
let mut psbt = handler.create_psbt(inputs, outputs)?;
handler.sign_psbt(&mut psbt, &private_key, None)?;
handler.finalize_psbt(&mut psbt)?;

// Extract the final transaction
let transaction = handler.extract_tx(&psbt)?;
```

### Metrics Service

The consolidated metrics service provides specialized collectors for different components:

```rust
use anya_core_metrics::{MetricsConfig, MetricsService};
use anya_core_metrics::bitcoin as bitcoin_metrics;
use anya_core_metrics::mcp as mcp_metrics;
use anya_core_metrics::validation as validation_metrics;
use anya_core_metrics::system as system_metrics;

// Create a metrics service
let config = MetricsConfig::default();
let mut metrics = MetricsService::new(config)?;
metrics.start()?;

// Record Bitcoin-specific metrics
bitcoin_metrics::record_network_metrics(&metrics, 10, 5, 1024.0, 512.0);
bitcoin_metrics::record_mempool_metrics(&metrics, 500, 2.5, 1_000_000);

// Record MCP server metrics
mcp_metrics::record_health_metrics(&metrics, "running", 3600);

// Record validation metrics
validation_metrics::record_transaction_validation_metrics(
    &metrics, true, 15.5, 2500
);

// Record system metrics
system_metrics::record_system_metrics(&metrics)?;
```

### MCP Server

The MCP Server implementation fixes the previous issues with improper startup sequences:

```rust
use anya_core_mcp_interface::http::HttpTransport;

// Create an HTTP transport
let transport = HttpTransport::new(8080);

// Start the server (properly handles the startup sequence)
transport.start_server().await?;
```

## MCP Server Startup Sequence

The MCP server implementation follows this critical sequence:

1. First updates health status to "starting"
2. Creates the HTTP transport only once
3. Starts the HTTP server before storing the transport
4. Explicitly handles errors when starting the HTTP server
5. Only stores the transport after successful server start
6. Updates health status to "running" once the server is successfully started

This pattern avoids the previous issue where the server wasn't properly binding and wasn't responding to health checks correctly.

## BIP-342 Support

All consolidated Bitcoin-related modules now support BIP-342 (Tapscript) with these features:

1. Taproot Script Validation
2. Leaf Version Checks 
3. Disabled Opcode Detection
4. Script Size Limits

## Adding New Features

When adding new features to Anya Core, follow these guidelines:

1. Check if your feature fits within an existing consolidated module
2. Enhance the existing module rather than creating a duplicate implementation
3. Maintain strict separation of concerns between packages
4. Write comprehensive tests for all new code
5. Document usage patterns in module-level documentation

## Migration Plan

For existing code using legacy implementations:

1. Identify which consolidated module replaces your current implementation
2. Update imports to reference the new module
3. Adapt your code to the new API (minimal changes should be needed)
4. Run tests to ensure compatibility
5. Remove the legacy implementation once all references are updated

## Testing Consolidated Modules

Each consolidated module has comprehensive tests. When making changes, ensure all tests pass:

```bash
# Test all consolidated modules
cargo test -p anya-core-protocol-adapters -p anya-core-metrics -p anya-core-mcp-interface

# Test specific modules
cargo test -p anya-core-protocol-adapters -- --nocapture bitcoin::validation
cargo test -p anya-core-protocol-adapters -- --nocapture bitcoin::psbt
cargo test -p anya-core-metrics -- --nocapture
cargo test -p anya-core-mcp-interface -- --nocapture http
```
