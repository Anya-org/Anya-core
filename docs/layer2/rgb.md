# RGB Assets Protocol

## Overview

RGB is a client-side validation protocol for Bitcoin that enables the creation and transfer of assets on Bitcoin while maintaining privacy and scalability. The Anya Core implementation provides full RGB protocol support with asset issuance, transfers, and validation.

## Features

- **Client-Side Validation**: Validation occurs off-chain for enhanced privacy
- **Bitcoin UTXO-Based**: Uses Bitcoin UTXOs as asset containers
- **Privacy-Preserving**: Asset details are not exposed on the Bitcoin blockchain
- **Scalable**: Unlimited asset types and efficient transfers
- **Smart Contract Support**: Advanced scripting capabilities for asset logic

## Architecture

RGB operates on three layers:

1. **Bitcoin Layer**: UTXO commitments and single-use seals
2. **RGB Layer**: Asset state transitions and validation
3. **Client Layer**: Asset schemas and business logic

## Configuration

```rust
use anya_core::layer2::rgb::{RgbConfig, RgbProtocol};

let config = RgbConfig {
    network: "mainnet".to_string(),
    data_directory: "/path/to/rgb/data".to_string(),
    bitcoin_rpc_url: "http://localhost:8332".to_string(),
    rgb_node_url: "http://localhost:3000".to_string(),
    enable_validation: true,
    cache_size: 1000,
};

let rgb = RgbProtocol::with_config(config);
```

## Usage

### Asset Issuance

```rust
use anya_core::layer2::{AssetParams, Layer2Protocol};
use anya_core::layer2::rgb::RgbProtocol;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rgb = RgbProtocol::new();
    rgb.initialize().await?;
    
    // Define asset parameters
    let asset_params = AssetParams {
        asset_id: "my_unique_asset".to_string(),
        name: "My Digital Asset".to_string(),
        symbol: "MDA".to_string(),
        precision: 8,
        decimals: 8,
        total_supply: 1_000_000,
        metadata: "Custom asset for digital collectibles".to_string(),
    };
    
    // Issue the asset
    let asset_id = rgb.issue_asset(asset_params).await?;
    println!("Asset issued with ID: {}", asset_id);
    
    Ok(())
}
```

### Asset Transfers

```rust
use anya_core::layer2::AssetTransfer;

// Prepare asset transfer
let transfer = AssetTransfer {
    asset_id: "asset_contract_id".to_string(),
    amount: 100,
    from: "sender_outpoint".to_string(),
    to: "receiver_blinding_factor".to_string(),
    recipient: "receiver_public_key".to_string(),
    metadata: Some("RGB asset transfer".to_string()),
};

// Execute transfer
let result = rgb.transfer_asset(transfer).await?;
println!("Transfer successful: {:?}", result);
```

### Asset Validation

```rust
use anya_core::layer2::Proof;

// Create proof for validation
let proof = Proof {
    proof_type: "asset_ownership".to_string(),
    data: proof_data.to_vec(),
    witness: Some(witness_data),
    metadata: HashMap::new(),
};

// Verify the proof
let verification_result = rgb.verify_proof(proof).await?;
if verification_result.is_valid {
    println!("Asset proof is valid");
} else {
    println!("Asset proof is invalid: {:?}", verification_result.error_message);
}
```

## API Reference

### RgbProtocol

The main RGB protocol implementation.

#### Methods

- `new() -> Self`: Create a new RGB protocol instance
- `with_config(config: RgbConfig) -> Self`: Create with custom configuration
- All methods from `Layer2Protocol` trait

### RgbConfig

Configuration for RGB protocol operations.

#### Fields

- `network: String`: Bitcoin network ("mainnet", "testnet", "regtest")
- `data_directory: String`: RGB data storage directory
- `bitcoin_rpc_url: String`: Bitcoin Core RPC endpoint
- `rgb_node_url: String`: RGB node endpoint
- `enable_validation: bool`: Enable client-side validation
- `cache_size: usize`: Asset cache size

### AssetRegistry

Manages RGB asset schemas and contracts.

#### Methods

