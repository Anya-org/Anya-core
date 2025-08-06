---
title: "Testing"
description: "Anya Core Testing Guide and Framework"
---

# Anya Core Testing Guide

## Test Organization

The test suite has been reorganized to eliminate duplicates and provide centralized utilities:

### Test Structure

```
tests/
├── common/
│   ├── mod.rs              # Common module exports
│   └── test_utilities.rs   # Centralized test utilities
├── bitcoin/                # Bitcoin protocol tests
├── hardware/               # Hardware optimization tests
├── dao/                    # DAO functionality tests
├── layer2/                 # Layer 2 protocol tests
├── web5/                   # Web5 integration tests
├── unit_tests/             # Unit tests
└── mod.rs                  # Main test module
```

### Centralized Test Utilities

All tests now use centralized utilities from `tests/common/test_utilities.rs`:

#### TestTransactionFactory
- `create_simple()` - Creates basic test transactions
- `create_dummy_transaction()` - Legacy compatibility wrapper
- `create_dummy_transaction_batch(size)` - Batch transaction creation

#### TestEnvironmentFactory
- `new_basic()` - Basic test environment
- `new_with_config(config)` - Custom configuration

#### MockFactory
- `create_bitcoin_keys()` - Mock Bitcoin key pairs
- `create_oracle_data()` - Mock DLC oracle data
- `create_secp256k1_context()` - Mock secp256k1 context

#### TestAssertions
- `assert_transaction_valid(tx)` - Transaction validation
- `assert_consensus_compliant(data)` - Consensus compliance
- `assert_performance_acceptable(metrics)` - Performance validation

## Running Tests

### Quick Test Run
```bash
cargo test
```

### Organized Test Execution
```bash
./scripts/run-all-tests.sh
```

### Category-Specific Tests
```bash
cargo test --test bitcoin_tests
cargo test --test hardware_tests
cargo test --test dao_tests
```

## Test Cleanup Completed

✅ **Eliminated Duplicates:**
- Removed duplicate `create_dummy_transaction()` functions
- Consolidated `TestEnvironment::new()` patterns
- Merged duplicate test directories
- Removed RISC-V test file duplicates

✅ **Standardized Patterns:**
- Centralized test utilities
- Consistent import structure
- Unified assertion helpers
- Common mock data creation

✅ **Improved Organization:**
- Clear test categorization
- Proper module structure
- Centralized re-exports
- Backward compatibility maintained

## Best Practices

1. **Use Centralized Utilities**: Always import from `crate::common::test_utilities`
2. **Follow Naming Conventions**: Use descriptive test function names
3. **Categorize Tests**: Place tests in appropriate directories
4. **Mock External Dependencies**: Use MockFactory for external resources
5. **Validate Results**: Use TestAssertions for consistent validation
