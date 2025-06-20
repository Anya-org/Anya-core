# Hardware Security Module (HSM) \[AIR-3\]\[AIS-3\]\[AIT-3\]\[AIP-3\]\[RES-3\]

<!-- markdownlint-disable MD013 line-length -->

## Overview

The HSM module provides secure cryptographic operations for Anya Core with special support for Bitcoin operations. This implementation follows official Bitcoin Improvement Proposals (BIPs) requirements for security, privacy, and integration.

## Key Features

### Core HSM Functionality

- Secure key management with comprehensive audit logging
- Support for various cryptographic operations (signing, verification, encryption, decryption)
- Key rotation and lifecycle management
- Pluggable provider architecture (software HSM, Cloud HSM, TPM, PKCS#11)

### Bitcoin-Specific Features

- Support for all Bitcoin address types (Legacy, SegWit, Taproot)
- BIP32/44/49/84/86 compliant key derivation paths
- Taproot (BIP341/342) implementation with script tree support
- Discrete Log Contracts (DLCs) with non-interactive oracle patterns
- SPV proof verification for Bitcoin transactions
- PSBT (BIP174) support for transaction construction and signing

## Architecture

The HSM module follows the hexagonal architecture pattern:

```
                  +----------------+
                  |    HSM API     |
                  +-------+--------+
                          |
+----------------+  +-----v--------+  +----------------+
|   Provider     |  |    HSM       |  |   Audit        |
|   Adapters     <--+   Manager    +-->   Logging      |
| (HSM types)    |  |              |  |                |
+----------------+  +-------+------+  +----------------+
                          |
                  +-------v--------+
                  | Bitcoin-specific|
                  | Operations      |
                  +----------------+
```

## Module Structure

- `mod.rs`: Main HSM manager implementation
- `provider.rs`: HSM provider trait and implementations
- `config.rs`: Configuration structures for the HSM
- `audit.rs`: Audit logging functionality
- `error.rs`: Error types and handling
- `bitcoin.rs`: Bitcoin-specific HSM operations
- `tests.rs`: Tests and examples

## Usage Examples

### Basic HSM Operations

```rust
// Create HSM configuration
let config = HsmConfig::development();

// Create HSM manager
let hsm_manager = HsmManager::new(config);

// Initialize HSM manager
hsm_manager.initialize().await?;

// Generate key pair
let key_params = KeyGenParams {
    id: Some("signing-key-1".to_string()),
    label: "Signing Key".to_string(),
    key_type: KeyType::Ec { curve: EcCurve::P256 },
    extractable: false,
    usages: vec![KeyUsage::Sign, KeyUsage::Verify],
    expires_at: None,
    attributes: HashMap::new(),
};

let public_key_info = hsm_manager.generate_key_pair(key_params).await?;

// Sign data
let signature = hsm_manager.sign(
    &public_key_info.id,
    SigningAlgorithm::EcdsaSha256,
    data_to_sign
).await?;

// Verify signature
let is_valid = hsm_manager.verify(
    &public_key_info.id,
    SigningAlgorithm::EcdsaSha256,
    data_to_sign,
    &signature
).await?;
```

### Bitcoin Operations

```rust
// Create Bitcoin HSM provider
let base_provider = Arc::new(hsm_manager);
let bitcoin_config = BitcoinHsmConfig {
    base_provider,
    network: BitcoinNetwork::Testnet,
    derivation_path_template: "m/86'/0'/0'/0/{}".to_string(),
    use_taproot: true,
    default_key_type: BitcoinKeyType::Taproot,
};

let bitcoin_provider = BitcoinHsmProvider::new(bitcoin_config);

// Generate Bitcoin key
let bitcoin_key = bitcoin_provider.generate_bitcoin_key(
    "wallet",
    Some(BitcoinKeyType::Taproot),
    Some(0)
).await?;

// Sign a Bitcoin transaction
let signature = bitcoin_provider.sign_bitcoin_transaction(
    &bitcoin_key.key_id,
    transaction_hex,
    BitcoinSignatureType::Schnorr,
    0x01 // SIGHASH_ALL
).await?;

// Create a Taproot asset
let asset_id = create_taproot_asset(
    &bitcoin_provider,
    r#"{"name":"Anya Token","ticker":"ANY"}"#,
    21000000
).await?;
```

## Security Considerations

1. **Key Protection**: Private keys should never leave the HSM boundary.
2. **Authentication**: Proper authentication should be implemented before using HSM operations.
3. **Audit Logs**: Always enable and monitor audit logs for security events.
4. **Secure Configuration**: Use appropriate HSM configurations based on environment:
   - Development: Use `HsmConfig::development()`
   - Production: Use `HsmConfig::production()`
5. **Hardware HSMs**: For production environments, use a hardware HSM implementation.

## Compliance

This implementation complies with the following standards:

- Bitcoin Improvement Proposals (BIPs) (Updated March 2025)
- BIP 341/342 (Taproot)
- BIP 174 (PSBT)
- BIP 340 (Schnorr Signatures)

## Future Development

Planned enhancements:

1. Lightning Network integration
2. Multi-party computation support
3. Hardware HSM provider implementations
4. Enhanced DLC functionality

## Testing

Comprehensive tests are included in `tests.rs`. Run the tests with:

```
cargo test --package anya-core --lib -- security::hsm::tests
```

## Documentation

For detailed documentation, refer to:

- [HSM Bitcoin Integration](../../../docs/hsm_bitcoin_integration.md)
- [API Reference](./mod.rs) (code documentation)
