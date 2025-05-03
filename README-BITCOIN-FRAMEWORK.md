# Anya Core - Bitcoin Development Framework v2.5

A comprehensive Bitcoin development framework implementing the hexagonal architecture pattern and maintaining strict protocol adherence with Bitcoin's core tenets of decentralization, immutability, and censorship resistance.

## Features

- **Full BIP Compliance**: Supports BIP-341 (Taproot), BIP-342 (Tapscript), and BIP-174 (PSBT)
- **Hexagonal Architecture**: Cleanly separated core logic, adapters, and ports
- **Layer 2 Solutions**: Lightning Network, RGB, RSK, and DLC implementations
- **Monitoring**: Prometheus metrics for real-time observability
- **Security Validation**: Comprehensive transaction validation with Taproot support

## Architecture

```
                      +----------------+
                      |  Bitcoin Core  |
                      +-------+--------+
                              |
                      +-------v--------+
                      |  Adapter Layer |
                      +-------+--------+
                              |
+----------------+    +-------v--------+    +----------------+
|   External     |    |   Application  |    |   Monitoring   |
|   Interfaces   <----+   Core Logic   +---->   & Metrics    |
| (APIs, Wallets)|    +-------+--------+    | (Prometheus)   |
+----------------+            |             +----------------+
                      +-------v--------+
                      |   Protocol     |
                      |   Adapters     |
                      +-------+--------+
                              |
                      +-------v--------+
                      |  Blockchain    |
                      |  Network       |
                      +----------------+
```

## Getting Started

### Prerequisites

- Rust toolchain (1.56 or later)
- Node.js 14+ (for web interfaces)
- Docker (optional, for containerized deployment)

### Installation

1. Clone the repository
```bash
git clone https://github.com/yourusername/anya-core.git
cd anya-core
```

2. Build the project
```bash
cargo build --release
```

3. Run tests
```bash
cargo test --all-features
```

## Development Workflow

- Commit messages must reference BIP standards: `feat(taproot): implement key path spending @BIP-341`
- All Taproot changes must pass compliance tests
- Use the hexagonal architecture pattern for all new components

## Verification

Verify Bitcoin Development Framework compliance:

```bash
./scripts/verify-framework.sh
```

## Security Validation

All transactions must pass comprehensive validation checks:

```rust
use anya_core::security::validate_transaction;

fn process_transaction(tx_bytes: &[u8]) {
    match validate_transaction(tx_bytes) {
        Ok(_) => println!("Transaction valid"),
        Err(e) => println!("Transaction invalid: {:?}", e),
    }
}
```

## Metrics

Start the metrics server to expose Prometheus metrics:

```rust
use anya_core::monitoring::start_monitoring;

fn main() {
    // Start metrics server on port 3000
    start_monitoring().expect("Failed to start monitoring");
    
    // Your application code
}
```

## Compliance Checklist

- [x] BIP 341/342 (Taproot)
- [x] BIP 174 (PSBT)
- [x] Miniscript Support
- [x] Testnet Validation

## Author

- **Author**: bo_thebig
- **Email**: botshelomokokoka@gmail.com

## License

This project is licensed under the MIT License - see the LICENSE file for details. 