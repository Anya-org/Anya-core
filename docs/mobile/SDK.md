---
title: "Sdk"
description: "Documentation for Sdk"
---

[AIR-3][AIS-3][BPC-3][RES-3]


# Mobile SDK

## Overview

Add a brief overview of this document here.


This document provides information about the Anya Core Mobile SDK for iOS and Android platforms.

## Table of Contents
- [Installation](#installation)
- [Getting Started](#getting-started)
- [API Reference](#api-reference)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

## Installation

### Android
Add to your app's `build.gradle`:

```gradle
dependencies {
    implementation 'org.anya:core-mobile:1.0.0'
}
```

### iOS
Add to your `Podfile`:

```ruby
target 'YourApp' do
  pod 'AnyaCore', '~> 1.0.0'
end
```

## Getting Started

### Initialize the SDK

#### Android (Kotlin)
```kotlin
import org.anya.core.AnyaSDK

class MainApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        AnyaSDK.initialize(context = this, config = Config(environment = Environment.PRODUCTION))
    }
}
```

#### iOS (Swift)
```swift
import AnyaCore

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    func application(_ application: UIApplication, 
                   didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        let config = Config(environment: .production)
        AnyaSDK.initialize(config: config)
        return true
    }
}
```

## API Reference

### Core Features

#### Wallet Management
- `createWallet()`: Create a new wallet
- `importWallet(mnemonic: String)`: Import existing wallet
- `getBalance()`: Get wallet balance

#### Transactions
- `sendPayment(amount: Long, address: String)`: Send payment
- `getTransactionHistory()`: Fetch transaction history
- `estimateFee()`: Estimate transaction fee

#### Security
- `enableBiometricAuth()`: Enable biometric authentication
- `backupWallet()`: Backup wallet to secure location
- `wipeWallet()`: Securely wipe wallet data

## Examples

### Creating a Wallet

```kotlin
// Android
val wallet = AnyaSDK.walletManager.createWallet()
val mnemonic = wallet.mnemonic
val address = wallet.address
```

```swift
// iOS
let wallet = try AnyaSDK.walletManager.createWallet()
let mnemonic = wallet.mnemonic
let address = wallet.address
```

### Sending a Transaction

```kotlin
// Android
val result = AnyaSDK.transactionManager.sendPayment(
    amount = 100000, // in satoshis
    address = "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq"
)
```

```swift
// iOS
do {
    let result = try AnyaSDK.transactionManager.sendPayment(
        amount: 100000, // in satoshis
        address: "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq"
    )
} catch {
    print("Error: \(error)")
}
```

## Troubleshooting

### Common Issues

#### Android Build Errors
```
> Could not resolve org.anya:core-mobile:1.0.0
```

**Solution**: Ensure you have added the repository to your project's `build.gradle`:

```gradle
allprojects {
    repositories {
        google()
        mavenCentral()
        maven { url 'https://repo.anya.org/maven' }
    }
}
```

#### iOS Linker Errors
```
Undefined symbols for architecture arm64:
  "_OBJC_CLASS_$_AnyaSDK", referenced from:
      objc-class-ref in AppDelegate.o
```

**Solution**:
1. Clean build folder (Cmd + Shift + K)
2. Build the project again
3. If issue persists, run `pod install --repo-update`

### Logging

Enable debug logging:

```kotlin
// Android
AnyaSDK.setLogLevel(LogLevel.DEBUG)
```

```swift
// iOS
AnyaSDK.setLogLevel(.debug)
```

## Support

For additional help, please contact:
- Email: support@anya.org
- GitHub Issues: [https://github.com/anya-org/anya-core/issues](https://github.com/anya-org/anya-core/issues)

## See Also

- [Related Document](#related-document)

