---
title: "BOB (Build on Bitcoin) Layer2 Documentation"
description: "Complete guide to BOB blockchain integration in Anya Core"
---

# BOB (Build on Bitcoin)

## Overview

BOB is an Ethereum-compatible L2 blockchain that is secured by Bitcoin. It combines the security of Bitcoin with the programmability of Ethereum, allowing developers to build DApps that leverage Bitcoin's security while benefiting from Ethereum's ecosystem.

## Features

- **Bitcoin Security**: Secured by Bitcoin's proof-of-work through innovative mechanisms
- **EVM Compatibility**: Full Ethereum Virtual Machine compatibility
- **Native Bitcoin Support**: Direct Bitcoin integration without bridges
- **Rollup Technology**: Optimistic rollup with fraud proofs
- **DeFi Ecosystem**: Access to Ethereum DeFi protocols

## Configuration

### Basic Configuration

```rust
use anya_core::layer2::bob::{BOBConfig, BOBClient};

let config = BOBConfig {
    network: "mainnet".to_string(),
    rpc_url: "https://rpc.gobob.xyz".to_string(),
    bitcoin_enabled: true,
    timeout_ms: 30000,
    gas_price: 1000000000, // 1 gwei
    gas_limit: 21000000,
    sequencer_url: "https://sequencer.gobob.xyz".to_string(),
};

let client = BOBClient::new(config);
```

### Environment Variables

```bash
BOB_NETWORK=mainnet
BOB_RPC_URL=https://rpc.gobob.xyz
BOB_BITCOIN_ENABLED=true
BOB_TIMEOUT_MS=30000
BOB_GAS_PRICE=1000000000
BOB_GAS_LIMIT=21000000
BOB_SEQUENCER_URL=https://sequencer.gobob.xyz
```

## Usage Examples

### Smart Contract Deployment

```rust
use anya_core::layer2::bob::SmartContractParams;

// Deploy an EVM-compatible contract on BOB
let contract_params = SmartContractParams {
    bytecode: "0x608060405234801561001057600080fd5b50...".to_string(),
    constructor_args: vec![
        "1000000000000000000000000".to_string(), // Initial supply
        "BOB Token".to_string(),
        "BOB".to_string(),
    ],
    gas_limit: 3000000,
    gas_price: 1000000000,
};

let result = client.deploy_contract(contract_params).await?;
println!("Contract deployed on BOB: {:?}", result);
```

### Bitcoin Integration

```rust
// Use Bitcoin directly in BOB smart contracts
let bitcoin_integration = client.integrate_bitcoin_transaction(
    "bitcoin_txid".to_string(),
    "bob_contract_address".to_string(),
    "process_bitcoin_tx".to_string(), // Contract function
    vec!["tx_data".to_string()],
).await?;

println!("Bitcoin integration result: {:?}", bitcoin_integration);
```

### Asset Operations

```rust
use anya_core::layer2::AssetTransfer;

// Transfer assets on BOB
let transfer = AssetTransfer {
    from: "0x1234...".to_string(),
    to: "0x5678...".to_string(),
    amount: 1000000000000000000, // 1 ETH equivalent
    asset_id: "ETH".to_string(),
    memo: Some("Payment on BOB".to_string()),
};

let result = client.transfer_asset(transfer).await?;
println!("BOB transfer: {:?}", result);
```

### Cross-Chain Operations

```rust
// Bridge assets from Ethereum to BOB
let bridge_result = client.bridge_from_ethereum(
    "ethereum_contract_address".to_string(),
    "bob_recipient_address".to_string(),
    1000000000000000000, // 1 ETH
    "USDC".to_string(),
).await?;

// Withdraw assets back to Ethereum
let withdrawal = client.withdraw_to_ethereum(
    "ethereum_address".to_string(),
    1000000000000000000, // 1 ETH equivalent
    "USDC".to_string(),
).await?;
```

## API Reference

### BOBClient

#### Methods

