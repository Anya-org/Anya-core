# C/C++ Dependencies Migration Implementation Guide

*Practical steps to migrate from C/C++ dependencies to Rust alternatives*

## Quick Start - Immediate Build Performance Improvements

### 1. OpenSSL → Rustls Migration (15 minutes)

**Current problematic dependencies:**
```toml
# These pull in openssl-sys (C++ dependency)
reqwest = { version = "0.12.9", features = ["json"] }
sqlx = { version = "0.8.2", features = ["runtime-tokio-rustls", "postgres"] }
```

**Replace with:**
```toml
# Pure Rust TLS - no C++ dependencies
reqwest = { version = "0.12.9", features = ["json", "rustls-tls"], default-features = false }
sqlx = { version = "0.8.2", features = ["runtime-tokio-rustls", "postgres", "tls-rustls"], default-features = false }
```

### 2. Compression → Pure Rust (5 minutes)

**Current:**
```toml
# zstd-sys pulls in C++ compression library
# (indirect dependency via rocksdb or other crates)
```

**Add explicit pure Rust feature:**
```toml
zstd = { version = "0.13.2", features = ["pure_rust"] }
```

### 3. Remove curl dependency (2 minutes)

**Check if curl is directly used:**
```bash
grep -r "curl::" src/ || echo "curl not directly used"
```

**If not used, remove:**
```toml
# curl = "0.4.48"  # REMOVE - use reqwest instead
```

## Phase 2: Database Migration (Major Impact)

### Current RocksDB Usage Analysis

**Find RocksDB usage in codebase:**
```bash
# Search for RocksDB usage
rg "rocksdb|RocksDB" src/ --type rust
rg "DB::" src/ --type rust
```

### Option A: redb (Recommended)

**Benefits:**
- Zero-copy operations
- ACID compliance
- Excellent performance
- Zero build time overhead

**Migration steps:**
```toml
# Replace in Cargo.toml
# rocksdb = "0.22.0"  # REMOVE
redb = "2.1.1"
```

**Code migration example:**
```rust
// OLD: RocksDB
use rocksdb::{DB, Options};

let mut opts = Options::default();
opts.create_if_missing(true);
let db = DB::open(&opts, path)?;
db.put(key, value)?;
let result = db.get(key)?;

// NEW: redb
use redb::{Database, ReadableTable, TableDefinition};

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("my_table");

let db = Database::create(path)?;
let write_txn = db.begin_write()?;
{
    let mut table = write_txn.open_table(TABLE)?;
    table.insert(key, value)?;
}
write_txn.commit()?;

let read_txn = db.begin_read()?;
let table = read_txn.open_table(TABLE)?;
let result = table.get(key)?;
```

### Option B: sled (Alternative)

```toml
# Alternative to redb
sled = "0.34.7"
```

**Code migration:**
```rust
// OLD: RocksDB
let db = rocksdb::DB::open(&opts, path)?;

// NEW: sled
let db = sled::open(path)?;
db.insert(key, value)?;
let result = db.get(key)?;
```

## Phase 3: Git Operations Migration

### Current libgit2 Usage

**Find git usage:**
```bash
rg "git2|libgit2" src/ --type rust
rg "Repository::" src/ --type rust
```

### Migration to gix (gitoxide)

```toml
# Replace in Cargo.toml
# git2 = "0.19.0"  # REMOVE
gix = "0.66.0"
```

**Code migration example:**
```rust
// OLD: git2
use git2::{Repository, Oid};

let repo = Repository::open(path)?;
let head = repo.head()?;
let commit = head.peel_to_commit()?;

// NEW: gix
use gix::{Repository, ObjectId};

let repo = Repository::open(path)?;
let head = repo.head_commit()?;
```

## Phase 4: Cryptography Evaluation

### secp256k1 Migration (Bitcoin-specific)

**Current:**
```toml
secp256k1 = { version = "0.29.1", features = ["global-context", "recovery", "rand"] }
```

**Pure Rust alternative:**
```toml
k256 = { version = "0.13.4", features = ["ecdsa", "schnorr", "arithmetic"] }
```

**Migration consideration:**
```rust
// Compatibility check needed for Bitcoin Core interop
// Test vectors should match exactly between implementations

// OLD: secp256k1
use secp256k1::{Secp256k1, SecretKey, PublicKey};

// NEW: k256
use k256::{SecretKey, PublicKey, ecdsa::SigningKey};
```

## Build Performance Testing

