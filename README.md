# Anya Core

Anya Core is a comprehensive Bitcoin development framework that adheres to the Bitcoin Development Framework v2.5 standards. It provides a robust set of tools and libraries for building Bitcoin applications with a focus on security, privacy, and decentralization.

## Features

- **Bitcoin Integration**: Full Bitcoin protocol support with Taproot capabilities
- **Lightning Network**: LDK-based Lightning Network implementation
- **Discrete Log Contracts (DLCs)**: Privacy-preserving DLCs using non-interactive oracle patterns
- **Cross-Chain Functionality**: Support for sidechains and Layer 2 solutions
  - **RSK**: Bitcoin-RSK bridge with SPV proofs
  - **Liquid**: Bitcoin-Liquid bridge with asset issuance and transfer
- **Wallet Management**: Secure wallet implementation with BIP39/44/84/86 support
- **Web5 Protocol**: Complete Web5 implementation with DIDs, DWNs, and protocol support
- **Hexagonal Architecture**: Clean separation of concerns with adapters and ports

## Project Structure

```
anya-core/
├── src/
│   ├── bitcoin/             # Bitcoin module
│   │   ├── adapters/        # Bitcoin implementation adapters
│   │   ├── anya-bitcoin/    # Core Bitcoin functionality
│   │   ├── cross_chain/     # Cross-chain integration
│   │   │   ├── rsk.rs       # RSK bridge implementation
│   │   │   ├── liquid.rs    # Liquid bridge implementation
│   │   │   └── mod.rs       # Cross-chain module entry point
│   │   ├── dlc/             # Discrete Log Contracts
│   │   ├── interface/       # Bitcoin interface definitions
│   │   ├── layer2/          # Layer 2 solutions
│   │   ├── sidechains/      # Sidechain implementations
│   │   ├── taproot/         # Taproot assets and functionality
│   │   ├── wallet/          # Wallet implementation
│   │   ├── lightning.rs     # Lightning Network implementation
│   │   └── mod.rs           # Bitcoin module entry point
│   ├── web5/                # Web5 module
│   │   ├── identity.rs      # DID implementation
│   │   ├── dwn.rs           # Decentralized Web Node implementation
│   │   ├── protocols.rs     # Protocol handling
│   │   └── mod.rs           # Web5 module entry point
│   ├── config.rs            # Configuration module
│   └── lib.rs               # Library entry point
├── Cargo.toml               # Project dependencies and configuration
└── README.md                # This file
```

## Bitcoin Module

The Bitcoin module provides a comprehensive implementation of Bitcoin functionality, including:

### Core Components

- **Interface**: Defines the core Bitcoin interfaces and traits
- **Adapters**: Implements the interfaces for different Bitcoin backends
- **Wallet**: Secure wallet implementation with support for various address types
- **Taproot**: Taproot assets and functionality
- **Lightning**: Lightning Network implementation using LDK
- **DLC**: Discrete Log Contracts implementation
- **Cross-Chain**: Support for cross-chain functionality
  - **RSK Bridge**: Bitcoin-RSK bridge with SPV proofs
  - **Liquid Bridge**: Bitcoin-Liquid bridge with asset issuance and transfer

### BIP Compliance

The Bitcoin module adheres to the following BIPs:

- BIP 341/342 (Taproot)
- BIP 174 (PSBT)
- BIP 39/44/84/86 (HD Wallets)
- BIP 32 (Hierarchical Deterministic Wallets)
- BIP 340 (Schnorr Signatures)

## Web5 Module

The Web5 module provides a complete implementation of the Web5 protocol, including:

### Core Components

- **DID Management**: Create, resolve, and manage Decentralized Identifiers
- **DWN Integration**: Store and retrieve data from Decentralized Web Nodes
- **Protocol Support**: Define and handle Web5 protocols
- **Credential Management**: Issue, verify, and revoke verifiable credentials
- **Messaging**: Secure messaging between DIDs

### Standards Compliance

The Web5 module adheres to the following standards:

- W3C DID Core Specification
- W3C Verifiable Credentials Data Model
- DIF DWN Specification
- TBD Web5 Protocol Specification

## Liquid Support

The Liquid module provides integration with the Liquid sidechain, including:

### Core Components

- **Liquid Bridge**: Transfer Bitcoin to and from the Liquid sidechain
- **Asset Issuance**: Issue and manage custom assets on Liquid
- **Confidential Transactions**: Support for Liquid's confidential transaction features
- **SPV Proofs**: Verify Bitcoin transactions on Liquid using SPV proofs

### Features

