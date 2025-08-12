# Anya Core - GitHub Copilot Development Instructions

**ALWAYS follow these instructions first and fallback to additional search and context gathering only if the information here is incomplete or found to be in error.**

Anya Core is a complex enterprise-grade Bitcoin infrastructure platform built in Rust with Node.js components, featuring Bitcoin protocol implementation, Layer2 integrations (Lightning, RGB, DLC), Web5 services, and ML capabilities.

## Working Effectively

### Bootstrap and Build Commands (NEVER CANCEL - Use Long Timeouts)

**CRITICAL TIMING REQUIREMENTS:**
- Initial dependency download: 11+ minutes (network timeouts common)
- First build (debug): 11+ minutes  
- Release build: 11+ minutes
- Test suite: 7+ minutes
- **NEVER CANCEL builds or tests - SET 60+ minute timeouts**

```bash
# Core build sequence (NEVER CANCEL - takes 20+ minutes total)
cargo check --all-features    # 11+ minutes, network timeouts expected
cargo build --release --all-features    # 11+ minutes  
cargo test --lib    # 7+ minutes - 135 tests pass

# Development build (faster incremental)
cargo build    # 2-5 minutes after initial build
cargo test    # 2-3 minutes after initial build
```

### Node.js Components (Partial functionality)

```bash
npm install    # 1+ minutes - 36 vulnerabilities expected
# Note: npm test FAILS - missing .cursor/mcp.json and ES module issues
# npm run test-mcp FAILS - ES module import cycle errors
```

### Linting and Code Quality

```bash
# Install clippy first
rustup component add clippy

# Run linting (EXPECT 82 clippy warnings currently)
cargo clippy --all-features -- -D warnings    # FAILS with 82 warnings
# Note: Current codebase has known clippy issues that need fixing
```

### Running the Application

```bash
# Built binary works correctly
./target/release/anya-core --help    # Shows CLI interface
./target/release/anya-core --version    # Shows: anya-core 1.3.0
./target/release/anya-core health    # ✅ Basic health check passed
./target/release/anya-core validate    # ✅ Configuration validation passed
```

## Validation Scenarios

### ALWAYS Test After Changes

**Core Rust Validation:**
1. `cargo check --all-features` - Verify compilation (11+ min, NEVER CANCEL)
2. `cargo test --lib` - Run all 135 unit tests (7+ min, NEVER CANCEL) 
3. `./target/release/anya-core health` - Verify binary functionality
4. `./target/release/anya-core validate` - Test configuration validation

**Known Working Scenarios:**
- Bitcoin wallet operations (135 tests pass)
- Layer2 protocol integration tests pass
- Security and cryptographic operations work
- Infrastructure high availability tests pass
- ML system basic functionality operational

**Known Issues to Document:**
- npm test FAILS due to missing .cursor/mcp.json file
- ES module import cycles in Node.js test files  
- 82 clippy warnings need addressing
- Network timeouts common during initial builds

## Exact Dependencies and Installation

### System Requirements
```bash
# Rust toolchain (verified working)
rustc 1.91.0-nightly (c8ca44c98 2025-08-10)
cargo 1.91.0-nightly (840b83a10 2025-07-30)

# Node.js (verified working)
node v20.19.4
npm 10.8.2
```

### Installation Process
```bash
# 1. Clone repository (if not already available)
git clone https://github.com/Anya-org/Anya-core.git
cd Anya-core

# 2. Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 3. Install clippy for linting
rustup component add clippy

# 4. Build process (CRITICAL: Set 60+ minute timeouts)
time cargo check --all-features    # Expected: 11+ minutes
time cargo build --release --all-features    # Expected: 11+ minutes  
time cargo test --lib    # Expected: 7+ minutes

# 5. Install Node.js dependencies
npm install    # Expected: 1+ minutes, 36 vulnerabilities normal
```

## Build Timing and Expectations

### NEVER CANCEL - Critical Timing Information

