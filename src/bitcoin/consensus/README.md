# Bitcoin Consensus Module

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

[AIS-3]: #ais-3 "Application Integration Standard Level 3"
[RES-3]: #res-3 "Resource Efficiency Standard Level 3"

## Overview

The Bitcoin Consensus module contains critical components that ensure Anya Core's Bitcoin transaction validation maintains 100% alignment with Bitcoin Core's consensus rules. This module implements the fundamental validation rules that govern the Bitcoin network, providing the foundation for secure, reliable Bitcoin operations.

## Components

The following files implement this module:

- **differential_fuzzer.rs** - Validates consensus rules by comparing against Bitcoin Core
- **invariant_checker.rs** - Enforces critical Bitcoin consensus invariants
- **mod.rs** - Core module definitions and exports

## Key Features

- **Consensus Compatibility**: Full alignment with Bitcoin Core consensus rules
- **Invariant Checking**: Strict validation of Bitcoin consensus invariants
- **Differential Fuzzing**: Comparing validation results with Bitcoin Core reference
- **Critical Error Detection**: Identification and handling of consensus-breaking conditions
- **Performance Optimizations**: Efficient validation without compromising security

## API Reference

### Consensus Invariant Checker

Enforces critical Bitcoin consensus rules:

```rust
/// Checks a critical consensus invariant that must be maintained
/// for Bitcoin consensus compatibility
pub struct ConsensusInvariant {
    /// Name of the invariant
    pub name: String,
    /// Description of what this invariant enforces
    pub description: String,
    /// Severity level of a violation
    pub severity: InvariantSeverity,
    /// Module affected by this invariant
    pub module: String,
}

/// Checker for consensus invariants
pub struct InvariantChecker {
    /// List of all invariants to check
    invariants: Vec<ConsensusInvariant>,
}

impl InvariantChecker {
    /// Create a new invariant checker
    pub fn new() -> Self {
        // Implementation
    }

    /// Check if a block satisfies all consensus invariants
    pub fn check_block(&self, block: &Block) -> Result<(), ConsensusError> {
        // Implementation
    }

    /// Check if a transaction satisfies all consensus invariants
    pub fn check_transaction(&self, tx: &Transaction) -> Result<(), ConsensusError> {
        // Implementation
    }
}
```

### Differential Fuzzer

Validates consensus by comparing against Bitcoin Core:

```rust
/// Differential fuzzer for consensus validation
pub struct DifferentialFuzzer {
    /// Connection to Bitcoin Core reference node
    bitcoin_core_connection: BitcoinConnection,
    /// Local validation implementation
    local_validator: TransactionValidator,
}

impl DifferentialFuzzer {
    /// Create a new differential fuzzer
    pub fn new(bitcoin_core_url: &str) -> Result<Self, BitcoinError> {
        // Implementation
    }

    /// Validate a transaction against both implementations
    pub fn validate_transaction(&self, tx: &Transaction) -> Result<DifferentialResult, BitcoinError> {
        // Implementation
    }

    /// Validate a block against both implementations
    pub fn validate_block(&self, block: &Block) -> Result<DifferentialResult, BitcoinError> {
        // Implementation
    }
}
```

## Usage Examples

### Consensus Invariant Checking

```rust
use anya_core::bitcoin::consensus::{InvariantChecker, ConsensusError};
use bitcoin::Transaction;

fn validate_with_consensus_rules(transaction: &Transaction) -> Result<(), ConsensusError> {
    // Create an invariant checker
    let checker = InvariantChecker::new();

    // Check if the transaction satisfies all consensus invariants
    checker.check_transaction(transaction)?;

    // Transaction is valid according to consensus rules
    println!("Transaction passed all consensus invariant checks");
    Ok(())
}
```

### Differential Validation

```rust
use anya_core::bitcoin::consensus::{DifferentialFuzzer, DifferentialResult};
use bitcoin::Transaction;

async fn compare_validation_with_bitcoin_core(tx_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the transaction
    let tx: Transaction = bitcoin::consensus::deserialize(&hex::decode(tx_hex)?)?;

    // Create a differential fuzzer
    let fuzzer = DifferentialFuzzer::new("http://localhost:8332")?;

    // Compare validation results
    let result = fuzzer.validate_transaction(&tx)?;

    match result {
        DifferentialResult::Match => {
            println!("✅ Validation matches Bitcoin Core");
            Ok(())
        },
        DifferentialResult::Mismatch { core_result, local_result } => {
            println!("❌ Validation mismatch detected!");
            println!("Bitcoin Core: {:?}", core_result);
            println!("Local implementation: {:?}", local_result);

            Err("Consensus validation mismatch".into())
        }
    }
}
```

## Error Handling

Common errors and their resolutions:

| Error | Cause | Resolution |
|-------|-------|------------|
| `ConsensusError::InvalidScript` | Script fails validation rules | Check script against Bitcoin consensus rules |
| `ConsensusError::InvalidBlockHeader` | Block header violates consensus | Verify header format and proof-of-work |
| `ConsensusError::InvalidTransaction` | Transaction violates consensus | Check transaction structure and inputs |
| `ConsensusError::InvariantViolation` | Consensus invariant violated | Critical issue requiring immediate fix |
| `BitcoinError::ConnectionFailed` | Cannot connect to reference node | Check Bitcoin Core connection settings |

## Testing

```bash
# Run all consensus module tests
cargo test bitcoin::consensus::

# Run differential fuzzer tests
cargo test bitcoin::consensus::differential_fuzzer::

# Run invariant checker tests
cargo test bitcoin::consensus::invariant_checker::
```

## Security Considerations

- [CONSENSUS CRITICAL] This module contains components that are ESSENTIAL for maintaining consensus with Bitcoin Core
- All changes must be extensively tested and reviewed by multiple security experts
- Failures in this module can lead to consensus forks and potential fund loss
- Comprehensive test coverage is maintained to prevent consensus bugs

## Compliance Standards

### AIR-3

Availability & Integrity Requirement Level 3: Enforces strict consensus rules with comprehensive validation ensuring transaction and block integrity at all times.

### AIS-3

Application Integration Standard Level 3: Provides clean APIs for integrating with Bitcoin validation systems and blockchain applications.

### BPC-3

Bitcoin Protocol Compatibility Level 3: Maintains 100% compatibility with Bitcoin Core consensus rules through differential testing and invariant checking.

### RES-3

Resource Efficiency Standard Level 3: Optimized validation procedures minimize resource usage without compromising security or accuracy.

## See Also

- [Bitcoin Core Documentation](../core/README.md)
- [Transaction Validation](../validation/README.md)
- [Bitcoin Protocol](../protocol/README.md)

See the comprehensive documentation in the [docs/](../../../docs/) directory.
