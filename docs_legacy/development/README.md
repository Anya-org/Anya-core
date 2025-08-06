# Development Guide

[AIR-3][AIS-3][BPC-3][RES-3]

**AI Labeling**: This documentation is AI-generated with technical review and validation.

**Date**: June 7, 2025

## Overview

This guide provides comprehensive information for developers working on Anya Core, covering development workflows, coding standards, testing practices, and contribution guidelines.

## Table of Contents

 Development Environment Setup
 Project Structure
 Coding Standards
 Development Workflow
 Testing Guidelines
 Code Review Process
 Debugging and Profiling
 Extension Development
 Documentation Development

## Development Environment Setup

### Prerequisites

- **Rust**: Latest stable version (1.75+)
- **Git**: Version 2.40+
- **Docker**: For containerized development
- **Bitcoin Core**: For local testing (testnet/regtest)

### Environment Configuration

```bash
# Clone the repository
git clone https://github.com/anya-org/anya-core.git
cd Anya-core

# Install dependencies
cargo check

# Setup development environment
./install.sh --dev

# Configure Git hooks
./configure-git-signing.sh
```

### IDE Setup

Recommended development environment:

- **VS Code** with Rust analyzer extension
- **RustRover** for advanced debugging
- **Neovim** with rust-tools.nvim

## Project Structure

```
src/
├── core/           # Core system components
├── bitcoin/        # Bitcoin protocol implementation
├── web5/          # Web5 protocol implementation
├── ml/            # Machine learning systems
├── extensions/    # Extension framework
├── security/      # Security primitives
├── networking/    # Network protocols
└── utils/         # Shared utilities
```

### Module Architecture

```rust
// Core module structure
pub mod core {
    pub mod consensus;
    pub mod validation;
    pub mod storage;
}

pub mod bitcoin {
    pub mod wallet;
    pub mod psbt;
    pub mod taproot;
    pub mod lightning;
}

pub mod web5 {
    pub mod did;
    pub mod dwn;
    pub mod protocols;
    pub mod vc;
}
```

## Coding Standards

### Rust Style Guidelines

Follow the official Rust style guide with project-specific additions:

```rust
// Use explicit error types
use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },
}

// Comprehensive error handling
pub fn validate_transaction(tx: &Transaction) -> Result<()> {
    if tx.inputs.is_empty() {
        return Err(ValidationError::InvalidTransaction(
            "Transaction must have at least one input".to_string()
        ).into());
    }
    
    // Additional validation logic
    Ok(())
}
```

### Security Guidelines

```rust
// Always use constant-time comparisons for sensitive data
use subtle::ConstantTimeEq;

fn verify_signature(signature: &[u8], expected: &[u8]) -> bool {
    signature.ct_eq(expected).into()
}

// Secure random number generation
use rand::rngs::OsRng;
use rand::RngCore;

fn generate_secure_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}
```

### Documentation Standards

```rust
/// Validates a Bitcoin transaction according to BIP standards.
/// 
/// This function performs comprehensive validation including:
/// - Input/output validation
/// - Script verification
/// - Signature validation
/// - BIP compliance checks
/// 
/// # Arguments
/// 
/// * `transaction` - The transaction to validate
/// * `utxo_set` - Available unspent transaction outputs
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the transaction is valid, or an error describing
/// the validation failure.
/// 
/// # Examples
/// 
/// ```rust
/// use anya_core::bitcoin::Transaction;
/// 
/// let tx = Transaction::new();
/// let utxos = UtxoSet::new();
/// 
/// match validate_transaction(&tx, &utxos) {
///     Ok(()) => println!("Transaction is valid"),
///     Err(e) => eprintln!("Validation failed: {}", e),
/// }
/// ```
/// 
/// # Security Considerations
/// 
/// This function performs cryptographic verification and should not be
/// used with untrusted input without proper sanitization.
pub fn validate_transaction(
    transaction: &Transaction,
    utxo_set: &UtxoSet,
) -> Result<(), ValidationError> {
    // Implementation
}
```

## Development Workflow

### Branch Management

```bash
# Feature development
git checkout -b feature/taproot-integration
git push -u origin feature/taproot-integration

# Bug fixes
git checkout -b bugfix/wallet-sync-issue

# Hotfixes
git checkout -b hotfix/security-patch
```

### Commit Guidelines

Follow conventional commits:

```bash
# Feature commits
git commit -m "feat(bitcoin): implement BIP 341 Taproot support"

# Bug fixes
git commit -m "fix(wallet): resolve balance calculation issue"

# Documentation
git commit -m "docs(api): update Bitcoin wallet API reference"

# Security fixes
git commit -m "security(crypto): fix timing attack vulnerability"
```

### Code Generation

```bash
# Generate API documentation
cargo doc --open

