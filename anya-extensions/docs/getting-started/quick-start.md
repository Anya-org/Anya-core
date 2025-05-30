# Quick Start Guide

[AIR-3][AIS-3][AIT-3][RES-3] **Get up and running with Anya Core extensions in 15 minutes. Complete guide for Bitcoin, Web5, and ML integration.**

*Last updated: May 30, 2025*

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Installation](#quick-installation)
- [First Steps](#first-steps)
- [Bitcoin Integration](#bitcoin-integration)
- [Web5 Integration](#web5-integration)
- [ML Integration](#ml-integration)
- [Your First Extension](#your-first-extension)
- [Common Tasks](#common-tasks)
- [Next Steps](#next-steps)

## Prerequisites

Before starting, ensure you have:

- **Rust 1.70+** installed (`rustup show`)
- **Node.js 18+** for Web5 components (`node --version`)
- **Python 3.9+** for ML components (`python3 --version`)
- **Git** for source control (`git --version`)
- **8GB RAM minimum** (16GB recommended)
- **50GB free disk space** (500GB+ for Bitcoin full node)

## Quick Installation

### 1. One-Command Setup

```bash
# Download and run the quick setup script
curl -sSL https://get.anya.org | sh

# Or clone and build manually
git clone https://github.com/anya-org/Anya-core.git
cd Anya-core
cargo install --path .
```

### 2. Initialize Anya

```bash
# Initialize Anya Core with default configuration
anya init

# Start the core service
anya start --daemon

# Verify installation
anya status
```

Expected output:
```
✅ Anya Core v2.5.0 - Running
✅ Extensions: 0 loaded, 12 available
✅ Bitcoin: Disconnected (configure to connect)
✅ Web5: Not configured
✅ ML: Ready (CPU backend)
```

## First Steps

### 1. Basic Configuration

```bash
# Create basic configuration
anya config init --interactive

# Or use quick defaults
anya config init --defaults
```

This creates `~/.anya/config.toml` with sensible defaults.

### 2. Install Core Extensions

```bash
# Install essential extension bundle
anya ext install --bundle core

# Install specific extensions
anya ext install bitcoin-core web5-dids ml-inference

# Verify installation
anya ext list --installed
```

### 3. Quick System Test

```bash
# Run system health check
anya health

# Test all components
anya test --quick

# View system information
anya info
```

## Bitcoin Integration

### 1. Bitcoin Testnet Setup (Recommended for First Run)

```bash
# Configure for Bitcoin testnet
anya bitcoin config --network testnet --quick-sync

# Start Bitcoin integration
anya bitcoin start

# Wait for initial sync (2-5 minutes for testnet)
anya bitcoin status
```

### 2. Create Your First Wallet

```bash
# Create a new wallet
anya bitcoin wallet create my_first_wallet

# Generate a receiving address
anya bitcoin wallet address --new

# Check wallet balance
anya bitcoin wallet balance
```

Sample output:
```
Wallet: my_first_wallet
Balance: 0.00000000 BTC
Unconfirmed: 0.00000000 BTC
Address: tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh
```

### 3. Get Testnet Coins

```bash
# Get testnet coins from faucet
anya bitcoin faucet --address tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh

# Or use the built-in testnet faucet
anya bitcoin testnet fund --amount 0.1
```

### 4. First Bitcoin Transaction

```bash
# Send testnet coins
anya bitcoin send \
  --to tb1qtest_destination_address \
  --amount 0.01 \
  --fee-rate 1

# Check transaction status
anya bitcoin tx status <txid>
```

## Web5 Integration

### 1. Create Your Digital Identity

```bash
# Initialize Web5 identity
anya web5 init

# Create your first DID
anya web5 did create --method ion --publish

# View your DID document
anya web5 did show
```

Sample output:
```
DID: did:ion:EiClkZMDxPKqC9c-umQfTkR8vvZ9JPhl_xLDI9Nfk38w5w
Status: Published
Methods: [authentication, assertionMethod, keyAgreement]
Services: [dwn]
```

### 2. Issue Your First Credential

```bash
# Create a simple credential
anya web5 credential create \
  --type "TestCredential" \
  --subject did:ion:EiClkZMDxPKqC9c-umQfTkR8vvZ9JPhl_xLDI9Nfk38w5w \
  --claim "name=John Doe" \
  --claim "role=Developer"

# Verify the credential
anya web5 credential verify test_credential.jwt
```

### 3. Set Up Decentralized Web Node (DWN)

```bash
# Configure DWN endpoints
anya web5 dwn config --endpoints https://dwn.tbddev.org/dwn0

# Install social protocol
anya web5 protocol install social

# Create your first record
anya web5 record create \
  --protocol social \
  --schema post \
  --data '{"content": "Hello Web5!", "timestamp": "2025-05-30T12:00:00Z"}'
```

### 4. Test Protocol Interaction

```bash
# Query your records
anya web5 record query --protocol social --schema post

# Share with another DID
anya web5 record share \
  --record <record-id> \
  --recipient did:ion:another_did_here
```

## ML Integration

### 1. Download ML Models

```bash
# Download pre-trained models
anya ml models download --bundle starter

# List available models
anya ml models list
```

Sample output:
```
✅ text-classifier (sentiment analysis)
✅ entity-extractor (NER)
✅ intent-classifier (conversation AI)
✅ embedding-model (semantic search)
```

### 2. Your First ML Inference

```bash
# Text classification example
anya ml infer \
  --model text-classifier \
  --input "I love using Anya Core!"

# Entity extraction example
anya ml infer \
  --model entity-extractor \
  --input "Send 0.1 BTC to Alice"

# Intent classification
anya ml infer \
  --model intent-classifier \
  --input "What's my wallet balance?"
```

Sample output:
```
Model: text-classifier
Input: "I love using Anya Core!"
Output: {"sentiment": "positive", "confidence": 0.95}

Model: entity-extractor
Input: "Send 0.1 BTC to Alice"
Output: {
  "entities": [
    {"type": "AMOUNT", "value": "0.1", "currency": "BTC"},
    {"type": "PERSON", "value": "Alice"}
  ]
}
```

### 3. Batch Processing

```bash
# Process multiple inputs
echo '["Great project!", "This is terrible", "Okay I guess"]' | \
anya ml infer --model text-classifier --batch

# Process from file
anya ml infer --model text-classifier --input-file inputs.txt --output-file results.json
```

## Your First Extension

### 1. Create Extension Template

```bash
# Generate extension scaffold
anya ext new my-first-extension --template basic

# Navigate to extension directory
cd my-first-extension
```

This creates:
```
my-first-extension/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── handlers/
├── tests/
├── README.md
└── extension.toml
```

### 2. Implement Basic Functionality

Edit `src/lib.rs`:

```rust
use anya_core::{Extension, ExtensionResult, Context};
use serde_json::json;

pub struct MyFirstExtension;

impl Extension for MyFirstExtension {
    fn name(&self) -> &str {
        "my-first-extension"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    async fn initialize(&mut self, ctx: &Context) -> ExtensionResult<()> {
        // Register command handlers
        ctx.register_handler("hello", |_args| {
            Box::pin(async move {
                Ok(json!({"message": "Hello from my extension!"}))
            })
        })?;

        // Register Bitcoin transaction handler
        ctx.register_handler("bitcoin_balance", |args| {
            Box::pin(async move {
                let wallet = args.get("wallet").unwrap_or("default");
                let balance = anya_bitcoin::wallet::get_balance(wallet).await?;
                Ok(json!({"wallet": wallet, "balance": balance}))
            })
        })?;

        Ok(())
    }
}

// Export the extension
anya_core::export_extension!(MyFirstExtension);
```

### 3. Build and Test

```bash
# Build the extension
cargo build --release

# Test the extension
cargo test

# Install locally
anya ext install --local target/release/libmy_first_extension.so

# Test the extension
anya ext call my-first-extension hello
anya ext call my-first-extension bitcoin_balance --wallet my_first_wallet
```

### 4. Package and Distribute

```bash
# Package extension
anya ext package

# Publish to community registry (optional)
anya ext publish --registry community
```

## Common Tasks

### Wallet Operations

```bash
# Create wallet
anya bitcoin wallet create <name>

# List wallets
anya bitcoin wallet list

# Get new address
anya bitcoin wallet address --new --wallet <name>

# Send transaction
anya bitcoin send --from <wallet> --to <address> --amount <btc>

# Transaction history
anya bitcoin wallet history --wallet <name>
```

### Identity Management

```bash
# List DIDs
anya web5 did list

# Export DID for backup
anya web5 did export --did <did> --output did_backup.json

# Import DID
anya web5 did import --file did_backup.json

# Rotate keys
anya web5 did rotate-keys --did <did>
```

### ML Operations

```bash
# List available models
anya ml models list --available

# Download specific model
anya ml models download <model-name>

# Update all models
anya ml models update --all

# Model information
anya ml models info <model-name>

# Benchmark performance
anya ml benchmark --model <model-name>
```

### Extension Management

```bash
# List available extensions
anya ext list --available

# Install extension
anya ext install <extension-name>

# Update extension
anya ext update <extension-name>

# Remove extension
anya ext remove <extension-name>

# Extension info
anya ext info <extension-name>
```

### System Monitoring

```bash
# System status
anya status --detailed

# Resource usage
anya system resources

# View logs
anya logs --tail --follow

# Performance metrics
anya metrics --live
```

## Next Steps

### Development
- **[API Reference](../development/api-reference.md)**: Complete API documentation
- **[Best Practices](../development/best-practices.md)**: Development best practices
- **[Architecture Guide](../development/architecture.md)**: System architecture deep dive

### Integration
- **[Core Integration](../integration/core-integration.md)**: Advanced core integration
- **[Third-party Integration](../integration/third-party-integration.md)**: External service integration
- **[Security Guidelines](../integration/security-guidelines.md)**: Security implementation

### Advanced Features
- **[Bitcoin Lightning](../bitcoin/lightning.md)**: Lightning Network integration
- **[Web5 Protocols](../web5/protocols.md)**: Custom protocol development
- **[ML Training](../ml/training.md)**: Custom model training

### Production
- **[Deployment Guide](../deployment/README.md)**: Production deployment
- **[Monitoring](../monitoring/README.md)**: System monitoring and alerting
- **[Security Hardening](../security/hardening.md)**: Production security

## Troubleshooting

### Common Issues

#### Installation Problems
```bash
# Update Rust toolchain
rustup update

# Clear build cache
cargo clean && cargo build --release

# Check system dependencies
anya system check-deps
```

#### Bitcoin Connection Issues
```bash
# Check Bitcoin node status
anya bitcoin node status

# Restart Bitcoin connection
anya bitcoin restart

# Check configuration
anya bitcoin config show
```

#### Web5 DID Resolution Failures
```bash
# Check DID resolver status
anya web5 resolvers status

# Update resolver endpoints
anya web5 resolvers update

# Test DID resolution
anya web5 did resolve <did>
```

#### ML Model Loading Issues
```bash
# Verify model files
anya ml models verify

# Re-download models
anya ml models download --force

# Check system resources
anya system resources
```

### Getting Help

- **Quick Help**: `anya help <command>`
- **Documentation**: [https://docs.anya.org](https://docs.anya.org)
- **Community**: [https://discord.gg/anya](https://discord.gg/anya)
- **GitHub**: [https://github.com/anya-org/Anya-core](https://github.com/anya-org/Anya-core)

### What You've Accomplished

After completing this quick start, you have:

✅ Installed and configured Anya Core  
✅ Set up Bitcoin testnet integration  
✅ Created your first Web5 digital identity  
✅ Performed ML inference with pre-trained models  
✅ Built and deployed your first extension  
✅ Learned essential system operations  

You're now ready to build sophisticated Bitcoin, Web5, and ML applications with Anya Core!

## Success Checklist

Before moving to advanced topics, verify:

- [ ] `anya status` shows all green checkmarks
- [ ] Bitcoin wallet can send/receive testnet transactions
- [ ] Web5 DID resolves successfully
- [ ] ML models produce expected inference results
- [ ] Custom extension loads and responds to commands
- [ ] System performance meets expectations

Congratulations! You're now an Anya Core developer. 🎉