### Measure Current Build Time
```bash
# Clean build test
cargo clean
time cargo build --release

# Note the time for comparison
```

### After Each Migration Phase
```bash
# Test incremental builds
touch src/lib.rs
time cargo build --release

# Test from-scratch builds
cargo clean
time cargo build --release
```

### Benchmark Template
```bash
#!/bin/bash
# build_benchmark.sh

echo "=== Build Performance Test ==="
echo "Phase: $1"
echo "Date: $(date)"

echo "Cleaning..."
cargo clean

echo "Timing clean build..."
time cargo build --release 2>&1 | tee build_log_$1.txt

echo "Build complete. Check build_log_$1.txt for details."
```

## Testing Strategy

### 1. Unit Tests (After each migration)
```bash
cargo test --lib
```

### 2. Integration Tests
```bash
cargo test --test integration
```

### 3. Performance Benchmarks
```bash
cargo bench
```

### 4. Cross-compilation Test
```bash
# Test on different targets
cargo check --target x86_64-pc-windows-gnu
cargo check --target aarch64-apple-darwin
```

## Rollback Strategy

### Feature Flags Approach
```toml
[features]
default = ["pure-rust-deps"]
pure-rust-deps = ["redb", "gix", "rustls-tls"]
native-deps = ["rocksdb", "git2", "openssl"]

[dependencies]
# Pure Rust options
redb = { version = "2.1.1", optional = true }
gix = { version = "0.66.0", optional = true }

# Native options (fallback)
rocksdb = { version = "0.22.0", optional = true }
git2 = { version = "0.19.0", optional = true }
```

### Quick Rollback Commands
```bash
# If issues arise, quickly rollback
git checkout HEAD~1 Cargo.toml
cargo build
```

## Monitoring and Validation

### 1. Build Time Tracking
```bash
# Create build time log
echo "$(date),$(git rev-parse HEAD),clean_build" >> build_times.csv
time cargo build --release 2>&1 | grep "real" >> build_times.csv
```

### 2. Binary Size Comparison
```bash
# Track binary size changes
ls -la target/release/anya-core
```

### 3. Performance Regression Tests
```bash
# Ensure no performance regression in key operations
cargo bench --bench database_operations
cargo bench --bench crypto_operations
```

## Troubleshooting Common Issues

### 1. Rustls Certificate Issues
```rust
// If certificate validation fails
reqwest::ClientBuilder::new()
    .use_rustls_tls()
    .danger_accept_invalid_certs(false) // Keep security
    .build()?
```

### 2. redb Migration Issues
```rust
// Handle schema migration
const OLD_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("old_format");
const NEW_TABLE: TableDefinition<&str, MyStruct> = TableDefinition::new("new_format");

// Migration function needed
fn migrate_database(db: &Database) -> Result<(), Box<dyn Error>> {
    // Implement data migration logic
    Ok(())
}
```

### 3. Git Compatibility Issues
```rust
// Ensure git operations remain compatible
fn verify_git_compatibility() -> Result<(), Box<dyn Error>> {
    // Test basic operations
    // Compare results with reference implementation
    Ok(())
}
```

## Expected Timeline

### Week 1: Phase 1 (OpenSSL → Rustls)
- **Monday**: Update Cargo.toml dependencies
- **Tuesday**: Fix compilation errors
- **Wednesday**: Test TLS functionality
- **Thursday**: Performance testing
- **Friday**: Integration testing

### Week 2-3: Phase 2 (Database Migration)
- **Week 2**: Analyze current RocksDB usage
- **Week 3**: Implement redb migration

### Week 4: Phase 3 (Git Operations)
- **Monday-Wednesday**: Migrate to gix
- **Thursday-Friday**: Testing and validation

### Week 5-6: Phase 4 (Cryptography Review)
- Evaluate secp256k1 alternatives
- Extensive testing for Bitcoin compatibility

## Success Metrics

### Build Performance
- **Target**: 40-60% reduction in clean build time
- **Measurement**: `time cargo build --release`

### Binary Size
- **Target**: 10-20% reduction in binary size
- **Measurement**: `ls -la target/release/`

### Developer Experience
- **Target**: Eliminate platform-specific build issues
- **Measurement**: Successful builds on Windows, macOS, Linux

### Security
- **Target**: Reduce C/C++ code dependencies by 80%
- **Measurement**: `cargo tree | grep -E "(sys|bindgen)"`

This implementation guide provides step-by-step instructions for migrating away from C/C++ dependencies, with specific code examples and testing strategies.
