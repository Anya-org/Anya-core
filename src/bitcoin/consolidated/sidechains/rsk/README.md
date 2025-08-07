# bitcoin/consolidated/sidechains/rsk Module

RSK Sidechain integration module

Implementation following official Bitcoin Improvement Proposals (BIPs)

## Overview

The `bitcoin/consolidated/sidechains/rsk` module provides RSK sidechain integration capabilities within the Anya Core Bitcoin infrastructure. RSK is a smart contract platform connected to the Bitcoin blockchain through merge-mining, enabling Ethereum-compatible smart contracts secured by Bitcoin's proof-of-work.

## Key Components

### RskAdapter

The main adapter implementing hexagonal architecture patterns:

- **Configuration Management**: RSK node connection and verification settings
- **Bitcoin Verification**: SPV and full node verification capabilities
- **Smart Contract Integration**: Contract address management and interaction

```rust
use anya_core::bitcoin::consolidated::sidechains::rsk::{RskAdapter, RskConfig, VerificationMode};

// Create RSK adapter configuration
let config = RskConfig {
    node_url: "https://public-node.rsk.co".to_string(),
    contract_address: "0x123...".to_string(),
    verification_mode: VerificationMode::SPV,
};

// Initialize adapter
let mut adapter = RskAdapter::new(config);
adapter.initialize()?;
```

### Bitcoin Verification

Comprehensive Bitcoin payment verification through RSK:

- **BitcoinSPV**: Simple Payment Verification using merkle proofs
- **BlockHeader**: Bitcoin block header validation
- **RskBitcoinVerifier**: Main verification engine

```rust
// Verify Bitcoin payment through RSK
let proof = BitcoinSPV::new(tx_data, merkle_proof);
let is_valid = adapter.verify_bitcoin_payment(proof)?;
```

### API Reference

#### RskConfig

- `node_url`: URL of the RSK node endpoint
- `contract_address`: Smart contract address for Bitcoin verification
- `verification_mode`: Type of verification (SPV, FullNode, Federated)

#### VerificationMode

- `SPV`: Simple Payment Verification using merkle proofs
- `FullNode`: Full node verification with complete block data
- `Federated`: Federated verification through threshold signatures

## For more information

See the comprehensive documentation in the [docs/](../../../docs/) directory.
