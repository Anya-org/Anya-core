# Anya Core

Anya Core is a powerful platform combining Bitcoin/crypto functionality, ML-based analytics, and Web5 decentralized data management with hexagonal architecture design principles.

## Features

- **Bitcoin Integration:** Bitcoin Core, Lightning Network, DLC support, Taproot/Schnorr signatures, Stacks, RSK, RGB, Liquid, and advanced cross-chain capabilities
- **Web5 Integration:** Decentralized Web Nodes (DWN), Decentralized Identifiers (DIDs), Protocol-based data management
- **Machine Learning & AI:** Federated learning, model optimization, and secure aggregation
- **DAO Governance:** Proposal management, quadratic voting, and time-locked execution
- **Hexagonal Architecture:** Clean separation between core domains and adapters

## Project Structure

```text
anya-core/
├── src/                  # Core source code
│   ├── bitcoin/          # Bitcoin protocol integration
│   │   └── lightning.rs  # Lightning Network implementation
│   ├── web5/             # Web5 protocol implementation
│   ├── ml/               # Machine learning components
│   ├── dao/              # DAO implementation
│   └── lib.rs            # Main library entry point
├── scripts/              # Utility scripts
├── docs/                 # Documentation
│   └── bitcoin/          # Bitcoin documentation
│       └── lightning.md  # Lightning Network documentation
├── tests/                # Test suite
├── examples/             # Example code
└── Cargo.toml            # Rust dependencies
```

## Quick Start

### Prerequisites

- Rust 1.70+ (`cargo`, `rustc`)
- Optional: Bitcoin Core 25.0+ for real Bitcoin network integration
- Optional: Web5 DWN Node for decentralized data management
- Optional: Python 3.10+ with ML frameworks for advanced ML features

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/anya/anya-core.git
   cd anya-core
   ```

2. Build the library:

   ```bash
   cargo build --release
   ```

3. Run tests:

   ```bash
   cargo test
   ```

### Usage

Here's a basic example of using Anya Core in your Rust project:

```rust
use anya_core::{AnyaCore, AnyaConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with default configuration
    let anya = AnyaCore::default()?;
    
    // Check that the system is operational
    println!("System operational: {}", anya.is_operational());
    
    // Get system status
    let status = anya.get_status()?;
    println!("ML enabled: {}",      status.ml_enabled);
    println!("Web5 enabled: {}",    status.web5_enabled);
    println!("Bitcoin enabled: {}", status.bitcoin_enabled);
    println!("DAO enabled: {}",     status.dao_enabled);
    
    // Use components as needed
    if let Some(bitcoin_manager) = &anya.bitcoin_manager {
        let wallet = bitcoin_manager.create_wallet(
            anya_core::bitcoin::wallet::WalletType::Standard, 
            "My Wallet"
        )?;
        println!("Created wallet: {}", wallet.name());
    }
    
    if let Some(web5_manager) = &anya.web5_manager {
        let did = web5_manager.create_did("key")?;
        println!("Created DID: {}", did.did);
    }
    
    Ok(())
}
```

## Bitcoin and Lightning Network

Anya Core provides comprehensive Bitcoin and multi-layer ecosystem functionality:

### Bitcoin Core Features

- Wallet management with advanced key derivation (BIP32, BIP39, BIP44)
- Transaction creation, signing, and broadcasting
- UTXO management and coin selection
- Network communication with Bitcoin Core or Electrum servers
- Support for SegWit, Taproot, and other Bitcoin script types

### Lightning Network Features

The Lightning Network implementation provides fast, low-cost Bitcoin transactions:

```rust
use anya_core::{AnyaCore, AnyaConfig};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with Lightning Network enabled
    let mut config = AnyaConfig::default();
    config.bitcoin_config.lightning_enabled = true;
    
    let anya = AnyaCore::new(config)?;
    
    // Access the Bitcoin manager
    if let Some(bitcoin_manager) = &anya.bitcoin_manager {
        // Access the Lightning node
        if let Some(lightning_node) = bitcoin_manager.lightning_node() {
            // Get node information
            let node_info = lightning_node.get_node_info()?;
            println!("Lightning node pubkey: {}", node_info.pubkey);
            
            // Connect to a peer
            lightning_node.connect_peer(
                "02eec7245d6b7d2ccb30380bfbe2a3648cd7a942653f5aa340edcea1f283686619",
                "127.0.0.1",
                9735
            )?;
            
            // Open a channel
            let channel = lightning_node.open_channel(
                "02eec7245d6b7d2ccb30380bfbe2a3648cd7a942653f5aa340edcea1f283686619",
                100_000,              // 100,000 satoshis
                Some(10_000 * 1000),  // 10,000 satoshis (in millisatoshis)
                false                 // public channel
            )?;
            println!("Channel opened with ID: {}", channel.channel_id);
            
            // Create an invoice
            let invoice = lightning_node.create_invoice(
                Some(50_000),        // 50,000 millisatoshis
                "Test payment",
                Some(3600)           // 1 hour expiry
            )?;
            println!("Invoice: {}", invoice.bolt11);
            
            // Pay an invoice
            let payment = lightning_node.pay_invoice(&invoice.bolt11, None)?;
            println!("Payment sent with hash: {}", payment.payment_hash);
            
            // Create Bitcoin-Lightning Bridge
            let lightning_node_arc = Arc::new(lightning_node.clone());
            let bridge = bitcoin::lightning::BitcoinLightningBridge::new(lightning_node_arc)?;
            
            // Create a funding address for a new channel
            let address = bridge.create_funding_address(
                "03f25d220b14f3daae528bbb98cf142caf3477c8d5258d9f81b0af0370163f0df2",
                200_000,              // 200,000 satoshi channel
                None,                 // No initial push
                false                 // Public channel
            )?;
            println!("Send funds to {} to open the channel", address);
        }
    }
    
    Ok(())
}
```

### Taproot and Schnorr Signatures

Anya Core implements Taproot and Schnorr signature capabilities, enabling:

- Enhanced privacy and security through key and script aggregation
- Reduced transaction sizes for complex scripts
- More efficient multi-signature setups
- Script-path and key-path spending options
- Advanced signature adaptor techniques

```rust
// Create a Taproot output
let taproot_descriptor = bitcoin_manager.create_taproot_descriptor(
    vec![script1, script2, script3],  // Alternative spending conditions
    main_internal_key,                // Internal key
    Some("Custom leaf version")       // Optional leaf version
)?;

