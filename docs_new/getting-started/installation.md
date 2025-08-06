# Installation Guide

This guide will help you install and configure Anya Core on your system.

## Prerequisites

### System Requirements

- **Operating System**: Linux (Ubuntu 20.04+), macOS (11+), Windows (WSL2)
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 10GB free space minimum, 50GB recommended for full sync
- **Network**: Stable internet connection for blockchain sync

### Required Software

#### Rust Toolchain

Anya Core requires Rust 1.70 or later:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Additional Dependencies

**Ubuntu/Debian:**

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    cmake
```

**macOS:**

```bash
# Install Xcode command line tools
xcode-select --install

# Using Homebrew
brew install cmake pkg-config openssl sqlite
```

**Windows (WSL2):**

```bash
# In WSL2 terminal
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    cmake
```

## Installation Methods

### Method 1: From Source (Recommended)

#### 1. Clone the Repository

```bash
git clone https://github.com/Anya-org/Anya-core.git
cd Anya-core
```

#### 2. Configure Build Features

Anya Core uses feature flags to enable specific functionality. Create a build configuration:

```bash
# Copy example configuration
cp config/example.toml config/local.toml

# Edit configuration as needed
nano config/local.toml
```

#### 3. Build the Project

```bash
# Standard build with all features
cargo build --release --all-features

# Or build with specific features only
cargo build --release --features "bitcoin,web5,ml"
```

#### 4. Run Tests

```bash
# Run all tests
cargo test --all-features

# Run specific module tests
cargo test bitcoin::tests
cargo test web5::tests
```

### Method 2: Using Docker

#### 1. Pull the Docker Image

```bash
docker pull ghcr.io/anya-org/anya-core:latest
```

#### 2. Run with Docker Compose

```bash
# Clone repository for docker-compose.yml
git clone https://github.com/Anya-org/Anya-core.git
cd Anya-core

# Start all services
docker-compose up -d

# Check status
docker-compose ps
```

### Method 3: Pre-built Binaries

Download pre-built binaries from the [releases page](https://github.com/Anya-org/Anya-core/releases):

```bash
# Download for your platform
wget https://github.com/Anya-org/Anya-core/releases/latest/download/anya-core-linux-x64.tar.gz

# Extract
tar -xzf anya-core-linux-x64.tar.gz

# Install
sudo cp anya-core /usr/local/bin/
```

## Configuration

### Basic Configuration

Create a configuration file at `config/anya.toml`:

```toml
[general]
# Network configuration
network = "mainnet"  # or "testnet", "regtest"
data_dir = "~/.anya"
log_level = "info"

[api]
# API server settings
host = "127.0.0.1"
port = 8080
cors_enabled = true
rate_limit = 100  # requests per minute

[bitcoin]
# Bitcoin node configuration
enabled = true
rpc_url = "http://127.0.0.1:8332"
rpc_user = "bitcoin"
rpc_password = "your_rpc_password"

[web5]
# Web5 protocol settings
enabled = true
did_method = "did:web5"
dwn_endpoints = ["https://dwn.your-domain.com"]

[ml]
# Machine learning configuration
enabled = true
model_cache_size = "1GB"
inference_timeout = 30  # seconds

[security]
# Security settings
hsm_enabled = false
key_derivation = "bip44"
encryption_at_rest = true
```

### Advanced Configuration

#### Bitcoin Node Setup

If running your own Bitcoin node:

```toml
[bitcoin]
enabled = true
node_type = "core"  # or "electrum"
rpc_url = "http://127.0.0.1:8332"
rpc_user = "your_username"
rpc_password = "your_password"
network = "mainnet"
prune = false
txindex = true
```

#### Layer2 Configuration

```toml
[layer2]
# Lightning Network
lightning_enabled = true
lightning_network = "mainnet"
lightning_data_dir = "~/.anya/lightning"

# RGB Protocol
rgb_enabled = true
rgb_data_dir = "~/.anya/rgb"

# DLC (Discreet Log Contracts)
dlc_enabled = true
dlc_data_dir = "~/.anya/dlc"
```

#### Web5 and Identity

```toml
[web5]
enabled = true
did_method = "did:web5"

[web5.dwn]
# Decentralized Web Node configuration
enabled = true
storage_backend = "sqlite"  # or "rocksdb"
max_storage_size = "10GB"
```

## Starting Anya Core

### Development Mode

```bash
# Start with development configuration
cargo run -- --config config/development.toml

# Or using the binary
./anya-core --config config/development.toml --log-level debug
```

### Production Mode

```bash
# Start as daemon
./anya-core \
    --config config/production.toml \
    --daemon \
    --pid-file /var/run/anya-core.pid
```

### Using systemd (Linux)

Create a systemd service file at `/etc/systemd/system/anya-core.service`:

```ini
[Unit]
Description=Anya Core Daemon
After=network.target

[Service]
Type=simple
User=anya
Group=anya
ExecStart=/usr/local/bin/anya-core --config /etc/anya/anya.toml
Restart=always
RestartSec=10

# Security settings
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/lib/anya

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl enable anya-core
sudo systemctl start anya-core
sudo systemctl status anya-core
```

## Verification

### Check Installation

```bash
# Verify binary works
anya-core --version

# Check configuration
anya-core --check-config config/anya.toml
```

### API Health Check

```bash
# Check if API server is running
curl -s http://localhost:8080/api/v1/health | jq

# Expected response:
# {
#   "status": "operational",
#   "version": "1.3.0",
#   "uptime_seconds": 123,
#   "components": {
#     "bitcoin": "connected",
#     "web5": "operational",
#     "ml": "ready"
#   }
# }
```

### Component Status

```bash
# Check individual components
curl -s http://localhost:8080/api/v1/system/status | jq

# Bitcoin connection
curl -s http://localhost:8080/api/v1/bitcoin/info | jq

# Web5 node status
curl -s http://localhost:8080/api/v1/web5/status | jq
```

## Common Issues

### Build Errors

**Error: `failed to run custom build command for 'openssl-sys'`**

```bash
# On Ubuntu/Debian
sudo apt install libssl-dev pkg-config

# On macOS
export PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig"
```

**Error: `linking with 'cc' failed`**

```bash
# Install build tools
sudo apt install build-essential
```

### Runtime Issues

**Error: `Connection refused to Bitcoin RPC`**

1. Check Bitcoin node is running
2. Verify RPC credentials in config
3. Ensure network connectivity

**Error: `Database migration failed`**

1. Check disk space
2. Verify database permissions
3. Run with `--reset-db` if needed (⚠️ data loss)

### Performance Issues

**High Memory Usage**

- Reduce ML model cache size
- Lower Bitcoin mempool size limits
- Use pruned Bitcoin node

**Slow Startup**

- Increase database cache size
- Use SSD storage
- Optimize network configuration

## Next Steps

After successful installation:

1. **[Core Concepts](concepts.md)** - Understand Anya Core architecture
2. **[Quick Start Tutorial](quickstart.md)** - Build your first integration
3. **[API Documentation](../api/README.md)** - Explore available APIs
4. **[Configuration Reference](../reference/configuration.md)** - Advanced configuration

## Support

**Installation Issues:**

- GitHub Issues: [Report installation problems](https://github.com/Anya-org/Anya-core/issues)
- Discord: Join our community for real-time help
- Documentation: [Troubleshooting Guide](../operations/troubleshooting.md)

**Security Concerns:**

- Email: <security@anya-core.dev>
- GPG Key: Available on our website

---

**Need help?** Join our [Discord community](https://discord.gg/anya-core) or check the [troubleshooting guide](../operations/troubleshooting.md).
