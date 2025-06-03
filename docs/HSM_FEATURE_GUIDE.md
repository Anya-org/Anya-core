---
title: "Hsm_feature_guide"
description: "Documentation for Hsm_feature_guide"
---

[AIR-3][AIS-3][BPC-3][RES-3]


# HSM Feature Guide

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


[AIS-3][RES-3][PFM-2]

## Overview

The Hardware Security Module (HSM) feature in Anya Core provides secure key management and cryptographic operations. It is designed with a modular architecture that supports various HSM providers, from software-based implementations for development to hardware-backed security devices for production.

## Feature Flags

The HSM functionality is controlled through Cargo feature flags, making it possible to compile and use Anya Core without HSM support if not needed. This approach provides flexibility for different deployment scenarios.

### Available Feature Flags

- `hsm`: Enables the full HSM functionality
- `complete`: A meta-feature that includes HSM and other optional features

### Building With or Without HSM

```bash
# Build without HSM functionality
cargo build

# Build with HSM functionality
cargo build --features hsm

# Build with all features including HSM
cargo build --features complete
```

## Provider Architecture

The HSM module is designed with a pluggable provider architecture:

1. **SoftwareHsmProvider**: Software-based implementation for development and testing
2. **HardwareHsmProvider**: Integration with generic hardware security devices
3. **TPM**: Support for Trusted Platform Module chips
4. **PKCS11**: Support for PKCS#11 compliant devices like smartcards and hardware tokens
5. **BitcoinHsmProvider**: Bitcoin-specific operations for keys and transactions
6. **LedgerHsmProvider**: Integration with Ledger hardware wallets

## Implementation Status

| Provider            | Status       | Notes                                          |
|---------------------|--------------|------------------------------------------------|
| SoftwareHsmProvider | Complete     | Fully functional for development               |
| HardwareHsmProvider | Beta         | Basic operations implemented                   |
| TPM                 | Alpha        | Core functionality working                     |
| PKCS11              | Alpha        | Basic integration completed                    |
| BitcoinHsmProvider  | Beta         | Bitcoin-specific operations implemented        |
| LedgerHsmProvider   | Alpha        | Initial support for key operations             |

## Using HSM in Your Code

When HSM functionality is disabled, a shim implementation is provided that maintains API compatibility but returns appropriate errors when HSM operations are attempted.

### Example: Working with HSM Regardless of Feature Flag

```rust
use anya_core::security::{HsmManager, HsmStatus};

async fn initialize_security() -> Result<(), Box<dyn std::error::Error>> {
    // This code works whether HSM is enabled or not
    match HsmManager::new(config) {
        Ok(hsm) => {
            println!("HSM available, initializing...");
            hsm.initialize().await?;
        },
        Err(e) => {
            println!("HSM not available: {}", e);
            // Fall back to alternative security mechanism
        }
    }
    
    Ok(())
}
```

## Hardware Support

The HSM module supports several hardware security devices through different providers:

1. **TPM Chips**: Available on most modern motherboards
2. **Hardware Security Modules**: Physical devices like YubiHSM or Nitrokey HSM
3. **Smartcards**: Through the PKCS#11 interface
4. **Hardware Wallets**: Currently Ledger, with plans for Trezor support

## Building Your Own Provider

The HSM architecture allows for custom providers through the `HsmProvider` trait:

```rust
#[async_trait]
pub trait HsmProvider: std::fmt::Debug + Send + Sync {
    async fn get_status(&self) -> Result<HsmProviderStatus, HsmError>;
    async fn generate_key(&self, params: KeyGenParams) -> Result<KeyInfo, HsmError>;
    async fn sign(&self, key_id: &str, data: &[u8], algorithm: SigningAlgorithm) -> Result<Vec<u8>, HsmError>;
    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8], algorithm: SigningAlgorithm) -> Result<bool, HsmError>;
    // ... additional methods
}
```

## Security Considerations

1. **Key Isolation**: Hardware-backed keys never leave the secure boundary
2. **Access Control**: Implementation of fine-grained access policies
3. **Audit Logging**: Comprehensive logging of all HSM operations
4. **Tamper Resistance**: Hardware providers offer physical tamper protection
5. **Performance vs Security**: Configurable trade-offs based on use case

## See Also

- [Related Document](#related-document)