// Generate a Taproot address
let taproot_address = wallet.derive_address(taproot_descriptor)?;

// Create and sign a transaction using Schnorr signatures
let tx = wallet.create_transaction(
    vec![(taproot_address.to_string(), 50000)],
    FeeRate::from_sat_per_vb(5.0),
    SignatureType::Schnorr
)?;
```

### DLC (Discreet Log Contracts)

Anya Core supports Discreet Log Contracts for trust-minimized Bitcoin contracts:

- Oracle integration for contract condition verification
- Multi-oracle setup with threshold conditions
- Contract creation, execution and settlement
- Contract templates for common use cases (futures, options, insurance)
- Non-custodial escrow mechanisms

```rust
// Create a DLC with specified terms
let dlc = bitcoin_manager.create_dlc(
    counterparty_pubkey,                // Counterparty public key
    vec![oracle1, oracle2],             // Oracles (with threshold settings)
    outcome_payouts,                    // Outcome-based payment distribution
    collateral_amount,                  // Your collateral amount
    timelock_date                       // Contract expiration date
)?;

// Execute a DLC based on oracle attestation
let outcome = dlc.execute_with_attestations(
    vec![attestation1, attestation2]    // Oracle attestations
)?;
```

### Multi-Layer Ecosystem

Anya Core embraces Bitcoin's multi-layer ecosystem with integrations for:

#### Stacks

```rust
// Interact with Stacks blockchain
let stacks_manager = anya.stacks_manager()?;

// Deploy a Clarity smart contract
let contract_id = stacks_manager.deploy_contract(
    "my-contract",
    clarity_code,
    deployment_options
)?;

// Call a Clarity smart contract function
let result = stacks_manager.call_contract_function(
    contract_id,
    "get-balance",
    vec![Value::Principal(caller_address)]
)?;
```

#### RSK (Rootstock)

```rust
// Initialize RSK functionality
let rsk_manager = anya.rsk_manager()?;

// Deploy a Solidity smart contract on RSK
let contract = rsk_manager.deploy_contract(
    contract_abi,
    contract_bytecode,
    constructor_args,
    gas_limit,
    gas_price
)?;

// Create a two-way peg transaction (BTC -> RBTC)
let peg_tx = rsk_manager.create_peg_in_transaction(
    bitcoin_amount,
    rsk_address,
    peg_options
)?;
```

#### RGB

```rust
// Create and issue an RGB asset
let rgb_asset = anya.rgb_manager()?.issue_asset(
    "MyToken",                          // Asset name
    1_000_000,                          // Total supply
    "An example token on RGB",          // Description
    rgb::AssetType::Fungible,
    metadata
)?;

