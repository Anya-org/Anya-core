# Discrete Log Contracts (DLC)

## Overview

Discrete Log Contracts (DLC) enable smart contracts on Bitcoin using Oracle signatures and Bitcoin's native scripting capabilities. The Anya Core implementation provides a complete DLC framework for creating, executing, and settling conditional payments.

## Features

- **Oracle-Based Execution**: Smart contracts executed based on external data
- **Non-Interactive Setup**: Minimal communication required between parties
- **Privacy-Preserving**: Contract terms not visible on the blockchain
- **Bitcoin-Native**: Uses only Bitcoin script capabilities
- **Flexible Outcomes**: Support for binary and multi-outcome contracts

## Architecture

DLC operates through:

1. **Contract Setup**: Parties agree on terms and oracle
2. **Funding**: Bitcoin is locked in a multi-signature address
3. **Oracle Attestation**: Oracle signs the outcome
4. **Settlement**: Funds are distributed based on oracle signature

## Configuration

```rust
use anya_core::layer2::dlc::{DlcConfig, DlcProtocol};

let config = DlcConfig {
    network: "mainnet".to_string(),
    oracle_endpoints: vec![
        "https://oracle1.example.com".to_string(),
        "https://oracle2.example.com".to_string(),
    ],
    timeout_hours: 24,
    fee_rate_sat_per_vb: 10,
    enable_multioracle: true,
    contract_timeout_blocks: 144, // ~24 hours
};

let dlc = DlcProtocol::with_config(config);
```

## Usage

### Contract Creation

```rust
use anya_core::layer2::{Layer2Protocol, dlc::DlcProtocol};
use anya_core::layer2::dlc::{ContractDescriptor, OracleInfo, Outcome};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dlc = DlcProtocol::new();
    dlc.initialize().await?;
    
    // Define contract outcomes
    let outcomes = vec![
        Outcome {
            id: "alice_wins".to_string(),
            payout_alice: 100000, // satoshis
            payout_bob: 0,
            probability: 0.5,
        },
        Outcome {
            id: "bob_wins".to_string(),
            payout_alice: 0,
            payout_bob: 100000,
            probability: 0.5,
        },
    ];
    
    // Set up oracle information
    let oracle_info = OracleInfo {
        public_key: "oracle_public_key".to_string(),
        endpoint: "https://sports-oracle.com".to_string(),
        event_id: "world_cup_final_2025".to_string(),
        attestation_time: 1735689600, // Unix timestamp
    };
    
    // Create contract descriptor
    let contract = ContractDescriptor {
        contract_id: "sports_bet_001".to_string(),
        oracle_info,
        outcomes,
        maturity_time: 1735689600,
        fee_rate: 10,
    };
    
    println!("DLC contract created: {:?}", contract);
    
    Ok(())
}
```

### Contract Execution

```rust
use anya_core::layer2::Proof;

// Create execution proof with oracle signature
let execution_proof = Proof {
    proof_type: "oracle_attestation".to_string(),
    data: oracle_signature.to_vec(),
    witness: Some(outcome_data),
    metadata: std::collections::HashMap::new(),
};

// Execute the contract
let verification_result = dlc.verify_proof(execution_proof).await?;
if verification_result.is_valid {
    println!("Contract executed successfully");
} else {
    println!("Contract execution failed: {:?}", verification_result.error_message);
}
```

### Asset-Based Contracts

```rust
use anya_core::layer2::AssetParams;

// Create DLC with asset-based outcomes
let asset_params = AssetParams {
    asset_id: "prediction_market_token".to_string(),
    name: "Prediction Market Token".to_string(),
    symbol: "PMT".to_string(),
    precision: 8,
    decimals: 8,
    total_supply: 1000000,
    metadata: "Token for prediction market outcomes".to_string(),
};

let asset_id = dlc.issue_asset(asset_params).await?;
println!("Prediction market asset created: {}", asset_id);
```

## API Reference

### DlcProtocol

The main DLC protocol implementation.

#### Methods

- `new() -> Self`: Create a new DLC protocol instance
- `with_config(config: DlcConfig) -> Self`: Create with custom configuration
- `create_contract(&self, descriptor: ContractDescriptor) -> Result<Contract, Error>`
- `execute_contract(&self, contract_id: &str, oracle_sig: &OracleSignature) -> Result<ExecutionResult, Error>`
- All methods from `Layer2Protocol` trait

### DlcConfig

Configuration for DLC operations.

#### Fields

- `network: String`: Bitcoin network
- `oracle_endpoints: Vec<String>`: Trusted oracle endpoints
- `timeout_hours: u64`: Contract timeout period
- `fee_rate_sat_per_vb: u64`: Fee rate for transactions
- `enable_multioracle: bool`: Enable multi-oracle contracts
- `contract_timeout_blocks: u32`: Timeout in Bitcoin blocks

### ContractDescriptor

Describes a DLC contract.

#### Fields

- `contract_id: String`: Unique contract identifier
- `oracle_info: OracleInfo`: Oracle configuration
- `outcomes: Vec<Outcome>`: Possible contract outcomes
- `maturity_time: u64`: Contract maturity timestamp
- `fee_rate: u64`: Transaction fee rate

### OracleInfo

Oracle configuration for DLC contracts.

#### Fields

- `public_key: String`: Oracle's public key
- `endpoint: String`: Oracle API endpoint
- `event_id: String`: Event identifier
- `attestation_time: u64`: Expected attestation time

### Outcome

Represents a possible contract outcome.

#### Fields

