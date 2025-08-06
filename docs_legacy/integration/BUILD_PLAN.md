# Integration Fixes Build Plan

## Overview

This document outlines the plan to address integration issues between anya-core and anya-mobile repos, with a focus on the FFI layer and mobile SDK.

## Current State

1. **anya-core SDK**:
   - Mobile SDK is implemented as a template in `/src/mobile/sdk.rs`
   - Basic FFI bindings are missing
   - The `ffi` feature is not properly defined in Cargo.toml
   - Stubbed async methods exist but are not fully integrated with core functionality

2. **anya-mobile Repository**:
   - Referenced as a separate repository (not a submodule)
   - Expected to use anya-core's FFI bindings
   - Written in pure Rust (100% according to repo stats)
   - Recent commit: "feat: Integrate anya-core for wallet management"

## Required Actions

### 1. FFI Feature Configuration

- Add proper FFI feature to Cargo.toml:

```toml
[features]
# Add to existing features
ffi = ["std"]  # Mobile FFI bindings
mobile = ["ffi", "bitcoin/std", "bitcoin/rand-std", "secp256k1/global-context"] 
```

### 2. FFI Implementation

- Complete the FFI bindings in `src/mobile/sdk.rs` for:
  - Wallet initialization
  - Transaction handling
  - Account management
  - Security operations

### 3. Mobile Wrapper Integration

- Create example Kotlin and Swift wrappers in `examples/mobile/`:
  - Android JNI bindings
  - iOS Swift/ObjC bridge
  - Documentation for mobile integration

### 4. Testing Infrastructure

- Add integration tests:
  - FFI layer tests
  - Wrapper tests
  - Mobile-specific use cases

### 5. Build System Enhancements

- Update build.rs to support mobile targets:
  - Android (arm64-v8a, armeabi-v7a, x86, x86_64)
  - iOS (arm64, x86_64-apple-ios)
  - Simulator support

### 6. Documentation Updates

- Complete mobile SDK documentation
- Add integration guides for anya-mobile repo

## Timeline and Dependencies

1. **Phase 1 (Immediate)**:
   - Add FFI feature to Cargo.toml
   - Complete basic FFI bindings
   - Fix compilation issues

2. **Phase 2 (Short-term)**:
   - Complete mobile wrappers
   - Add example integration code
   - Establish testing infrastructure

3. **Phase 3 (Mid-term)**:
   - Performance optimizations
   - Full feature parity with core
   - Comprehensive documentation

## Success Criteria

- `cargo check --features ffi` completes without errors
- `cargo test --features ffi` passes all tests
- Example Android and iOS apps can successfully:
  - Initialize wallet
  - Send transactions
  - Perform all basic operations

## Assignees

- FFI Layer: TBD
- Mobile Wrappers: TBD
- Testing: TBD
- Documentation: TBD
