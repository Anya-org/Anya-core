---
title: "RSK (Rootstock) Layer2 Documentation"
description: "Complete guide to RSK blockchain integration in Anya Core"
---

# RSK (Rootstock)

## Overview

RSK (Rootstock) is a smart contract platform secured by the Bitcoin network through merge-mining. It provides Ethereum-compatible smart contracts while leveraging Bitcoin's security model, offering the best of both worlds.

## Features

- **Bitcoin-Secured**: Secured by Bitcoin miners through merge-mining
- **EVM Compatible**: Run Ethereum smart contracts without modification
- **Two-Way Peg**: Native bridge between Bitcoin and RSK
- **RIF Services**: Comprehensive infrastructure services (storage, communications, payments)
- **Fast Transactions**: 30-second block times with instant confirmation

## Configuration

### Basic Configuration

```rust
use anya_core::layer2::rsk::{RSKConfig, RSKClient};

let config = RSKConfig {
    network: "mainnet".to_string(),
    rpc_url: "https://public-node.rsk.co".to_string(),
    bridge_enabled: true,
    timeout_ms: 30000,
    gas_price: 60000000, // 0.06 gwei
    gas_limit: 6800000,
};

let client = RSKClient::new(config);
```

### Environment Variables

```bash
RSK_NETWORK=mainnet
RSK_RPC_URL=https://public-node.rsk.co
RSK_BRIDGE_ENABLED=true
RSK_TIMEOUT_MS=30000
RSK_GAS_PRICE=60000000
RSK_GAS_LIMIT=6800000
```

## Usage Examples

### Smart Contract Deployment

```rust
use anya_core::layer2::rsk::SmartContractParams;

// Deploy an ERC-20 token contract
let contract_params = SmartContractParams {
    bytecode: "0x608060405234801561001057600080fd5b50...".to_string(),
    constructor_args: vec![
        "MyToken".to_string(),
        "MTK".to_string(),
        "1000000".to_string(),
    ],
    gas_limit: 2000000,
    gas_price: 60000000,
};

let result = client.deploy_contract(contract_params).await?;
println!("Contract deployed: {:?}", result);
```

### Bitcoin Bridge Operations

```rust
// Peg Bitcoin to RSK (get RBTC)
let peg_in_result = client.peg_in(
    "bitcoin_txid".to_string(),
    "rsk_address".to_string(),
    100000, // satoshis
).await?;

// Peg RBTC back to Bitcoin
let peg_out_result = client.peg_out(
    "bitcoin_address".to_string(),
    100000, // wei (RBTC)
).await?;
```

### Token Operations

```rust
use anya_core::layer2::AssetTransfer;

// Transfer RBTC
let transfer = AssetTransfer {
    from: "0x1234...".to_string(),
    to: "0x5678...".to_string(),
    amount: 1000000000000000000, // 1 RBTC in wei
    asset_id: "RBTC".to_string(),
    memo: Some("Payment".to_string()),
};

let result = client.transfer_asset(transfer).await?;
println!("RBTC transfer: {:?}", result);
```

### Contract Interaction

```rust
// Call contract function
let call_result = client.call_contract(
    "0xcontract_address".to_string(),
    "transfer".to_string(),
    vec!["0xrecipient".to_string(), "1000000000000000000".to_string()],
    Some(100000), // gas limit
).await?;

println!("Contract call result: {:?}", call_result);
```

## API Reference

### RSKClient

#### Methods

- `new(config: RSKConfig) -> Self`
- `connect() -> Result<(), Layer2Error>`
- `disconnect() -> Result<(), Layer2Error>`
- `get_state() -> Result<ProtocolState, Layer2Error>`
- `deploy_contract(params: SmartContractParams) -> Result<TransferResult, Layer2Error>`
- `call_contract(address: String, method: String, args: Vec<String>, gas: Option<u64>) -> Result<TransferResult, Layer2Error>`
- `transfer_asset(transfer: AssetTransfer) -> Result<TransferResult, Layer2Error>`
- `peg_in(btc_txid: String, rsk_address: String, amount: u64) -> Result<TransferResult, Layer2Error>`
- `peg_out(btc_address: String, amount: u64) -> Result<TransferResult, Layer2Error>`
- `get_bridge_status() -> Result<BridgeStatus, Layer2Error>`
- `verify_proof(proof: Proof) -> Result<VerificationResult, Layer2Error>`
- `validate_transaction(tx_hash: String) -> Result<ValidationResult, Layer2Error>`

### Configuration Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `network` | String | Network type (mainnet/testnet) | "mainnet" |
| `rpc_url` | String | RPC endpoint URL | "<https://public-node.rsk.co>" |
| `bridge_enabled` | bool | Enable Bitcoin bridge | true |
| `timeout_ms` | u64 | Request timeout in milliseconds | 30000 |
| `gas_price` | u64 | Gas price in wei | 60000000 |
| `gas_limit` | u64 | Gas limit for transactions | 6800000 |

### Smart Contract Types

#### SmartContractParams

```rust
pub struct SmartContractParams {
    pub bytecode: String,
    pub constructor_args: Vec<String>,
    pub gas_limit: u64,
    pub gas_price: u64,
}
```

#### BridgeStatus

