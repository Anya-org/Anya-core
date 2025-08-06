# Bitcoin Sidechains: RSK Module

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

## Introduction

The RSK module provides integration with Rootstock (RSK), a smart contract platform that is merge-mined with Bitcoin. RSK enables Ethereum-compatible smart contracts with Bitcoin's security guarantees through a two-way peg mechanism.

## Features

- Complete RSK blockchain interaction with Ethereum-compatible JSON-RPC
- Two-way peg operations (peg-in/peg-out) with Bitcoin
- Smart contract deployment and interaction
- RBTC (RSK's native token) management
- Merge-mining verification and monitoring

## Core Components

### NetworkType

RSK network types with corresponding chain IDs:

```rust
pub enum NetworkType {
    /// Mainnet
    Mainnet, // chain_id: 30

    /// Testnet
    Testnet, // chain_id: 31

    /// Regtest (local development)
    Regtest, // chain_id: 33
}
```

### RskClient

Client for interacting with RSK nodes:

```rust
pub struct RskClient {
    // Implementation details
}

impl RskClient {
    /// Create a new RSK client
    pub fn new(url: &str, network_type: NetworkType) -> AnyaResult<Self> {
        // Creates a client connected to an RSK node
    }

    /// Get the current block number
    pub async fn get_block_number(&self) -> AnyaResult<u64> {
        // Retrieves the current block number
    }

    /// Deploy a smart contract
    pub async fn deploy_contract(&self, abi: &str, bytecode: &str, args: &[&str]) -> AnyaResult<H160> {
        // Deploys a smart contract and returns the address
    }

    /// Call a smart contract method
    pub async fn call_contract<T>(&self, contract_addr: &str, abi: &str, method: &str, args: &[&str]) -> AnyaResult<T> {
        // Calls a smart contract method
    }
}
```

### BridgeOperations

Manages two-way peg operations between Bitcoin and RSK:

```rust
pub struct BridgeOperations {
    // Implementation details
}

impl BridgeOperations {
    /// Create a new bridge operations instance
    pub fn new(btc_wallet: BitcoinWallet, rsk_client: RskClient) -> Self {
        // Creates a new bridge operations instance
    }

    /// Perform a peg-in from Bitcoin to RSK
    pub async fn peg_in(&self, amount: u64, destination_address: &str) -> AnyaResult<PegOperation> {
        // Executes a peg-in operation
    }

    /// Perform a peg-out from RSK to Bitcoin
    pub async fn peg_out(&self, amount: u64, destination_address: &str) -> AnyaResult<PegOperation> {
        // Executes a peg-out operation
    }

    /// Get the status of a peg operation
    pub async fn get_peg_status(&self, operation_id: &str) -> AnyaResult<PegStatus> {
        // Retrieves the status of a peg operation
    }
}
```

## Error Handling

The RSK module defines specific error types:

```rust
pub enum ClientError {
    /// Network error
    NetworkError(String),

    /// RPC error
    RpcError(String),

    /// Contract error
    ContractError(String),

    /// Transaction error
    TransactionError(String),

    // Additional error types
}

pub enum BridgeError {
    /// Invalid bridge configuration
    InvalidConfiguration(String),

    /// Transaction creation error
    TransactionCreationError(String),

    /// Peg-in error
    PegInError(String),

    // Additional error types
}
```

## Usage Examples

### Basic RSK Interaction

```rust
use anya::bitcoin::sidechains::rsk::{RskClient, NetworkType};

async fn interact_with_rsk() -> AnyaResult<()> {
    // Create RSK client
    let client = RskClient::new("https://public-node.rsk.co", NetworkType::Mainnet)?;

    // Get current block number
    let block_number = client.get_block_number().await?;
    println!("Current RSK block: {}", block_number);

    // Get RBTC balance
    let balance = client.get_balance("0x5aaEB6053f3e94c9b9a09f33669435E7ef1bEAeD").await?;
    println!("RBTC Balance: {} wei", balance);

    Ok(())
}
```

### Smart Contract Deployment

```rust
use anya::bitcoin::sidechains::rsk::{RskClient, NetworkType};

async fn deploy_example_contract() -> AnyaResult<()> {
    let client = RskClient::new("https://public-node.rsk.co", NetworkType::Testnet)?;

    // Contract ABI and bytecode
    let abi = r#"[{"inputs":[{"name":"initialValue","type":"uint256"}],"stateMutability":"nonpayable","type":"constructor"},...]"#;
    let bytecode = "0x608060405234801561001057600080fd5b5060405161...";

    // Deploy contract with initial value 100
    let contract_address = client.deploy_contract(abi, bytecode, &["100"]).await?;
    println!("Contract deployed at: 0x{:x}", contract_address);

    Ok(())
}
```

### Performing a Peg-In

```rust
use anya::bitcoin::sidechains::rsk::{BridgeOperations, RskClient, NetworkType};
use anya::bitcoin::wallet::BitcoinWallet;

async fn perform_btc_to_rsk_transfer(amount_btc: f64, rsk_address: &str) -> AnyaResult<()> {
    // Initialize Bitcoin wallet
    let btc_wallet = BitcoinWallet::load("my_wallet.dat", "password")?;

    // Create RSK client
    let rsk_client = RskClient::new("https://public-node.rsk.co", NetworkType::Mainnet)?;

    // Create bridge operations
    let bridge = BridgeOperations::new(btc_wallet, rsk_client);

    // Convert BTC to satoshis
    let amount_sats = (amount_btc * 100_000_000.0) as u64;

    // Perform peg-in
    let operation = bridge.peg_in(amount_sats, rsk_address).await?;
    println!("Peg-in initiated: {}", operation.id);
    println!("Bitcoin transaction: {}", operation.btc_txid);

    Ok(())
}
```

## Security Considerations

RSK's security model relies on merge-mining with Bitcoin, meaning Bitcoin miners can simultaneously mine on the RSK blockchain without additional investment in mining hardware. Key security considerations:

1. Ensure proper validation of peg-in and peg-out operations
2. Use recommended confirmation thresholds (100+ for high-value transactions)
3. Be aware of the federated peg model's trust assumptions
4. Always verify signatures and transaction proofs

## For more information

- [RSK Developer Portal](https://developers.rsk.co/)
- [RSK Technical Documentation](https://developers.rsk.co/rsk/)
- See the comprehensive documentation in the project documentation
