# Extension Installation Guide

[AIR-3][AIS-3][AIT-3][RES-3] **Comprehensive installation guide for Anya Core extensions with Bitcoin, Web5, and ML integration support.**

*Last updated: June 7, 2025*

## Table of Contents

- [Prerequisites](#prerequisites)
- [Core Installation](#core-installation)
- [Extension Installation](#extension-installation)
- [Bitcoin Node Setup](#bitcoin-node-setup)
- [Web5 SDK Configuration](#web5-sdk-configuration)
- [ML Runtime Setup](#ml-runtime-setup)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### System Requirements

- **Operating System**: Linux (Ubuntu 22.04+), macOS (12.0+), Windows (WSL2)
- **Memory**: Minimum 8GB RAM (16GB recommended for ML workloads)
- **Storage**: 50GB available space (500GB+ for Bitcoin full node)
- **Network**: Stable internet connection for blockchain sync

### Required Dependencies

```bash
# Install Rust (1.70.0 or later)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Node.js (18.0+ for Web5 compatibility)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Python (3.9+ for ML components)
sudo apt-get install python3.9 python3.9-pip python3.9-venv

# Install Git and build tools
sudo apt-get install git build-essential pkg-config libssl-dev
```

### Bitcoin Node Requirements

```bash
# For Bitcoin Core integration
sudo apt-get install bitcoind

# Or install from source for latest features
wget https://bitcoincore.org/bin/bitcoin-core-25.0/bitcoin-25.0-x86_64-linux-gnu.tar.gz
tar -xzf bitcoin-25.0-x86_64-linux-gnu.tar.gz
sudo cp bitcoin-25.0/bin/* /usr/local/bin/
```

## Core Installation

### 1. Clone Anya Core Repository

```bash
git clone https://github.com/anya-org/Anya-core.git
cd Anya-core
```

### 2. Build Core Components

```bash
# Build the main Anya Core
cargo build --release

# Build with all features enabled
cargo build --release --all-features

# Build with specific feature sets
cargo build --release --features "bitcoin,web5,ml"
```

### 3. Install Core Dependencies

```bash
# Install Rust dependencies
cargo install --path .

# Install Python ML dependencies
pip3 install -r requirements.txt

# Install Node.js Web5 dependencies
npm install -g @web5/api @web5/dids @web5/credentials
```

## Extension Installation

### Extension Manager Setup

```bash
# Install the extension manager
cargo install anya-extension-manager

# Initialize extension registry
anya-ext init --registry https://extensions.anya.org

# Configure extension paths
export ANYA_EXT_PATH="$HOME/.anya/extensions"
mkdir -p $ANYA_EXT_PATH
```

### Installing Core Extensions

```bash
# Install Bitcoin extension suite
anya-ext install bitcoin-core
anya-ext install bitcoin-wallet
anya-ext install bitcoin-lightning

# Install Web5 extension suite
anya-ext install web5-dids
anya-ext install web5-credentials
anya-ext install web5-protocols

# Install ML extension suite
anya-ext install ml-inference
anya-ext install ml-training
anya-ext install ml-models
```

### Manual Extension Installation

```bash
# Clone extension repository
git clone https://github.com/anya-org/anya-bitcoin-ext.git
cd anya-bitcoin-ext

# Build and install
cargo build --release
cargo install --path .

# Register with core system
anya-ext register --path ./target/release/anya-bitcoin-ext
```

### Extension Configuration

```toml
# ~/.anya/extensions.toml
[extensions]
enabled = ["bitcoin-core", "web5-dids", "ml-inference"]

[bitcoin-core]
network = "mainnet"  # or "testnet", "regtest"
rpc_host = "127.0.0.1"
rpc_port = 8332
rpc_user = "bitcoinrpc"
rpc_password = "your_password_here"

[web5-dids]
resolver_endpoints = ["https://resolver.identity.foundation"]
default_method = "did:ion"

[ml-inference]
backend = "onnx"  # or "tensorflow", "pytorch"
device = "cpu"    # or "cuda", "metal"
```

## Bitcoin Node Setup

### Bitcoin Core Configuration

```bash
# Create Bitcoin data directory
mkdir -p ~/.bitcoin

# Configure Bitcoin Core
cat > ~/.bitcoin/bitcoin.conf << EOF
# Network settings
testnet=0  # Set to 1 for testnet
rpcuser=bitcoinrpc
rpcpassword=$(openssl rand -hex 32)
rpcallowip=127.0.0.1
rpcbind=127.0.0.1
rpcport=8332

# Performance settings
dbcache=4000
maxconnections=125
maxuploadtarget=5000

# Enable transaction indexing
txindex=1
addresstype=bech32
changetype=bech32

# Enable RPC methods for Anya
rpcworkqueue=32
rpcthreads=16
EOF

# Start Bitcoin daemon
bitcoind -daemon

# Wait for initial sync (can take hours/days)
bitcoin-cli getblockchaininfo
```

### Lightning Network Setup (Optional)

```bash
# Install LND (Lightning Network Daemon)
wget https://github.com/lightningnetwork/lnd/releases/download/v0.16.0-beta/lnd-linux-amd64-v0.16.0-beta.tar.gz
tar -xzf lnd-linux-amd64-v0.16.0-beta.tar.gz
sudo cp lnd-linux-amd64-v0.16.0-beta/* /usr/local/bin/

# Configure LND
mkdir -p ~/.lnd
cat > ~/.lnd/lnd.conf << EOF
[Application Options]
debuglevel=info
maxpendingchannels=5
alias=anya-node

[Bitcoin]
bitcoin.active=1
bitcoin.mainnet=1
bitcoin.node=bitcoind

[Bitcoind]
bitcoind.rpchost=localhost:8332
bitcoind.rpcuser=bitcoinrpc
bitcoind.rpcpass=your_bitcoin_rpc_password
bitcoind.zmqpubrawblock=tcp://127.0.0.1:28332
bitcoind.zmqpubrawtx=tcp://127.0.0.1:28333
EOF

# Start LND
lnd
```

## Web5 SDK Configuration

### DID Resolver Setup

```bash
# Install Web5 DID resolver
npm install -g @web5/dids

# Configure resolver endpoints
cat > ~/.anya/web5-config.json << EOF
{
  "didResolvers": {
    "ion": {
      "endpoint": "https://beta.ion.msidentity.com/api/v1.0/identifiers/",
      "cache": true,
      "cacheTTL": 3600
    },
    "key": {
      "local": true
    },
    "web": {
      "timeout": 5000
    }
  },
  "credentialFormats": ["jwt", "jsonld"],
  "protocolDefinitions": {
    "social": "https://areweweb5yet.com/protocols/social",
    "chat": "https://areweweb5yet.com/protocols/chat"
  }
}
EOF
```

### Identity Wallet Setup

```bash
# Initialize Web5 identity wallet
anya-ext web5 init-wallet --seed-phrase "your twelve word seed phrase here"

# Create default DID
anya-ext web5 create-did --method ion --publish

# Export DID document
anya-ext web5 export-did --format json > ~/.anya/identity.json
```

## ML Runtime Setup

### ONNX Runtime Installation

```bash
# Install ONNX Runtime
pip3 install onnxruntime

# For GPU acceleration (optional)
pip3 install onnxruntime-gpu

# Verify installation
python3 -c "import onnxruntime; print(onnxruntime.get_device())"
```

### Model Repository Setup

```bash
# Initialize ML model repository
mkdir -p ~/.anya/models

# Download pre-trained models
anya-ext ml download-models --repository huggingface
anya-ext ml download-models --repository anya-official

# Configure model paths
cat > ~/.anya/ml-config.toml << EOF
[models]
repository_path = "/home/user/.anya/models"
cache_size = "10GB"
download_timeout = 300

[inference]
backend = "onnx"
device = "cpu"
batch_size = 32
max_sequence_length = 512

[training]
enabled = false
checkpoint_interval = 1000
validation_split = 0.2
EOF
```

## Verification

### Core System Verification

```bash
# Verify Anya Core installation
anya --version
anya status

# Test core functionality
anya test --quick

# Check extension availability
anya-ext list --installed
```

### Bitcoin Integration Verification

```bash
# Test Bitcoin connection
anya bitcoin status
anya bitcoin getblockchaininfo

# Verify transaction capabilities
anya bitcoin createwallet test_wallet
anya bitcoin getnewaddress

# Test Lightning (if configured)
anya lightning getinfo
```

### Web5 Integration Verification

```bash
# Test DID functionality
anya web5 did list
anya web5 did resolve did:key:example

# Test credential operations
anya web5 credential create --type "TestCredential"
anya web5 credential verify --file test.vc.jwt

# Test protocol operations
anya web5 protocol install --definition social
```

### ML Integration Verification

```bash
# Test ML models
anya ml models list
anya ml inference --model bert-base --input "Hello world"

# Test training capabilities
anya ml train --model test --dataset sample.json --epochs 1

# Performance benchmark
anya ml benchmark --all-models
```

## Troubleshooting

### Common Installation Issues

#### Rust Compilation Errors

```bash
# Update Rust toolchain
rustup update

# Clear cache and rebuild
cargo clean
cargo build --release

# Install missing dependencies
sudo apt-get install libclang-dev
```

#### Bitcoin Node Sync Issues

```bash
# Check sync status
bitcoin-cli getblockchaininfo

# Restart with different peers
bitcoin-cli stop
bitcoind -daemon -addnode=node.example.com

# Clear corrupted data (last resort)
rm -rf ~/.bitcoin/blocks ~/.bitcoin/chainstate
```

#### Web5 DID Resolution Failures

```bash
# Check network connectivity
curl -s https://beta.ion.msidentity.com/api/v1.0/identifiers/

# Update resolver endpoints
anya web5 config update-resolvers

# Clear DID cache
rm -rf ~/.anya/web5-cache
```

#### ML Model Loading Issues

```bash
# Check model file integrity
anya ml verify-models

# Re-download corrupted models
anya ml download-models --force

# Check system resources
anya ml system-info
```

### Performance Optimization

#### System Tuning

```bash
# Increase file descriptor limits
echo "* soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Optimize Bitcoin sync
bitcoin-cli setnetworkactive false
bitcoin-cli setnetworkactive true

# Configure swap for ML workloads
sudo swapon --show
sudo fallocate -l 8G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

#### Resource Monitoring

```bash
# Monitor system resources
anya system monitor

# Check Bitcoin resource usage
anya bitcoin resources

# Monitor ML inference performance
anya ml performance --watch
```

### Getting Help

- **Documentation**: [https://docs.anya.org](https://docs.anya.org)
- **Community Forum**: [https://forum.anya.org](https://forum.anya.org)
- **GitHub Issues**: [https://github.com/anya-org/Anya-core/issues](https://github.com/anya-org/Anya-core/issues)
- **Discord**: [https://discord.gg/anya](https://discord.gg/anya)

For installation support, please include:

- Operating system and version
- Anya Core version (`anya --version`)
- Error messages with full stack traces
- System resource information (`anya system info`)

## Next Steps

After successful installation:

1. **Configuration**: See [Configuration Guide](./configuration.md)
2. **Quick Start**: Follow [Quick Start Guide](./quick-start.md)
3. **Development**: Read [Development Guide](../development/README.md)
4. **Best Practices**: Review [Best Practices](../development/best-practices.md)