- `new(config: BOBConfig) -> Self`
- `connect() -> Result<(), Layer2Error>`
- `disconnect() -> Result<(), Layer2Error>`
- `get_state() -> Result<ProtocolState, Layer2Error>`
- `deploy_contract(params: SmartContractParams) -> Result<TransferResult, Layer2Error>`
- `call_contract(address: String, method: String, args: Vec<String>, gas: Option<u64>) -> Result<TransferResult, Layer2Error>`
- `transfer_asset(transfer: AssetTransfer) -> Result<TransferResult, Layer2Error>`
- `integrate_bitcoin_transaction(btc_txid: String, contract: String, method: String, args: Vec<String>) -> Result<TransferResult, Layer2Error>`
- `bridge_from_ethereum(eth_contract: String, bob_recipient: String, amount: u64, asset: String) -> Result<TransferResult, Layer2Error>`
- `withdraw_to_ethereum(eth_address: String, amount: u64, asset: String) -> Result<TransferResult, Layer2Error>`
- `get_bitcoin_integration_status() -> Result<BitcoinStatus, Layer2Error>`
- `verify_proof(proof: Proof) -> Result<VerificationResult, Layer2Error>`
- `validate_transaction(tx_hash: String) -> Result<ValidationResult, Layer2Error>`

### Configuration Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `network` | String | Network type (mainnet/testnet) | "mainnet" |
| `rpc_url` | String | RPC endpoint URL | "<https://rpc.gobob.xyz>" |
| `bitcoin_enabled` | bool | Enable Bitcoin integration | true |
| `timeout_ms` | u64 | Request timeout in milliseconds | 30000 |
| `gas_price` | u64 | Gas price in wei | 1000000000 |
| `gas_limit` | u64 | Gas limit for transactions | 21000000 |
| `sequencer_url` | String | Sequencer endpoint URL | "<https://sequencer.gobob.xyz>" |

### Bitcoin Integration Types

#### BitcoinStatus

```rust
pub struct BitcoinStatus {
    pub enabled: bool,
    pub latest_block: u64,
    pub confirmations_required: u32,
    pub supported_opcodes: Vec<String>,
}
```

#### SmartContractParams

```rust
pub struct SmartContractParams {
    pub bytecode: String,
    pub constructor_args: Vec<String>,
    pub gas_limit: u64,
    pub gas_price: u64,
}
```

## Bitcoin Integration

### Direct Bitcoin Access

BOB allows smart contracts to directly interact with Bitcoin:

1. **Bitcoin State**: Read Bitcoin blockchain state from smart contracts
2. **Transaction Verification**: Verify Bitcoin transactions on-chain
3. **UTXO Access**: Access Bitcoin UTXO data
4. **Script Execution**: Execute Bitcoin scripts within smart contracts

### Security Model

1. **Bitcoin Finality**: Leverages Bitcoin's proof-of-work for final settlement
2. **Fraud Proofs**: Optimistic rollup with challenge period
3. **Validator Network**: Decentralized validator set
4. **Economic Security**: Staking and slashing mechanisms

## EVM Compatibility

### Ethereum Features

BOB supports full Ethereum compatibility:

- **Smart Contracts**: Deploy existing Ethereum contracts
- **DeFi Protocols**: Use Ethereum DeFi applications
- **Development Tools**: Truffle, Hardhat, Remix support
- **Wallet Integration**: MetaMask and other Ethereum wallets

### BOB Enhancements

Additional features unique to BOB:

- **Bitcoin Precompiles**: Special contracts for Bitcoin operations
- **Hybrid Transactions**: Combine Bitcoin and Ethereum operations
- **Native Bitcoin Types**: Bitcoin-specific data types in contracts
- **Cross-Chain Messaging**: Direct communication with Bitcoin

## Security Considerations

### Rollup Security

- **Fraud Proofs**: Challenge invalid state transitions
- **Challenge Period**: Time for fraud proof submission
- **Validator Slashing**: Economic penalties for malicious behavior
- **Data Availability**: Ensure transaction data availability