// Transfer RGB assets
let transfer = rgb_manager.create_transfer(
    rgb_asset.id,
    recipient_address,
    500,                                // Amount
    transfer_options
)?;
```

#### Liquid

```rust
// Initialize Liquid functionality
let liquid_manager = anya.liquid_manager()?;

// Issue a Liquid asset
let asset = liquid_manager.issue_asset(
    "EXAMPLE",                          // Ticker
    1_000_000,                          // Amount
    "Example Asset on Liquid",          // Name
    0,                                  // Precision
    issuance_options
)?;

// Create a confidential transaction
let tx = liquid_manager.create_confidential_transaction(
    inputs,
    outputs,
    confidentiality_options
)?;
```

### Web5 Bitcoin Integration

Anya Core combines Bitcoin with Web5 capabilities:

```rust
// Create a Bitcoin-anchored DID
let did = anya.web5_manager()?.create_did(
    "btcr",                             // Bitcoin Reference DID method
    Some(bitcoin_wallet)                // Associated Bitcoin wallet
)?;

// Sign and anchor a DID document to Bitcoin
let operation = web5_manager.anchor_did_to_bitcoin(
    did,
    document_update,
    anchoring_options
)?;

// Verify a Bitcoin-anchored credential
let verification = web5_manager.verify_credential(
    credential,
    bitcoin_proof_options
)?;
```

### Cross-Chain Functionality

Anya Core enables seamless operations across the Bitcoin ecosystem:

```rust
// Perform atomic swap between Bitcoin and Liquid
let swap = anya.cross_chain_manager()?.create_atomic_swap(
    BitcoinAsset::BTC(0.1),             // 0.1 BTC
    LiquidAsset::L_BTC(0.099),          // 0.099 L-BTC (accounting for fees)
    counterparty_details,
    timelock_options
)?;

// Create a cross-chain DLC between Bitcoin and Stacks
let cross_dlc = cross_chain_manager.create_cross_chain_dlc(
    BitcoinNetwork::Mainnet,
    StacksNetwork::Mainnet,
    contract_terms,
    oracle_details
)?;
```

### Key Lightning Features

1. **Node Management**: Run a Lightning node with peer connections
2. **Channel Management**: Open, close, and manage payment channels
3. **Payments & Invoices**: Create and pay BOLT-11 invoices
4. **Bitcoin Integration**: Seamless integration with Bitcoin on-chain operations
5. **Wallet Integration**: Use the same wallet for both Bitcoin and Lightning

### Demo Programs

Try the Lightning Network functionality with our demo programs:

```bash
# Build and run the Lightning demo
cargo run --bin lightning_demo

# Test all Lightning Network functionality
cargo test --features=full bitcoin::lightning
```

### Configuration

You can customize Anya Core behavior through the `AnyaConfig` struct:

```rust
use anya_core::{AnyaCore, AnyaConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom configuration
    let mut config = AnyaConfig::default();
    
    // Customize Bitcoin configuration
    config.bitcoin_config.network = bitcoin::Network::Testnet;
    config.bitcoin_config.rpc_url = Some("http://localhost:18332".to_string());
    
    // Enable Lightning Network
    config.bitcoin_config.lightning_enabled = true;
    
    // Customize ML configuration
    config.ml_config.use_gpu = true;
    
    // Initialize with custom configuration
    let anya = AnyaCore::new(config)?;
    
    // Use the system as needed
    
    Ok(())
}
```

## Packaging

To create a distributable package:

```bash
./scripts/package.sh
```

This will create a package in the `dist` directory, including all necessary libraries, binaries, and documentation.

Options:

- `--version VERSION`: Set package version
- `--output-dir DIR`: Set output directory
- `--name NAME`: Set package name
- `--debug`: Build debug version instead of release
- `--no-docs`: Don't include documentation
- `--no-examples`: Don't include examples

## Development

### Running Tests

```bash
cargo test
```

For a more comprehensive test that initializes the entire system:

```bash
./scripts/test_core.sh
```

### Lightning Network Testing

To specifically test the Lightning Network functionality:

```bash
# Test the Lightning Network components
cargo test --features=full bitcoin::lightning

# Run the Lightning Network demo
cargo run --bin lightning_demo
```

### Adding New Components

The system follows hexagonal architecture principles, so when adding new components:

1. Define interfaces (ports) in the appropriate module
2. Implement adapters for external dependencies
3. Keep core business logic independent from external concerns
4. Update the main `AnyaCore` struct to include your new component

## API Documentation

Generate and view API documentation with:

```bash
cargo doc --open
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.

## Acknowledgments

Special thanks to our contributors and the following projects:

- Bitcoin Core
- Lightning Development Kit (LDK)
- Web5/TBD
- TensorFlow/PyTorch

## Last Updated
2025-03-01
