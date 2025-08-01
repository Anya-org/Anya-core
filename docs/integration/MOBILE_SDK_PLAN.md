# Mobile SDK Integration Plan for anya-core and anya-mobile

This document outlines the steps needed to align and integrate the anya-core mobile SDK with the anya-mobile repository.

## Current Issues

1. **Build Configuration:**
   - The `ffi` feature flag is not properly defined in Cargo.toml
   - Secp256k1 version conflicts when building with full dependencies

2. **Implementation Status:**
   - SDK functions are stubbed/template implementations
   - FFI bindings are incomplete
   - Mobile wrappers are missing

3. **Integration:**
   - No proper connection between anya-core and anya-mobile
   - Missing platform-specific build scripts

## Implementation Plan

### Phase 1: Fix Build Configuration

1. **Add Feature Flags:**
   ```toml
   [features]
   ffi = ["std"]
   mobile = ["ffi", "std", "bitcoin/std", "bitcoin/rand-std", "secp256k1/global-context", "secp256k1/rand"]
   ```

2. **Fix Dependency Versions:**
   - Update `secp256k1` to use consistent version and features
   - Ensure compatibility with bitcoin crate

### Phase 2: Complete Core SDK Implementation

1. **Enhance MobileSDK:**
   - Complete all stub implementations
   - Connect to actual wallet functionality
   - Implement proper error handling

2. **Complete FFI Layer:**
   - Expose all required functions
   - Add memory management helpers
   - Add comprehensive documentation

3. **Create Isolated Build:**
   - See `minimal_mobile_build.sh` for a template

### Phase 3: Create Mobile Wrappers

1. **Android/Kotlin Integration:**
   - Complete JNI bindings
   - Create Kotlin wrapper class
   - Add Android-specific features

2. **iOS/Swift Integration:**
   - Create Objective-C bridging header
   - Implement Swift wrapper class
   - Add iOS-specific features

3. **Platform Build Scripts:**
   - Create build scripts for Android (.so generation)
   - Create build scripts for iOS (.a/.dylib generation)

### Phase 4: Testing and Documentation

1. **Create Test Suite:**
   - Unit tests for SDK functions
   - Integration tests for FFI layer
   - Mobile app test examples

2. **Documentation:**
   - API reference
   - Integration guide
   - Example applications

## Integration with anya-mobile

The anya-mobile repository should import the SDK as follows:

### Android Integration

```kotlin
// In anya-mobile's build.gradle
dependencies {
    implementation project(':anya-core-sdk')
    // or
    implementation 'org.anya:core-mobile:1.0.0'
}
```

### iOS Integration

```ruby
# In anya-mobile's Podfile
pod 'AnyaCore', '~> 1.0.0'
```

## Example Usage

### Kotlin Example
```kotlin
val anyaCore = AnyaCore()
if (anyaCore.initializeWallet("your mnemonic here")) {
    val txid = anyaCore.sendTransaction("recipient_address", 10000L)
    val walletInfo = anyaCore.getWalletInfo()
    // Process wallet info as JSON
}
```

### Swift Example
```swift
if AnyaCore.initializeWallet(mnemonic: "your mnemonic here") {
    let txid = AnyaCore.sendTransaction(recipient: "recipient_address", amount: 10000)
    let walletInfo = AnyaCore.getWalletInfo()
    // Process wallet info as JSON
}
```
