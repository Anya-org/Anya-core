# Source of Truth Registry - v1.3 Updates

## Overview

The Source of Truth Registry has been enhanced in v1.3 with improved blockchain anchoring capabilities, Taproot integration, and better Web5 compatibility. These enhancements provide stronger guarantees for data integrity, immutability, and verifiability.

## New Features

### 1. Enhanced Blockchain Anchoring

The registry now supports advanced blockchain anchoring features:

- **Status Tracking**: Each anchor now has a well-defined lifecycle (Created → Broadcast → Confirmed → Final)
- **Network Configuration**: Explicit network selection (mainnet, testnet, signet, regtest)
- **Confirmation Management**: Tracking of confirmation counts with configurable thresholds
- **State Synchronization**: The `sync_with_blockchain()` method automatically updates anchor statuses

### 2. Documentation Duplication Detection

Advanced documentation duplication detection capabilities:

- **Content Similarity**: Detect similar documentation content across the codebase
- **Normalized Comparison**: Compare normalized content to find similar documents with different formatting
- **Section-Level Analysis**: Analyze individual sections within documentation files
- **Repository Scanning**: Scan the entire repository for duplicated documentation
- **Similarity Metrics**: Calculate similarity scores to identify potential duplication

### 3. Taproot Integration

Leveraging Bitcoin's Taproot capabilities:

- **Taproot Anchoring**: Create anchors using Taproot outputs with the `create_taproot_anchor()` method
- **Script Trees**: Support for complex script trees to encode registry data
- **Enhanced Privacy**: Better privacy guarantees through Taproot's inherent properties
- **Feature-Flagged**: Available when the `taproot` feature is enabled

### 4. Web5 Integration

Seamless integration with Web5 standards:

- **DWN Anchoring**: Anchor registry data to Web5 Decentralized Web Nodes
- **Verification**: Verify registry integrity through Web5's decentralized infrastructure
- **Feature-Flagged**: Available when the `web5` feature is enabled

## API Changes

### New Types

```rust
/// Status of a blockchain anchor
pub enum AnchorStatus {
    Created,
    Broadcast,
    Confirmed(u32), // Number of confirmations
    Final,
    Failed(String), // Reason for failure
}

/// Taproot-specific anchoring data
pub struct TaprootAnchorData {
    pub output_script: Vec<u8>,
    pub internal_key: Vec<u8>,
    pub script_tree_hashes: Vec<[u8; 32]>,
    pub control_block: Option<Vec<u8>>,
}

/// Enhanced documentation entry for duplication detection
pub struct DocumentationEntry {
    pub content_hash: [u8; 32],
    pub normalized_hash: [u8; 32],
    pub title: String,
    pub file_path: String,
    pub section: String,
    pub word_count: usize,
    pub similarity_score: Option<f32>,
    pub similar_to: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}
```

### New Methods

- `create_taproot_anchor()`: Create a Taproot-specific anchor
- `mark_anchor_as_broadcast()`: Update anchor status to broadcast
- `mark_anchor_as_failed()`: Mark an anchor as failed with reason
- `sync_with_blockchain()`: Synchronize registry with blockchain state
- `set_bitcoin_network()`: Configure which Bitcoin network to use
- `get_bitcoin_network()`: Get the current network configuration
- `check_documentation_duplication()`: Check for duplicated documentation content
- `normalize_documentation_content()`: Normalize documentation for comparison
- `scan_repo_for_documentation_duplication()`: Scan entire repository for duplicate docs

### Web5 Integration Module

```rust
#[cfg(feature = "web5")]
pub mod web5_anchoring {
    pub async fn anchor_registry_via_web5(...) -> Result<String, Box<dyn Error>>;
    pub async fn verify_registry_via_web5(...) -> Result<bool, Box<dyn Error>>;
}
```

## Usage Examples

### Basic Anchoring

```rust
// Initialize registry
let registry = SourceOfTruthRegistry::new("registry.json").await?;

// Enable anchoring and set network
registry.enable_blockchain_anchoring();
registry.set_bitcoin_network("testnet")?;

// Create an anchor
let anchor = registry.anchor_to_blockchain().await?;
println!("Created anchor with txid: {}", anchor.txid);

// Later, synchronize with blockchain
let updated_txids = registry.sync_with_blockchain().await?;
```

### Taproot Anchoring

```rust
// Generate keys and script tree (in real implementation)
let internal_key = vec![/* ... */];
let script_tree_hashes = vec![/* ... */];

// Create Taproot anchor
let anchor = registry.create_taproot_anchor(
    internal_key,
    script_tree_hashes,
    "testnet"
).await?;
```

### Web5 Integration

```rust
// Anchor via Web5
let record_id = web5_anchoring::anchor_registry_via_web5(&registry).await?;

// Verify via Web5
let is_valid = web5_anchoring::verify_registry_via_web5(&registry, &record_id).await?;
```

## Implementation Notes

- All blockchain operations are properly error-handled and include logging
- Network settings default to testnet for safety
- Real implementations would connect to an actual Bitcoin node
- Web5 integration is conditionally compiled only when the feature is enabled
