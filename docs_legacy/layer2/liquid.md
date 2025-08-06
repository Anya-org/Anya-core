---
title: "Liquid Network Layer2 Documentation"
description: "Complete guide to Liquid Network integration in Anya Core"
---

# Liquid Network

## Overview

The Liquid Network is a federated Bitcoin sidechain that enables confidential transactions, asset issuance, and advanced script capabilities through Elements opcodes. Anya Core provides full integration with Liquid Network for enhanced privacy and functionality.

## Features

- **Confidential Transactions**: Hide transaction amounts while maintaining verifiability
- **Asset Issuance**: Create and manage digital assets on Bitcoin
- **Fast Settlement**: 1-minute block times with federated consensus
- **Advanced Scripts**: Enhanced scripting capabilities beyond Bitcoin Script
- **Two-Way Peg**: Secure movement of Bitcoin between networks

## Configuration

### Basic Configuration

```rust
use anya_core::layer2::liquid::{LiquidConfig, LiquidClient};

let config = LiquidConfig {
    network: "mainnet".to_string(),
    rpc_url: "https://liquid.network/rpc".to_string(),
    confidential: true,
    timeout_ms: 30000,
    federation_pubkeys: vec![
        "02142b5513b2bb94c35310618b6e7c80b08c04b0e3c26ba7e1b306b7f3fecefbfb".to_string(),
        // ... additional federation keys
    ],
    required_signatures: 11,
    elementsd_path: "/usr/local/bin/elementsd".to_string(),
};

let client = LiquidClient::new(config);
```

### Environment Variables

```bash
LIQUID_NETWORK=mainnet
LIQUID_RPC_URL=https://liquid.network/rpc
LIQUID_CONFIDENTIAL=true
LIQUID_TIMEOUT_MS=30000
LIQUID_ELEMENTSD_PATH=/usr/local/bin/elementsd
```

## Usage Examples

### Asset Management

```rust
use anya_core::layer2::{AssetParams, AssetTransfer};

// Create a new asset
let asset_params = AssetParams {
    asset_id: "new_asset".to_string(),
    name: "My Digital Asset".to_string(),
    symbol: "MDA".to_string(),
    precision: 8,
    total_supply: 1_000_000,
    description: "A sample digital asset".to_string(),
};

let result = client.create_asset(asset_params).await?;
println!("Asset created: {:?}", result);
```

### Confidential Transactions

```rust
// Send confidential transaction
let transfer = AssetTransfer {
    from: "sender_address".to_string(),
    to: "receiver_address".to_string(),
    amount: 50000,
    asset_id: "L-BTC".to_string(),
    memo: Some("Confidential payment".to_string()),
};

let result = client.transfer_asset(transfer).await?;
println!("Confidential transfer: {:?}", result);
```

### Peg Operations

```rust
// Peg-in Bitcoin to Liquid
let peg_in_result = client.peg_in(
    "bitcoin_txid".to_string(),
    "liquid_address".to_string(),
    100000, // satoshis
).await?;

// Peg-out Liquid Bitcoin back to Bitcoin
let peg_out_result = client.peg_out(
    "bitcoin_address".to_string(),
    100000, // satoshis
).await?;
```

## API Reference

### LiquidClient

#### Methods

- `new(config: LiquidConfig) -> Self`
- `connect() -> Result<(), Layer2Error>`
- `disconnect() -> Result<(), Layer2Error>`
- `get_state() -> Result<ProtocolState, Layer2Error>`
- `create_asset(params: AssetParams) -> Result<TransferResult, Layer2Error>`
- `transfer_asset(transfer: AssetTransfer) -> Result<TransferResult, Layer2Error>`
- `verify_proof(proof: Proof) -> Result<VerificationResult, Layer2Error>`
- `validate_transaction(tx_id: String) -> Result<ValidationResult, Layer2Error>`
- `peg_in(bitcoin_txid: String, liquid_address: String, amount: u64) -> Result<TransferResult, Layer2Error>`
- `peg_out(bitcoin_address: String, amount: u64) -> Result<TransferResult, Layer2Error>`

