# Anya Core - Bitcoin Development Framework v2.5

This is a summary of the Bitcoin Development Framework implementation in Anya Core. For the complete implementation details, please refer to the authoritative source document: [BITCOIN_FRAMEWORK_IMPLEMENTATION.md](./BITCOIN_FRAMEWORK_IMPLEMENTATION.md)

## Overview

This framework implements Bitcoin development using the hexagonal architecture pattern, maintaining strict protocol adherence with Bitcoin's core tenets.

## Key Features

- **Full BIP Compliance**: BIP-341/342 (Taproot), BIP-174 (PSBT), BIP-340 (Schnorr)
- **Hexagonal Architecture**: Core logic, adapters, and ports cleanly separated
- **Layer 2 Solutions**: Lightning Network, RGB, RSK, and DLC support
- **Monitoring**: Prometheus metrics integration
- **Security**: Comprehensive transaction validation

## Simplified Architecture

```
                      +----------------+
                      |  Bitcoin Core  |
                      +-------+--------+
                              |
+----------------+    +-------v--------+    +----------------+
|   External     |    |   Application  |    |   Monitoring   |
|   Interfaces   <----+   Core Logic   +---->   & Metrics    |
+----------------+    +----------------+    +----------------+
```

## Quick Start

### Prerequisites

- Rust toolchain (1.56+)
- Node.js 14+ (for web interfaces)

### Build

```bash
cargo build --release
cargo test --all-features
```

## Compliance Checklist

- [x] BIP-341/342 (Taproot)
- [x] BIP-174 (PSBT)
- [x] BIP-340 (Schnorr)
- [x] Hexagonal Architecture

## Author

- **Author**: bo_thebig
- **Email**: <botshelomokokoka@gmail.com>
- **Last Updated**: 2025-05-01

For complete implementation details, development guidelines, and architecture specifications, please see [BITCOIN_FRAMEWORK_IMPLEMENTATION.md](./BITCOIN_FRAMEWORK_IMPLEMENTATION.md). 