### Bitcoin Integration Security

- **SPV Proofs**: Simplified Payment Verification for Bitcoin transactions
- **Confirmation Requirements**: Multiple Bitcoin confirmations required
- **Reorg Protection**: Handle Bitcoin reorganizations safely
- **Oracle Security**: Secure Bitcoin state oracle mechanisms

### Smart Contract Security

- **EVM Security**: Standard Ethereum smart contract security considerations
- **Bitcoin Interaction**: Additional security for Bitcoin integration
- **Cross-Chain Risks**: Consider risks in cross-chain operations
- **Upgrade Mechanisms**: Secure contract upgrade patterns

## Best Practices

### Development

1. **Test Thoroughly**: Use BOB testnet for comprehensive testing
2. **Bitcoin Integration**: Understand Bitcoin finality requirements
3. **Gas Optimization**: Optimize for BOB's gas economics
4. **Error Handling**: Handle cross-chain operation failures gracefully

### Bitcoin Integration

1. **Confirmation Wait**: Wait for sufficient Bitcoin confirmations
2. **Reorg Handling**: Handle Bitcoin chain reorganizations
3. **Fee Management**: Account for Bitcoin transaction fees
4. **State Synchronization**: Keep Bitcoin state synchronized

### Cross-Chain Operations

1. **Bridge Security**: Understand bridge trust assumptions
2. **Liquidity Management**: Ensure adequate bridge liquidity
3. **Timing Considerations**: Account for settlement delays
4. **Failure Recovery**: Implement recovery mechanisms

## Troubleshooting

### Common Issues

#### Connection Problems

```rust
// Test BOB network connectivity
match client.connect().await {
    Ok(_) => println!("Connected to BOB network"),
    Err(e) => println!("Connection failed: {}", e),
}
```

#### Bitcoin Integration Issues

- Verify Bitcoin node connectivity
- Check Bitcoin transaction confirmations
- Validate SPV proofs

#### Smart Contract Failures

- Check gas limits and prices
- Verify contract bytecode
- Test on BOB testnet first

### Debugging

Enable debug logging:

```bash
RUST_LOG=anya_core::layer2::bob=debug cargo run
```

### Support Resources

- [BOB Documentation](https://docs.gobob.xyz/)
- [BOB GitHub](https://github.com/bob-collective)
- [BOB Explorer](https://explorer.gobob.xyz/)
- [Anya Core Issues](https://github.com/anya-org/anya-core/issues)

## Examples

### Bitcoin-Powered DeFi

```rust
use anya_core::layer2::bob::{BOBConfig, BOBClient, SmartContractParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize BOB client
    let config = BOBConfig::default();
    let mut client = BOBClient::new(config);
    
    // Connect to BOB network
    client.connect().await?;
    
    // Deploy Bitcoin-aware DeFi contract
    let defi_contract = SmartContractParams {
        bytecode: include_str!("contracts/bitcoin_defi.hex").to_string(),
        constructor_args: vec![
            "BitcoinDeFi".to_string(),
            "BTCDEFI".to_string(),
        ],
        gas_limit: 5000000,
        gas_price: 1000000000,
    };
    
    let deployment = client.deploy_contract(defi_contract).await?;
    println!("DeFi contract deployed: {:?}", deployment);
    
    // Integrate Bitcoin transaction
    let btc_integration = client.integrate_bitcoin_transaction(
        "bitcoin_tx_hash".to_string(),
        deployment.contract_address.unwrap(),
        "processDeposit".to_string(),
        vec!["deposit_data".to_string()],
    ).await?;
    
    println!("Bitcoin integration completed: {:?}", btc_integration);
    
    Ok(())
}
```

## Integration Notes

- Compatible with existing Ethereum infrastructure and tools
- Supports direct Bitcoin operations without bridge dependencies
- Integration with Lightning Network for instant Bitcoin payments
- Compatible with other Layer2 solutions through standard bridges
