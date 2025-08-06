# Storage Module Documentation

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

[AIS-3]: #ais-3 "Application Integration Standard Level 3"
[RES-3]: #res-3 "Resource Efficiency Standard Level 3"

## Overview

The Storage module provides a unified interface for all storage operations in the Anya Core system, replacing SQLite dependencies with a more flexible and decentralized approach. This module supports various storage implementations including decentralized storage, IPFS, in-memory storage, and persistent disk storage.

## Core Interfaces

### KeyValueStorage

A fundamental trait for key-value storage implementations, providing basic CRUD operations.

#### Key Operations

- `set`: Store a value for a key
- `get`: Get a value for a key
- `delete`: Delete a key and its value
- `exists`: Check if a key exists
- `list_keys`: List keys with a prefix

#### Usage Example

```rust
use anya_core::storage::KeyValueStorage;

async fn store_data<S: KeyValueStorage>(storage: &S) -> anyhow::Result<()> {
    // Store a value
    storage.set("user:1", r#"{"name":"Alice","balance":100}"#).await?;

    // Check if key exists
    if storage.exists("user:1").await? {
        // Retrieve the value
        let data = storage.get("user:1").await?;

        // List all user keys
        let all_users = storage.list_keys("user:").await?;
    }

    Ok(())
}
```

### UnifiedStorage

A comprehensive trait that provides high-level storage operations for the Anya Core system.

#### Key Operations

- Asset management
- Transaction recording
- RGB schema operations
- Bitcoin anchor services
- Metadata operations

## Storage Implementations

### DecentralizedStorage

Implementation for decentralized storage networks with content addressing.

#### Key Features

- Content-addressed storage
- Asset transfer history
- Bitcoin proofs
- RGB asset management

### IPFSStorage

InterPlanetary File System (IPFS) integration for distributed content storage.

#### Key Features

- Content encryption
- Batch operations
- File metadata
- Pin management

### PersistentStorage

Durable on-disk storage solution for long-term data retention.

#### Key Features

- Asset records
- Configurable storage options
- Performance metrics
- Transaction durability

### MemoryStorage

In-memory implementation for testing and ephemeral data.

## Data Structures

The Storage module defines several important data structures:

- **RGBAsset**: Represents a digital asset on the RGB protocol
- **BitcoinProof**: Cryptographic proof anchored to the Bitcoin blockchain
- **AssetTransfer**: Record of an asset transfer between parties
- **ContentId**: Unique identifier for content in decentralized storage
- **IPFSFileMetadata**: Metadata for files stored in IPFS

## Integration Points

The Storage module integrates with:

- **Layer2 Module**: For RGB protocol operations
- **Bitcoin Module**: For blockchain anchoring
- **Network Module**: For P2P data exchange
- **Security Module**: For encryption and access control

## Compliance Standards

### AIR-3

Availability & Integrity Requirement Level 3: The Storage module ensures high availability and data integrity through redundant storage options, cryptographic verification, and robust error handling.

### AIS-3

Application Integration Standard Level 3: Provides comprehensive APIs for seamless integration with other Anya Core components and external storage systems.

### BPC-3

Bitcoin Protocol Compatibility Level 3: Implements Bitcoin-compatible storage solutions, particularly for anchoring data to the Bitcoin blockchain and handling RGB assets.

### RES-3

Resource Efficiency Standard Level 3: Optimized for efficient storage operations with minimal resource overhead, including batched operations and efficient indexing.