```rust
pub struct BridgeStatus {
    pub operational: bool,
    pub peg_in_enabled: bool,
    pub peg_out_enabled: bool,
    pub min_peg_in_amount: u64,
    pub min_peg_out_amount: u64,
    pub federation_size: u32,
}
```

## Bitcoin Integration

### Merge Mining

RSK uses merge mining to leverage Bitcoin's security:

1. **Merged Block**: Bitcoin miners include RSK block headers
2. **Proof of Work**: RSK inherits Bitcoin's proof-of-work security
3. **No Additional Energy**: Uses same computational power as Bitcoin mining
4. **Incentive Alignment**: Miners earn both BTC and RSK transaction fees

### Two-Way Peg

The RSK bridge enables seamless Bitcoin transfers:

1. **Peg-In**: Lock Bitcoin to receive RBTC on RSK
2. **Federation**: Multi-signature federation manages locked Bitcoin
3. **Peg-Out**: Burn RBTC to unlock Bitcoin
4. **Security**: Time delays and multiple confirmations for security

## Smart Contracts

### EVM Compatibility

RSK supports Ethereum smart contracts with minor differences:

- **Gas Costs**: Different gas cost structure
- **Block Time**: 30-second blocks vs Ethereum's 12-15 seconds
- **Native Currency**: RBTC instead of ETH
- **Opcodes**: Some Ethereum opcodes not supported

### Development Tools

- **Truffle**: Full Truffle framework support
- **Hardhat**: Compatible with Hardhat development environment
- **Remix**: Web-based IDE with RSK support
- **Web3.js**: Standard Web3 library integration

## Security Considerations

### Bridge Security

- **Federation Model**: Multi-signature federation secures the bridge
- **Time Delays**: Withdrawal delays for additional security
- **Emergency Procedures**: Mechanisms for handling bridge emergencies
- **Audit History**: Regular security audits of bridge contracts

### Smart Contract Security

- **EVM Security**: Inherits Ethereum's smart contract security model
- **RSK Specifics**: Be aware of RSK-specific gas costs and limits
- **Bridge Integration**: Consider bridge security when designing contracts

### Network Security

- **Bitcoin Hashrate**: Security proportional to Bitcoin network hashrate
- **Merge Mining**: Additional miners strengthen the network
- **Reorg Protection**: Bitcoin-level reorganization protection

## Best Practices

### Development

1. **Test on Testnet**: Always test on RSK testnet before mainnet
2. **Gas Optimization**: Optimize contracts for RSK's gas model
3. **Bridge Awareness**: Understand bridge mechanics for cross-chain apps
4. **Monitor Network**: Keep track of RSK network status and upgrades

### Bridge Usage

1. **Confirm Timing**: Allow sufficient confirmations for bridge operations
2. **Amount Limits**: Respect minimum and maximum bridge amounts
3. **Fee Planning**: Account for bridge fees in your application
4. **Error Handling**: Implement robust error handling for bridge failures

### Smart Contracts

1. **EVM Differences**: Account for RSK-specific EVM differences
2. **Gas Management**: Use appropriate gas prices and limits
3. **Security Audits**: Audit contracts before deployment
4. **Upgrade Patterns**: Plan for contract upgrades and governance

## Troubleshooting

### Common Issues

#### Bridge Operation Failures

```rust
// Check bridge status
let bridge_status = client.get_bridge_status().await?;
if !bridge_status.operational {
    println!("Bridge is currently not operational");
}
```

#### Transaction Failures

- Check gas price and limit settings
- Verify account has sufficient RBTC for gas
- Confirm network connectivity

#### Contract Deployment Issues

- Validate contract bytecode
- Ensure sufficient gas for deployment
- Check constructor parameters

### Debugging

Enable debug logging:

```bash
RUST_LOG=anya_core::layer2::rsk=debug cargo run
```

### Support Resources

- [RSK Documentation](https://developers.rsk.co/)
- [RSK Explorer](https://explorer.rsk.co/)
- [RIF Services](https://www.rifos.org/)
- [Anya Core Issues](https://github.com/anya-org/anya-core/issues)

## Examples

### Complete DApp Development

```rust
use anya_core::layer2::rsk::{RSKConfig, RSKClient, SmartContractParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let config = RSKConfig::default();
    let mut client = RSKClient::new(config);
    
    // Connect to network
    client.connect().await?;
    
    // Deploy token contract
    let token_contract = SmartContractParams {
        bytecode: include_str!("contracts/token.hex").to_string(),
        constructor_args: vec![
            "MyDAppToken".to_string(),
            "MDT".to_string(),
            "1000000000000000000000000".to_string(), // 1M tokens
        ],
        gas_limit: 3000000,
        gas_price: 60000000,
    };
    
    let deployment = client.deploy_contract(token_contract).await?;
    println!("Token contract deployed: {:?}", deployment);
    
    // Bridge Bitcoin to RSK
    let peg_in = client.peg_in(
        "bitcoin_txid_here".to_string(),
        "0x_rsk_address_here".to_string(),
        50000000, // 0.5 BTC in satoshis
    ).await?;
    
    println!("Bridge operation completed: {:?}", peg_in);
    
    Ok(())
}
```

## Integration Notes

- Compatible with Ethereum development tools and libraries
- Supports cross-chain bridges to other blockchain networks
- Integration with Bitcoin Lightning Network for enhanced functionality
- Compatible with DeFi protocols ported from Ethereum
