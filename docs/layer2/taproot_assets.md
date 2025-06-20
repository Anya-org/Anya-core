---
title: "Taproot Assets Layer2 Documentation"
description: "Complete guide to Taproot Assets protocol integration in Anya Core"
---

# Taproot Assets

## Overview

Taproot Assets (formerly Taro) is a protocol for issuing assets on Bitcoin that leverages Taproot to enable scalable, private, and efficient asset transfers. Built by Lightning Labs, it allows for the creation and transfer of assets on Bitcoin and Lightning Network.

## Features

- **Bitcoin Native**: Assets live directly on Bitcoin using Taproot
- **Lightning Compatible**: Instant asset transfers over Lightning Network
- **Scalable**: Efficient use of Bitcoin block space
- **Private**: Leverages Taproot privacy features
- **Programmable**: Smart contract capabilities through Tapscript

## Configuration

### Basic Configuration

```rust
use anya_core::layer2::taproot_assets::{TaprootAssetsConfig, TaprootAssetsClient};

let config = TaprootAssetsConfig {
    network: "mainnet".to_string(),
    lnd_host: "localhost:10009".to_string(),
    tapd_host: "localhost:10029".to_string(),
    tls_cert_path: "/path/to/tls.cert".to_string(),
    macaroon_path: "/path/to/admin.macaroon".to_string(),
    timeout_ms: 30000,
};

let client = TaprootAssetsClient::new(config);
```

### Environment Variables

```bash
TAPROOT_ASSETS_NETWORK=mainnet
TAPROOT_ASSETS_LND_HOST=localhost:10009
TAPROOT_ASSETS_TAPD_HOST=localhost:10029
TAPROOT_ASSETS_TLS_CERT_PATH=/path/to/tls.cert
TAPROOT_ASSETS_MACAROON_PATH=/path/to/admin.macaroon
TAPROOT_ASSETS_TIMEOUT_MS=30000
```

## Usage Examples

### Asset Creation

```rust
use anya_core::layer2::{AssetParams, AssetTransfer};

// Create a new Taproot asset
let asset_params = AssetParams {
    asset_id: "my_asset".to_string(),
    name: "My Taproot Asset".to_string(),
    symbol: "MTA".to_string(),
    precision: 8,
    total_supply: 21_000_000,
    description: "A sample Taproot asset".to_string(),
};

let result = client.create_asset(asset_params).await?;
println!("Asset created: {:?}", result);
```

### Asset Transfers

```rust
// Send Taproot assets over Lightning
let transfer = AssetTransfer {
    from: "source_address".to_string(),
    to: "destination_address".to_string(),
    amount: 1000000, // amount in asset units
    asset_id: "my_asset".to_string(),
    memo: Some("Lightning asset transfer".to_string()),
};

let result = client.transfer_asset(transfer).await?;
println!("Asset transfer: {:?}", result);
```

### Lightning Integration

```rust
// Create Lightning invoice for Taproot assets
let invoice_result = client.create_asset_invoice(
    "my_asset".to_string(),
    1000000, // amount
    "Payment for services".to_string(), // memo
    3600, // expiry in seconds
).await?;

println!("Asset invoice: {:?}", invoice_result);

// Pay Lightning invoice with Taproot assets
let payment_result = client.pay_asset_invoice(
    "lntb1...".to_string(), // Lightning invoice
    "my_asset".to_string(),
).await?;
```

### Asset Discovery

```rust
// List all available assets
let assets = client.list_assets().await?;
for asset in assets {
    println!("Asset: {} ({}) - Balance: {}", 
             asset.name, asset.symbol, asset.balance);
}

// Get asset details
let asset_info = client.get_asset_info("my_asset".to_string()).await?;
println!("Asset info: {:?}", asset_info);
```

## API Reference

### TaprootAssetsClient

#### Methods

- `new(config: TaprootAssetsConfig) -> Self`
- `connect() -> Result<(), Layer2Error>`
- `disconnect() -> Result<(), Layer2Error>`
- `get_state() -> Result<ProtocolState, Layer2Error>`
- `create_asset(params: AssetParams) -> Result<TransferResult, Layer2Error>`
- `transfer_asset(transfer: AssetTransfer) -> Result<TransferResult, Layer2Error>`
- `list_assets() -> Result<Vec<AssetInfo>, Layer2Error>`
- `get_asset_info(asset_id: String) -> Result<AssetInfo, Layer2Error>`
- `create_asset_invoice(asset_id: String, amount: u64, memo: String, expiry: u64) -> Result<InvoiceResult, Layer2Error>`
- `pay_asset_invoice(invoice: String, asset_id: String) -> Result<PaymentResult, Layer2Error>`
- `verify_proof(proof: Proof) -> Result<VerificationResult, Layer2Error>`
- `validate_transaction(tx_id: String) -> Result<ValidationResult, Layer2Error>`

### Configuration Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `network` | String | Network type (mainnet/testnet) | "mainnet" |
| `lnd_host` | String | LND gRPC host | "localhost:10009" |
| `tapd_host` | String | Taproot Assets daemon host | "localhost:10029" |
| `tls_cert_path` | String | Path to TLS certificate | "/path/to/tls.cert" |
| `macaroon_path` | String | Path to authentication macaroon | "/path/to/admin.macaroon" |
| `timeout_ms` | u64 | Request timeout in milliseconds | 30000 |

### Asset Types

#### AssetInfo

```rust
pub struct AssetInfo {
    pub asset_id: String,
    pub name: String,
    pub symbol: String,
    pub precision: u8,
    pub total_supply: u64,
    pub balance: u64,
    pub genesis_point: String,
    pub asset_type: AssetType,
}
```

#### InvoiceResult