**Initial Build (clean state):**
- Dependency resolution: 11+ minutes (many network timeouts expected)
- Compilation: 11+ minutes (includes RocksDB C++ compilation)
- Total first build: 22+ minutes minimum

**Network Issues Expected:**
- "Connection timed out after 30000 milliseconds" warnings normal
- crates.io index updates can be slow
- Spurious network errors (3 tries) are common and handled automatically

**Key Dependencies that Take Time:**
- librocksdb-sys: Heavy C++ compilation 
- Bitcoin ecosystem crates: Many dependencies
- Crypto libraries: secp256k1, ring, etc.
- Tokio async runtime and related crates

### Test Expectations
- 135 unit tests PASS (0 failed)
- Test categories: Bitcoin (28), Layer2 (5), Security (15), HSM (12), Config (8), Utils (45)
- Performance benchmarks: 1 skipped (timing-dependent)

## Common Tasks and Workflows

### Development Workflow
```bash
# After making changes, always run this sequence:
cargo check --all-features    # Verify compilation  
cargo build    # Build for testing
cargo test --lib    # Run test suite
./target/release/anya-core health    # Verify functionality

# Before committing (expect failures):
cargo clippy --all-features -- -D warnings    # 82 warnings currently
npm test    # FAILS - known issues with .cursor/mcp.json
```

### Project Structure Navigation
```
src/
├── bitcoin/          # Bitcoin protocol (BIP-340, 341, 342 implementation)
├── layer2/           # Layer2 protocols (Lightning, RGB, DLC, Taproot Assets)
├── security/         # Cryptographic operations and HSM integration  
├── core/            # Core system components and configuration
├── ml/              # Machine learning adapters and inference
├── web5/            # Web5 decentralized identity and data
├── infrastructure/  # High availability and enterprise features
└── api/             # External API interfaces
```

### Feature Flags (Key Cargo Features)
```bash
# Core builds
cargo build --features "bitcoin,mobile"    # Basic functionality
cargo build --features "complete"    # All features enabled  
cargo build --release --all-features    # Production build

# Specific feature sets
--features "hsm-full"    # Hardware Security Module support
--features "enterprise"    # Enterprise database features
--features "web5"    # Web5 integration (optional)
```

## Important Files and Configuration

### Key Configuration Files
- `Cargo.toml` - Main Rust project with complex feature flags
- `package.json` - Node.js scripts (some broken)
- `.github/workflows/unified-ci.yml` - CI pipeline
- `rust-toolchain.toml` - Rust version specification

### Build Artifacts to Ignore
- `target/` - Rust build output (large, rebuild as needed)
- `node_modules/` - npm dependencies  
- `.cargo/` - Cargo cache
- Cargo.lock changes (auto-generated)

## Critical Warnings and Issues

### Build Issues to Document
1. **Network timeouts are NORMAL during initial build**
2. **npm test FAILS** - missing .cursor/mcp.json, ES module cycles
3. **clippy has 82 warnings** - code quality issues exist
4. **Long build times expected** - 20+ minutes total for fresh build

### Always Use Long Timeouts
- **cargo build commands: 60+ minutes timeout minimum**
- **cargo test: 30+ minutes timeout minimum**  
- **Never cancel long-running builds - they will complete**

### Version Information
- Anya Core: v1.3.0
- Rust: 1.91.0-nightly 
- Node.js: v20.19.4
- Build system: Cargo workspace with complex dependencies

## CI/CD Integration

### GitHub Actions Workflow
- Located: `.github/workflows/unified-ci.yml`
- Requires: protobuf-compiler installation
- Runs: cargo check, cargo deny
- **Important**: CI may need extended timeouts for builds

### Required Tools for CI
```bash
sudo apt-get update && sudo apt-get install -y protobuf-compiler
cargo install cargo-deny    # For dependency checking
```

This complex codebase requires patience during builds and comprehensive testing after changes. Always validate that the core binary functionality works after modifications.