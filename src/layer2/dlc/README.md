# Layer2: Discreet Log Contract Module

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

## Introduction

The Discreet Log Contract (DLC) module implements Bitcoin-based smart contracts that leverage cryptographic oracles for settlement. DLCs enable complex financial agreements on Bitcoin without requiring on-chain script complexity, maintaining privacy and scalability.

## Features

- Complete DLC protocol implementation
- Multi-oracle support for redundancy and security
- Numeric, categorical, and enumerated outcome types
- Contract negotiation and lifecycle management
- Integration with trusted oracle services
- Privacy-preserving contract settlement

## Core Components

### DlcConfig

Configuration settings for DLC protocol:

```rust
pub struct DlcConfig {
    /// Network type: mainnet, testnet, regtest
    pub network: String,
    /// Oracle endpoints
    pub oracle_endpoints: Vec<String>,
    /// Contract timeout in blocks
    pub contract_timeout: u32,
    /// Maximum contract value in satoshis
    pub max_contract_value: u64,
}
```

### DlcContract

Representation of a discreet log contract:

```rust
pub struct DlcContract {
    pub contract_id: String,
    pub oracle_pubkey: String,
    pub outcome_payouts: HashMap<String, u64>,
    pub maturity: u64,
    pub collateral: u64,
    pub status: ContractStatus,
}
```

### ContractStatus

Status of DLC contracts:

```rust
pub enum ContractStatus {
    Offered,
    Accepted,
    Signed,
    Confirmed,
    Closed,
    Refunded,
}
```

### DlcProtocol

Main implementation of the DLC protocol:

```rust
pub struct DlcProtocol {
    config: DlcConfig,
    connected: Arc<RwLock<bool>>,
    contracts: Arc<RwLock<HashMap<String, DlcContract>>>,
}
```

## Usage Examples

### Creating a Binary Outcome Contract

```rust
use anya::layer2::dlc::{DlcConfig, DlcProtocol};
use anya::layer2::Layer2Protocol;
use std::collections::HashMap;

async fn create_binary_bet() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize DLC protocol
    let config = DlcConfig {
        network: "testnet".to_string(),
        oracle_endpoints: vec!["https://oracle.example.com/api".to_string()],
        contract_timeout: 144, // 1 day in blocks
        max_contract_value: 10_000_000, // 0.1 BTC
    };

    let dlc = DlcProtocol::new(config);
    dlc.initialize().await?;

    // Set up oracle information
    let oracle_pubkey = "02a7ae1e0971fc1689bd66d2a7296da3c1629b2d23b5f14e8550920f0f7fb8116c".to_string();

    // Define payouts for different outcomes
    let mut outcome_payouts = HashMap::new();
    outcome_payouts.insert("Team A wins".to_string(), 9000); // 90% to proposer if Team A wins
    outcome_payouts.insert("Team B wins".to_string(), 1000); // 10% to proposer if Team B wins

    // Create contract
    let contract_id = dlc.create_contract(
        oracle_pubkey,
        outcome_payouts,
        1650000000, // Unix timestamp for maturity
        10000000,   // 0.1 BTC collateral
    ).await?;

    println!("Created DLC contract: {}", contract_id);

    Ok(())
}
```

### Accepting and Settling a Contract

```rust
use anya::layer2::dlc::{DlcProtocol, ContractStatus};

async fn accept_and_settle_contract(
    dlc: &DlcProtocol,
    contract_id: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Accept a contract
    dlc.accept_contract(contract_id).await?;

    // Sign and fund the contract
    dlc.sign_contract(contract_id).await?;

    // Get contract status
    let contract = dlc.get_contract(contract_id).await?;
    println!("Contract status: {:?}", contract.status);

    // When the oracle publishes the outcome
    if contract.status == ContractStatus::Confirmed {
        // Execute settlement based on oracle attestation
        let settlement = dlc.settle_contract(contract_id).await?;
        println!("Contract settled. Amount received: {} satoshis", settlement.amount);
        println!("Settlement transaction: {}", settlement.transaction_id);
    }

    Ok(())
}
```

### Creating a Price Oracle Contract

```rust
use anya::layer2::dlc::{DlcProtocol, OracleInfo, NumericOutcome};

async fn create_price_contract(
    dlc: &DlcProtocol,
    oracle_pubkey: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Define a numeric outcome (e.g., BTC price)
    let price_oracle = OracleInfo {
        pubkey: oracle_pubkey.to_string(),
        announcement_signature: "".to_string(),
        endpoints: vec!["https://price-oracle.example.com/api".to_string()],
    };

    // Create contract with numeric outcomes
    let outcome = NumericOutcome {
        name: "BTC-USD".to_string(),
        unit: "USD".to_string(),
        precision: 2,
        min_value: 1000000, // $10,000.00
        max_value: 10000000, // $100,000.00
        start_range: 2500000, // $25,000.00
        payout_function: "linear".to_string(),
        payout_params: HashMap::new(),
    };

    let contract_id = dlc.create_numeric_contract(
        price_oracle,
        outcome,
        1650000000, // Unix timestamp for maturity
        5000000,    // 0.05 BTC collateral
    ).await?;

    println!("Created price oracle contract: {}", contract_id);

    Ok(())
}
```

## Security Considerations

1. **Oracle Trust**: DLCs still require trusted oracles. Consider using multiple oracles for critical contracts
2. **Contract Privacy**: While DLCs are more private than on-chain contracts, metadata leakage is still possible
3. **Collateral Management**: Ensure sufficient collateral is locked and managed properly
4. **Oracle Attestation Verification**: Always verify oracle signatures before settlement
5. **Timelock Management**: Set appropriate timelocks for contract expiration and refunds

## For More Information

- [DLC Specification](https://github.com/discreetlogcontracts/dlcspecs)
- [Oracle Protocol](https://github.com/discreetlogcontracts/dlcspecs/blob/master/Oracle.md)
- Project documentation
