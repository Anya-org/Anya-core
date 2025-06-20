---
title: "Installation Troubleshooting"
description: "Common installation issues and their solutions"
last_updated: 2025-06-17
---

# Installation Troubleshooting

This guide helps resolve common issues encountered during installation of Anya Core.

[AIR-3][AIS-3][BPC-3][RES-3]

## Common Issues

### 1. Docker-related Issues

#### Docker Service Not Running

**Symptoms:**

- "Cannot connect to Docker daemon" errors
- Installation scripts fail at container creation stage

**Solutions:**

```bash
# Check Docker service status
systemctl status docker

# Start Docker service if stopped
sudo systemctl start docker

# Enable Docker to start on boot
sudo systemctl enable docker
```

#### Permission Denied

**Symptoms:**

- "Permission denied" when running Docker commands
- Installation stops with access errors

**Solutions:**

```bash
# Add current user to the docker group
sudo usermod -aG docker $USER

# Apply changes (requires logout/login)
newgrp docker

# Try the installation again
./scripts/install/main_installer.sh
```

### 2. Dependency Issues

#### Missing Rust Components

**Symptoms:**

- "rustc not found" or similar errors
- Build failures mentioning missing Rust components

**Solutions:**

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust to latest stable
rustup update stable

# Add required components
rustup component add rustfmt clippy
```

#### Python Environment Issues

**Symptoms:**

- "Python module not found" errors
- Version conflicts

**Solutions:**

```bash
# Ensure Python 3.8+ is installed
python3 --version

# Install required Python packages
pip3 install -r requirements.txt

# If using virtual environments
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### 3. Network Issues

#### Firewall Blocking Connections

**Symptoms:**

- Installation hangs when downloading dependencies
- Connection timeouts

**Solutions:**

```bash
# Check if required ports are open
sudo ufw status

# Allow required ports (adjust as needed)
sudo ufw allow 3000
sudo ufw allow 9090
sudo ufw allow 9093
```

#### Proxy Configuration

**Symptoms:**

- Downloads fail but internet connection works
- Connection errors through corporate networks

**Solutions:**

```bash
# Set HTTP proxy for Docker
mkdir -p ~/.docker
cat > ~/.docker/config.json << EOF
{
  "proxies": {
    "default": {
      "httpProxy": "http://proxy.example.com:8080",
      "httpsProxy": "http://proxy.example.com:8080",
      "noProxy": "localhost,127.0.0.1"
    }
  }
}
EOF

# Set proxy for shell
export HTTP_PROXY="http://proxy.example.com:8080"
export HTTPS_PROXY="http://proxy.example.com:8080"
export NO_PROXY="localhost,127.0.0.1"
```

### 4. Storage Issues

#### Insufficient Disk Space

**Symptoms:**

- "No space left on device" errors
- Installation stops unexpectedly

**Solutions:**

```bash
# Check available disk space
df -h

# Clean up Docker resources
docker system prune -a

# Identify large files/directories
du -sh /* | sort -hr | head -10
```

### 5. Bitcoin Core Integration Issues

#### Block Data Synchronization Problems

**Symptoms:**

- Installation completes but Bitcoin Core node doesn't sync
- Errors about blockchain data corruption

**Solutions:**

```bash
# Check Bitcoin Core logs
docker-compose logs bitcoin-core

# Reset blockchain data (caution: full resync required)
./scripts/reset_bitcoin_data.sh

# Use pruned mode for faster sync
./scripts/install/main_installer.sh --bitcoin-prune=550
```

## Advanced Troubleshooting

### Generate Diagnostic Report

If you're experiencing issues not covered above, generate a diagnostic report:

```bash
./scripts/diagnostics.sh > diagnostic_report.txt
```

This creates a comprehensive report that can help identify the issue.

### Debug Mode

Run the installer in debug mode for more verbose output:

```bash
./scripts/install/main_installer.sh --debug
```

### Manual Component Testing

Test individual components:

```bash
# Test Bitcoin Core connectivity
./scripts/test_bitcoin_connection.sh

# Test monitoring stack
./scripts/test_monitoring.sh

# Validate configuration
./scripts/validate_config.sh
```

## Getting Help

If you're still experiencing issues:

1. Check the [GitHub Issues](https://github.com/anya-org/anya-core/issues) for similar problems
2. Join our [Discord community](https://discord.gg/anya-core) for real-time support
3. Open a new issue with your diagnostic report attached

## See Also

- [Installation Guide](../INSTALLATION.md)
- [Installation Review](../../INSTALLATION_REVIEW.md)
- [Docker Configuration](../docs/docker-config.md)
