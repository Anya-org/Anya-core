---
title: "Anya Core"
description: "Enterprise Bitcoin Infrastructure Platform with Layer2 Protocol Integration"
---

# Anya Core `AIR-3` `AIS-3` `BPC-3` `RES-3`

A modular Bitcoin infrastructure platform designed for enterprise applications, Layer2 protocol integration, and decentralized AI/ML systems.

[![Rust](https://img.shields.io/badge/Rust-1.70+-green)](https://rust-lang.org)

Last Updated: August 9, 2025
Version: 1.3.0
Status: ✅ Build and crates.io dry-run verified; deployment readiness depends on your environment

## 📖 Documentation Hub

👉 **[Complete Documentation](docs/)** - Comprehensive guides and references

**Quick Navigation:**

- [🚀 Getting Started](#quick-start) - Setup and basic usage
- [⚙️ Installation Guide](#installation--configuration) - Detailed setup instructions
- [🏗️ Architecture Overview](#core-features) - System design and components
- [🔧 Bitcoin Integration](docs/bitcoin/) - BIP compliance and Bitcoin features
- [⚡ Layer2 Protocols](docs/layer2/) - Lightning, RGB, DLC, and more
- [🔒 Security Guide](docs/security/) - Security policies and best practices

**Additional Documentation:**

- Security: docs/security/README.md
- Bitcoin: docs/bitcoin/README.md
- Layer2: docs/layer2/README.md
- AI Labeling: docs_legacy/standards/AI_LABELING.md

---

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Installation & Configuration](#installation--configuration)
- [Core Features](#core-features)
- [Technical Implementation](#technical-implementation)
- [Development Status](#development-status)
- [Testing & Validation](#testing--validation)
- [Contributing](#contributing)
- [License & Resources](#license--resources)

---

## Overview

Anya Core is an advanced integrated system combining Bitcoin/crypto functionality, ML-based analytics, and Web5 decentralized data management with enterprise-grade security and revenue tracking. Built on a hexagonal architecture, it provides a robust, scalable, and secure platform for institutional-grade operations.

**Core Bitcoin Features:**

- Bitcoin transaction processing and validation
- Taproot (BIP-341) and Tapscript (BIP-342) support
- PSBT (Partially Signed Bitcoin Transactions) implementation
- Schnorr signatures (BIP-340) integration
- Comprehensive Bitcoin wallet functionality

**Layer2 Protocol Framework:**

- Lightning Network payment channels
- RGB protocol for asset issuance
- Discrete Log Contracts (DLC)
- State channels implementation
- Taproot Assets support
- Cross-protocol interoperability

**Enterprise Infrastructure:**

- Hexagonal architecture with clean separation of concerns
- Hardware Security Module (HSM) integration
- Comprehensive monitoring and metrics
- Modular component design
- Production-ready security measures

> AI Labeling: This project follows an AI labeling system with standardized compliance markers for AI readiness, security, and Bitcoin protocol compliance. See: docs_legacy/standards/AI_LABELING.md

**System Architecture:**

```text
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   Applications  │    │   Web APIs   │    │  External Tools │
└─────────────────┘    └──────────────┘    └─────────────────┘
         │                       │                       │
    ┌────┴───────────────────────┴───────────────────────┴────┐
    │                   Anya Core                             │
    │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  │
    │  │   Bitcoin   │  │   Layer2    │  │   Enterprise    │  │
    │  │   Protocol  │  │  Protocols  │  │  Infrastructure │  │
    │  └─────────────┘  └─────────────┘  └─────────────────┘  │
    └─────────────────────────────────────────────────────────┘
```

---

## Quick Start

### 1. Prerequisites

```bash
# System requirements
- Rust 1.70+ with Cargo
- Git for version control
- 4GB+ RAM available
- 10GB+ disk space

# Optional for full features
- Bitcoin Core 24.0+ (for full Bitcoin integration)
- Docker (for containerized deployment)
```

### 2. Clone and Build

```bash
git clone https://github.com/Anya-org/Anya-core.git
cd Anya-core
cargo build --release
```

### 3. Basic Configuration

Create a basic configuration file:

```bash
mkdir -p ~/.config/anya-core
cat > ~/.config/anya-core/config.toml << EOF
[bitcoin]
network = "testnet"
rpc_url = "http://localhost:18332"

[layer2]
enabled = ["lightning", "rgb"]

[security]
hsm_enabled = false
log_level = "info"
EOF
```

### 4. Run Tests

```bash
# Run core functionality tests
cargo test --lib

# Run integration tests (requires setup)
cargo test --test integration_tests
```

### 5. Basic Usage

```rust
use anya_core::{BitcoinClient, Layer2Manager};

// Initialize Bitcoin client
let bitcoin_client = BitcoinClient::new(config)?;

// Create Layer2 manager for protocol integration
let layer2_manager = Layer2Manager::new()?;

// Process Bitcoin transactions
let tx_result = bitcoin_client.process_transaction(raw_tx)?;
println!("Transaction processed: {}", tx_result.txid);
```

---

## Installation & Configuration

### System Requirements

**Minimum Requirements:**

-   CPU: 8+ cores recommended
-   RAM: 16GB+ recommended
-   Storage: 1TB+ SSD recommended
-   Network: 1Gbps+ recommended
-   GPU: Optional for ML acceleration
-   NPU: Optional for advanced ML

**Software Dependencies:**

-   Rust 1.70+
-   PostgreSQL 14+
-   Redis 7.0+
-   Bitcoin Core 24.0+
-   Python 3.10+ (for ML components)

### Installation Methods

#### Option 1: Quick Setup (Development)

```bash
# Clone and build
git clone https://github.com/Anya-org/Anya-core.git
cd Anya-core
cargo build --release

# Run setup script
./scripts/setup.sh
```

#### Option 2: Manual Installation

```bash
# Install Rust if not present
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build --release --features "complete"

# Install system-wide (optional)
sudo cp target/release/anya-core /usr/local/bin/
```

### Configuration System

**Configuration Files:**

- Main config: `~/.config/anya-core/config.toml`
- Bitcoin settings: `~/.config/anya-core/bitcoin.toml`
- Layer2 protocols: `~/.config/anya-core/layer2.toml`

**Example Configuration:**

```toml
# ~/.config/anya-core/config.toml
[bitcoin]
network = "testnet"                    # testnet, mainnet, regtest
rpc_url = "http://localhost:18332"     # Bitcoin Core RPC
data_dir = "~/.anya-core/bitcoin"

[layer2]
enabled_protocols = ["lightning", "rgb", "dlc"]
lightning_node_url = "localhost:9735"

[security]
hsm_enabled = false                    # Enable for production
audit_logging = true
max_concurrent_operations = 100

[monitoring]
metrics_enabled = true
prometheus_port = 9090
log_level = "info"                     # trace, debug, info, warn, error
```

**Environment Variable Overrides:**

```bash
# Override any config value with environment variables
export ANYA_BITCOIN_NETWORK=mainnet
export ANYA_BITCOIN_RPC_URL=http://localhost:8332
export ANYA_SECURITY_HSM_ENABLED=true
export ANYA_MONITORING_LOG_LEVEL=debug
```

---

## Core Features

### Bitcoin Protocol Implementation

**Consensus Layer:**

- **BIP-341 (Taproot):** Full implementation with key-path and script-path spending
- **BIP-342 (Tapscript):** Complete script validation and execution
- **BIP-340 (Schnorr):** Schnorr signature verification and batch validation
- **BIP-174 (PSBT):** Partially Signed Bitcoin Transaction processing
- **Transaction Validation:** Comprehensive validation including SegWit and Taproot

**Wallet Functionality:**

- HD wallet implementation (BIP-32/39/44)
- Multi-signature support
- Hardware wallet integration ready
- PSBT creation, signing, and finalization

### Layer2 Protocol Framework

**Currently Implemented:**

- **Lightning Network:** Payment channel management and routing foundation
- **RGB Protocol:** Client-side validation framework for assets
- **State Channels:** Generalized off-chain state management
- **DLC (Discrete Log Contracts):** Oracle-based contract framework
- **Taproot Assets:** Asset issuance on Bitcoin using Taproot

**Integration Status:**

- **BOB Protocol:** Bitcoin-EVM bridge (framework ready)
- **RSK Integration:** Rootstock sidechain support (in development)
- **Liquid Network:** Sidechain integration (planned)
- **Stacks:** Bitcoin layer for smart contracts (planned)

### Enterprise Infrastructure

**Architecture:**

- **Hexagonal Design:** Clean separation between business logic and adapters
- **Modular Components:** Independent protocol implementations
- **Error Handling:** Comprehensive error types and recovery mechanisms
- **Logging & Metrics:** Structured logging with Prometheus metrics

**Security Features:**

- **HSM Integration:** Hardware Security Module support for key management
- **Audit Trail:** Comprehensive logging for compliance requirements
- **Input Validation:** Strict validation for all external inputs
- **Constant-time Operations:** Cryptographic operations resistant to timing attacks

**Monitoring & Operations:**

- **Health Checks:** System health monitoring endpoints
- **Metrics Collection:** Performance and operational metrics
- **Configuration Management:** Flexible configuration with environment overrides
- **Resource Management:** Memory and CPU usage optimization

---
---

## Technical Implementation

### Dependencies & Technology Stack

**Core Languages & Frameworks:**

- **Rust:** Primary implementation language for performance and safety
- **Tokio:** Async runtime for concurrent operations
- **Bitcoin Libraries:** `bitcoin`, `secp256k1`, `bdk` for Bitcoin protocol implementation

**Key Dependencies (from Cargo.toml excerpt):**

```toml
[dependencies]
# Bitcoin ecosystem
bitcoin = { version = "0.32.6", features = ["std", "serde", "rand", "secp-recovery", "base64", "rand-std" ] }
secp256k1 = { version = "0.29.1", features = ["global-context", "recovery", "rand"] }
miniscript = { version = "12.3.5", features = ["std", "compiler"] }
bdk_wallet = { version = "2.0.0", features = ["std", "all-keys"], optional = true }

# Async runtime and networking
tokio = { version = "1.45.1", features = ["full"] }
hyper = { version = "1.6.0", features = ["full"] }

# Serialization and data handling
serde = { version = "1.0.219", features = ["derive", "rc"] }
serde_json = { version = "1.0.140", features = ["std", "preserve_order", "arbitrary_precision"] }

# Error handling and logging
anyhow = { version = "1.0.98", features = ["std", "backtrace"] }
tracing = { version = "0.1.41" }
```

### Project Structure

```text
src/
├── bitcoin/              # Bitcoin protocol implementation
│   ├── consensus/        # Consensus rules and validation
│   ├── script/          # Script execution and Taproot support
│   └── transaction/     # Transaction processing and PSBT
├── layer2/              # Layer2 protocol implementations
│   ├── lightning/       # Lightning Network integration
│   ├── rgb/            # RGB protocol for assets
│   ├── dlc/            # Discrete Log Contracts
│   └── manager.rs      # Protocol coordination
├── security/           # Security and cryptographic operations
│   ├── hsm/           # Hardware Security Module integration
│   └── audit/         # Security auditing framework
├── core/              # Core system components
│   ├── config/        # Configuration management
│   └── error/         # Error handling
└── api/               # External API interfaces
```

### BIP Compliance Status

**Fully Implemented:**

- ✅ **BIP-340:** Schnorr Signatures for secp256k1
- ✅ **BIP-341:** Taproot: SegWit version 1 spending rules
- ✅ **BIP-342:** Validation of Taproot Scripts
- ✅ **BIP-174:** Partially Signed Bitcoin Transaction Format

**In Progress:**

- 🔄 **BIP-370:** PSBT Version 2 (85% complete)
- 🔄 **BIP-322:** Generic Signed Message Format (partial)

<!-- Test coverage varies by environment. See CI artifacts (ci_metrics.json) or the pipeline dashboard for current numbers. -->

---

### Build & Test

```bash
# Build (all features)
cargo build -r --all-features

# Run unit tests
cargo test --lib

# Optional: run integration tests (requires local services)
cargo test --tests -- --nocapture

# Optional: security checks
cargo audit
cargo geiger
```

### Development Status

Default behavior: if no external nodes are configured or reachable, Anya Core promotes itself to a self-node (master) for each enabled Layer2 protocol. This replaces the prior default of “pure simulation.” To run against real networks, provide real node endpoints (e.g., Lightning node URL/macaroon/cert, RGB node) using `.env` or config files. See: docs/layer2/PRODUCTION_RUNBOOK.md and `config/examples/layer2.toml`.

#### Self-node (Master) fallback

- When enabled (default), if real networking is disabled or the node cannot reach the minimum required peers, the system activates a loopback peer and marks the protocol as synced locally.
- This ensures the node operates as the main node when no peer infrastructure is present, allowing local operation and testing without external dependencies.
- Toggle via config or environment variables:
  - `prefer_self_as_master = true` (default)
  - `enable_self_node_fallback = true` (default)
  - `enable_real_networking = true` (default)
  - `ANYA_LAYER2_PREFER_SELF_AS_MASTER=true`
  - `ANYA_LAYER2_ENABLE_SELF_NODE_FALLBACK=true`
- The decision flow is implemented in the production Layer2 adapter: if `enable_real_networking = false` or connected peers `< min_peers`, the self-node mode is activated.

See `config/examples/layer2.toml` for a reference configuration.

### Layer2 Protocol Status

| Protocol | Status | Implementation | Test Coverage | Notes |
|----------|--------|----------------|---------------|-------|
| **Lightning Network** | ✅ Production with real node | Async implementation + adapter | Unit/integration tests pass | Simulation by default; wire to LN node for prod |
| **RGB Protocol** | ✅ Production with RGB core | Schemas/issuance/transfer + adapter | Unit/integration tests pass | Simulation by default; wire to RGB core for prod |
| **Taproot Assets** | ✅ Production (lib-backed) | Asset issuance and proofs | Unit tests pass | Requires Bitcoin connectivity for live issuance |
| **State Channels** | 🟡 Framework Ready | Generalized off-chain state | Basic tests | Needs concrete backend + conformance |
| **DLC** | 🟡 Framework Ready | Oracle contract scaffolding | Basic tests | Needs oracle/network wiring + conformance |
| **BOB Protocol** | 🟡 Framework Ready | EVM bridge foundation | Framework tests | Hardware integration Phase 2 |
| **RSK** | 🟡 Framework Ready | Sidechain architecture | Framework tests | Full implementation Phase 3 |
| **Stacks** | 🟡 Framework Ready | Smart contract foundation | Framework tests | Integration Phase 3 |
| **Liquid Network** | 🟡 Framework Ready | Sidechain foundation | Framework tests | Full implementation Phase 3 |

<!-- Historical build/test summaries removed to avoid drift. Run local commands above or consult CI dashboards for up-to-date status. -->

---

## Testing & Validation

### Running Tests

**Unit Tests:**

```bash
# Run all unit tests
cargo test --lib

# Run specific module tests
cargo test bitcoin::
cargo test layer2::
cargo test security::
```

**Integration Tests:**

```bash
# Run Layer2 integration tests
cargo test --test layer2_integration_comprehensive

# Run Bitcoin protocol tests
cargo test --test bitcoin_integration

# HSM simulator tests (dev only)
cargo test --features dev-sim,hsm-full --no-fail-fast -- --nocapture
```

**BIP Compliance Testing:**

```bash
# Validate BIP compliance
./scripts/bitcoin/validate-bip-compliance.js

# Run BIP test vectors
cargo test --test bip_tests -- --nocapture
```

### Validation Tools

**Security Validation:**

```bash
# Run security audit
cargo audit

# Check for unsafe code
cargo geiger

# Run clippy for code quality
cargo clippy -- -D warnings
```

**Performance Testing:**

```bash
# Run benchmarks
cargo bench

# Memory usage analysis
cargo test --release -- --nocapture | grep "memory"
```

---

## Contributing

We welcome contributions to Anya Core! This project is actively developed and we appreciate community involvement.

### How to Contribute

**1. Development Setup:**

```bash
# Fork and clone the repository
git clone https://github.com/your-username/Anya-core.git
cd Anya-core

# Set up development environment
./scripts/setup.sh
cargo build

# Run tests to ensure everything works
cargo test --lib
```

**2. Areas for Contribution:**

- 🔧 **Core Bitcoin Features:** Transaction processing, script validation, wallet functionality
- ⚡ **Layer2 Protocols:** Lightning Network, RGB, DLC implementations
- 🔒 **Security Enhancements:** HSM integration, audit frameworks, cryptographic operations
- 📚 **Documentation:** API documentation, tutorials, architecture guides
- 🧪 **Testing:** Unit tests, integration tests, BIP compliance validation

**3. Development Guidelines:**

- Follow [Bitcoin Development Framework](docs/bitcoin/README.md) patterns
- Ensure all code follows [AI Labeling Standards](docs_legacy/standards/AI_LABELING.md)
- Add comprehensive tests for new features
- Update documentation for any API changes

**4. Submission Process (signed commits required):**

```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Make changes and test
cargo test --lib
cargo clippy -- -D warnings

# Commit with proper labels
git commit -m "feat(bitcoin): implement BIP-XXX support

[AIR-3][AIS-3][BPC-3] Detailed description of changes

Labels: [AIR-3][AIS-3][BPC-3]
Scope: bitcoin, protocol"

# Submit pull request
git push origin feature/your-feature-name
```

### Getting Help

- 📖 **Documentation:** Start with [docs/](docs/) for comprehensive guides
- 🐛 **Issues:** Report bugs via [GitHub Issues](https://github.com/Anya-org/Anya-core/issues)
- 💬 **Discussions:** Join discussions for questions and feature requests
- 📧 **Contact:** Reach out to maintainers for complex technical questions

---

## License & Resources

### License

This project is dual-licensed under:

- **MIT License** - For open source use
- **Apache 2.0 License** - For enterprise applications

See [LICENSE](LICENSE) for full license text.

### Resources & Links

**Project Resources:**

- **Repository:** [https://github.com/Anya-org/Anya-core](https://github.com/Anya-org/Anya-core)
- **Documentation:** [docs/](docs/) - Complete documentation suite
- **Issue Tracker:** [GitHub Issues](https://github.com/Anya-org/Anya-core/issues)
- **Security Policy:** [docs/security/](docs/security/) - Security guidelines and reporting

**Bitcoin Standards:**

- **BIP Repository:** [https://github.com/bitcoin/bips](https://github.com/bitcoin/bips)
- **Bitcoin Core:** [https://github.com/bitcoin/bitcoin](https://github.com/bitcoin/bitcoin)
- **Lightning Specification:** [https://github.com/lightning/bolts](https://github.com/lightning/bolts)

**Community:**

- **Contributing Guide:** docs_legacy/CONTRIBUTING.md
- **Code of Conduct:** CODE_OF_CONDUCT.md
- **Security Policy:** scripts/enterprise/SECURITY.md

---

## Acknowledgments

Special thanks to the following projects and communities that make Anya Core possible:

- **Bitcoin Core** - Reference implementation and standards
- **Rust Bitcoin** - Rust Bitcoin ecosystem libraries
- **Lightning Development Kit (LDK)** - Lightning Network implementation
- **RGB Protocol** - Client-side validation framework
- **TBD/Block** - Web5 and identity technologies

---

Maintainer: Botshelo Mokoka (<botshelomokoka+anya-core@gmail.com>)
Last Updated: August 9, 2025
Version: 1.3.0

---

## Developer shortcuts & environment-honest testing

- CLI aliases (via `.cargo/config.toml`):
  - `cargo core-health` – run core health check
  - `cargo core-validate` – run configuration validation
  - `cargo core-start` – start core binary
- Network-bound tests: If DNS/ports aren’t reachable locally, network validation tests will fail; treat as advisory in dev environments.
- HSM simulator: The dev simulator is locked-by-default; tests should initialize and unlock via a custom `unlock` operation (pin "1234") and use per-test timeouts to avoid hangs.

### Recent Security & Dependency Remediation (August 2025)

Environment-honest principle: we do not mask missing infrastructure as passing tests. Instead, tests explicitly `skip` when required credentials, binaries, or network reachability are absent (e.g., Bitcoin RPC 400/401 unauth, connection refused, or network validation flag not set). This preserves trust in green test runs.

Security hardening just applied:

- Removed `cargo-geiger` runtime dependency which pulled in vulnerable `gix*` crates exposing multiple 2024 RUSTSEC advisories (RUSTSEC-2024-0335, 0348, 0349, 0351, 0352, 0353) concerning path traversal, Windows device name exploitation, and SSH username option smuggling.
- Regenerated dependency graph (`cargo update`) updating: `bitcoin 0.32.7`, `bdk_wallet 2.1.0`, `tokio 1.47.1`, plus minor patch bumps (clap, h2, tokio-util, etc.).
- Re-ran `cargo deny check` – all categories now pass: advisories ok, bans ok, licenses ok, sources ok.
- Retained `.gitignore` exclusion for `Cargo.lock` to keep workspace dependency flexibility; if reproducible builds are required for deployment artifacts, either commit a frozen `Cargo.lock` on a release branch or run CI with `-Z minimal-versions` to verify forward-compat.

Operational guidance:

```bash
# Security validation (current set)
cargo deny check           # advisory, license, bans, sources
cargo audit || true        # supplemental (advisory overlap; may duplicate deny)

# Environment-honest targeted smoke (skips instead of false green):
cargo test --test bitcoin_rpc_smoke -- --nocapture
```

Rationale: Removing the vulnerable chain avoids shipping tooling-only crates in production surfaces, keeping attack surface minimal while preserving the option to run geiger scans out-of-tree (use a separate tool invocation without adding a dependency).
