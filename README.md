# Anya Core

Anya Core is a comprehensive Bitcoin development framework that adheres to the Bitcoin Development Framework v2.5 standards. It provides a robust set of tools and libraries for building Bitcoin applications with a focus on security, privacy, and decentralization.

*Last Updated: March 1, 2025*

## Features

- **Bitcoin Integration**: Full Bitcoin protocol support with Taproot capabilities
  - **Enhanced Wallet**: Multi-output PSBT creation and management
  - **Hardware Wallet Support**: Compatible with major hardware wallet vendors
  - **Transaction Management**: Advanced fee management and PSBT handling
- **Lightning Network**: LDK-based Lightning Network implementation
- **Discrete Log Contracts (DLCs)**: Privacy-preserving DLCs using non-interactive oracle patterns
- **Cross-Chain Functionality**: Support for sidechains and Layer 2 solutions
  - **RSK**: Bitcoin-RSK bridge with SPV proofs
  - **Liquid**: Bitcoin-Liquid bridge with asset issuance and transfer
- **RGB Assets**: Complete RGB asset management
  - **Asset Issuance**: Create and issue RGB assets with customizable parameters
  - **Asset Transfer**: Transfer assets with metadata and batch transfers
  - **Asset Verification**: Verify asset ownership and validity
- **Wallet Management**: Secure wallet implementation with BIP39/44/84/86 support
- **Web5 Protocol**: Complete Web5 implementation with DIDs, DWNs, and protocol support
  - **Bitcoin-Anchored DWNs**: Enhanced data integrity using Bitcoin anchoring
  - **Bitcoin-Anchored Verifiable Credentials**: Credentials secured by Bitcoin's immutability
  - **Decentralized Identity**: Support for multiple DID methods with caching mechanism
  - **Advanced DWN Capabilities**: Secure, private, and encrypted data storage
  - **Comprehensive API**: Full suite of Web5 endpoints with WebSocket support
- **Hexagonal Architecture**: Clean separation of concerns with adapters and ports
- **RESTful API**: Actix Web-based API with JWT authentication and WebSocket support

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
│   │   ├── anchoring.rs     # Bitcoin anchoring implementation
│   │   ├── protocols.rs     # Protocol handling
│   │   ├── credentials.rs   # Verifiable credentials implementation
│   │   └── mod.rs           # Web5 module entry point
│   ├── rgb/                 # RGB module
│   │   ├── assets.rs        # RGB asset implementation
│   │   ├── transfer.rs      # Asset transfer functionality
│   │   ├── metadata.rs      # Metadata handling for assets
│   │   ├── verification.rs  # Asset verification
│   │   └── mod.rs           # RGB module entry point
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
  - **Multi-Output PSBT**: Create PSBTs with multiple outputs
  - **PSBT Enhancement**: Add metadata and hardware wallet compatibility
  - **PSBT Signing**: Sign PSBTs with various key types
  - **PSBT Import/Export**: Import and export PSBTs to/from various formats
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

## RGB Module

The RGB module provides a complete implementation of the RGB protocol for asset issuance and management:

### Core Components

- **Asset Issuance**: Create and issue RGB assets with customizable parameters
- **Asset Transfer**: Transfer assets with metadata and batch transfers
  - **Metadata Support**: Attach metadata to asset transfers
  - **Batch Transfers**: Transfer assets to multiple recipients in a single operation
  - **Transfer Verification**: Verify transfer validity and authenticity
- **Asset Verification**: Verify asset ownership and validity
  - **Ownership Verification**: Verify ownership of assets
  - **Asset History**: Track asset transfer history
  - **Metadata Verification**: Verify metadata integrity

## Web5 Module

The Web5 module provides a complete implementation of the Web5 protocol, including:

### Core Components

- **DID Management**: Create, resolve, and manage Decentralized Identifiers
  - **Resolution Caching**: Cache DID resolutions for improved performance
  - **Multiple DID Methods**: Support for various DID methods
- **DWN Integration**: Store and retrieve data from Decentralized Web Nodes
  - **Bitcoin Anchoring**: Anchor DWN messages to Bitcoin blockchain for enhanced data integrity
  - **Encrypted Storage**: End-to-end encrypted data storage in DWNs
  - **Query Capabilities**: Advanced query functionality for DWN records
  - **Permissions**: Granular permission system for DWN records
  - **Replication**: Data replication across multiple DWNs
- **Protocol Support**: Define and handle Web5 protocols
- **Credential Management**: Issue, verify, and revoke verifiable credentials
  - **Bitcoin Anchoring**: Anchor credentials to Bitcoin blockchain
  - **Revocation**: Credential revocation with Bitcoin transactions
  - **Status Verification**: Verify credential status using blockchain confirmation
  - **Credential Updates**: Update credentials with new claims while preserving history
- **Messaging**: Secure messaging between DIDs

### Bitcoin Anchoring