```rust
pub struct InvoiceResult {
    pub payment_request: String,
    pub payment_hash: String,
    pub asset_id: String,
    pub amount: u64,
    pub expiry: u64,
}
```

#### PaymentResult

```rust
pub struct PaymentResult {
    pub payment_hash: String,
    pub payment_preimage: String,
    pub status: PaymentStatus,
    pub fee_sat: u64,
    pub asset_id: String,
    pub amount: u64,
}
```

## Taproot Assets Protocol

### Asset Issuance

1. **Genesis Output**: Create genesis UTXO with asset metadata
2. **Merkle Tree**: Construct Merkle tree of asset commitments
3. **Taproot Script**: Embed asset commitments in Taproot script
4. **Bitcoin Transaction**: Publish to Bitcoin blockchain

### Asset Transfers

1. **Asset Inputs**: Reference previous asset UTXOs
2. **Transfer Logic**: Define transfer amounts and recipients
3. **Witness Data**: Include asset proofs in transaction witness
4. **Settlement**: Settle on Bitcoin or Lightning Network

### Lightning Integration

1. **Channel Funding**: Fund Lightning channels with Taproot assets
2. **HTLC Extensions**: Extend HTLCs for multi-asset payments
3. **Atomic Swaps**: Enable atomic swaps between different assets
4. **Routing**: Route asset payments through Lightning Network

## Security Considerations

### Taproot Security

- **Schnorr Signatures**: Uses Schnorr signatures for efficiency and privacy
- **Script Privacy**: Taproot provides script privacy for asset logic
- **Quantum Resistance**: Preparation for post-quantum cryptography

### Asset Security

- **Proof Verification**: Client-side validation of all asset proofs
- **Double-Spend Prevention**: Bitcoin's UTXO model prevents double-spending
- **Cryptographic Commitments**: Assets secured by cryptographic commitments

### Lightning Security

- **Channel Security**: Lightning channel security applies to asset transfers
- **Routing Privacy**: Onion routing protects payment privacy
- **Atomic Payments**: All-or-nothing payment semantics

## Best Practices

### Development

1. **Test Thoroughly**: Use testnet and regtest for development
2. **Proof Validation**: Always validate asset proofs client-side
3. **Error Handling**: Implement robust error handling for network issues
4. **Backup Management**: Secure backup of asset keys and proofs

### Asset Management

1. **Genesis Security**: Secure the genesis key for asset issuance
2. **Supply Management**: Carefully manage asset supply and issuance
3. **Metadata Standards**: Follow emerging standards for asset metadata
4. **Version Control**: Plan for protocol upgrades and compatibility

### Lightning Integration

1. **Channel Management**: Properly manage Lightning channel states
2. **Fee Management**: Account for Lightning routing fees
3. **Liquidity Planning**: Ensure sufficient channel liquidity
4. **Monitoring**: Monitor channel and payment status

## Troubleshooting

### Common Issues

#### Connection Problems

```rust
// Test connectivity to services
match client.connect().await {
    Ok(_) => println!("Connected to Taproot Assets daemon"),
    Err(e) => println!("Connection failed: {}", e),
}
```

#### Asset Creation Failures

- Verify sufficient Bitcoin for on-chain fees
- Check asset parameters are valid
- Ensure proper permissions for asset creation

#### Transfer Issues

- Confirm sufficient asset balance
- Verify recipient address format
- Check Lightning channel capacity

### Debugging

Enable debug logging:

```bash
RUST_LOG=anya_core::layer2::taproot_assets=debug cargo run
```

### Support Resources

- [Taproot Assets Documentation](https://docs.lightning.engineering/the-lightning-network/taproot-assets)
- [Lightning Labs GitHub](https://github.com/lightninglabs/taproot-assets)
- [Bitcoin Taproot BIP](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)
- [Anya Core Issues](https://github.com/anya-org/anya-core/issues)

## Examples

### Complete Asset Ecosystem

```rust
use anya_core::layer2::taproot_assets::{TaprootAssetsConfig, TaprootAssetsClient};
use anya_core::layer2::{AssetParams, AssetTransfer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let config = TaprootAssetsConfig::default();
    let mut client = TaprootAssetsClient::new(config);
    
    // Connect to services
    client.connect().await?;
    
    // Create a new asset
    let asset_params = AssetParams {
        asset_id: "game_token".to_string(),
        name: "Game Token".to_string(),
        symbol: "GAME".to_string(),
        precision: 2,
        total_supply: 1_000_000,
        description: "Gaming platform utility token".to_string(),
    };
    
    let asset_creation = client.create_asset(asset_params).await?;
    println!("Game token created: {:?}", asset_creation);
    
    // Create Lightning invoice for asset payment
    let invoice = client.create_asset_invoice(
        "game_token".to_string(),
        10000, // 100.00 GAME tokens
        "Premium upgrade".to_string(),
        3600,
    ).await?;
    
    println!("Payment invoice: {}", invoice.payment_request);
    
    // Transfer assets to another user
    let transfer = AssetTransfer {
        from: "player1_address".to_string(),
        to: "player2_address".to_string(),
        amount: 500, // 5.00 GAME tokens
        asset_id: "game_token".to_string(),
        memo: Some("Reward for achievement".to_string()),
    };
    
    let transfer_result = client.transfer_asset(transfer).await?;
    println!("Asset transfer completed: {:?}", transfer_result);
    
    Ok(())
}
```

## Integration Notes

- Compatible with existing Lightning Network infrastructure
- Supports atomic swaps with Bitcoin and other Taproot assets
- Integration with Bitcoin wallets through Taproot script paths
- Compatible with PSBT (Partially Signed Bitcoin Transactions) workflow
