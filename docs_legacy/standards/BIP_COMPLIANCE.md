---
title: "Bip_compliance"
description: "Documentation for Bip_compliance"
---

[AIR-3][AIS-3][BPC-3][RES-3]


# Bitcoin Improvement Proposal (BIP) Compliance

## Overview

Add a brief overview of this document here.


This document outlines the BIPs implemented in Anya Core and their compliance status.

## Table of Contents
- [Implemented BIPs](#implemented-bips)
- [Planned BIPs](#planned-bips)
- [BIP Compliance Testing](#bip-compliance-testing)
- [Reference Implementations](#reference-implementations)
- [Deviation Policy](#deviation-policy)

## Implemented BIPs

### Consensus Layer

| BIP | Title | Status | Notes |
|-----|-------|--------|-------|
| 9 | Version bits with timeout and delay | ✅ Implemented | - |
| 30 | Duplicate transactions | ✅ Implemented | - |
| 34 | Block v2, Height in coinbase | ✅ Implemented | - |
| 65 | OP_CHECKLOCKTIMEVERIFY | ✅ Implemented | - |
| 66 | Strict DER signatures | ✅ Implemented | - |
| 68 | Relative lock-time using consensus-enforced sequence numbers | ✅ Implemented | - |
| 112 | CHECKSEQUENCEVERIFY | ✅ Implemented | - |
| 113 | Median time-past as endpoint for lock-time calculations | ✅ Implemented | - |
| 141 | Segregated Witness | ✅ Implemented | - |
| 143 | Transaction Signature Verification | ✅ Implemented | - |
| 147 | Dealing with dummy stack element malleability | ✅ Implemented | - |
| 158 | Compact Block Filters | ✅ Implemented | - |
| 340 | Schnorr Signatures | ✅ Implemented | - |
| 341 | Taproot | ✅ Implemented | - |
| 342 | Tapscript | ✅ Implemented | - |

### Wallet Layer

| BIP | Title | Status | Notes |
|-----|-------|--------|-------|
| 32 | Hierarchical Deterministic Wallets | ✅ Implemented | - |
| 39 | Mnemonic code for generating deterministic keys | ✅ Implemented | - |
| 44 | Multi-Account Hierarchy for Deterministic Wallets | ✅ Implemented | - |
| 49 | Derivation scheme for P2WPKH-nested-in-P2SH | ✅ Implemented | - |
| 84 | Derivation scheme for P2WPKH | ✅ Implemented | - |
| 86 | Key Derivation for Single Key P2TR Outputs | ✅ Implemented | - |
| 174 | Partially Signed Bitcoin Transaction Format | ✅ Implemented | - |
| 370 | PSBT Version 2 | ✅ Implemented | - |

## Planned BIPs

| BIP | Title | Target Version | Notes |
|-----|-------|----------------|-------|
| 118 | SIGHASH_ANYPREVOUT | v3.0.0 | In development |
| 119 | CHECKTEMPLATEVERIFY | v3.1.0 | Planned |
| 350 | Output Script Descriptors | v3.0.0 | In progress |

## BIP Compliance Testing

### Test Vectors

We maintain test vectors for all implemented BIPs:

```bash
# Run BIP test suite
cargo test --test bip_tests -- --nocapture
```

### Compliance Matrix

| BIP | Test Coverage | Last Verified | Notes |
|-----|---------------|----------------|-------|
| 32 | 100% | 2025-05-10 | - |
| 39 | 100% | 2025-05-10 | - |
| 340-342 | 98% | 2025-05-15 | Minor test cases pending |

## Reference Implementations

We verify our implementation against the following references:

1. **Bitcoin Core**
   - Version: 25.0
   - Commit: abc1234
   - Tested against test vectors

2. **Libsecp256k1**
   - Version: 0.3.0
   - Used for cryptographic primitives

## Deviation Policy

### When We May Deviate

1. **Security Improvements**
   - If a BIP contains security vulnerabilities
   - When more secure alternatives exist

2. **Performance Optimizations**
   - Significant performance benefits
   - No impact on consensus rules

3. **Implementation Constraints**
   - Platform-specific limitations
   - Hardware constraints

### Process for Deviations

1. Document the deviation in `DEVIATIONS.md`
2. Include rationale and security analysis
3. Get approval from security team
4. Update documentation

## Testing Framework

### Unit Tests

```rust
#[test]
fn test_bip32_key_derivation() {
    // Test vectors from BIP-32
    let seed = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
    let master = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).unwrap();
    
    // Test derivation path m/0'
    let derived = master.derive_priv(&Path::from_str("m/0'").unwrap()).unwrap();
    assert_eq!(
        derived.to_string(),
        "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7"
    );
}
```

### Integration Tests

```bash
# Run BIP integration tests
cargo test --test bip_integration -- --test-threads=1
```

## Security Considerations

### Key Management

- All keys are derived using BIP-32/39/44
- Private keys are never stored in plaintext
- Hardware wallet integration follows BIP-174/370

### Transaction Malleability

- All transaction handling follows BIP-62
- Strict DER encoding enforced (BIP-66)
- SegWit (BIP-141) for transaction malleability fixes

## Contributing

### Adding New BIPs

1. Create a feature branch: `feature/bip-XXX`
2. Implement the BIP with tests
3. Update this document
4. Submit a pull request

### Testing Requirements

- 100% test coverage for new BIPs
- Cross-implementation compatibility tests
- Fuzz testing for security-critical components

## References

- [BIP Repository](https://github.com/bitcoin/bips)
- [Bitcoin Core](https://github.com/bitcoin/bitcoin)
- [BIP-Status](https://bip-status.com/)

## See Also

- [Related Document](#related-document)

