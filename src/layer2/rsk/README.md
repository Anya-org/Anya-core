# Layer2: RSK Module

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

## Introduction

The RSK module provides integration with Rootstock (RSK), a smart contract platform that is merge-mined with Bitcoin. This implementation enables interaction with RSK's Ethereum-compatible blockchain while maintaining the security guarantees of the Bitcoin network.

## Features

- Complete RSK blockchain interaction via Web3 API
- Smart contract deployment and execution
- Two-way peg operations with Bitcoin
- RBTC (RSK's native token) management
- Integration with Bitcoin mining through merge-mining
- Transaction monitoring and verification

## Core Components

### RskProtocol

Main implementation of the RSK protocol adapter:

```rust
pub struct RskProtocol {
    connected: Arc<RwLock<bool>>,
    transactions: Arc<RwLock<HashMap<String, TransactionResult>>>,
}

impl RskProtocol {
    pub fn new() -> Self {
        Self {
            connected: Arc::new(RwLock::new(false)),
            transactions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
```

### Layer2Protocol Implementation

RSK implements the Layer2Protocol trait to provide standardized access:

```rust
#[async_trait]
impl Layer2Protocol for RskProtocol {
    async fn initialize(&self) -> Result<(), Layer2Error> {
        // Initialize the RSK protocol
    }

    async fn connect(&self) -> Result<(), Layer2Error> {
        // Connect to RSK node
    }

    async fn health_check(&self) -> Result<ProtocolHealth, Layer2Error> {
        // Check RSK network health
    }

    async fn get_state(&self) -> Result<ProtocolState, Layer2Error> {
        // Get current protocol state
    }

    // Additional trait methods...
}
```

## Usage Examples

### Basic RSK Interaction

```rust
use anya::layer2::rsk::RskProtocol;
use anya::layer2::Layer2Protocol;

async fn interact_with_rsk() -> Result<(), Box<dyn std::error::Error>> {
    // Create RSK protocol instance
    let rsk = RskProtocol::new();

    // Initialize and connect
    rsk.initialize().await?;
    rsk.connect().await?;

    // Check RSK health
    let health = rsk.health_check().await?;
    println!("RSK network healthy: {}", health.healthy);

    // Get RSK state information
    let state = rsk.get_state().await?;
    println!("RSK version: {}", state.version);
    println!("Current block: {}", state.height);

    Ok(())
}
```

### Smart Contract Interaction

```rust
use anya::layer2::rsk::{RskProtocol, SmartContractParams};
use std::collections::HashMap;

async fn deploy_contract() -> Result<(), Box<dyn std::error::Error>> {
    let rsk = RskProtocol::new();
    rsk.initialize().await?;

    // Contract bytecode and ABI
    let bytecode = "0x60806040...";
    let abi = r#"[{"inputs":[],"stateMutability":"nonpayable","type":"constructor"}...]"#;

    // Contract constructor parameters
    let mut params = HashMap::new();
    params.insert("initialValue".to_string(), "1000".to_string());

    // Deploy smart contract
    let tx_result = rsk.deploy_contract(
        bytecode,
        abi,
        &params,
        None, // No gas limit override
    ).await?;

    println!("Contract deployed!");
    println!("Transaction ID: {}", tx_result.transaction_id);
    println!("Contract address: {}", tx_result.metadata.get("contract_address").unwrap_or(&"unknown".to_string()));

    // Wait for confirmation
    let confirmed = rsk.wait_for_confirmation(&tx_result.transaction_id, 2).await?;
    println!("Contract deployment confirmed: {}", confirmed.is_confirmed());

    Ok(())
}
```

### Two-Way Peg Operations

```rust
use anya::layer2::rsk::{RskProtocol, PegOperation};

async fn perform_peg_operations() -> Result<(), Box<dyn std::error::Error>> {
    let rsk = RskProtocol::new();
    rsk.initialize().await?;

    // Peg-in (BTC to RBTC)
    let peg_in_address = rsk.get_peg_in_address().await?;
    println!("Send BTC to this address: {}", peg_in_address);

    // Monitor peg-in status
    let peg_in_status = rsk.check_peg_in_status("tx_hash").await?;
    println!("Peg-in status: {:?}", peg_in_status);

    // Peg-out (RBTC to BTC)
    let peg_out_result = rsk.peg_out(
        "btc_address",
        1_000_000, // 0.01 RBTC in satoshis
    ).await?;

    println!("Peg-out initiated: {}", peg_out_result.transaction_id);

    Ok(())
}
```

## Security Considerations

1. **Merge-Mining Validation**: Always validate the security of the merge-mining process
2. **Smart Contract Security**: Follow standard Ethereum smart contract security best practices
3. **Two-Way Peg Security**: Use appropriate confirmation thresholds for peg operations
4. **Validation Chain**: Verify the RSK validation chain for critical operations
5. **Private Key Management**: Securely manage private keys for RSK accounts

## Integration with Other Modules

The RSK module integrates with:

- **Bitcoin Core**: For on-chain peg operations
- **Key Management**: For secure signing operations
- **Smart Contract**: For deploying and interacting with RSK smart contracts
- **Monitoring**: For tracking network status and transaction confirmations

## For More Information

- [RSK Developer Portal](https://developers.rsk.co/)
- [RSK Technical Documentation](https://developers.rsk.co/rsk/)
- Project documentation
