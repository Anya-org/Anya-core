# Anya Core Installation v2.5

## Compliance Requirements

- **BIP-341 (Taproot)**: SILENT_LEAF pattern verification
- **BIP-174 (PSBT)**: Version 2 mandatory
- **AIS-3 Security**:
  - Secure RNG (NIST SP 800-90A)
  - Constant-time operations
  - Memory-safe practices

## Updated Security Requirements

- **BIP-370 (PSBT v2)**: Fee rate validation mandatory
- **AIS-3 Enhancements**:
  - Constant-time hash comparisons for all security operations
  - Memory isolation for cryptographic operations
  - Hardware-backed RNG where available

## Audit Trail Format

```json
{
  "timestamp": 1712345678,
  "bip_compliance": {
    "bip341": true,
    "bip342": false,
    "bip174": true,
    "psbt_v2": true
  },
  "security_checks": {
    "rng_secure": true,
    "constant_time": true,
    "mem_safe": true
  },
  "files": [
    {
      "path": "/opt/anya/conf/bitcoin.conf",
      "sha256": "a1b2c3..."
    }
  ]
}
```

## Validation Commands

```bash
# Verify installation
anya-validator --check compliance --level bpc3

# Test cryptographic safety
anya-validator --check crypto --algo sha256

# Generate audit report
anya-audit --format json > installation_audit.json

# Verify PSBT v2 compliance
anya-validator --check psbt-v2 --level strict

# Test Taproot commitments
anya-validator --check taproot --silent-leaf

# Verify Silent Payments implementation
anya-validator --check silent-payments --level strict

# Test Silent Payment address generation
anya-test silent-payments --create-address

# Test Silent Payment scanning
anya-test silent-payments --scan --tx-file test_vectors.json
```

This implementation provides:

1. **BIP-341 Compliance**:
   - SILENT_LEAF pattern verification
   - Taproot configuration validation

2. **BIP-174 Enforcement**:
   - PSBT v2 requirement
   - Transaction serialization checks

3. **AIS-3 Security**:
   - NIST-compliant RNG
   - Constant-time hash comparisons
   - Memory-safe buffers

4. **Audit Trail**:
   - JSON-formatted installation records
   - Cryptographic file hashes
   - Compliance status tracking

To use:

1. Build with `cargo build --release`
2. Run with elevated privileges: `sudo ./target/release/anya_installer`

The installer automatically:

- Creates secure Bitcoin configuration
- Generates audit logs
- Validates against BIP standards
- Enforces cryptographic best practices

Would you like me to add any specific component or expand on a particular security aspect?

## Overview

The Anya Core Installer provides a comprehensive interface for managing Bitcoin network configurations and system dependencies. It supports multiple network types and allows for custom RPC configurations.

## Installation

### Prerequisites

- Rust (latest stable version)
- Git
- System dependencies (automatically installed by installer)

### Installation Process

```bash
# Install Anya Core with default settings
anya-installer install

# Install specific network configuration
anya-installer install --network mainnet

# Install with custom RPC settings
anya-installer install --network testnet \
  --rpc-url "https://custom-rpc.example.com" \
  --rpc-user "custom_user" \
  --rpc-password "custom_password"
```

## Configuration

### Network Configuration

The installer supports three network types:

- `mainnet`: Bitcoin main network
- `testnet`: Bitcoin test network
- `regtest`: Local regression testing network

### Default RPC Endpoints

The installer uses PublicNode's RPC endpoints by default:

- **Mainnet**: `https://bitcoin-rpc.publicnode.com`
- **Testnet**: `https://bitcoin-testnet-rpc.publicnode.com`
- **Regtest**: `http://localhost:18443/` (for local development)

### BDK Integration

The installer is configured to use Bitcoin Dev Kit (BDK) for wallet management:

```bash
# Configure BDK wallet directory
anya-installer configure --network testnet --bdk-wallet-dir /path/to/wallets

# View BDK configuration
anya-installer configure --show
```

### Custom RPC Configuration

While PublicNode endpoints are recommended, you can configure custom RPC settings:

```bash
# Set custom RPC configuration
anya-installer configure \
  --network testnet \
  --rpc-url "https://custom-rpc.example.com" \
  --rpc-user "custom_user" \
  --rpc-password "custom_password"
```

### Advanced Channel Management

1. **Channel Security**: Use secure channel limits and fees

### Logging Configuration

Log levels can be set to control verbosity:

- `trace`: Most detailed
- `debug`: Detailed
- `info`: Normal
- `warn`: Warnings
- `error`: Errors only

```bash
# Set log level
anya-installer configure --log-level debug
```

## Auto-Configuration

The Anya Core Installer now includes an intelligent auto-configuration feature that optimizes your setup based on your system's resources. This feature automatically determines the best configuration for your system based on:

1. **System Memory**
   - 8GB+ RAM: Enables all features (BDK, LDK, DLC, RGB, RSK, Web5)
   - 4GB-8GB RAM: Enables most features (BDK, LDK, DLC, RGB, RSK)
   - <4GB RAM: Enables basic features (BDK, LDK)

2. **CPU Cores**
   - 4+ cores: Enables CPU-intensive features (DLC, RGB, RSK)
   - <4 cores: Disables CPU-intensive features

3. **Disk Space**
   - 100GB+: Enables full blockchain and data storage
   - <100GB: Disables blockchain and data storage

4. **Network Bandwidth**
   - 100Mbps+: Uses mainnet for optimal performance
   - <100Mbps: Uses testnet for better performance

### Using Auto-Configuration

To use the auto-configuration feature, simply run:

