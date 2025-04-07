# BIP-353 Silent Payments Developer Guide [AIS-3][BPC-3][AIP-3]

## Introduction

This guide provides technical details for developers implementing or working with BIP-353 Silent Payments in the Anya Core platform. Silent Payments enhance privacy in Bitcoin transactions by allowing receivers to share a static address that doesn't appear on-chain, with all payments being unlinkable.

## Key Components

The Silent Payments implementation is located in the `packages/privacy` module and consists of:

```text
packages/privacy/
├── Cargo.toml
└── src/
    ├── lib.rs                  # Core module definitions and types
    └── silent_payments/
        ├── mod.rs              # Main module entry point
        ├── address.rs          # Address encoding/decoding
        ├── keys.rs             # Key management
        ├── scanner.rs          # Transaction scanning
        ├── sender.rs           # Payment creation
        ├── util.rs             # Utility functions
        └── tests.rs            # Test suite
```

## Usage Examples

### Creating a Silent Payment Address

```rust
use anya_privacy::silent_payments::{KeyManager, SilentPaymentAddress};
use bitcoin::network::constants::Network;

// Generate new random keys for Silent Payments
let key_manager = KeyManager::new_random()?;

// Generate an address (defaults to mainnet)
let address = key_manager.generate_address();
println!("Silent Payment address: {}", address);

// For testnet
let mut key_manager = KeyManager::new_random()?;
key_manager.set_network(Network::Testnet);
let testnet_address = key_manager.generate_address();
```

### Sending to a Silent Payment Address

```rust
use anya_privacy::silent_payments::SilentPaymentSender;
use bitcoin::secp256k1::SecretKey;
use bitcoin::{OutPoint, Amount};

// Create a sender
let sender = SilentPaymentSender::new();

// Parse an address
let address = "sp1qz8kg82...".parse::<SilentPaymentAddress>()?;

// Get your private key for signing
let sender_secret = get_private_key();

// Create transaction inputs
let outpoint = OutPoint::new(txid, vout);
let amount = Amount::from_sat(50_000);

// Create a payment output
let output = sender.create_payment_output(
    &address,
    &sender_secret,
    &outpoint,
    amount,
)?;

// Add this output to your transaction
```

### Scanning for Incoming Payments

```rust
use anya_privacy::silent_payments::SilentPaymentScanner;

// Create a scanner with your keys
let mut scanner = SilentPaymentScanner::new(
    scan_secret_key,
    spend_public_key,
)?;

// When a new block arrives
for tx in block.transactions {
    let payments = scanner.scan_transaction(&tx, Some(block_height))?;
    
    // Process detected payments
    for payment in payments {
        println!("Received {} sats in {}:{}", 
            payment.amount, 
            payment.txid, 
            payment.vout
        );
    }
}
```

## BIP-32 Integration

Silent Payments supports BIP-32 key derivation from HD wallets:

```rust
use bitcoin::util::bip32::{ExtendedPrivKey, DerivationPath};

// Default derivation paths (m/352h/0h/0h/0/0 and m/352h/0h/0h/1/0)
let key_manager = KeyManager::derive_from_xpriv(
    &master_xpriv,
    None,        // Use default scan path
    None,        // Use default spend path
    Network::Bitcoin,
)?;

// Custom derivation paths
let key_manager = KeyManager::derive_from_xpriv(
    &master_xpriv,
    Some("m/352h/0h/1h/0/0"),  // Custom scan path
    Some("m/352h/0h/1h/1/0"),  // Custom spend path
    Network::Bitcoin,
)?;
```

## Security Considerations [AIS-3]

### Key Protection

All sensitive key material uses automatic zeroization:

```rust
impl Drop for SecretKeyWrapper {
    fn drop(&mut self) {
        // Securely zero the key material when dropped
        self.key.0.zeroize();
    }
}
```

### Constant-Time Operations

All cryptographic operations use constant-time implementations to prevent timing side-channel attacks:

```rust
// Example: constant-time comparison
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    result == 0
}
```

## Configuration Options

The module can be configured in your `Cargo.toml`:

```toml
[dependencies]
anya-privacy = { version = "1.0", features = ["hardware-security"] }

[features]
silent-payments = ["anya-privacy/default"]
full-privacy = ["silent-payments", "anya-privacy/hardware-security"]
```

## Testing

Run the test suite with:

```bash
cargo test -p anya-privacy --features="silent-payments"
```

Test vectors are available in `src/silent_payments/tests.rs`.

## Implementation Details

### Key Cryptographic Operations

1. **Shared Secret Derivation**: 

   ```text
   shared_point = scan_pubkey * sender_secret
   ```

2. **Output Key Tweak**:

   ```text
   tweak = SHA256(shared_point || outpoint || spend_pubkey)
   ```

3. **Output Key Derivation**:

   ```text
   output_key = spend_pubkey + tweak*G
   ```

4. **Script Generation**:

   ```text
   script = P2TR(output_key)  # Pay-to-Taproot
   ```

## Debugging

Set environment variables for logging:

```bash
RUST_LOG=anya_privacy=debug cargo run --example scan_payments
```

## Specification Compliance

The implementation follows the official BIP-353 specification. Any deviations or implementation-specific decisions are documented in `COMPLIANCE.md`.
