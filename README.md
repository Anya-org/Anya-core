---
title: "Anya Core Readme"
description: "Comprehensive documentation for Anya Core platform."
---

# Anya Core

A powerful platform combining Bitcoin/crypto functionality, ML-based analytics, Web5 decentralized data management, and a Bitcoin-style DAO governance system.

[![BIP-341](https://img.shields.io/badge/BIP-341_Compliant-green)](https://bips.xyz/341)
[![AIS-3](https://img.shields.io/badge/AIS-3_Secured-blue)](https://bips.xyz)
[![PSBT-v2](https://img.shields.io/badge/PSBT_v2-100%25-brightgreen)](https://bips.xyz/370)
[![BIP-Standards](https://img.shields.io/badge/BIP-Standards_Compliant-green)](https://bips.xyz)

*Last Updated: June 7, 2025*  
**Latest Release: v1.1.0**

> **Note:** This release includes comprehensive Taproot updates, enhanced HSM security, and a refactored installation system following official BIPs.

---

## Table of Contents
- [Overview](#overview)
- [Quick Start](#quick-start)
- [Installation & Configuration](#installation--configuration)
- [Documentation](#documentation)
- [Core Features](#core-features)
- [Technical Stack](#technical-stack)
- [Governance & DAO](#governance--dao)
- [Storage Architecture](#storage-architecture)
- [Testing & CI/CD](#testing--cicd)
- [Contributing](#contributing)
- [License](#license)
- [Additional Resources](#additional-resources)

---

## Overview

Anya Core is a modular, enterprise-ready platform for Bitcoin, Web5, and AI/ML applications. It features:
- Bitcoin Core & Lightning integration
- Web5 DWN decentralized storage
- DAO governance and tokenomics
- Hardware Security Module (HSM) support
- Modular, hexagonal architecture
- Advanced monitoring, security, and compliance

> **AI Labeling:** This project follows the [canonical AI Labeling System](docs/standards/AI_LABELING.md) based on official BIPs. All components are labeled with appropriate Core and Extended category labels.

![Anya Architecture](docs/images/anya_architecture.png)

---

## Quick Start

### 1. Clone and Setup
```bash
git clone https://github.com/anya/anya-core.git
cd anya-core
./scripts/setup.sh
cargo build --release
```

### 2. Configuration
- Main config: `config/anya.conf` (TOML/INI style)
- All settings can be overridden with environment variables.

Example environment variables:
```env
WEB5_DWN_URL=http://localhost:3000
BITCOIN_RPC_URL=http://localhost:8332
BITCOIN_RPC_USER=user
BITCOIN_RPC_PASS=password
ML_MODEL_PATH=/path/to/models
```

### 3. Install & Run
```bash
# Standard install
tsudo ./scripts/install.sh
# Minimal install
sudo ./scripts/install.sh --type=minimal
# Full install
sudo ./scripts/install.sh --type=full
```

### 4. Verify
```bash
./scripts/install/utils/monitor_health.sh
./scripts/test/debug_test.sh
```

---

## Installation & Configuration

### Prerequisites
- Rust 1.70+
- Bitcoin Core 24.0+
- Web5 DWN Node
- Clarinet 2.3.0+ (for DAO contracts)

### Configuration System
- **File:** `config/anya.conf` (edit `[web5] dwn_endpoint` for Web5 node URL)
- **Environment:** All config can be overridden with env vars (e.g., `export WEB5_DWN_URL=...`)
- **Secrets:** Set sensitive data (like DWN node URL for CI) as GitHub Actions secrets (e.g., `DWN_NODE_URL`). Never commit `.env` files with secrets.

#### Web5 DWN Endpoint Alignment
- **Local:** Set in `config/anya.conf` under `[web5]` as `dwn_endpoint = "http://localhost:3000"`
- **CI/CD:** Set `DWN_NODE_URL` secret in GitHub repo to match your local value.
- **Sync Script:** Use `./sync-dwn-secret.sh <github-repo> <github-username>` to sync your local value to GitHub Actions (requires GitHub CLI).

---

## Documentation

- [Documentation System Guide](docs/DOCUMENTATION_SYSTEM.md)
- [View Documentation](https://docs.anya.org/)
- [Contribute to Docs](CONTRIBUTING.md#documentation)
- [Documentation Standards](docs/standards/MARKDOWN_STYLE_GUIDE.md)

### Documentation Tools
| Script | Purpose |
|--------|---------|
| `./scripts/setup_docs.sh` | Set up docs environment |
| `./scripts/serve_docs.sh` | Serve docs locally |
| `./scripts/update_docs.sh` | Update docs to standards |
| `./scripts/review_docs.sh` | Review docs for issues |
| `./scripts/check_links.sh` | Check for broken links |
| `./scripts/doc_status.sh` | Docs status report |
| `./scripts/generate_toc.sh` | Generate/update TOC |

---

## Core Features

### Architecture
- Hexagonal, modular, and domain-driven
- Clean separation of concerns (ports/adapters)
- Advanced error handling, telemetry, and health monitoring
- Thread-safe caching and circuit breaker patterns

### HSM Support
- Modular HSM provider architecture
- Software and hardware HSM support (TPM, PKCS#11, YubiHSM, Ledger)
- Bitcoin-specific HSM operations
- Feature-flag enabled for flexible deployment

### DAO & Tokenomics
- Bitcoin-style token issuance (21B supply)
- Adaptive emission schedule, DAO-controlled halving
- Decentralized governance, proposals, and voting
- Integrated DEX, protocol treasury, and logging

### Blockchain & Web5
- Bitcoin Core & Lightning support
- Taproot/Schnorr, DLC, Layer 2, cross-chain
- Web5 DWN decentralized storage, identity-centric

### ML & AI
- Model optimization, federated learning, analytics
- Real-time inference, distributed training, A/B testing

### Security & Monitoring
- Comprehensive security operations, audit trail, threat detection
- Distributed tracing, metrics, dashboards

---

## Technical Stack

- Rust, Python, Node.js, React
- Bitcoin Core, Web5, Clarinet
- Prometheus, Grafana (monitoring)
- Data storage: DWN (Decentralized Web Node)

#### Core Dependencies (Cargo.toml)
```toml
[dependencies]
tokio = { version = "1.34", features = ["full"] }
bitcoin = { version = "0.31.0", features = ["rand"] }
tracing = { version = "0.1", features = ["attributes"] }
metrics = "0.21"
web5 = { version = "0.1.0", features = ["storage"] }
ml-core = { version = "0.1.0" }
clarity-repl = { git = "https://github.com/hirosystems/clarinet", tag = "v1.7.0" }
```

---

## Governance & DAO

- **Token:** 21,000,000,000 AGT, adaptive halving, DAO-controlled emission
- **Distribution:** 35% Treasury, 25% Liquidity, 20% Team, 15% Community, 5% Partners
- **Governance:** Proposals, ML-driven scoring, treasury management, multi-chain
- **Security:** Multi-sig, threat detection, ZK governance
- **Compliance:** BPC-3, DAO-4, AIS-3

---

## Storage Architecture

- Web5 DWN for decentralized, encrypted, protocol-driven storage
- Identity-based access (DIDs), automatic replication, flexible querying
- No central DB dependency, offline-first

---

## Testing & CI/CD

- Unit, integration, and property testing
- GitHub Actions workflows for build, test, compliance, and deployment
- All secrets and config aligned between local and CI
- Use `run-all.ps1` to clean, verify, and test all components

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

This project is dual-licensed under [Apache 2.0](LICENSE.md) and MIT. Enterprise features are under a separate license (see [Enterprise License](./enterprise/LICENSE)).

---

## Additional Resources

- [Documentation](https://docs.anya-core.org)
- [API Reference](https://api.anya-core.org)
- [Community Forum](https://community.anya-core.org)
- [Development Blog](https://blog.anya-core.org)

---

## Acknowledgments

Special thanks to our contributors and the following projects:
- Bitcoin Core
- Lightning Network
- Web5
- TBD
- Block

---

## Validation & Compliance

Run the following to verify system compliance:
```shell
./scripts/validate_upgrade.ps1 -CheckAll -ProtocolLevel 3
```

This project adheres to:
- Bitcoin Protocol Compliance Level 3 (BPC-3)
- DAO Governance Standard Level 4 (DAO-4)
- AI Security Standard Level 3 (AIS-3)

---

Author: Bo The Big (<botshelomokokoka@gmail.com>)

---