- `id: String`: Outcome identifier
- `payout_alice: u64`: Payout amount for Alice
- `payout_bob: u64`: Payout amount for Bob
- `probability: f64`: Outcome probability (0.0 to 1.0)

## Oracle Integration

### Oracle Selection

```rust
// Configure multiple oracles for redundancy
let oracles = vec![
    OracleInfo {
        public_key: "oracle1_pubkey".to_string(),
        endpoint: "https://oracle1.com".to_string(),
        event_id: "btc_price_2025".to_string(),
        attestation_time: 1735689600,
    },
    OracleInfo {
        public_key: "oracle2_pubkey".to_string(),
        endpoint: "https://oracle2.com".to_string(),
        event_id: "btc_price_2025".to_string(),
        attestation_time: 1735689600,
    },
];
```

### Oracle Communication

```rust
// The DLC implementation handles oracle communication automatically
// Oracles provide signed attestations for contract resolution
```

## Security Considerations

### Oracle Trust

1. **Oracle Selection**: Choose reputable oracles with good track records
2. **Multi-Oracle Setup**: Use multiple oracles to reduce single points of failure
3. **Oracle Verification**: Verify oracle signatures and attestations

### Contract Security

1. **Timelock Safety**: Set appropriate timeouts for contract resolution
2. **Fee Management**: Account for fee variations in contract design
3. **Dispute Resolution**: Plan for oracle failure scenarios

### Key Management

1. **Secure Storage**: Store contract keys securely
2. **Backup Procedures**: Backup contract state and keys
3. **Access Control**: Limit access to contract signing keys

## Best Practices

### Contract Design

1. **Clear Outcomes**: Define unambiguous contract outcomes
2. **Fair Payouts**: Design equitable payout structures
3. **Reasonable Timeouts**: Set appropriate contract durations

### Oracle Management

1. **Oracle Diversity**: Use oracles from different providers
2. **Event Specificity**: Choose specific, verifiable events
3. **Attestation Timing**: Plan for oracle response times

## Use Cases

### Binary Prediction Markets

```rust
// Simple binary outcome contract
let binary_outcomes = vec![
    Outcome {
        id: "yes".to_string(),
        payout_alice: 200000,
        payout_bob: 0,
        probability: 0.6,
    },
    Outcome {
        id: "no".to_string(),
        payout_alice: 0,
        payout_bob: 200000,
        probability: 0.4,
    },
];
```

### Sports Betting

```rust
// Multi-outcome sports betting contract
let sports_outcomes = vec![
    Outcome { id: "team_a_wins".to_string(), payout_alice: 150000, payout_bob: 50000, probability: 0.4 },
    Outcome { id: "team_b_wins".to_string(), payout_alice: 50000, payout_bob: 150000, probability: 0.4 },
    Outcome { id: "draw".to_string(), payout_alice: 100000, payout_bob: 100000, probability: 0.2 },
];
```

### Financial Derivatives

```rust
// Price-based derivative contract
let price_outcomes = vec![
    Outcome { id: "above_50k".to_string(), payout_alice: 200000, payout_bob: 0, probability: 0.3 },
    Outcome { id: "below_50k".to_string(), payout_alice: 0, payout_bob: 200000, probability: 0.7 },
];
```

## Troubleshooting

### Common Issues

1. **Oracle Connectivity**: Verify oracle endpoints are accessible
2. **Signature Verification**: Check oracle signature formats and keys
3. **Timeout Handling**: Ensure proper timeout configuration

### Debugging

```rust
// Enable detailed logging for DLC operations
use log::{info, debug};

debug!("DLC contract created with ID: {}", contract_id);
info!("Oracle attestation received and verified");
```

## Examples

### Complete DLC Workflow

```rust
use anya_core::layer2::{Layer2Protocol, dlc::DlcProtocol};
use anya_core::layer2::dlc::{ContractDescriptor, OracleInfo, Outcome};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dlc = DlcProtocol::new();
    dlc.initialize().await?;
    dlc.connect().await?;
    
    // Create a simple binary contract
    let oracle_info = OracleInfo {
        public_key: "oracle_pubkey_hex".to_string(),
        endpoint: "https://price-oracle.com".to_string(),
        event_id: "btc_usd_price_2025_12_31".to_string(),
        attestation_time: 1735689600,
    };
    
    let outcomes = vec![
        Outcome {
            id: "above_100k".to_string(),
            payout_alice: 100000,
            payout_bob: 0,
            probability: 0.5,
        },
        Outcome {
            id: "below_100k".to_string(),
            payout_alice: 0,
            payout_bob: 100000,
            probability: 0.5,
        },
    ];
    
    let contract = ContractDescriptor {
        contract_id: "btc_price_bet_2025".to_string(),
        oracle_info,
        outcomes,
        maturity_time: 1735689600,
        fee_rate: 10,
    };
    
    // Submit contract (this would create the actual DLC)
    let contract_data = serde_json::to_vec(&contract)?;
    let contract_id = dlc.submit_transaction(&contract_data).await?;
    println!("DLC contract submitted: {}", contract_id);
    
    // Check contract status
    let status = dlc.check_transaction_status(&contract_id).await?;
    println!("Contract status: {:?}", status);
    
    Ok(())
}
```

## References

- [DLC Specification](https://github.com/discreetlogcontracts/dlcspecs)
- [DLC Protocol Documentation](https://github.com/p2pderivatives/dlc-specs)
- [Oracle Standards](https://github.com/discreetlogcontracts/dlcspecs/blob/master/Oracle.md)
- [Bitcoin Layer2 Protocol Documentation](README.md)
