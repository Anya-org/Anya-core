# Anya Core

[![BIP-341](https://img.shields.io/badge/BIP-341_Compliant-green)](https://bips.xyz/341)
[![AIS-3](https://img.shields.io/badge/AIS-3_Secured-blue)](https://bitcoin-development-framework.org)
[![PSBT-v2](https://img.shields.io/badge/PSBT_v2-100%25-brightgreen)](https://bips.xyz/370)
[![BDF-v2.5](https://img.shields.io/badge/Bitcoin%20Dev%20Framework-v2.5-yellow)](https://bitcoin-development-framework.org)

[AIR-3][AIS-3][AIT-3][AIM-3][AIP-3][AIE-3][BPC-3][W5C-3][DID-3][PFM-3][SCL-3][RES-3][UXA-3][DAO-3]

*Last Updated: May 24, 2025*

**Latest Release: v0.3.0-rc2**

> **Note**: This release includes comprehensive updates to the Taproot implementation, enhanced security features with HSM integration, and a fully refactored installation system following Bitcoin Development Framework v2.5 standards.

A powerful platform combining Bitcoin/crypto functionality, ML-based analytics,
Web5 decentralized data management, and a Bitcoin-style DAO governance system.

> For Enterprise features and capabilities, please see our
> [Enterprise Platform Documentation](./enterprise/README.md)

![Anya Architecture](docs/images/anya_architecture.png)

> **AI Labeling**: This project follows the [canonical AI Labeling System](docs/standards/AI_LABELING.md)
> based on the Bitcoin Development Framework v2.5 standards. All components are labeled with
> appropriate Core and Extended category labels.

## Documentation Quick Links [AIR-3][AIS-3][BPC-3][RES-3]

### Core Documentation

- [Documentation Index](docs/INDEX.md) - Main documentation index
- [System Architecture](SYSTEM_MAP.md) - Complete system architecture
- [Bitcoin Development Framework v2.5](docs/standards/BITCOIN_DEVELOPMENT_FRAMEWORK.md) - Standards compliance
- [AI Labeling System](docs/standards/AI_LABELING.md) - Component labeling standards

### Installation & Setup

- [Installation Guide](docs/installation/README.md) - Comprehensive installation instructions
- [Quick Start Guide](docs/getting-started/quickstart.md) - Get up and running in minutes
- [Configuration Reference](docs/configuration/README.md) - Detailed configuration options

### Developer Resources

- [API Reference](docs/api/README.md) - Complete API documentation
- [Development Guide](docs/development/README.md) - Contributing and development workflow
- [Testing Guide](docs/testing/README.md) - Testing framework and guidelines

### Advanced Topics

- [Monitoring & Observability](monitoring/README.md) - Comprehensive monitoring with Prometheus, Grafana, and Alertmanager
- [Taproot Implementation](docs/bitcoin/taproot.md) - Taproot integration details
- [Security Model](docs/security/README.md) - Security best practices and features
- [Performance Tuning](docs/performance/README.md) - Optimization guidelines

## Licensing

This core platform is released under the [MIT License](LICENSE.md), allowing for
free use, modification, and distribution. However, please note that the
[Enterprise features](./enterprise/README.md) are subject to a separate
proprietary license with different terms, including revenue sharing requirements.
See the [Enterprise License](./enterprise/LICENSE) for details.

## Core Features

### Hexagonal Architecture

[AIR-3][AIS-3][AIT-3][PFM-2][SCL-3][RES-3]

- Clean separation of concerns with ports and adapters
- Domain-driven design principles
- Advanced error handling and telemetry
- Circuit breaker pattern implementation
- Comprehensive health monitoring
- Thread-safe caching layer

### Hardware Security Module (HSM) Support

[AIS-3][RES-3][PFM-2]

- Modular HSM provider architecture
- Software-based HSM implementation for development
- Hardware device support including TPM and PKCS#11
- Bitcoin-specific HSM operations
- Feature-flag enabled for flexible deployment
- Comprehensive key management and cryptographic operations

### Installation System

[AIR-3][AIS-3][AIT-3][PFM-2]

- Hexagonal architecture with clean separation of concerns
- Multi-language support (Python, Rust, Node.js, Go)
- Checkpoint system with progress tracking
- Component-based installation with dependency management
- Multiple deployment configurations
- Cross-platform compatibility

### DAO & Tokenomics System

[AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3][RES-3][SCL-2]

- Bitcoin-style token issuance (21 billion total supply)
- Adaptive emission schedule with governance-controlled halving
- Strategic token distribution with protocol treasury
- Decentralized governance with proposals and voting
- Integrated DEX for liquidity and trading
- Comprehensive logging and transparency

### Blockchain Integration

[AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][PFM-2][RES-3][SCL-2]

- Bitcoin Core & Lightning Network support
- DLC (Discreet Log Contracts)
- Taproot/Schnorr signatures
- Layer 2 solutions
- Cross-chain capabilities
- Custom chain support

### Machine Learning & AI

[AIR-3][AIS-3][AIT-3][AIM-3][AIP-3][AIE-3][PFM-2][SCL-2][RES-2]

- Model optimization
- Federated learning
- Pipeline optimization
- Basic analytics
- Prediction models

### Web5 Integration & Storage

[AIR-3][AIS-3][AIT-3][AIP-3][W5C-3][DID-3][PFM-2][SCL-3][RES-2]

- Decentralized Web Nodes (DWN)
- Decentralized data storage
- Protocol-based data management
- Identity-centric storage
- Secure data encryption
- Record-based storage
- Automated data replication
- Protocol optimization
- Identity management
- Custom protocols

### Decentralized Communication

[AIR-3][AIS-3][AIT-2][AIP-3][PFM-2][SCL-2][RES-3]

- Nostr protocol integration
(NIPs 01, 02, 04, 05, 13, 15, 20)
- End-to-end encrypted messaging
- Multi-relay support with health monitoring
- Automatic relay selection and load balancing
- Simple key subscription system
- Secure key management and backup

### Development Infrastructure

[AIR-3][AIS-2][AIT-3][PFM-2]

- Comprehensive checkpoint system
- AI labeling integration
(Core and Extended categories)
- Automated checkpoint creation
(merges, thresholds)
- Development milestone tracking
- GitHub Actions workflow integration

### Monitoring & Metrics

[AIR-3][AIM-3][PFM-3][RES-3][SCL-2]

- Distributed tracing
- Performance metrics
- Resource monitoring
- Health checks
- Basic dashboards

## Technical Stack

### Prerequisites

- Rust 1.70+
- Bitcoin Core 24.0+
- Web5 DWN Node
- Clarinet 2.3.0+ (for DAO contracts)

### Core Dependencies

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

## Quick Start

### Installation

Anya Core features an intelligent installation system that automatically detects hardware capabilities and configures the system optimally.

#### Quick Install

```bash
# Standard installation with auto-detection
sudo ./scripts/install.sh

# Minimal installation for low-resource environments
sudo ./scripts/install.sh --type=minimal

# Full installation with all features
sudo ./scripts/install.sh --type=full
```

#### Installation Options

```bash
# Display all installation options
./scripts/install.sh --help

# Install with specific configuration
sudo ./scripts/install.sh --network=testnet --hardening=strict --run-tests

# Install with explicit feature flags
sudo ./scripts/install.sh --features=std,hsm,bitcoin_integration

# Upgrade an existing installation
sudo ./scripts/install.sh

# Force clean installation
sudo ./scripts/install.sh --force-clean
```

#### Manual Installation (Alternative)

```bash
# Clone the repository
git clone https://github.com/anya/anya-enterprise.git

# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential cmake pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build with specific features
cargo build --release --features="std,hsm,bitcoin_integration"

# Run tests
cargo test --all-features
```

#### Verify Installation

```bash
# Check system health
./scripts/install/utils/monitor_health.sh

# Run diagnostics
./scripts/test/debug_test.sh

# Verify hardware detection
./scripts/install/utils/script_cleanup.sh
```

#### Configuration

The installation creates a default configuration in `config/anya.conf`. You can modify this or use environment variables:

```bash
# Environment variables
export BITCOIN_RPC_URL=http://localhost:8332
export BITCOIN_RPC_USER=bitcoinrpc
export BITCOIN_RPC_PASSWORD=your_password
export WEB5_DWN_URL=http://localhost:3000
export NETWORK=testnet
```

### 1. Clone and Setup

   ```bash
# Clone the repository
   git clone https://github.com/anya/anya-core.git
   cd anya-core

# Install dependencies
./scripts/setup.sh

# Build the project
   cargo build --release
   ```

### 2. Configuration

```env
# Web5 Settings
WEB5_DWN_URL=http://localhost:3000
WEB5_STORAGE_PATH=/path/to/web5/data

# Bitcoin Settings
BITCOIN_RPC_URL=http://localhost:8332
BITCOIN_RPC_USER=user
BITCOIN_RPC_PASS=password

# ML Settings
ML_MODEL_PATH=/path/to/models
NPU_ENABLED=true

# DAO Settings
DAO_CONTRACT_PATH=/path/to/contracts
TOKEN_NAME=Anya Governance Token
TOKEN_SYMBOL=AGT
TOTAL_SUPPLY=21000000000
INITIAL_BLOCK_REWARD=5000
HALVING_INTERVAL=210000

# Monitoring
METRICS_ENDPOINT=http://localhost:9090
TRACING_ENDPOINT=http://localhost:4317
```

### 3. DAO Setup

If you don't have Clarinet installed, you can use the provided installation script:

```powershell
# On Windows
.\scripts\install-clarinet.ps1
```

To verify the DAO configuration:

```powershell
# On Windows
.\scripts\verify-clarinet-config.ps1
```

To run DAO tests:

```bash
# With Clarinet installed
clarinet test

# Without Clarinet (simulation only)
.\scripts\run-dao-tests.ps1
```

## Documentation

### System Architecture Documentation

- [Architecture Overview](docs/ARCHITECTURE.md)
- [Hexagonal Design](docs/HEXAGONAL.md)
- [System Map](docs/SYSTEM_MAP.md)
- [DAO System Map](docs/DAO_SYSTEM_MAP.md)
- [Error Handling](docs/ERROR_HANDLING.md)
- [ML System](docs/ML_SYSTEM_ARCHITECTURE.md)
- [Web5 Integration](docs/WEB5_INTEGRATION.md)
- [Tokenomics System](docs/TOKENOMICS_SYSTEM.md)

### Development Documentation

- [API Reference](docs/API.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Security Guidelines](docs/SECURITY.md)
- [Testing Strategy](docs/TESTING.md)
- [Checkpoint System](docs/CHECKPOINT_SYSTEM.md)
- [Checkpoint Guide](docs/CHECKPOINT_GUIDE.md)
- [AI and Component Labeling Guide](docs/standards/AI_LABELING.md)
- [Implementation Milestones](docs/IMPLEMENTATION_MILESTONES.md)

### Deployment Documentation

- [Deployment Guide](docs/DEPLOYMENT.md)
- [Configuration Guide](docs/CONFIGURATION.md)
- [Monitoring Setup](docs/MONITORING.md)

## Core Components

### DAO Component Features

[AIR-3][AIS-3][AIT-3][BPC-3][DAO-3][SCL-2]

- **Governance System**
  - Proposal creation and management
  - Token-weighted voting
  - Administrative controls
  - Configurable parameters
- **Modified Bitcoin-Style Tokenomics**
  - 21 billion token supply
  - Adaptive halving mechanism
  - Initial block reward of 10,000 AGT
  - DAO-controlled emission parameters
- **Strategic Distribution**
  - 35% Protocol Treasury (DAO-managed)
  - 25% Liquidity Provision
  - 20% Team & Development (with vesting)
  - 15% Community Incentives
  - 5% Strategic Partners & Advisors
- **DEX Integration**
  - Protocol-owned liquidity
  - Trading operations
  - Price oracle
  - Buyback and burn mechanism

### ML Component Features

[AIR-3][AIS-3][AIT-3][AIM-3][AIP-3][AIE-3][PFM-2][SCL-2][RES-2]

- Advanced model management and execution
- Real-time inference with metrics tracking
- Model versioning and selection
- Validation and error handling
- Performance monitoring and optimization
- Support for distributed training
- Model A/B testing capabilities

### Security Component Features

[AIR-3][AIS-3][AIT-3][AIP-3][PFM-2][RES-3]

- Comprehensive security operations
  - Authentication and authorization
  - Encryption and decryption
  - Digital signatures and verification
- Strong audit trail implementation
- Security event monitoring
- Rate limiting and threat detection
- Policy management and enforcement
- Compliance tracking and reporting

### Protocol Component Features

[AIR-3][AIS-3][AIT-3][BPC-3][PFM-3][RES-3][SCL-2]

- Advanced transaction handling
  - Multiple operation types
(Create, Sign, Broadcast)
  - Input/output validation
  - Fee estimation and management
- Support for various script types
  - P2PKH, P2SH, P2WPKH, P2WSH, P2TR
- Transaction monitoring and tracking
- Mempool management
- PSBT support
- Multi-signature operations

### Enterprise Component Features

[AIR-3][AIS-3][AIT-3][BPC-3][PFM-3][RES-3][SCL-3][DAO-3]

- Comprehensive business operations
  - Atomic swaps
  - Liquid transfers
  - DLC contracts
  - State chain transfers
  - Multi-party computation
  - Portfolio rebalancing
- Risk management and compliance
- Audit trail and reporting
- SLA monitoring and enforcement
- Batch operation support
- Workflow management

## System Architecture

[AIR-3][AIS-3][AIT-3][PFM-3][SCL-3][RES-3]

### Core Design Principles

1. **Memory Safety**
[AIR-3][AIS-3][RES-3]
   - Rust's ownership system
   - Thread-safe primitives
   - Resource management

2. **Error Handling System**
[AIR-3][AIS-3][AIT-3][RES-3]
   - Comprehensive error types
   - Validation at multiple layers
   - Error aggregation and analysis
   - Retry strategies

3. **Metrics & Monitoring System**
[AIR-3][AIM-3][PFM-3][RES-3]
   - Unified metrics collection
   - Health checks
   - Performance tracking
   - Alerting system

4. **Security Architecture**
[AIR-3][AIS-3][AIP-3][RES-3]
   - Context validation
   - Audit logging
   - Threat detection
   - Security event correlation

### Implementation Details

#### Repository Layer Details

[AIR-3][AIS-3][PFM-2][SCL-2]

- CRUD operations
- Data validation
- Caching support
- Transaction management
- Audit logging

#### Service Layer Components

[AIR-3][AIS-3][AIT-3][PFM-3][RES-3]

- Business logic
- Operation processing
- Security checks
- Metrics collection
- Health monitoring

#### Handler Layer Organization

[AIR-3][AIS-3][AIT-2][PFM-2]

- Request/response mapping
- Input validation
- Error handling
- Metrics tracking
- Security enforcement

## Testing Strategy

[AIR-3][AIT-3][RES-2]

### Unit Testing Approach [AIT-3]

- Component-level tests
- Mock implementations
- Error case coverage
- Performance benchmarks

### Integration Testing Methods

[AIT-3][PFM-2]

- Cross-component testing
- End-to-end scenarios
- Performance testing
- Security testing

### Property Testing Framework

[AIT-3][AIS-3]

- Invariant verification
- Fuzz testing
- Boundary testing
- Concurrency testing

## Performance Optimization

[AIR-3][PFM-3][SCL-3]

### Caching Strategy

[PFM-3][SCL-3]

- In-memory caching
- Distributed caching
- Cache invalidation
- Cache metrics

### Concurrency Model

[PFM-3][SCL-3][RES-3]

- Async operations
- Thread pooling
- Resource management
- Deadlock prevention

### Monitoring Capabilities

[AIM-3][PFM-3]

- Performance metrics
- Resource utilization
- Bottleneck detection
- Trend analysis

## Deployment Requirements

### System Requirements

- Rust 1.70+
- Bitcoin Core 24.0+
- Web5 DWN Node
- Clarinet 2.3.0+
- PostgreSQL 14+
- Redis 7+

### Configuration Options

```bash
# Core Settings
RUST_LOG=info
RUST_BACKTRACE=1

# Database
DATABASE_URL=postgresql://user:pass@localhost/anya
REDIS_URL=redis://localhost:6379

# Bitcoin Core
BTC_RPC_URL=http://localhost:8332
BTC_RPC_USER=user
BTC_RPC_PASS=pass

# Web5
WEB5_DWN_URL=http://localhost:3000

# DAO Settings
DAO_PROPOSAL_THRESHOLD=100
DAO_VOTING_PERIOD=10080
DAO_EXECUTION_DELAY=2880

# Security
ENCRYPTION_KEY=<secure-key>
JWT_SECRET=<jwt-secret>
```

## Configuration System [AIR-3][AIS-3][SCL-2]

The Anya platform uses a flexible configuration system that supports multiple
configuration sources:

1. **Configuration Files** (`config/`)
   - `default.yaml`: Default configuration values
   - Environment-specific configs (e.g., `development.yaml`, `production.yaml`)

2. **Environment Variables**
   - All configuration can be overridden using environment variables
   - Variables are prefixed with `ANYA_`
   - Example: `ANYA_NETWORK_CAPACITY=2000`

3. **Secure Credentials**
   - Sensitive data is stored securely using encryption
   - Credentials are managed through the `CredentialManager`
   - Never commit `.env` files containing secrets

### Configuration Structure Example

```yaml
network:
  capacity: 1000
  node_connection_limit: 100
  performance_threshold: 0.6

dao:
  contract_name: "anya-dao"
  proposal_threshold: 100000000
  voting_period_blocks: 1008
  token_supply: 21000000000
  block_reward: 10000
  min_halving_interval: 105000
  treasury_reserve_ratio: 0.15

features:
  experimental_ml: false
  advanced_optimization: false
  quantum_resistant: false
```

### Dynamic Configuration Capabilities

[AIR-3][PFM-3][SCL-3]

The platform supports dynamic configuration updates:

- Network limits adjust based on system resources
- Timelock periods scale with network activity
- Performance thresholds adapt to usage patterns

### Security Configuration

[AIR-3][AIS-3][AIP-3]

- Sensitive configuration is encrypted at rest
- Credentials are stored securely using the `SecureStorage` module
- Environment-specific secrets are managed via `.env` files (not committed to VCS)

## Decentralized Governance (DAO)

[AIR-3][AIS-3][AIT-3][AIP-3][AIE-3][DAO-3]

### Governance Token (AGT)

- **Total Supply**: 21,000,000,000 AGT
- **Initial Block Reward**: 10,000 AGT
- **Halving Interval**: Adaptive, minimum 105,000 blocks
- **Distribution Model**:
  - 35% Protocol Treasury (DAO-managed)
    • 15% Strategic Reserves
    • 20% Ecosystem Development
  - 25% Liquidity Provision
    • 15% Initial DEX Liquidity
    • 10% Ongoing Liquidity Mining
  - 20% Team & Development
    • 4-year vesting with 1-year cliff
    • Milestone-based release triggers
  - 15% Community Incentives
    • Governance participation
    • Protocol usage rewards
  - 5% Strategic Partners & Advisors
    • 3-year vesting schedule
- **Emission Model**: Adaptive Bitcoin-inspired mechanism with governance controls

### Governance System Features

[AIR-3][DAO-3][AIP-3]

- **Proposal Framework**:
  - Low barrier to entry (100 AGT proposal threshold)
  - Multi-dimensional proposal evaluation
  - ML-driven proposal scoring
  - Adaptive governance parameters
  - Treasury management proposals

### Treasury Management Framework

[AIR-3][DAO-3][AIP-3][BPC-3]

- **Protocol-Owned Liquidity**:
  - DAO-controlled liquidity positions
  - Minimum reserve requirements (15% of circulation)
  - Strategic buyback and burn mechanisms
  - Market stability operations

### Governance Intelligence

[AIR-3][AIM-3][AIE-3][DAO-3]

- **Machine Learning Enhanced**:
  - Predictive proposal outcome analysis
  - Risk assessment modeling
  - Sentiment analysis integration
  - Dynamic governance optimization

### Cross-Platform Governance

[AIR-3][DAO-3][BPC-2][W5C-3]

- **Multi-Chain Compatibility**:
  - Stacks Blockchain Integration
  - Web5 Decentralized Identity Support
  - Interoperability Protocols

### Governance Security Measures

[AIR-3][AIS-3][DAO-3][RES-3]

- **Advanced Protection Mechanisms**:
  - Multi-signature proposal execution
  - Intelligent threat detection
  - Automated security audits
  - Zero-knowledge proof governance

### Compliance and Ethics Framework

[AIR-3][AIE-3][DAO-3]

- **Governance Principles**:
  - Transparent decision-making
  - Privacy-preserving technologies
  - Ethical AI governance
  - Continuous improvement mechanisms

### Technical Specifications

- **Supported Platforms**:
  - Clarity (Core Implementation)
  - Rust (API Integration)
  - Web5 Decentralized Infrastructure

### Version Information

- **Current Version**: 3.1.0
- **Last Updated**: 2025-03-04
- **Compatibility**:
  - Stacks v2.4
  - Web5 Protocol v1.0
  - Bitcoin Core Compatibility

### Governance Manifesto

> "Intelligence is our governance, decentralization is our method, and
> human potential is our ultimate goal."

## Storage Architecture

[AIR-3][AIS-3][AIP-3][W5C-3][DID-3][SCL-3]

Anya uses Web5's Decentralized Web Nodes (DWN) for all data storage, providing:

### Storage Features

- **Decentralized Storage**: Data is stored across the DWN network
- **Identity-Based Access**: Data access is controlled by DIDs
- **Protocol-Driven**: Data schemas and interactions defined by protocols
- **Encrypted by Default**: All data and communications are encrypted
- **Automatic Replication**: Data is replicated across nodes
- **Flexible Querying**: Rich query capabilities for data retrieval

### Data Types Supported

- User profiles and preferences
- Transaction records
- Analytics data
- Machine learning models
- System configurations
- Audit logs
- DAO proposals and votes

### Storage Benefits

- No central database dependency
- Built-in encryption and security
- Automatic data replication
- Identity-based access control
- Protocol-based data validation
- Offline-first capability

## Contributing Guidelines

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

## Project Status Information

- **Current Version**: 3.1.0
- **Status**: Production/Stable
- **Last Update**: 2025-03-12

## License Information

This project is licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.

## Additional Resources

### External Links

- [Documentation](https://docs.anya-core.org)
- [API Reference](https://api.anya-core.org)
- [Community Forum](https://community.anya-core.org)
- [Development Blog](https://blog.anya-core.org)

### Acknowledgments

Special thanks to our contributors and the following projects:

- Bitcoin Core
- Lightning Network
- Web5
- TBD
- Block

### Last Updated

*2025-03-12*

## Validation

Run the following to verify system compliance:

```shell
./scripts/validate_upgrade.ps1 -CheckAll -ProtocolLevel 3
```

## Standards Compliance

This project adheres to:

- Bitcoin Protocol Compliance Level 3 (BPC-3)
- DAO Governance Standard Level 4 (DAO-4)
- AI Security Standard Level 3 (AIS-3)

## Bitcoin Protocol Compliance

```mermaid
graph TD
    A[Bitcoin Core] --> B{{BIP-341}}
    B --> C[Taproot]
    B --> D[SILENT_LEAF]
    A --> E{{BIP-174}}
    E --> F[PSBT v2]
    style B fill:#9f9,stroke:#333
    style E fill:#9f9,stroke:#333
```

## Core Features

```rust
// From src/security/hsm/mod.rs
#[bip341]
fn verify_taproot(commitment: [u8; 32]) -> Result<()> {
    use bitcoin::secp256k1::{Secp256k1, XOnlyPublicKey};
    
    let secp = Secp256k1::new();
    let (xonly, _) = XOnlyPublicKey::from_slice(&commitment)?;
    
    let script = Script::new_v1_p2tr(&secp, xonly, None)?;
    assert_eq!(script.as_bytes(), SILENT_LEAF);
    Ok(())
}
```

## Dependency Matrix

```toml
[workspace.dependencies]
bitcoin = { version = "0.32.1", features = ["bip341"] }
secp256k1 = { version = "0.28.0", features = ["bip340"] }
bdk = { version = "0.30.0", features = ["psbt-v2"] }

[upgrades]
bitcoin = { target = "0.33.0", deadline = "2025-09-01" }

[compliance]
bitcoin-core-version = "24.0.1"
bdf-version = "2.5"
last-audit = "2025-08-24"
```

## Security Implementation

```mermaid
sequenceDiagram
    User->>+HSM: Sign Transaction
    HSM->>+Security: Verify BIP-341
    Security-->>-HSM: Valid Signature
    HSM->>+Mobile: Signed PSBT
    Mobile->>+Network: Broadcast
```

## Mobile Integration

```toml
[features]
mobile = [
    "bitcoin/mobile",
    "secp256k1/global-context",
    "bdk/psbt-v2"
]

[target.'cfg(mobile)'.dependencies]
jsi = { version = "0.12", features = ["bip341"] }
```

## Audit Trail

```json
{
  "2025-08": {
    "security_audit": "passed",
    "bip_coverage": ["341", "174", "370"],
    "fuzz_tests": "2.4M iterations",
    "hsm_checks": "YubiHSM2 validated"
  }
}
```

## Compliance Checklist

- [x] BIP-341 (Taproot)
- [x] BIP-174 (PSBT v2)
- [x] BIP-342 (Tapscript)
- [x] AIS-3 Security
- [x] BIP-370 (Enhanced PSBT)

---

**Last Updated**: 2025-08-24 | [Full Roadmap](ROADMAP.md) | [Audit Reports](docs/SECURITY.md)

## CI/CD Pipeline

```mermaid
sequenceDiagram
    User->>+HSM: Sign Transaction
    HSM->>+Security: Verify BIP-341
    Security-->>-HSM: Valid Signature
    HSM->>+Mobile: Signed PSBT
    Mobile->>+Network: Broadcast
```

## Run-All Script

A new `run-all.ps1` script has been added to simplify the process of running all necessary operations:

1. Cleaning the build artifacts
2. Verifying all components
3. Running basic tests
4. Running comprehensive tests (in quick mode)

### Usage

To run all operations:

```powershell
.\run-all.ps1
```

The script will:

- Log all operations to the logs directory
- Generate a comprehensive summary report
- Handle failures gracefully
- Provide timing information for each stage

Author: Bo The Big (<botshelomokokoka@gmail.com>)

## Hardware Security Module (HSM) Integration

Anya Core now includes comprehensive Hardware Security Module (HSM) integration with multiple provider types:

- **Software HSM**: Secure key management for development and testing
- **Simulator HSM**: Testing HSM functionality without hardware
- **Hardware HSM**: Integration with physical HSM devices (YubiHSM, Ledger, etc.)
- **Bitcoin HSM**: Specialized Bitcoin key management with hardware wallets

### Key Features

- **Multiple Key Types**: RSA, EC (secp256k1, P-256/384/521), Ed25519, AES
- **Secure Operations**: Signing, verification, encryption, decryption
- **Comprehensive Audit**: Full audit logging for all HSM operations
- **Bitcoin Integration**: Support for Bitcoin-specific key derivation and operations
- **Taproot Ready**: Full support for Taproot transactions and signatures

### Usage Example

```rust
// Initialize HSM Manager
let config = HsmConfig {
    provider_type: HsmProviderType::BitcoinHsm,
    bitcoin: BitcoinConfig {
        network: BitcoinNetworkType::Testnet,
        use_taproot: true,
        ..Default::default()
    },
    ..Default::default()
};

let hsm_manager = HsmManager::new(config).await?;
hsm_manager.initialize().await?;

// Generate key
let key_info = hsm_manager.generate_key_pair(
    KeyType::Ec { curve: EcCurve::Secp256k1 },
    "payment-key"
).await?;

// Sign data
let message = "transaction data".as_bytes();
let signature = hsm_manager.sign_data(
    "payment-key",
    message,
    SignatureAlgorithm::EcdsaSha256
).await?;
```

This implementation follows the Bitcoin Development Framework v2.5 requirements for HSM integration, with full compliance with the Hexagonal Architecture principles.

## Installation

### Linux Installation

For automated installation on Linux systems, we provide comprehensive installation scripts:

```bash
# Quick automatic installation with system-optimization (NEW)
sudo ./scripts/install/install-anya.sh

# Installation with custom options
sudo ./scripts/install/install-anya.sh --network=mainnet --type=full

# Standard installation (recommended)
sudo ./scripts/install/auto_install.sh

# Advanced options
sudo ./scripts/install/auto_install.sh --network=testnet --type=full --hardening=strict --auto-run

# Manual installation steps
sudo ./scripts/install/linux_install.sh    # Install dependencies and build
sudo ./scripts/install/systemd_config.sh   # Configure systemd service

# Uninstallation
sudo ./scripts/install/uninstall.sh
```

The new system-optimized installation automatically:

1. Analyzes your system capabilities (CPU, memory, disk, hardware security modules)
2. Configures build parameters based on available resources
3. Sets appropriate resource limits in systemd service
4. Optimizes HSM configuration based on detected hardware
5. Tunes performance parameters for your specific system

For more installation options, run:

```bash
sudo ./scripts/install/install-anya.sh --help
```
