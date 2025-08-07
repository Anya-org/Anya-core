# bitcoin/layer2 Module

Layer 2 Bitcoin Protocol Implementations [AIR-3][AIS-3][BPC-3][RES-3]

This module contains implementations of various Layer 2 protocols and solutions that can be used with Bitcoin to extend its functionality beyond the base layer.

## Overview

The `bitcoin/layer2` module provides a standardized interface for interacting with Bitcoin Layer 2 solutions and implements several key Layer 2 protocols. Layer 2 solutions are protocols built on top of the Bitcoin blockchain that extend its functionality while inheriting Bitcoin's security model, allowing for improved scalability, privacy, and functionality.

Currently, this module focuses on:

- **RGB Protocol**: A client-side validation protocol for Bitcoin-based assets and smart contracts
- General abstractions for Layer 2 operations through the `Layer2Protocol` trait

## Core Components

### Layer2Protocol Trait

The `Layer2Protocol` trait defines a standard interface for all Layer 2 protocols:

```rust
pub trait Layer2Protocol {
    fn generate_address(&self, address_type: &str) -> BitcoinResult<String>;
    fn create_transaction(&self, outputs: Vec<(String, u64)>) -> BitcoinResult<Transaction>;
    fn verify_merkle_proof(&self, tx_hash: &[u8], block_header: &[u8]) -> BitcoinResult<bool>;
    fn get_transaction(&self, txid: &str) -> BitcoinResult<Transaction>;
    fn get_block(&self, hash: &str) -> BitcoinResult<Vec<u8>>;
    fn broadcast_transaction(&self, tx: &Transaction) -> BitcoinResult<String>;
    fn send_transaction(&self, tx: &Transaction) -> BitcoinResult<String>;
    fn get_block_height(&self) -> BitcoinResult<u64>;
}
```

### RGB Protocol Implementation

The RGB protocol is a client-side validation system for managing complex digital assets on Bitcoin. Key components include:

- **RGBManager**: Primary interface for RGB operations

  ```rust
  #[async_trait::async_trait]
  pub trait RGBManager: Send + Sync {
      async fn create_asset(&self, params: AssetCreationParams) -> AnyaResult<RGBAsset>;
      async fn transfer_asset(&self, transfer: AssetTransfer) -> AnyaResult<TransferStatus>;
      async fn get_asset(&self, asset_id: &str) -> AnyaResult<Option<RGBAsset>>;
      // ... additional methods
  }
  ```

- **RGBAsset**: Represents a digital asset on the RGB protocol
- **AssetTransfer**: Defines parameters for transferring RGB assets
- **RGBFactory**: Creates and configures RGB manager instances

## Usage Examples

### Creating an RGB Asset

```rust
use crate::bitcoin::layer2::{AssetCreationParams, RGBFactory};
use std::collections::HashMap;

async fn create_custom_token() -> AnyaResult<()> {
    // Create an RGB manager with default configuration
    let rgb_manager = RGBFactory::default_manager();

    // Define asset creation parameters
    let mut metadata = HashMap::new();
    metadata.insert("website".to_string(), "https://example.com".to_string());

    let params = AssetCreationParams {
        name: "Example Token".to_string(),
        description: Some("An example RGB token".to_string()),
        total_supply: 1_000_000,
        precision: 8,
        metadata,
        schema_id: "rgb:20".to_string(), // RGB-20 for fungible assets
        issuer: "Example Issuer".to_string(),
    };

    // Create the asset
    let asset = rgb_manager.create_asset(params).await?;
    println!("Asset created with ID: {}", asset.id);

    Ok(())
}
```

### Transferring an RGB Asset

```rust
use crate::bitcoin::layer2::{AssetTransfer, RGBFactory};

async fn transfer_asset(asset_id: &str, recipient: &str, amount: u64) -> AnyaResult<()> {
    let rgb_manager = RGBFactory::default_manager();

    let transfer = AssetTransfer {
        asset_id: asset_id.to_string(),
        amount,
        recipient: recipient.to_string(),
        change_address: None,
        fee_rate: 1, // sat/vB
        tx_options: None,
    };

    let status = rgb_manager.transfer_asset(transfer).await?;
    println!("Transfer status: {:?}", status);

    Ok(())
}
```

## Error Handling

The layer2 module uses the standard `AnyaResult` type for error handling. Common errors include:

- Network connectivity issues
- Invalid parameters
- Insufficient funds
- Protocol-specific validation errors

Example of proper error handling:

```rust
async fn handle_rgb_operations() {
    let rgb_manager = RGBFactory::default_manager();

    match rgb_manager.get_asset("some_asset_id").await {
        Ok(Some(asset)) => {
            println!("Found asset: {}", asset.name);
        },
        Ok(None) => {
            println!("Asset not found");
        },
        Err(e) => {
            eprintln!("Error retrieving asset: {}", e);
            // Implement appropriate recovery or fallback logic
        }
    }
}
```

## Testing

To test the layer2 implementations, use the provided test utilities:

```bash
# Run all layer2 tests
cargo test --package anya-core --lib bitcoin::layer2

# Test only RGB implementation
cargo test --package anya-core --lib bitcoin::layer2::rgb
```

## Security Considerations

When working with Layer 2 protocols:

1. Always validate the cryptographic proofs provided by the protocol
2. Ensure proper backup of state data, as many Layer 2 protocols rely on client-side validation
3. Be aware of the security model of each Layer 2 solution and how it differs from Bitcoin's base layer

## Future Developments

Planned additions to the layer2 module include:

- Lightning Network implementation
- Statechains
- Liquid/Elements sidechains
- DLC (Discreet Log Contracts) support

## For more information

- See the comprehensive documentation in the [docs/](../../../docs/) directory
- RGB protocol specifications: [RGB Protocol GitHub](https://github.com/rgb-org/rgb-core)
- For implementation details, see the individual modules under `src/bitcoin/layer2/`
