# Test Organization Plan

This document outlines the structured approach for organizing and prioritizing test fixes in the Anya-core project. Tests will be categorized by logical components and prioritized based on importance and dependencies.

## Test Organization Structure

Tests should be organized to match the modular structure of the main codebase:

```
tests/
├── bitcoin/           # Bitcoin-related tests
│   ├── core/          # Core Bitcoin functionality
│   ├── layer2/        # Layer 2 protocols (RGB, DLC, etc.)
│   ├── protocol/      # Bitcoin protocols and BIPs
│   └── security/      # Bitcoin security tests
├── dao/               # DAO-related tests
│   ├── governance/    # Governance mechanisms
│   └── business/      # Business logic
├── ml/                # Machine Learning tests
├── web5/              # Web5 protocol tests
├── integration/       # Cross-module integration tests
├── common/            # Common test utilities and fixtures
└── security/          # Security and compliance tests
```

## Test Prioritization Strategy

Tests will be fixed in the following order:

1. **Core Bitcoin Tests** - Most fundamental layer
2. **Layer 2 Protocol Tests** - Building on Bitcoin core
3. **DAO Tests** - Business logic and governance
4. **Web5 Tests** - Identity and decentralized data
5. **ML Tests** - AI and machine learning
6. **Integration Tests** - Cross-component functionality
7. **Security Tests** - Security and compliance

## Issue Categories

Issues found in tests fall into these categories:

1. **Missing Dependencies** - External crates like `clarity_repl` or `clarinet` not in Cargo.toml
2. **Module Structure Mismatch** - Test imports don't match current module structure
3. **API Signature Changes** - Tests using outdated function signatures
4. **Bench Tests in Regular Tests** - Using `#[bench]` without nightly feature
5. **Dead Code** - Unused test utilities and functions
6. **Bitcoin Version API Compatibility** - Tests using API from older Bitcoin versions
7. **Import Visibility** - Private functions being imported by other modules

## Action Plan by Component

### 1. Bitcoin Core Tests

Issues:

- Fix imports to match current structure
- Address API changes from Bitcoin v0.32.6
- Fix visibility issues in test functions
- Update `TweakedPublicKey` usage to new API

### 2. Layer2 Tests (RGB, DLC)

Issues:

- Fix integration with main Bitcoin types
- Update mock implementations to match current traits
- Consolidate duplicate test fixtures

### 3. DAO Tests

Issues:

- Add missing dependency `clarity_repl` to Cargo.toml
- Add missing dependency `clarinet` to Cargo.toml
- Fix imports and module structure

### 4. Integration Tests

Issues:

- Update imports to match current API
- Fix cross-module dependencies

### 5. Common Test Utilities

Issues:

- Consolidate duplicate test utilities
- Update utility functions to match current API
- Mark unused utilities with underscores

## Implementation Priorities

1. **First Pass: Dependencies and Structure**
   - Add missing dependencies
   - Fix module structure and imports
   - Make test functions public where needed

2. **Second Pass: API Updates**
   - Update tests to use current API signatures
   - Fix Bitcoin v0.32.6 compatibility issues

3. **Third Pass: Dead Code and Cleanup**
   - Address warnings about unused code
   - Consolidate duplicate test utilities
   - Fix benchmarking tests

4. **Fourth Pass: Integration Tests**
   - Fix cross-component integration tests
   - Update mocks to match current implementations

## Specific Issues to Fix

1. Add missing dependencies:
   - `clarity_repl` for DAO tests
   - `clarinet` for DAO tests

2. Fix Bitcoin test imports:
   - Update imports for Bitcoin v0.32.6 compatibility
   - Fix `TweakedPublicKey` API usage

3. Fix visibility issues:
   - Make test functions public with `pub` keyword

4. Fix benchmarks:
   - Convert to criterion benchmarks or make compatible with stable Rust

5. Update test utilities:
   - Consolidate common test utilities
   - Mark unused variables with underscores

6. Fix Layer2 integration tests:
   - Update mock implementations to match current traits