# Run code generation
cargo build

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Testing Guidelines

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_transaction_validation() {
        let tx = Transaction::new();
        let utxos = UtxoSet::new();
        
        assert!(validate_transaction(&tx, &utxos).is_ok());
    }
    
    // Property-based testing
    proptest! {
        #[test]
        fn test_signature_roundtrip(
            private_key in any::<[u8; 32]>(),
            message in any::<Vec<u8>>()
        ) {
            let signature = sign_message(&private_key, &message);
            prop_assert!(verify_signature(&signature, &message));
        }
    }
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_bitcoin_integration() {
    let node = BitcoinTestNode::new().await;
    let wallet = Wallet::new(&node).await;
    
    // Test transaction creation and broadcast
    let tx = wallet.create_transaction(recipient, amount).await?;
    let txid = wallet.broadcast_transaction(tx).await?;
    
    // Verify transaction in mempool
    assert!(node.get_mempool_entry(&txid).await.is_ok());
}
```

### Performance Testing

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_signature_verification(c: &mut Criterion) {
    let (private_key, public_key) = generate_keypair();
    let message = b"test message";
    let signature = sign_message(&private_key, message);
    
    c.bench_function("signature_verification", |b| {
        b.iter(|| {
            verify_signature(
                black_box(&signature),
                black_box(message),
                black_box(&public_key)
            )
        })
    });
}

criterion_group!(benches, benchmark_signature_verification);
criterion_main!(benches);
```

## Code Review Process

### Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests are comprehensive and pass
- [ ] Documentation is updated
- [ ] Security considerations addressed
- [ ] Performance implications considered
- [ ] BIP compliance verified (for Bitcoin features)

### Security Review

```rust
// Security review checklist for cryptographic code
// 1. Constant-time operations for sensitive data
// 2. Proper random number generation
// 3. Input validation and sanitization
// 4. Error handling doesn't leak information
// 5. Memory is securely cleared when appropriate

use zeroize::Zeroize;

struct SecretKey([u8; 32]);

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}
```

## Debugging and Profiling

### Debugging Tools

```bash
# Debug build with full symbols
cargo build --features debug

# Run with debugger
rust-gdb target/debug/anya-core

# Memory leak detection
valgrind --tool=memcheck target/debug/anya-core
```

### Performance Profiling

```bash
# CPU profiling
perf record -g target/release/anya-core
perf report

# Memory profiling
heaptrack target/release/anya-core
```

### Logging and Tracing

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(sensitive_data))]
pub async fn process_transaction(tx: Transaction, sensitive_data: &SecretKey) -> Result<()> {
    info!("Processing transaction: {}", tx.id());
    
    match validate_transaction(&tx).await {
        Ok(()) => {
            info!("Transaction validated successfully");
            Ok(())
        }
        Err(e) => {
            warn!("Transaction validation failed: {}", e);
            Err(e)
        }
    }
}
```

## Extension Development

### Creating Extensions

```rust
use anya_core::extensions::{Extension, ExtensionContext};

pub struct MyExtension {
    config: MyConfig,
}

impl Extension for MyExtension {
    fn name(&self) -> &str {
        "my-extension"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn initialize(&mut self, ctx: &ExtensionContext) -> Result<()> {
        // Extension initialization logic
        Ok(())
    }
    
    async fn handle_event(&self, event: Event) -> Result<()> {
        // Event handling logic
        Ok(())
    }
}
```

### Extension Testing

```rust
#[tokio::test]
async fn test_extension_lifecycle() {
    let mut extension = MyExtension::new();
    let ctx = ExtensionContext::test();
    
    // Test initialization
    extension.initialize(&ctx).await?;
    
    // Test event handling
    let event = Event::new("test");
    extension.handle_event(event).await?;
    
    assert!(extension.is_running());
}
```

### Extension Testing

```rust
#[tokio::test]
async fn test_extension_lifecycle() {
    let mut extension = MyExtension::new();
    let ctx = ExtensionContext::test();

    // Test initialization
    extension.initialize(&ctx).await?;

    // Test event handling
    let event = Event::new("test");
    extension.handle_event(event).await?;

    assert!(extension.is_running());
}
```

## Documentation Development

### Prerequisites

- **Python**: Latest stable version (3.x)
- **pip**: Python package installer

### Environment Configuration

```bash
# Navigate to the project root
cd anya-core

# Install documentation dependencies
pip install -r requirements-docs.txt

# Start the local development server
mkdocs serve
```

The documentation will be available at `http://127.0.0.1:8000`. The server will automatically reload when you make changes to the documentation files.

## Best Practices

### Error Handling

- Use `anyhow` for application errors
- Use `thiserror` for library errors
- Provide context with error chains
- Don't panic in library code

### Performance

- Use `Arc` and `Rc` judiciously
- Prefer `&str` over `String` for parameters
- Use zero-copy parsing when possible
- Profile before optimizing

### Security

- Validate all inputs
- Use secure random number generation
- Clear sensitive data from memory
- Follow cryptographic best practices

### Testing

- Aim for >80% code coverage
- Use property-based testing for algorithms
- Test error conditions
- Include integration tests

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Bitcoin Developer Documentation](https://developer.bitcoin.org/)
- [Web5 Specifications](https://web5.dev/)
- [Security Guidelines](../security/README.md)
- [Testing Framework](../testing/README.md)

## Support

For development questions and support:

- **GitHub Discussions**: Technical questions and design discussions
- **Discord**: Real-time development chat
- **Email**: <security@anya-core.dev> (security-related issues only)

---

This guide is maintained by the Anya Core development team and is updated regularly to reflect current best practices and project requirements.