The Web5 module provides Bitcoin anchoring for both DWN messages and verifiable credentials:

- **Anchoring Process**:
  1. Generate a commitment hash of the DWN message or credential
  2. Create a Bitcoin transaction with the commitment hash in an OP_RETURN output
  3. Submit the transaction to the Bitcoin network
  4. Monitor transaction confirmations
  5. Update record/credential status based on confirmation status

- **Verification Process**:
  1. Extract the commitment hash from the DWN message or credential
  2. Retrieve the Bitcoin transaction using the provided transaction ID
  3. Verify the commitment hash matches the OP_RETURN data
  4. Check the number of confirmations
  5. Return verification status

- **Security Benefits**:
  - Immutability: Data integrity guaranteed by Bitcoin's immutability
  - Timestamping: Proof of existence at a specific Bitcoin block height
  - Non-repudiation: Cryptographic proof that the data existed at a specific time
  - Censorship resistance: Data verification does not rely on centralized services

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

## Recent Updates

### March 2025 (v0.2.0)

- Added comprehensive RGB asset transfer functionality with metadata support
- Enhanced wallet implementation with multi-output PSBT creation and hardware wallet compatibility
- Implemented Web5 credential verification with Bitcoin anchoring
- Added Web5 DID resolution caching for improved performance
- Completed API routes for Web5 functionality
- Added batch transfer functionality for RGB assets

## Getting Started

### Prerequisites

- Rust 1.70.0 or higher
- Bitcoin Core 24.0 or higher (for certain functionality)
- Liquid/Elements Core 22.0 or higher (for Liquid functionality)

### Installation

#### Option 1: Using the Unified Installer (Recommended)

The unified installer automates the entire setup process, including dependencies, configuration, and testing.

```bash
# Clone the repository
git clone https://github.com/Anya-org/anya-core.git
cd anya-core

# Build the installer
cargo build --bin installer --release

# Run the installer (with guided setup)
./target/release/installer install

# Configure the installation
./target/release/installer configure --network testnet

# Run tests to verify installation
./target/release/installer test --report
```

#### Option 2: Manual Installation

```bash
# Clone the repository
git clone https://github.com/Anya-org/anya-core.git
cd anya-core

# Build the project
cargo build --release

### Installation Options

The unified installer supports the following options:

```
# Show help
./target/release/installer --help

# Install only core components
./target/release/installer install --core-only

# Dry run (no changes)
./target/release/installer install --dry-run

# Configure with specific options
./target/release/installer configure --network mainnet --log-level debug --data-dir /path/to/data

# Test specific components
./target/release/installer test --component bitcoin
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

### Web5 Example with Bitcoin Anchoring

```rust
use anya_core::{web5, bitcoin, config};

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = config::Config::from_env();
    
    // Initialize Bitcoin module
    bitcoin::init();
    
    // Create a wallet for anchoring
    let wallet = bitcoin::wallet::create_wallet(&config);
    
    // Initialize Web5 module with Bitcoin anchoring
    let web5_config = web5::Web5Config {
        did_method: "ion".to_string(),
        dwn_endpoint: "https://dwn.tbddev.org".to_string(),
        enable_anchoring: true,
        min_confirmations: 3,
        enable_credential_anchoring: true,
        ..Default::default()
    };
    
    let web5_manager = web5::Web5Manager::new(web5_config, Some(wallet))?;
    
    // Create a DID
    let did = web5_manager.create_did()?;
    println!("Created DID: {}", did.id);
    
    // Store data in a DWN with Bitcoin anchoring
    let data = serde_json::json!({
        "name": "Alice",
        "email": "alice@example.com"
    });
    
    // Create and anchor the record
    let record_id = web5_manager.create_record_with_anchoring(
        &did.id, 
        "https://schema.org/Person", 
        data
    ).await?;
    
    println!("Stored record: {}", record_id);
    
    // Check anchoring status
    let status = web5_manager.get_record_anchoring_status(&record_id).await?;
    println!("Anchoring status: {:?}", status);
    
    // Wait for confirmation (in a real app, you would poll periodically)
    if status.confirmations < 3 {
        println!("Waiting for more confirmations...");
    }
    
    // Issue a verifiable credential with Bitcoin anchoring
    let credential = web5_manager.issue_credential_with_anchoring(
        &did.id,
        "https://example.com/credentials/examples/v1",
        serde_json::json!({
            "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
            "name": "Alice Smith",
            "degree": {
                "type": "BachelorDegree",
                "name": "Bachelor of Science and Arts"
            }
        })
    ).await?;
    
    println!("Issued credential: {}", credential.id);
    
    // Verify a credential with Bitcoin confirmation check
    let verification = web5_manager.verify_credential_with_anchoring(&credential.id).await?;
    println!("Credential verification result: {:?}", verification);
    
    Ok(())
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

# Run Web5 Bitcoin anchoring tests
cargo test --package anya-core --lib web5::anchoring

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
