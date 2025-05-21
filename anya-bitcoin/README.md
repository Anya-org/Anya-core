# Anya Bitcoin [BPC-3][AIS-3][RES-3]

*Last Updated: May 21, 2025*

**Version: v0.3.0-rc1**

> **Note**: This release includes full Taproot support, enhanced security features, and performance improvements.

A comprehensive Bitcoin implementation following the Bitcoin Development Framework v2.5 standards with a focus on security, performance, and extensibility. Built with a hexagonal architecture for clean separation of concerns and maximum flexibility.

## Overview

Anya Bitcoin provides a complete Bitcoin implementation with support for Layer 2 protocols, RISC-V integration, and extensible architecture. It follows a hexagonal architecture pattern for clean separation of concerns.

## Quick Start [BPC-3][UXA-2]

### Prerequisites

- Rust 1.65+ (stable)
- Clang and LLVM
- Protobuf compiler
- OpenSSL/LibreSSL

### Building from Source

```bash
# Clone the repository
git clone https://github.com/your-org/anya-core.git
cd anya-core/anya-bitcoin

# Build in release mode
cargo build --release

# Run tests
cargo test --all-features

# Start a testnet node
cargo run --release -- --network=testnet
```

## Taproot Support [BIP-341][BIP-342]

Full support for Taproot (BIP 341) and Tapscript (BIP 342) including:

- Schnorr signatures (BIP 340)
- Taproot key path spending
- Tapscript script path spending
- Signature hashing (BIP 341)
- Taproot descriptor support

Example Taproot address generation:

```rust
use anya_bitcoin::core::taproot::TaprootBuilder;
use secp256k1::{Secp256k1, KeyPair};

let secp = Secp256k1::new();
let keypair = KeyPair::new(&secp, &mut rand::thread_rng());
let builder = TaprootBuilder::new();
// ... build taproot tree ...
```
│   ├── adapters/                   # External system adapters
│   │   ├── rpc/                    # RPC adapters
│   │   ├── storage/                # Storage adapters
│   │   └── protocols/              # Protocol adapters
│   ├── riscv/                      # RISC-V VM implementation
│   │   ├── vm/                     # Virtual Machine core
│   │   ├── instructions/           # Instruction set
│   │   └── contracts/              # Smart contract framework
│   ├── security/                   # Security implementations
│   │   ├── hsm/                    # HSM integration
│   │   └── crypto/                 # Cryptographic operations
│   ├── protocol/                   # Bitcoin protocol implementation
│   └── testing/                    # Comprehensive tests
│       ├── core/                   # Core functionality tests
│       ├── layer2/                 # Layer 2 protocol tests
│       ├── riscv/                  # RISC-V VM tests
│       └── integration/            # Cross-component tests
└── docs/                           # Documentation
    ├── architecture/               # Architecture documentation
    ├── standards/                  # Standards documentation
    └── layer2/                     # Layer 2 documentation
```

## Key Features

- **Full Bitcoin Core Implementation**: Complete implementation of the Bitcoin protocol
- **Hexagonal Architecture**: Clean separation between domain, ports, and adapters
- **Layer 2 Support**: Comprehensive implementation of multiple Layer 2 solutions
- **RISC-V Integration**: RISC-V VM implementation for smart contracts
- **Comprehensive Security**: HSM integration, Taproot support, and security validation
- **Extensive Testing**: Comprehensive test coverage across all components

## Layer 2 Solutions

| Technology | Status | Features |
|------------|--------|----------|
| BOB | Complete | Bitcoin relay, EVM compatibility, BitVM integration |
| Lightning Network | 75% Complete | Channels, payments, routing |
| RGB Protocol | 75% Complete | Smart contracts, asset issuance |
| RSK | 75% Complete | Two-way peg, smart contracts |
| DLC | 75% Complete | Oracles, contracts, outcomes |
| Taproot Assets | 75% Complete | Asset issuance, transfers, Merkle proofs |

## Implementation Details

### Core Bitcoin (BPC-3)

- **Consensus**: Full implementation of Bitcoin consensus rules (BIP-340, BIP-341, BIP-342)
- **P2P Network**: Complete P2P networking stack
- **Mempool**: Transaction memory pool management
- **Script**: Script execution and validation

### RISC-V Integration

The RISC-V VM implementation provides:

- Bitcoin script extensions
- Smart contract capabilities
- Cross-layer interoperability

## Getting Started

```rust
use anya_bitcoin::{BitcoinNode, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Bitcoin node with default configuration
    let config = Config::default();
    let mut node = BitcoinNode::new(config)?;
    
    // Start the node
    node.start()?;
    
    // Access Layer 2 protocols
    if let Some(registry) = node.layer2_registry() {
        let bob = registry.get_protocol("bob")?;
        let lightning = registry.get_protocol("lightning")?;
        
        // Use Layer 2 protocols
        // ...
    }
    
    Ok(())
}
```

## Testing

```bash
# Run core tests
cargo test --package anya-bitcoin --test core

# Run Layer 2 tests
cargo test --package anya-bitcoin --test layer2

# Run RISC-V tests
cargo test --package anya-bitcoin --test riscv
``` 
