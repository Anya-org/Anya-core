---
name: Bitcoin Module Compilation Issues
about: Track compilation issues after Bitcoin module restructuring
title: '[BITCOIN] Fix compilation issues in restructured Bitcoin module'
labels: bug, bitcoin, hexagonal-architecture
assignees: bo_thebig
---

## Description

After restructuring the Bitcoin module to follow a hexagonal architecture pattern and implementing BIP-341 and BIP-342, several compilation issues need to be addressed before the changes can be fully integrated.

## Compilation Errors

### 1. Duplicate Type Definitions

The following types have conflicting implementations of `Clone`:

- `bip341::TaprootMerkleTree`
- `bip341::TaprootLeaf`
- `bip341::TaprootSpend`
- `bip341::TaprootOutput`

These duplicates need to be removed and centralized in the `core/src/bip` directory.

### 2. Missing Dependencies

The following dependencies need to be added to Cargo.toml:

- `chrono`
- `humantime_serde`
- `opcodes`

### 3. Result Type Errors

Several trait implementations are using `Result<T>` instead of `Result<T, E>`:

- `trait HsmConnector`
- `trait BitcoinInterface`
- Various other traits

### 4. Undefined Types

The following types are referenced but not defined:

- `SimulatorHsmProvider`
- `SoftwareHsmProvider`
- `HardwareHsmProvider`
- `BitcoinHsmProvider`
- `YubiConnector`
- `LedgerConnector`
- `Web5Adapter`
- `MLAgentSystem`
- `TokenomicsEngine`

## Steps to Resolve

1. **Remove Duplicate Types**
   - [ ] Move all Taproot type definitions to `core/src/bip/bip341.rs`
   - [ ] Update imports to reference the centralized types
   - [ ] Remove duplicate definitions from `src/bitcoin/bip341.rs`

2. **Add Missing Dependencies**
   - [ ] Add `chrono` to Cargo.toml
   - [ ] Add `humantime_serde` to Cargo.toml
   - [ ] Add `opcodes` or use bitcoin-specific opcodes

3. **Fix Result Type Errors**
   - [ ] Update trait definitions to use `Result<T, E>` with proper error types
   - [ ] Update implementations to match the corrected trait definitions

4. **Implement Missing Types**
   - [ ] Create placeholder implementations for the undefined types
   - [ ] Or update references to use existing types

## Related Changes

This issue is part of the larger Bitcoin module restructuring effort documented in:

- [Bitcoin Module Restructuring Summary](docs/bitcoin/MODULE_RESTRUCTURING_SUMMARY.md)
- [Bitcoin Architecture Update](docs/bitcoin/ARCHITECTURE_UPDATE.md)

## Acceptance Criteria

- [ ] All compilation errors are resolved
- [ ] Project builds successfully
- [ ] BIP validation tool runs without errors
- [ ] Existing functionality is preserved
- [ ] Added unit tests for new code 
