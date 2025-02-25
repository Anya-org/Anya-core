# Anya Core

Anya Core is a powerful platform combining Bitcoin/crypto functionality, ML-based analytics, and Web5 decentralized data management with hexagonal architecture design principles.

## Features

- **Bitcoin Integration:** Bitcoin Core, Lightning Network, DLC support, and Taproot/Schnorr signatures
- **Web5 Integration:** Decentralized Web Nodes (DWN), Decentralized Identifiers (DIDs), Protocol-based data management
- **Machine Learning & AI:** Federated learning, model optimization, and secure aggregation
- **DAO Governance:** Proposal management, quadratic voting, and time-locked execution
- **Hexagonal Architecture:** Clean separation between core domains and adapters

## Project Structure

```
anya-core/
├── src/                  # Core source code
│   ├── bitcoin/          # Bitcoin protocol integration
│   ├── web5/             # Web5 protocol implementation
│   ├── ml/               # Machine learning components
│   ├── dao/              # DAO implementation
│   └── lib.rs            # Main library entry point
├── scripts/              # Utility scripts
├── docs/                 # Documentation
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
    println!("ML enabled: {}", status.ml_enabled);
    println!("Web5 enabled: {}", status.web5_enabled);
    println!("Bitcoin enabled: {}", status.bitcoin_enabled);
    println!("DAO enabled: {}", status.dao_enabled);
    
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
- Lightning Network Daemon
- Web5/TBD
- TensorFlow/PyTorch

*Last updated: 2025-02-24* 