# Source of Truth Registry v1.3 Documentation

*Last updated: 2025-08-10*

## Overview

The Source of Truth Registry has been enhanced in v1.3 with improved blockchain anchoring capabilities, Taproot integration, and better Web5 compatibility. These enhancements provide stronger guarantees for data integrity, immutability, and verifiability.

## New Features

### 1. Enhanced Blockchain Anchoring

The registry now supports advanced blockchain anchoring features:

- **Status Tracking**: Each anchor now has a well-defined lifecycle (Created → Broadcast → Confirmed → Final)
- **Network Configuration**: Explicit network selection (mainnet, testnet, signet, regtest)
- **Confirmation Management**: Tracking of confirmation counts with configurable thresholds
- **State Synchronization**: The `sync_with_blockchain()` method automatically updates anchor statuses

### 2. Taproot Integration

Leveraging Bitcoin's Taproot capabilities:

- **Script Trees**: Support for complex spending conditions using Taproot script trees
- **Key Aggregation**: Improved multisignature schemes with key aggregation
- **Privacy Enhancements**: Better privacy with indistinguishable transaction types
- **Enhanced Signing**: Adaptor signatures and script path spending

### 3. Web5 Compatibility

Improved interoperability with Web5 standards:

- **DID Integration**: Direct integration with Decentralized Identifiers
- **Verifiable Credentials**: Support for standard VC formats
- **DWN Compatibility**: Aligned with Decentralized Web Node specifications
- **DWAS Support**: Decentralized Web Application Support integration

## Implementation Details

### Key Components Updated

1. **BlockchainAnchor Structure**
   - Added network identification
   - Added Taproot-specific data support
   - Added status tracking with the AnchorStatus enum

2. **TaprootAnchorData Structure**
   - New structure for Taproot-specific anchoring
   - Supports output scripts, internal keys, and script trees

3. **AnchorStatus Enum**
   - Improved lifecycle tracking: Created → Broadcast → Confirmed → Final
   - Added failure handling with reason reporting

4. **ValidationExtensions**
   - Enhanced validation rules for Taproot outputs
   - Support for validating script paths and key paths

### API Changes

The Registry API has been updated with new methods:

```rust
// Create a Taproot-enabled anchor
pub fn create_taproot_anchor(&self, data: &[u8], script_tree: ScriptTree) -> Result<BlockchainAnchor, Error>;

// Get anchor status
pub fn get_anchor_status(&self, anchor_id: &str) -> Result<AnchorStatus, Error>;

// Synchronize anchors with blockchain state
pub fn sync_with_blockchain(&self) -> Result<SyncReport, Error>;
```

## Implementation Fixes

The following issues have been addressed in the codebase to ensure proper alignment for the v1.3 upgrade:

### 1. Type Mismatch Fixes

- `src/lib.rs`: Fixed type mismatches in function signatures for:
  - `verify_taproot_transaction`: Changed parameter type to accept a single transaction
  - `verify_transaction_batch`: Changed parameter types to match expected usage in `bitcoin/validation.rs`

### 2. Unused Variables Fixed

- `src/handlers/rgb.rs`: Fixed unused `handler` variable by properly utilizing it in the `get_asset_history` function
- `src/mobile/sdk.rs`: Fixed unused `_wallet` and `_destination` variables in `backup_wallet` function
- `src/security/crypto/signature.rs`: Fixed unused `_message` and `_private_key` parameters in `sign` function

### 3. Import Issues Resolved

- Removed unused imports from `src/security/crypto/symmetric.rs`
- Removed unused imports from `src/api/routes.rs`
- Fixed import conflicts in `src/web5/did_resolver.rs`

## Usage Examples

### Creating a Taproot Anchor

```rust
use anya_core::registry::{Registry, TaprootOptions, ScriptTree};

fn create_anchor(registry: &Registry, data: &[u8]) -> Result<(), Error> {
    // Create a simple script tree
    let script_tree = ScriptTree::new()
        .add_leaf("timelock", Script::new_timelock(144))
        .add_leaf("multisig", Script::new_multisig(2, &[pubkey1, pubkey2, pubkey3]));
    
    // Create the anchor with Taproot capabilities
    let anchor = registry.create_taproot_anchor(data, script_tree)?;
    
    // The anchor is now created but not yet broadcast
    assert_eq!(anchor.status, AnchorStatus::Created);
    
    // Broadcast the anchor
    registry.broadcast_anchor(&anchor.id)?;
    
    Ok(())
}
```

### Checking Anchor Status

```rust
use anya_core::registry::{Registry, AnchorStatus};

fn check_status(registry: &Registry, anchor_id: &str) -> Result<(), Error> {
    // Get current status
    let status = registry.get_anchor_status(anchor_id)?;
    
    match status {
        AnchorStatus::Created => println!("Anchor created but not broadcast"),
        AnchorStatus::Broadcast => println!("Anchor broadcast but not confirmed"),
        AnchorStatus::Confirmed(confirmations) => {
            println!("Anchor confirmed with {} confirmations", confirmations)
        },
        AnchorStatus::Final => println!("Anchor finalized"),
        AnchorStatus::Failed(reason) => println!("Anchor failed: {}", reason),
    }
    
    Ok(())
}
```

## Related Documentation

- [Integration Guide](/docs/v1.3/integration_guide.md)
- [API Reference](/docs/v1.3/api_reference.md)
- [Migration from v1.2](/docs/v1.3/migration_guide.md)

*This documentation supersedes the previous v1.3 documentation files.*
