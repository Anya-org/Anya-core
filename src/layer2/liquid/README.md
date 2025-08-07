# Layer2: Liquid Module

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

## Introduction

The Liquid module implements Blockstream's Liquid Network sidechain, providing fast settlement, confidential transactions, and asset issuance functionality. Liquid is a federated sidechain secured by a group of functionaries who collectively manage the two-way peg with Bitcoin.

## Features

- Full Liquid Network protocol implementation
- Confidential transactions for privacy
- Asset issuance and management
- Fast block settlement (1 minute blocks)
- Two-way Bitcoin pegging (peg-in/peg-out)
- Federation monitoring and management
- Atomic swaps between assets

## Core Components

### LiquidAssetType

Types of assets on the Liquid Network:

```rust
pub enum LiquidAssetType {
    Bitcoin,       // L-BTC (Liquid Bitcoin)
    IssuedAsset,   // Custom issued assets
    Reissuable,    // Reissuable tokens
    NonReissuable, // Non-reissuable tokens
}
```

### LiquidConfig

Configuration settings for Liquid Network:

```rust
pub struct LiquidConfig {
    pub network: String,
    pub node_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub wallet_name: String,
    pub enable_confidential_transactions: bool,
    pub block_time_seconds: u32,
    pub asset_registry_url: Option<String>,
    // Enhanced federation features
    pub federation_endpoint: Option<String>,
    pub federation_id: Option<String>,
    pub api_key: Option<String>,
    pub enable_coinjoin: bool,
    pub min_confirmations: u32,
    pub max_transaction_size: usize,
    pub fee_rate: f64,
}
```

### LiquidAsset

Representation of assets on the Liquid Network:

```rust
pub struct LiquidAsset {
    pub asset_id: String,
    pub asset_type: LiquidAssetType,
    pub name: String,
    pub ticker: Option<String>,
    pub precision: u8,
    pub total_supply: Option<u64>, // None for reissuable assets
    pub issuer_pubkey: String,
    pub domain: Option<String>,
    pub contract_hash: Option<String>,
    pub created_at: u64,
    pub metadata: HashMap<String, String>,
}
```

### LiquidTransaction

Detailed transaction information:

```rust
pub struct LiquidTransaction {
    pub tx_id: String,
    pub block_height: Option<u64>,
    pub confirmations: u32,
    pub inputs: Vec<LiquidInput>,
    pub outputs: Vec<LiquidOutput>,
    pub fee: u64,
    pub size: u32,
    pub weight: u32,
    pub confidential: bool,
}
```

## Usage Examples

### Connecting to Liquid Network

```rust
use anya::layer2::liquid::{LiquidConfig, LiquidProtocol};
use anya::layer2::Layer2Protocol;

async fn connect_to_liquid() -> Result<(), Box<dyn std::error::Error>> {
    // Configure Liquid connection
    let config = LiquidConfig {
        network: "liquid".to_string(), // mainnet
        node_url: "http://localhost:18884".to_string(),
        rpc_user: "user".to_string(),
        rpc_password: "password".to_string(),
        wallet_name: "my_liquid_wallet".to_string(),
        enable_confidential_transactions: true,
        block_time_seconds: 60,
        asset_registry_url: Some("https://assets.blockstream.info".to_string()),
        federation_endpoint: Some("https://blockstream.info/liquid/api".to_string()),
        federation_id: Some("liquid_federation".to_string()),
        api_key: None,
        enable_coinjoin: false,
        min_confirmations: 2,
        max_transaction_size: 400_000,
        fee_rate: 0.1,
    };

    // Initialize protocol
    let liquid = LiquidProtocol::new(config);
    liquid.initialize().await?;

    // Get network status
    let status = liquid.get_protocol_health().await?;
    println!("Connected to Liquid network");
    println!("Block height: {}", status.block_height);
    println!("Connected peers: {}", status.connected_peers);
    println!("Synced: {}", status.synced);

    Ok(())
}
```

### Issuing a New Asset

```rust
use anya::layer2::liquid::{LiquidProtocol, LiquidAssetType};
use std::collections::HashMap;

async fn issue_asset(
    liquid: &LiquidProtocol
) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare asset metadata
    let mut metadata = HashMap::new();
    metadata.insert("description".to_string(), "Test token for demo".to_string());
    metadata.insert("issuer".to_string(), "Example Company".to_string());
    metadata.insert("url".to_string(), "https://example.com".to_string());

    // Issue a new asset
    let asset = liquid.issue_asset(
        "TestToken",
        Some("TTK"),
        1_000_000_000, // 1 billion units
        8,             // 8 decimal places
        true,          // Reissuable
        false,         // Public (non-confidential issuance)
        &metadata,
    ).await?;

    println!("Asset issued successfully");
    println!("Asset ID: {}", asset.asset_id);
    println!("Total supply: {}", asset.total_supply.unwrap_or(0));

    Ok(())
}
```

### Performing a Peg-In

```rust
use anya::layer2::liquid::LiquidProtocol;

async fn perform_peg_in(
    liquid: &LiquidProtocol,
    amount: u64
) -> Result<(), Box<dyn std::error::Error>> {
    // Generate a peg-in address
    let peg_in = liquid.create_peg_in_address(amount).await?;

    println!("Send Bitcoin to this address: {}", peg_in.bitcoin_address);
    println!("Claim script: {}", peg_in.claim_script);

    // Later, claim the funds on Liquid
    let claim_result = liquid.claim_peg_in(&peg_in.claim_script).await?;
    println!("Peg-in claimed! Transaction ID: {}", claim_result.transaction_id);
    println!("L-BTC amount: {}", claim_result.amount);

    Ok(())
}
```

## Security Considerations

1. **Functionary Trust**: Liquid's security model relies on functionaries. Monitor federation status regularly
2. **Confidential Transaction Verification**: Ensure proper implementation of confidential transaction verification
3. **Asset Validation**: Validate issued assets carefully, as they can have similar tickers and names
4. **Block Finality**: Consider waiting for multiple confirmations for high-value transactions
5. **Peg-in/Peg-out Security**: Follow best practices for two-way peg operations

## Federation Monitoring

This module provides tools to monitor the Liquid federation:

```rust
async fn monitor_federation(liquid: &LiquidProtocol) -> Result<(), Box<dyn std::error::Error>> {
    let federation = liquid.get_federation_status().await?;

    println!("Federation status:");
    println!("Total functionaries: {}", federation.total_functionaries);
    println!("Active functionaries: {}", federation.active_functionaries);
    println!("Required signatures: {}", federation.required_signatures);
    println!("Healthy: {}", federation.is_healthy());

    Ok(())
}
```

## For More Information

- [Liquid Network Documentation](https://docs.blockstream.com/liquid/technical_overview.html)
- [Elements Project](https://elementsproject.org/)
- Project documentation
