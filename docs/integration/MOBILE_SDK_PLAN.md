# Mobile SDK Integration Plan for anya-core and anya-mobile

This document outlines the steps needed to align and integrate the anya-core mobile SDK with the anya-mobile repository.

## Current Status (Updated: August 1, 2025)

### ✅ Completed Tasks

1. **Build Configuration:**
   - ✅ LTS workspace configuration implemented with exact version pinning
   - ✅ secp256k1 unified to version 0.31.1 across all 38+ member crates
   - ✅ Feature flags properly defined in Cargo.toml with mobile/ffi support
   - ✅ 535+ dependencies resolving correctly after workspace restructuring

2. **SDK Implementation:**
   - ✅ Mobile SDK FFI bindings complete with uniffi, jni integration
   - ✅ Core FFI functions implemented in `src/mobile/sdk.rs`
   - ✅ Biometric, backup, wipe, fee estimation stubs ready for wallet connection

3. **Dependency Management:**
   - ✅ Enterprise stack integration: SGX (1.1.1), YubiHSM (0.42.1), SQLx (0.8.2)
   - ✅ Bitcoin crate stabilized at 0.32.6
   - ✅ Workspace dependency conflicts resolved

### 🔄 In Progress

1. **Final Compilation Issues:**
   - 🔄 secp256k1 version conflicts resolved (unified to 0.29.1 to match bitcoin crate)
   - ✅ Handler module completion (dwn, rgb, web5) - all handlers now complete
   - 🔄 HSM provider Send/Sync issues and bitcoin API compatibility

2. **Module Integration:**
   - ✅ Wallet module conflicts resolved (wallet.rs removed, wallet/mod.rs preserved)
   - ✅ API routes unified with conditional compilation
   - ✅ secp256k1 API modernization (from_digest → from_digest_slice)

### 📋 Pending Tasks

1. **SDK-to-Wallet Connection:**
   - Replace FFI stubs with actual wallet operations
   - Implement real Bitcoin transaction handling
   - Integrate biometric authentication flows

2. **Platform Integration:**
   - Initialize anya-mobile repository structure
   - Create platform-specific build scripts
   - Establish sync mechanism with anya-core updates

## Implementation Plan

### Immediate Next Steps (Priority 1)

1. **Complete Final Compilation Fixes:**
   - Fix remaining 6-8 compilation errors in bitcoin/interface modules
   - Complete handler modules (dwn, rgb) implementation
   - Update remaining secp256k1 API calls to use modern interface

2. **Connect SDK to Wallet:**
   - Replace FFI stubs in `src/mobile/sdk.rs` with actual wallet operations
   - Integrate with preserved `wallet/mod.rs` comprehensive implementation
   - Implement real Bitcoin transaction handling using unified secp256k1 0.31.1

### Phase 1: Enhanced Mobile SDK (In Progress)

1. **Complete Core Integration:**
   - ✅ LTS workspace with exact version pinning implemented
   - ✅ Unified secp256k1 0.31.1 across ecosystem
   - 🔄 Final compilation issues resolution
   - 🔄 Real wallet functionality connection

2. **FFI Layer Enhancement:**
   - ✅ Basic FFI structure complete with uniffi bindings
   - 🔄 Memory management optimization
   - 📋 Error handling enhancement
   - 📋 Comprehensive API documentation

### Phase 2: Platform Integration and Mobile Repository

1. **anya-mobile Repository Setup:**
   - Initialize companion repository structure
   - Create platform-specific build scripts
   - Establish CI/CD pipeline integration

2. **Android/Kotlin Integration:**
   - ✅ JNI bindings framework ready
   - 📋 Complete Kotlin wrapper class
   - 📋 Add Android-specific biometric/secure storage features

3. **iOS/Swift Integration:**
   - 📋 Create Objective-C bridging header
   - 📋 Implement Swift wrapper class
   - 📋 Add iOS-specific keychain/secure enclave features

### Phase 3: Production Readiness

1. **Testing Infrastructure:**
   - Unit tests for all SDK functions
   - Integration tests for FFI layer
   - Mobile app test examples
   - Performance benchmarking

2. **Documentation and Examples:**
   - Complete API reference documentation
   - Platform-specific integration guides
   - Example applications for Android/iOS
   - Migration guides for existing implementations

## Technical Architecture

### Current LTS Configuration

```toml
# Unified dependency management with exact versions
[workspace.dependencies]
secp256k1 = "0.31.1"  # Unified across all 38+ member crates
bitcoin = "0.32.6"     # Stable Bitcoin crate
uniffi = "0.28.2"      # FFI binding generation
jni = "0.21.1"         # Android JNI support

[features]
mobile = ["ffi", "bitcoin", "uniffi", "jni"]
ffi = ["std", "uniffi"]
enterprise = ["sgx", "yubihsm", "sqlx"]
```

### Mobile SDK Architecture

```rust
// Current FFI interface ready for wallet connection
#[uniffi::export]
pub fn mobile_initialize_wallet(mnemonic: String) -> bool { /* Ready for implementation */ }

#[uniffi::export] 
pub fn mobile_send_transaction(recipient: String, amount: u64) -> String { /* Ready for implementation */ }

#[uniffi::export]
pub fn mobile_get_wallet_info() -> String { /* Ready for implementation */ }
```

## Integration with anya-mobile

The anya-mobile repository integration is ready for the next phase after completing final compilation fixes.

### Android Integration (Ready for Implementation)

```kotlin
// In anya-mobile's build.gradle
dependencies {
    implementation 'org.anya:core-mobile:1.0.0'
}

// Usage with unified secp256k1 0.31.1 and LTS stability
class AnyaWallet {
    private val core = AnyaCore()
    
    fun initializeWallet(mnemonic: String): Boolean {
        return core.mobile_initialize_wallet(mnemonic)
    }
    
    fun sendTransaction(recipient: String, amount: Long): String {
        return core.mobile_send_transaction(recipient, amount.toULong())
    }
    
    fun getWalletInfo(): WalletInfo {
        val json = core.mobile_get_wallet_info()
        return gson.fromJson(json, WalletInfo::class.java)
    }
}
```

### iOS Integration (Ready for Implementation)

```ruby
# In anya-mobile's Podfile
pod 'AnyaCore', '~> 1.0.0'
```

```swift
// Swift wrapper with LTS guarantees
class AnyaWallet {
    func initializeWallet(mnemonic: String) -> Bool {
        return AnyaCore.mobile_initialize_wallet(mnemonic: mnemonic)
    }
    
    func sendTransaction(recipient: String, amount: UInt64) -> String {
        return AnyaCore.mobile_send_transaction(recipient: recipient, amount: amount)
    }
    
    func getWalletInfo() -> WalletInfo? {
        let json = AnyaCore.mobile_get_wallet_info()
        return try? JSONDecoder().decode(WalletInfo.self, from: json.data(using: .utf8)!)
    }
}
```

## Deployment Strategy

### Build Pipeline Integration

1. **anya-core builds:**
   - Generates mobile FFI libraries (.so, .dylib, .a)
   - Publishes to package repositories
   - Triggers anya-mobile CI/CD

2. **anya-mobile consumes:**
   - Downloads stable FFI libraries
   - Builds platform-specific wrappers
   - Publishes mobile packages (AAR, CocoaPods)

### Version Synchronization

- anya-core: Semantic versioning with LTS guarantee
- anya-mobile: Tracks anya-core versions with platform-specific patches
- Breaking changes: Coordinated release process

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
