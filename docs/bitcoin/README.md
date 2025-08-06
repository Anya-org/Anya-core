# Bitcoin Module Documentation

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

[AIS-3]: #ais-3 "Application Integration Standard Level 3"
[RES-3]: #res-3 "Resource Efficiency Standard Level 3"

## Overview

The Bitcoin module provides core Bitcoin functionality for the Anya Core system, implementing official Bitcoin Improvement Proposals (BIPs) and supporting hexagonal architecture for modularity and extensibility.

## Core Components

### Adapters

Implements the `BitcoinAdapter` interface for connecting to various Bitcoin backends (Bitcoin Core, Electrum, Esplora, etc.).

### Protocol Compliance

Implements the `BitcoinProtocol` trait and BPCLevel for strict protocol compliance and versioning.

### Node Management

Implements the `BitcoinNode` interface for managing Bitcoin nodes, including connection, synchronization, and health monitoring.

### Wallet Management

Implements the `BitcoinWallet` interface for wallet operations, including address management, transaction creation, and signing.

### Layer2 Support

Exports the `Layer2Protocol` trait for integration with Layer2 solutions (Lightning, RGB, etc.).

### Taproot & Validation

Implements Taproot support and consolidated validation logic for transaction and block verification.

### Configuration

Provides the `BitcoinConfig` struct for configuring network, wallet, and node parameters.

### Compatibility

Includes compatibility modules for legacy import patterns and test utilities.

## Usage Example

```rust
use anya_core::bitcoin::{BitcoinAdapter, BitcoinConfig, BitcoinWallet};

let config = BitcoinConfig::default();
let adapter = BitcoinAdapter::new(config)?;
let wallet = BitcoinWallet::new(&adapter)?;
let address = wallet.get_new_address()?;
let txid = wallet.send_to_address(&address, 100_000)?;
```

## Integration Points

- **Layer2 Module**: For advanced protocol support
- **Lightning Module**: For Lightning Network operations
- **Validation Module**: For transaction and block validation
- **Performance Module**: For monitoring Bitcoin operations

## Compliance Standards

### AIR-3

Ensures high availability and integrity by following best practices for node management, wallet operations, and protocol compliance.

### AIS-3

Comprehensive APIs for integration with other modules and external Bitcoin services.

### BPC-3

Implements Bitcoin protocol features and BIPs for full compatibility.

### RES-3

Efficient transaction processing, resource management, and optimized network operations.