### Configuration Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `network` | String | Network type (mainnet/testnet) | "mainnet" |
| `rpc_url` | String | RPC endpoint URL | "<https://liquid.network/rpc>" |
| `confidential` | bool | Enable confidential transactions | true |
| `timeout_ms` | u64 | Request timeout in milliseconds | 30000 |
| `federation_pubkeys` | Vec<String> | Federation signer public keys | Default keys |
| `required_signatures` | u32 | Minimum required signatures | 11 |
| `elementsd_path` | String | Path to Elements daemon | "/usr/local/bin/elementsd" |

## Security Considerations

### Federation Trust Model

- Liquid uses a federated consensus model with trusted functionaries
- Transactions require 11 of 15 federation signatures
- Federation members include major Bitcoin companies and exchanges

### Confidential Transactions

- Uses Pedersen commitments for amount hiding
- Range proofs ensure no inflation
- Confidential assets maintain privacy without sacrificing verifiability

### Peg Security

- Two-way peg secured by federation multisig
- Emergency recovery mechanisms in case of federation failure
- Regular audits of peg wallet reserves

## Best Practices

### Development

1. **Test on Testnet**: Always test on Liquid testnet before mainnet deployment
2. **Monitor Federation**: Keep track of federation status and health
3. **Handle Reorgs**: Implement proper reorganization handling
4. **Backup Keys**: Secure backup of all private keys and seed phrases

### Production

1. **Monitor Connectivity**: Implement health checks for RPC connectivity
2. **Error Handling**: Robust error handling for network failures
3. **Rate Limiting**: Respect RPC rate limits to avoid service disruption
4. **Security Updates**: Keep Elements daemon updated

## Troubleshooting

### Common Issues

#### Connection Failures

```rust
// Check network connectivity
match client.connect().await {
    Ok(_) => println!("Connected successfully"),
    Err(e) => println!("Connection failed: {}", e),
}
```

#### Asset Creation Errors

- Verify sufficient L-BTC for fees
- Check asset parameters are valid
- Ensure proper permissions for asset issuance

#### Peg Operation Issues

- Confirm Bitcoin transaction has sufficient confirmations
- Verify Liquid address format is correct
- Check federation is operational

### Debugging

Enable debug logging:

```bash
RUST_LOG=anya_core::layer2::liquid=debug cargo run
```

### Support Resources

- [Liquid Network Documentation](https://docs.liquid.net/)
- [Elements Project](https://elementsproject.org/)
- [Anya Core Issues](https://github.com/anya-org/anya-core/issues)

## Examples

### Complete Asset Workflow

```rust
use anya_core::layer2::liquid::{LiquidConfig, LiquidClient};
use anya_core::layer2::{AssetParams, AssetTransfer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let config = LiquidConfig::default();
    let mut client = LiquidClient::new(config);
    
    // Connect to network
    client.connect().await?;
    
    // Create asset
    let asset_params = AssetParams {
        asset_id: "company_shares".to_string(),
        name: "Company Shares".to_string(),
        symbol: "COMP".to_string(),
        precision: 0,
        total_supply: 1000,
        description: "Company equity shares".to_string(),
    };
    
    let asset_result = client.create_asset(asset_params).await?;
    println!("Asset created: {:?}", asset_result);
    
    // Transfer asset
    let transfer = AssetTransfer {
        from: "issuer_address".to_string(),
        to: "investor_address".to_string(),
        amount: 100,
        asset_id: "company_shares".to_string(),
        memo: Some("Initial allocation".to_string()),
    };
    
    let transfer_result = client.transfer_asset(transfer).await?;
    println!("Transfer completed: {:?}", transfer_result);
    
    Ok(())
}
```

## Integration Notes

- Compatible with all Bitcoin wallets through two-way peg
- Supports atomic swaps with other Layer2 protocols
- Integration with Lightning Network for instant payments
- Compatible with Bitcoin Script and enhanced Elements opcodes