- **L-BTC Management**: Send, receive, and manage L-BTC (Liquid Bitcoin)
- **Asset Management**: Issue, transfer, and manage custom assets
- **Confidential Transactions**: Privacy-preserving transactions with blinded amounts and asset types
- **Multi-signature Support**: Advanced multi-signature capabilities for enhanced security

## Getting Started

### Prerequisites

- Rust 1.70.0 or higher
- Bitcoin Core 24.0 or higher (for certain functionality)
- Liquid/Elements Core 22.0 or higher (for Liquid functionality)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/anya-core.git
cd anya-core

# Build the project
cargo build --release
```

### Configuration

Create a `.env` file in the project root with the following configuration:

```
BITCOIN_NETWORK=testnet
BITCOIN_RPC_URL=http://localhost:18332
BITCOIN_RPC_USER=your_rpc_username
BITCOIN_RPC_PASS=your_rpc_password
ENABLED_FEATURES=taproot,lightning,dlc,web5,liquid
WEB5_DID_METHOD=ion
WEB5_DWN_ENDPOINT=https://dwn.tbddev.org
LIQUID_RPC_URL=http://localhost:7041
LIQUID_RPC_USER=your_liquid_rpc_username
LIQUID_RPC_PASS=your_liquid_rpc_password
```

## Usage

### Bitcoin Example

```rust
use anya_core::{bitcoin, config};

fn main() {
    // Load configuration
    let config = config::Config::from_env();
    
    // Initialize Bitcoin module
    bitcoin::init();
    
    // Create a wallet
    let wallet = bitcoin::wallet::create_wallet(&config);
    
    // Generate a new address
    let address = wallet.generate_address();
    println!("New address: {}", address);
}
```

### Web5 Example

```rust
use anya_core::{web5, config};

fn main() {
    // Load configuration
    let config = config::Config::from_env();
    
    // Initialize Web5 module
    let web5_manager = web5::Web5Manager::new(config.web5_config).unwrap();
    
    // Create a DID
    let did = web5_manager.create_did().unwrap();
    println!("Created DID: {}", did.id);
    
    // Store data in a DWN
    let data = serde_json::json!({
        "name": "Alice",
        "email": "alice@example.com"
    });
    
    let record_id = web5_manager.create_record(&did.id, "https://schema.org/Person", data).unwrap();
    println!("Stored record: {}", record_id);
}
```

### Liquid Example

```rust
use anya_core::{bitcoin, config};

fn main() {
    // Load configuration
    let config = config::Config::from_env();
    
    // Initialize Bitcoin module (includes Liquid)
    bitcoin::init();
    
    // Create a Liquid bridge
    let mut bridge = bitcoin::cross_chain::create_bridge(
        "Bitcoin-Liquid Bridge",
        "Liquid",
        100000, // 0.001 BTC minimum
        None,   // No maximum
        102,    // 102 confirmations required
        10,     // 0.1% fee
    );
    
    // Create a transaction to Liquid
    let mut transaction = bitcoin::cross_chain::create_transaction(
        &mut bridge,
        "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4", // Bitcoin sender
        "VJL7xGMPkX4BoKYvCBNqYUNLd3UcguxHyA",         // Liquid recipient
        1000000, // 0.01 BTC
    ).unwrap();
    
    // Execute the transaction
    let txid = bitcoin::cross_chain::execute_transaction(&mut bridge, &mut transaction).unwrap();
    println!("Transaction created: {}", txid);
    
    // Issue a custom asset on Liquid
    let asset = bitcoin::cross_chain::liquid::issue_liquid_asset(
        "My Token",
        "TKN",
        8,          // 8 decimal places
        1_000_000,  // 1 million tokens
        true,       // Reissuable
        &[1, 2, 3, 4], // Private key (simplified)
    ).unwrap();
    
    println!("Asset issued: {}", asset.asset_id);
}
```

## Testing

```bash
# Run all tests
cargo test

# Run Bitcoin module tests
cargo test --package anya-core --lib bitcoin

# Run Web5 module tests
cargo test --package anya-core --lib web5

# Run Liquid module tests
cargo test --package anya-core --lib bitcoin::cross_chain::liquid
```

## Hexagonal Architecture

Anya Core follows a hexagonal architecture pattern, which separates the core business logic from external concerns:

- **Core Domain**: The central business logic
- **Ports**: Interfaces that the core domain exposes
- **Adapters**: Implementations of the ports that connect to external systems

This architecture ensures:
- Decentralized component management
- Protocol-level interoperability
- Real-time system observability
- Backward-compatible upgrades

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
