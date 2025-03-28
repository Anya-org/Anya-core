# Anya Core v2.6

Bitcoin Development Framework Implementation following BDF v2.5 requirements.

## New Features in v2.6

1. **Automated Hardware Fallback Testing**
   - Automatic detection of HSM, SGX, FPGA, and TPM hardware
   - Seamless fallback to software emulation when hardware is unavailable
   - Comprehensive testing suite for hardware/software validation

2. **Database State Rollback for Atomic Installation**
   - Transaction-based installation phases
   - Database snapshot and restore for each installation phase
   - Atomic rollback functionality for safe installation/upgrade

3. **Enhanced Cross-Platform Support for Windows**
   - Native Windows service integration
   - Windows Registry configuration
   - Firewall rules and security management
   - Windows Event Log integration

4. **Improved Validator Address Rotation**
   - Multisig address generation (Legacy, SegWit, Native SegWit, Taproot)
   - Automated key rotation with expiration settings
   - HSM integration for hardware-backed keys when available
   - Rotation status monitoring and alerts

5. **CPU-Specific Cryptographic Optimizations**
   - Dynamic detection of CPU features (AVX, AVX2, SSE4, BMI, SHA extensions)
   - Optimized cryptographic implementations based on hardware capabilities
   - Performance benchmarking for cryptographic operations
   - Multiple optimization levels (None, Basic, Standard, Advanced, Maximum)

## Installation

   ```bash
# Install with default settings
cargo install --path .

# Run with hardware detection
anya-core test-hardware

# Initialize validators (2-of-3 multisig)
anya-core rotate-validators --init

# Optimize cryptographic operations for current CPU
anya-core optimize-crypto
```

## Testing

```bash
# Run the verification script to test all components
./scripts/verify_installation.sh

# Run individual tests
cargo test --lib
cargo test --bin anya-core
```

## CertiK Compliance

This implementation fully complies with CertiK audit requirements, including:
- BIP-341/342 (Taproot) compliance
- BIP-174/370 (PSBT v2) compliance
- AIS-3 security standards
- PFM-3 performance benchmarks

## System Requirements

- Linux/Windows 64-bit
- Minimum: 2 CPU cores, 4GB RAM, 500GB storage
- Recommended: 4+ CPU cores, 8GB+ RAM, 1TB+ storage
- Optional: HSM (YubiHSM, Nitrokey), TPM, SGX

## Documentation

See [INSTALLATION.md](docs/INSTALLATION.md) for detailed installation instructions.
See [COMPLIANCE_CHECKLIST.md](docs/COMPLIANCE_CHECKLIST.md) for compliance details.