- `register_schema(&mut self, schema: Schema) -> Result<String, Error>`
- `get_schema(&self, schema_id: &str) -> Option<&Schema>`
- `list_assets(&self) -> Vec<AssetInfo>`

## RGB Schemas

### Fungible Assets (RGB20)

```rust
// RGB20 tokens are fungible assets similar to ERC-20
// Automatically supported through the standard RGB implementation
let fungible_params = AssetParams {
    asset_id: "rgb20_token".to_string(),
    name: "RGB20 Token".to_string(),
    symbol: "R20".to_string(),
    precision: 8,
    decimals: 8,
    total_supply: 21_000_000,
    metadata: "RGB20 fungible token".to_string(),
};
```

### Non-Fungible Tokens (RGB21)

```rust
// RGB21 tokens are non-fungible assets (NFTs)
let nft_params = AssetParams {
    asset_id: "unique_nft_001".to_string(),
    name: "Unique Digital Art".to_string(),
    symbol: "ART".to_string(),
    precision: 0, // NFTs are indivisible
    decimals: 0,
    total_supply: 1, // Single unique token
    metadata: "Unique digital artwork with provenance".to_string(),
};
```

### Custom Schemas (RGB25)

```rust
// RGB25 allows for custom asset schemas with advanced logic
// Implementation depends on specific use case requirements
```

## Security Considerations

### Client-Side Validation

1. **Schema Verification**: Always verify asset schemas before accepting transfers
2. **History Validation**: Validate complete asset history back to issuance
3. **Seal Verification**: Ensure single-use seals are properly consumed

### Private Key Management

1. **Blinding Factors**: Securely manage blinding factors for privacy
2. **Asset Keys**: Use separate keys for asset operations when possible
3. **Backup Procedures**: Implement comprehensive backup strategies

## Best Practices

### Asset Design

1. **Schema Selection**: Choose appropriate schemas for your use case
2. **Supply Management**: Plan total supply and distribution carefully
3. **Metadata Standards**: Use consistent metadata formats

### Performance Optimization

1. **Batch Operations**: Group multiple transfers when possible
2. **Validation Caching**: Cache validation results for repeated checks
3. **UTXO Management**: Optimize UTXO selection for transfers

## Troubleshooting

### Common Issues

1. **Validation Failures**: Check asset history and schema compliance
2. **Transfer Errors**: Verify UTXO availability and asset ownership
3. **Sync Issues**: Ensure Bitcoin node is fully synchronized

### Debugging

```rust
// Enable detailed logging for RGB operations
use log::info;

// The RGB implementation provides comprehensive logging
info!("RGB asset operation completed successfully");
```

## Examples

### Complete Asset Lifecycle

```rust
use anya_core::layer2::{Layer2Protocol, AssetParams, AssetTransfer};
use anya_core::layer2::rgb::RgbProtocol;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rgb = RgbProtocol::new();
    rgb.initialize().await?;
    
    // 1. Issue a new asset
    let asset_params = AssetParams {
        asset_id: "demo_asset".to_string(),
        name: "Demo Asset".to_string(),
        symbol: "DEMO".to_string(),
        precision: 8,
        decimals: 8,
        total_supply: 1000000,
        metadata: "Demo asset for testing".to_string(),
    };
    
    let asset_id = rgb.issue_asset(asset_params).await?;
    println!("Asset issued: {}", asset_id);
    
    // 2. Transfer some assets
    let transfer = AssetTransfer {
        asset_id: asset_id.clone(),
        amount: 1000,
        from: "issuer_outpoint".to_string(),
        to: "receiver_outpoint".to_string(),
        recipient: "receiver_pubkey".to_string(),
        metadata: Some("Initial distribution".to_string()),
    };
    
    let transfer_result = rgb.transfer_asset(transfer).await?;
    println!("Transfer completed: {:?}", transfer_result);
    
    // 3. Verify asset state
    let state = rgb.get_state().await?;
    println!("RGB state: {:?}", state);
    
    Ok(())
}
```

## References

- [RGB Protocol Specification](https://rgb.info/)
- [RGB Standards](https://standards.rgb.info/)
- [RGB Node Documentation](https://github.com/RGB-WG/rgb-node)
- [Bitcoin Layer2 Protocol Documentation](README.md)
