---
title: "Stacks Blockchain Layer2 Documentation"
description: "Complete guide to Stacks blockchain integration in Anya Core"
---

# Stacks Blockchain

## Overview

Stacks is a layer-1 blockchain that settles on Bitcoin and enables smart contracts and decentralized applications (DApps) through Proof of Transfer (PoX). Anya Core provides full integration with Stacks for smart contract functionality while maintaining Bitcoin's security.

## Features

- **Smart Contracts**: Clarity smart contract language with Bitcoin finality
- **Proof of Transfer**: Novel consensus mechanism that recycles Bitcoin's energy
- **STX Stacking**: Earn Bitcoin by locking STX tokens
- **Bitcoin Integration**: Direct interaction with Bitcoin state
- **DeFi Ecosystem**: Comprehensive DeFi protocols built on Bitcoin

## Configuration

### Basic Configuration

```rust
use anya_core::layer2::stacks::{StacksConfig, StacksClient};

let config = StacksConfig {
    network: "mainnet".to_string(),
    rpc_url: "https://stacks-node-api.mainnet.stacks.co".to_string(),
    pox_enabled: true,
    timeout_ms: 30000,
};

let client = StacksClient::new(config);
```

### Environment Variables

```bash
STACKS_NETWORK=mainnet
STACKS_RPC_URL=https://stacks-node-api.mainnet.stacks.co
STACKS_POX_ENABLED=true
STACKS_TIMEOUT_MS=30000
```

## Usage Examples

### Smart Contract Deployment

```rust
use anya_core::layer2::stacks::ContractParams;

// Deploy a smart contract
let contract_params = ContractParams {
    name: "my-token".to_string(),
    source_code: r#"
        (define-fungible-token my-token)
        (define-public (mint (amount uint) (recipient principal))
            (ft-mint? my-token amount recipient))
    "#.to_string(),
    contract_id: "my-contract".to_string(),
};

let result = client.deploy_contract(contract_params).await?;
println!("Contract deployed: {:?}", result);
```

### STX Stacking

```rust
// Stack STX tokens to earn Bitcoin
let stacking_result = client.stack_stx(
    1000000, // Amount in microSTX
    "bitcoin_address".to_string(),
    12, // Number of cycles
).await?;

println!("Stacking result: {:?}", stacking_result);
```

### Token Operations

```rust
use anya_core::layer2::AssetTransfer;

// Transfer STX tokens
let transfer = AssetTransfer {
    from: "sender_address".to_string(),
    to: "receiver_address".to_string(),
    amount: 100000, // microSTX
    asset_id: "STX".to_string(),
    memo: Some("Payment for services".to_string()),
};

let result = client.transfer_asset(transfer).await?;
println!("STX transfer: {:?}", result);
```

### Contract Function Calls

```rust
// Call a smart contract function
let call_result = client.call_contract_function(
    "my-contract".to_string(),
    "mint".to_string(),
    vec!["1000".to_string(), "recipient_address".to_string()],
).await?;

println!("Contract call result: {:?}", call_result);
```

## API Reference

### StacksClient

#### Methods

- `new(config: StacksConfig) -> Self`
- `connect() -> Result<(), Layer2Error>`
- `disconnect() -> Result<(), Layer2Error>`
- `get_state() -> Result<ProtocolState, Layer2Error>`
- `deploy_contract(params: ContractParams) -> Result<TransferResult, Layer2Error>`
- `call_contract_function(contract: String, function: String, args: Vec<String>) -> Result<TransferResult, Layer2Error>`
- `transfer_asset(transfer: AssetTransfer) -> Result<TransferResult, Layer2Error>`
- `stack_stx(amount: u64, btc_address: String, cycles: u32) -> Result<TransferResult, Layer2Error>`
- `get_stacking_info() -> Result<StackingInfo, Layer2Error>`
- `verify_proof(proof: Proof) -> Result<VerificationResult, Layer2Error>`
- `validate_transaction(tx_id: String) -> Result<ValidationResult, Layer2Error>`

### Configuration Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `network` | String | Network type (mainnet/testnet) | "mainnet" |
| `rpc_url` | String | RPC endpoint URL | "<https://stacks-node-api.mainnet.stacks.co>" |
| `pox_enabled` | bool | Enable Proof of Transfer | true |
| `timeout_ms` | u64 | Request timeout in milliseconds | 30000 |

### Smart Contract Types

#### ContractParams

```rust
pub struct ContractParams {
    pub name: String,
    pub source_code: String,
    pub contract_id: String,
}
```

#### StackingInfo

```rust
pub struct StackingInfo {
    pub stacked_amount: u64,
    pub reward_address: String,
    pub cycles_remaining: u32,
    pub total_rewards: u64,
}
```

## Proof of Transfer (PoX)

### How It Works

