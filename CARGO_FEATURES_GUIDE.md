# Cargo Features Guide for Anya-Core

This document explains how to effectively use Cargo features in the Anya-Core project to control functionality and dependencies.

## Core Feature Flags

### Primary Feature Groups

| Feature Flag | Description | Dependencies |
|--------------|-------------|--------------|
| `default` | Default feature set | `std`, `rust-bitcoin` |
| `complete` | Complete system with all core functionality | `hsm` |
| `hsm` | Hardware Security Module support | None |
| `std` | Standard library support | None |

### Bitcoin Integration Features

| Feature Flag | Description | Dependencies |
|--------------|-------------|--------------|
| `bitcoin_integration` | Bitcoin core integration | None |
| `rust-bitcoin` | Rust Bitcoin library support | `bitcoin`, `bdk` |
| `rsk` | RSK integration for Bitcoin verification | None |

### System Features

| Feature Flag | Description | Dependencies |
|--------------|-------------|--------------|
| `system-alignment` | System optimization and alignment | `bitcoin_integration`, `hsm` |
| `memory_tracking` | Performance monitoring and memory tracking | None |
| `web5` | Web5 integration features | None |
| `rgb` | RGB Protocol support | None |

## Using Features in Development

### Basic Usage

To build with default features:

```bash
cargo build
```

To build with specific features:

```bash
cargo build --features "hsm web5"
```

To build without default features:

```bash
cargo build --no-default-features
```

### Common Feature Combinations

#### Minimal Build

```bash
cargo build --no-default-features --features std
```

#### Full Enterprise Build

```bash
cargo build --features "complete system-alignment web5 memory_tracking rgb"
```

#### Development Build

```bash
cargo build --features "bitcoin_integration memory_tracking"
```

### Testing with Features

To run tests with specific features:

```bash
cargo test --features "rgb hsm"
```

To run tests for a specific module with features:

```bash
cargo test --features rgb -- layer2::rgb
```

## Feature Dependencies in Workspace

The features are organized hierarchically across the workspace:

```
anya-core
├── anya-bitcoin
│   └── (inherits workspace dependencies)
└── anya-extensions
    └── (inherits workspace dependencies)
```

### Workspace Feature Inheritance

The workspace dependencies are defined in the root `Cargo.toml` and should be used by member crates to ensure consistency:

```toml
# In member crate Cargo.toml
[dependencies]
bitcoin = { workspace = true, features = ["rand"] }
```

## Best Practices

1. **Use workspace inheritance** whenever possible to maintain consistency
2. **Document feature dependencies** when adding new features
3. **Test features in isolation** to ensure they work correctly
4. **Consider feature combinations** when developing new functionality
5. **Update this guide** when adding or modifying features

## Feature Stability Status

| Feature Flag | API Stability | Implementation Status | Testing Status |
|--------------|--------------|----------------------|----------------|
| `default` | Stable | Complete | Comprehensive |
| `hsm` | Stable | Complete | Comprehensive |
| `complete` | Stable | Complete | Comprehensive |
| `std` | Stable | Complete | Comprehensive |
| `bitcoin_integration` | Stable | Complete | Comprehensive |
| `rust-bitcoin` | Stable | Complete | Comprehensive |
| `rsk` | Stable | Complete | Moderate |
| `system-alignment` | Stable | Complete | Moderate |
| `web5` | Evolving | Partial | Limited |
| `memory_tracking` | Stable | Complete | Moderate |
| `rgb` | New | Partial | Limited |

## Locking Features

When a feature is considered stable and well-tested, it can be "locked" by:

1. Marking it as stable in this document
2. Creating a feature lock file in the relevant module directory
3. Adding comprehensive tests for the feature

Currently locked features: `default`, `hsm`, `complete`, `std`, `bitcoin_integration`, `rust-bitcoin`, `system-alignment`, `memory_tracking`
