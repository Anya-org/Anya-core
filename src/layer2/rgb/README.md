# Layer2: RGB Module

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

## Introduction

The RGB module implements the RGB protocol, a scalable and confidential smart contract system for Bitcoin and Lightning Network. RGB leverages Bitcoin's security while enabling complex asset issuance and management operations off-chain, with minimal on-chain footprint.

## Features

- Full RGB protocol implementation (v0.3)
- Fungible and non-fungible asset creation and management
- Client-side validation paradigm
- Confidential transactions with zero-knowledge proofs
- Multi-asset support
- UTXO-based ownership model
- Integration with Lightning Network for instant transfers

## Core Components

### RgbAssetSchema

Definition of an RGB asset's schema and rules:

```rust
pub struct RgbAssetSchema {
    pub schema_id: String,
    pub version: String,
    pub asset_type: AssetType,
    pub supply_policy: SupplyPolicy,
    pub decimal_precision: u8,
    pub metadata_schema: Vec<MetadataField>,
    pub rights: AssetRights,
}
```

### AssetType

Types of assets that can be issued on RGB:

```rust
pub enum AssetType {
    Fungible,
    NonFungible,
    UniqueDigitalAsset,
    IdentityAsset,
}
```

### SupplyPolicy

Asset supply management policies:

```rust
pub enum SupplyPolicy {
    Fixed(u64),
    Inflatable { max_supply: Option<u64> },
    Burnable,
    Replaceable,
}
```

### RgbAsset

Representation of an RGB asset:

```rust
pub struct RgbAsset {
    pub asset_id: String,
    pub schema_id: String,
    pub name: String,
    pub ticker: Option<String>,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub decimal_precision: u8,
    pub issuer: String,
    pub genesis_timestamp: u64,
    pub metadata: HashMap<String, String>,
    pub contract_data: Vec<u8>,
    // Additional fields for compatibility
    pub id: String,              // Alias for asset_id
    pub precision: u8,           // Alias for decimal_precision
    pub issued_supply: u64,      // Current issued supply
    pub owner: String,           // Current owner (same as issuer initially)
    pub created_at: u64,         // Creation timestamp
    pub updated_at: Option<u64>, // Last update timestamp
}
```

## Usage Examples

### Issuing a New RGB Asset

```rust
use anya::layer2::rgb::{RgbProtocol, RgbAssetSchema, AssetType, SupplyPolicy, AssetRights};
use anya::layer2::Layer2Protocol;
use std::collections::HashMap;

async fn create_rgb_token() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize RGB protocol
    let rgb = RgbProtocol::new();
    await rgb.initialize().await?;

    // Define asset schema
    let schema = RgbAssetSchema {
        schema_id: "rgb:20".to_string(), // RGB-20 for fungible assets
        version: "0.3".to_string(),
        asset_type: AssetType::Fungible,
        supply_policy: SupplyPolicy::Fixed(1_000_000), // 1 million tokens
        decimal_precision: 8,
        metadata_schema: vec![],
        rights: AssetRights {
            can_burn: false,
            can_replace: false,
            can_rename: false,
            can_issue_more: false,
        },
    };

    // Prepare asset parameters
    let mut metadata = HashMap::new();
    metadata.insert("description".to_string(), "My RGB Token".to_string());

    // Issue the asset
    let asset = rgb.issue_asset(
        "MyToken",
        Some("MTK"),
        1_000_000, // 1 million tokens
        8, // decimal precision
        &metadata,
        &schema,
    ).await?;

    println!("Asset issued: {}", asset.asset_id);
    println!("Total supply: {}", asset.total_supply);

    Ok(())
}
```

### Transferring RGB Assets

```rust
use anya::layer2::rgb::{RgbProtocol, TransferRequest};
use anya::layer2::Layer2Protocol;

async fn transfer_rgb_asset(
    rgb: &RgbProtocol,
    asset_id: &str,
    recipient: &str,
    amount: u64
) -> Result<(), Box<dyn std::error::Error>> {
    // Create transfer request
    let request = TransferRequest {
        asset_id: asset_id.to_string(),
        recipient: recipient.to_string(),
        amount,
        memo: Some("Asset transfer".to_string()),
        expiration: None,
    };

    // Execute transfer
    let result = rgb.transfer_asset(request).await?;

    println!("Transfer ID: {}", result.transaction_id);
    println!("Status: {:?}", result.status);

    // Wait for confirmations
    let final_result = rgb.wait_for_confirmation(&result.transaction_id, 2).await?;
    println!("Transfer confirmed: {}", final_result.is_confirmed());

    Ok(())
}
```

## Integration with Lightning

RGB assets can be transferred through Lightning Network channels:

```rust
async fn rgb_over_lightning(
    rgb: &RgbProtocol,
    asset_id: &str,
    invoice: &str,
    amount: u64
) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare Lightning payment with RGB asset
    let payment = rgb.prepare_lightning_payment(asset_id, invoice, amount).await?;

    // Execute payment
    let result = rgb.send_lightning_payment(payment.payment_id).await?;

    println!("Payment status: {:?}", result.status);
    println!("RGB transfer proof: {}", result.transfer_proof);

    Ok(())
}
```

## Security Considerations

1. **State Storage**: RGB requires secure client-side storage for asset state
2. **Validation Process**: Always validate state transitions with multiple sources
3. **Backup Management**: Regularly back up RGB contract data and proofs
4. **Confidentiality**: While asset transfers are confidential, proper network privacy measures are recommended
5. **Integration Points**: When integrating with Lightning, ensure proper invoice validation

## For More Information

- [RGB Protocol Specification](https://github.com/rgb-org/rgb-spec)
- [Client-Side Validation Paradigm](https://github.com/LNP-BP/client-side-validation)
- Project documentation