1. **Bitcoin Commitment**: Stacks miners commit Bitcoin to participate in mining
2. **Leader Election**: Committed Bitcoin determines mining probability
3. **Block Production**: Selected miners produce Stacks blocks
4. **Reward Distribution**: STX stackers receive the committed Bitcoin

### Stacking Process

1. **Lock STX**: Commit STX tokens for specified cycles
2. **Choose BTC Address**: Provide Bitcoin address for rewards
3. **Earn Bitcoin**: Receive Bitcoin rewards proportional to stake
4. **Unlock Tokens**: Retrieve STX after stacking period

## Smart Contracts with Clarity

### Language Features

- **Decidable**: No recursion, guaranteed to terminate
- **Transparent**: Code and execution are on-chain
- **Bitcoin-aware**: Can read Bitcoin state and transactions
- **Type-safe**: Strong typing prevents common errors

### Example Contract

```clarity
;; Simple token contract
(define-fungible-token my-token)

(define-public (transfer (amount uint) (sender principal) (recipient principal))
  (begin
    (asserts! (is-eq tx-sender sender) (err u1))
    (ft-transfer? my-token amount sender recipient)))

(define-public (mint (amount uint) (recipient principal))
  (begin
    (asserts! (is-eq tx-sender contract-caller) (err u2))
    (ft-mint? my-token amount recipient)))
```

## Security Considerations

### Bitcoin Finality

- Stacks transactions achieve Bitcoin-level finality
- Reorganizations follow Bitcoin's chain reorganization
- Security inherited from Bitcoin's proof-of-work

### Smart Contract Security

- Clarity prevents reentrancy attacks
- No infinite loops or recursion possible
- Built-in overflow protection

### PoX Considerations

- Stacking rewards depend on total participation
- Bitcoin price volatility affects reward value
- Network security scales with Bitcoin commitments

## Best Practices

### Development

1. **Test Thoroughly**: Use Clarinet for local smart contract testing
2. **Audit Contracts**: Have contracts audited before mainnet deployment
3. **Monitor Network**: Keep track of PoX cycles and network health
4. **Handle Forks**: Implement proper Bitcoin reorg handling

### Smart Contracts

1. **Keep Simple**: Minimize contract complexity
2. **Use Standards**: Follow SIP standards for tokens and NFTs
3. **Error Handling**: Implement comprehensive error handling
4. **Gas Optimization**: Optimize for transaction costs

### Stacking

1. **Diversify**: Don't stake all tokens in one cycle
2. **Monitor Rewards**: Track stacking performance and yields
3. **Secure Address**: Use secure Bitcoin addresses for rewards
4. **Understand Risks**: Be aware of slashing and lock-up periods

## Troubleshooting

### Common Issues

#### Connection Problems

```rust
// Test network connectivity
match client.get_state().await {
    Ok(state) => println!("Network state: {:?}", state),
    Err(e) => println!("Connection error: {}", e),
}
```

#### Contract Deployment Failures

- Check STX balance for transaction fees
- Verify contract syntax with Clarinet
- Ensure unique contract name

#### Stacking Issues

- Verify minimum stacking amount (100,000 STX)
- Check Bitcoin address format
- Confirm PoX cycle timing

### Debugging

Enable debug logging:

```bash
RUST_LOG=anya_core::layer2::stacks=debug cargo run
```

### Support Resources

- [Stacks Documentation](https://docs.stacks.co/)
- [Clarity Language Reference](https://docs.stacks.co/clarity/)
- [Stacks Explorer](https://explorer.stacks.co/)
- [Anya Core Issues](https://github.com/anya-org/anya-core/issues)

## Examples

### Complete DeFi Integration

```rust
use anya_core::layer2::stacks::{StacksConfig, StacksClient, ContractParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let config = StacksConfig::default();
    let mut client = StacksClient::new(config);
    
    // Connect to network
    client.connect().await?;
    
    // Deploy AMM contract
    let amm_contract = ContractParams {
        name: "simple-amm".to_string(),
        source_code: include_str!("contracts/amm.clar").to_string(),
        contract_id: "my-amm".to_string(),
    };
    
    let deployment = client.deploy_contract(amm_contract).await?;
    println!("AMM deployed: {:?}", deployment);
    
    // Add liquidity
    let add_liquidity = client.call_contract_function(
        "my-amm".to_string(),
        "add-liquidity".to_string(),
        vec!["1000000".to_string(), "2000000".to_string()],
    ).await?;
    
    // Start stacking
    let stacking = client.stack_stx(
        5000000, // 5 STX
        "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        6,
    ).await?;
    
    println!("DeFi setup complete: {:?}", stacking);
    
    Ok(())
}
```

## Integration Notes

- Compatible with Bitcoin infrastructure
- Supports cross-chain bridges to other networks
- Integration with Bitcoin Lightning for instant payments
- Compatible with existing Bitcoin wallets through STX addresses