```bash
anya-installer configure --auto
```

The installer will:

1. Analyze your system resources
2. Determine the optimal configuration
3. Set up appropriate directory structures
4. Configure optimal RPC settings
5. Set appropriate channel limits for LDK
6. Configure backup directories with timestamps
7. Select the most appropriate network (mainnet/testnet)

### Auto-Configuration Benefits

1. **Resource Optimization**
   - Automatically adjusts feature set based on available resources
   - Prevents system overload
   - Optimizes performance

2. **Security**
   - Uses secure defaults
   - Configures appropriate backup settings
   - Sets optimal channel limits

3. **Ease of Use**
   - No manual configuration required
   - Works out-of-the-box
   - Handles complex setup automatically

4. **Performance**
   - Optimizes for your specific hardware
   - Balances resource usage
   - Provides optimal network configuration

### Example Auto-Configuration

```bash
# Auto-configure based on system resources
anya-installer configure --auto

# View the auto-configured settings
anya-installer configure --show
```

### Manual Overrides

While the auto-configuration is designed to work well for most users, you can still manually override any settings:

```bash
# Auto-configure with manual overrides
anya-installer configure \
  --auto \
  --network mainnet \
  --dlc-enabled true \
  --rgb-enabled true
```

### Best Practices with Auto-Configuration

1. **Initial Setup**
   - Use auto-configuration for initial setup
   - Review the generated configuration
   - Make manual adjustments if needed

2. **Resource Monitoring**
   - Monitor system resources after setup
   - Adjust configurations if needed
   - Use backup features regularly

3. **Security**
   - Review auto-generated RPC settings
   - Verify backup configurations
   - Check channel limits

4. **Performance**
   - Monitor system performance
   - Adjust configurations based on usage
   - Use testnet for development

## Lightning Network (LDK) Configuration

The installer supports Lightning Network integration using LDK (Lightning Development Kit):

### Default Configuration

```bash
# Enable Lightning Network
anya-installer configure --network testnet --ldk-enabled true

# View LDK configuration
anya-installer configure --show
```

### Channel Manager Configuration

The channel manager is configured with:

- Channel limit: 100 channels
- Minimum channel size: 0.01 BTC
- Maximum channel size: ~0.1677 BTC
- Base fee: 1000 msat
- Proportional fee: 1%
- CLTV expiry delta: 144 blocks

### Router Configuration

The router is configured with:

- Network graph sync interval: 5 minutes
- Scorer penalty half-life: 24 hours
- Base penalty: 1000 msat

### Wallet Configuration

The LDK wallet is configured with:

- Auto backup: Enabled
- Backup interval: 1 hour
- Storage path: `lightning/wallet`
- Backup path: `lightning/backup`

### Advanced Channel Management

1. **Channel Security**: Use secure channel limits and fees
2. **Backup**: Regularly backup LDK wallet and network graph

### Monitoring

The installer provides monitoring capabilities for:

- Channel health
- Network connectivity
- Routing performance
- Wallet balance

### Best Practices

1. Always use `--dry-run` before making changes
2. Keep RPC credentials secure
3. Regularly test configurations
4. Maintain backup of configuration files
5. Use PublicNode endpoints for reliability
6. Enable BDK for secure wallet management
7. Monitor channel health regularly
8. Use secure fee settings
9. Regularly backup LDK data

## Testing

The installer includes comprehensive testing capabilities:

```bash
# Run all tests
anya-installer test

# Run specific component tests
anya-installer test --component network

# Generate test report
anya-installer test --report
```

### Network Tests

The installer verifies:

- RPC connection
- Network status
- Block height
- Fee estimation

### BDK Tests

The installer tests:

- Wallet creation
- Address generation
- Transaction signing
- Balance checking

## Troubleshooting

If you encounter issues:

1. Enable verbose logging: `--verbose`
2. Run in dry mode: `--dry-run`
3. Check configuration: `configure --show`
4. Review logs in `data/logs`
5. Verify RPC connection using curl
6. Check BDK wallet directory permissions
7. Verify LDK channel status
8. Check network graph sync
9. Verify router configuration

**Error**: "PSBT version 2 required"

- Verify Bitcoin Core 24.0+ is installed
- Check config contains `psbt_version=2`
- Run `anya-installer reconfigure --psbt-v2`

## Support

For support and documentation, visit: <https://anya.org/docs>

### BDK Documentation

- [BDK GitHub](https://github.com/bitcoindevkit/bdk)
- [BDK Book](https://bitcoindevkit.org/book/)
- [PublicNode Documentation](https://publicnode.com/docs)

### LDK Documentation

- [LDK GitHub](https://github.com/lightningdevkit/ldk)
- [LDK Book](https://lightningdevkit.org/book/)
- [LDK Examples](https://github.com/lightningdevkit/ldk-examples)
- [LDK API Docs](https://lightningdevkit.org/api/)

### Network Resources

- [Bitcoin Network Status](https://status.bitcoin.org/)
- [PublicNode Status](https://status.publicnode.com/)
- [BDK Wallet Management](https://bitcoindevkit.org/book/working_with_wallets.html)
- [LDK Wallet Management](https://lightningdevkit.org/book/working_with_wallets.html)
- [Channel Management](https://lightningdevkit.org/book/channel_management.html)
- [Router Configuration](https://lightningdevkit.org/book/router.html)

Mobile:

- React Native 0.72+
- Android Studio/Xcode with Rust toolchain

## Continuous Integration

```bash
# Run CI checks locally
act -j build-and-test
act -j security-audit

# Run security tests
cargo test --release --lib --bins -p anya-installer -- security::
```
