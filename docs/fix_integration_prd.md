# Anya-Core v1.3 Integration Fixes PRD

## Overview

This Product Requirements Document outlines the systematic approach to fixing integration issues in Anya-Core v1.3 upgrade from the working v1.2 (main branch). The document serves as a tracking tool for the fixes required to ensure the successful compilation and functioning of the codebase.

## Current Status

- **Branch**: fix/integration-fixes
- **Target Version**: 1.3.0
- **Base Version**: 1.2.0 (working main branch)
- **Last Updated**: August 1, 2025

## Issue Categories

1. **Compilation Errors**
   - Critical issues preventing successful build
   - Syntax errors
   - Type mismatches
   - Module conflicts

2. **Warnings**
   - Unused imports
   - Unused variables
   - Deprecated API usage
   - Potential future issues

3. **Feature Flag Issues**
   - Inconsistent feature flags
   - Missing conditional compilation
   - Feature flag naming discrepancies

4. **Dead Code and TODOs**
   - Unreachable code
   - Commented out functionality
   - Unimplemented TODO items
   - Disabled tests

## Work Plan

Each issue is broken down into small, manageable chunks that can be addressed independently:

### 1. Fix Layer2 Protocol Mock Implementation

**Status**: Fixed
**Files**:

- src/layer2/mock/mod.rs
- src/layer2/mod.rs

**Issue**: Circular type definition in MockLayer2Protocol
**Solution**: Properly implement the mock type with MockAll

### 2. Resolve Bitcoin Module Conflicts

**Status**: Fixed
**Files**:

- src/bitcoin/wallet.rs
- src/bitcoin/wallet/mod.rs

**Issue**: Conflicting module paths
**Solution**: Consolidate wallet implementation into a single module structure

### 3. Fix Feature Flag Inconsistencies

**Status**: Fixed
**Files**:

- src/lib.rs
- Various files using feature flags

**Issue**: Inconsistent feature flags (bitcoin_integration vs bitcoin)
**Solution**: Standardize on the "bitcoin" feature flag throughout the codebase

### 4. Address Unused Imports and Variables

**Status**: In Progress
**Files**:

- src/api/routes.rs
- src/handlers/rgb.rs
- src/mobile/sdk.rs
- src/security/crypto/symmetric.rs
- src/security/hsm/providers/hardware.rs

**Issue**: Unused imports and variables causing warnings
**Solution**: Remove or properly use the imports and variables

### 5. Review and Fix Dead Code

**Status**: Pending
**Files**: Codebase-wide

**Issue**: Dead code and unreachable sections
**Solution**: Remove or properly implement dead code sections

### 6. Address TODOs and Implement Missing Functionality

**Status**: Pending
**Files**: Various files with TODO comments

**Issue**: Incomplete functionality marked with TODO comments
**Solution**: Implement or defer with proper documentation

### 7. Enable Disabled Tests

**Status**: Pending
**Files**: Test files with disabled tests

**Issue**: Tests that are commented out or marked with `#[ignore]`
**Solution**: Fix and enable tests or document why they remain disabled

### 8. Document API Changes

**Status**: Pending
**Files**: 

- docs/api/API_REFERENCE.md
- Other relevant documentation

**Issue**: Documentation may be out of sync with code changes
**Solution**: Update documentation to reflect API changes in v1.3

## Progress Tracking

| Task | Status | Notes |
|------|--------|-------|
| Fix Layer2 Protocol Mock | Completed | Fixed by removing circular type definition |
| Resolve Bitcoin Module Conflicts | Completed | Consolidated wallet implementation into wallet/mod.rs |
| Fix Feature Flag Inconsistencies | Completed | Standardized on "bitcoin" feature throughout codebase |
| Address Unused Imports/Variables | In Progress | Fixed `src/security/crypto/symmetric.rs`, `src/api/routes.rs` |
| Fix Bitcoin Validation Type Mismatches | Not Started | Type mismatch in `src/bitcoin/validation.rs` for taproot transactions |
| Fix API Parameter Inconsistencies | Not Started | Method parameter types don't match between implementation and usage |
| Review and Fix Dead Code | Not Started | |
| Address TODOs | Not Started | |
| Enable Disabled Tests | Not Started | |
| Document API Changes | Not Started | |
| Verify Documentation Against Code | In Progress | Using section "Documentation Verification" as checklist |
| Fix Markdown Linting Issues | In Progress | Addressing MD024, MD040, MD047 warnings in documentation files |

## Testing Strategy

1. After each fix, run `cargo check` to verify compilation
2. Run unit tests for affected modules with `cargo test --package <package_name> -- <test_path>`
3. Run integration tests after all fixes are applied
4. Perform manual verification of key functionality

## Completion Criteria

- All compilation errors resolved
- All warnings addressed
- Tests passing
- Documentation updated
- Feature parity with v1.2 maintained
- New v1.3 features properly integrated

## Critical Issues

### 1. Bitcoin Validation Type Mismatches

**Files**:

- `src/bitcoin/validation.rs`
- `src/lib.rs`

**Issues**:

- Type mismatches between function implementations and their usage:

  ```rust
  // In lib.rs (function definition)
  pub fn verify_taproot_transaction(&self, _tx: &str, ...)
  
  // In validation.rs (function usage)
  intel_opt.verify_taproot_transaction(tx)  // tx is of type &Transaction
  ```

- Similar issues with `verify_transaction_batch` method:

  ```rust
  // Expected: &[&str]
  // Found: &Vec<bitcoin::Transaction>
  ```

**Planned Fix**:

- Update function signatures to match parameter types
- Or, convert parameter types at call sites
- Use serialization/deserialization where needed

### 2. Module Organization Issues

**Files**:

- `src/bitcoin/mod.rs`
- `src/bitcoin/wallet.rs` and `src/bitcoin/wallet/mod.rs`

**Issues**:

- Duplicate wallet module implementations
- Conflicts between files causing compilation errors

**Planned Fix**:

- Consolidate wallet implementation to a single location
- Update imports and references accordingly

### 3. Unused Code and Imports

**Files**:

- Multiple files across the codebase

**Issues**:

- Unused imports causing compiler warnings
- Unused variables marked with `_` but still causing warnings
- Dead code that is no longer used

**Planned Fix**:

- Systematically clean up unused imports
- Remove or properly implement dead code
- Use proper attributes like `#[allow(dead_code)]` where appropriate

## Documentation Verification

To ensure documentation alignment with code implementation:

1. **Validate Module Documentation**
   - Confirm that all module-level documentation accurately describes current functionality
   - Check that API examples in documentation are valid and compilable
   - Verify that version-specific information is updated for v1.3

2. **Code Comment Verification**
   - Review inline documentation for accuracy
   - Update outdated comments, especially where APIs have changed
   - Ensure compliance tags (AIR-3, AIS-3, etc.) are correctly applied following AI labeling guidelines
   - Fix any markdown linting issues in documentation files

3. **External Documentation Audit**
   - Cross-check Markdown documentation files against actual code implementation
   - Update API reference documentation to match current interfaces
   - Verify that integration guides contain up-to-date examples
   - Validate feature documentation against actual implementation

4. **Documentation-Code Consistency Check**
   - Run automated documentation checks where available
   - Verify that all exported APIs are properly documented
   - Ensure that code examples in documentation match the current implementation
   - Update version numbers and compatibility tables to reflect v1.3 changes

## Implementation Plan

### 1. Layer2 Protocol Mock

**Status**: Completed
**Files**: 
- `src/layer2/mock/mod.rs`
- `src/layer2/mod.rs`

**Issue**: Circular type definition causing compilation error: `pub type MockLayer2Protocol = MockLayer2Protocol;`
**Solution**: Let mockall generate the mock type automatically

### 2. Bitcoin Module Conflicts

**Status**: Completed
**Files**:
- `src/bitcoin/wallet.rs`
- `src/bitcoin/wallet/mod.rs`

**Issue**: File for module wallet found at both paths, causing compiler error
**Solution**: Remove the duplicated `wallet.rs` file and use wallet/mod.rs

### 3. Bitcoin Validation Type Mismatches

**Status**: Not Started
**Files**:
- `src/bitcoin/validation.rs`
- `src/lib.rs`

**Issue**: Type mismatch between function signatures in lib.rs and their usage in validation.rs
**Solution**: 
- Update the `verify_taproot_transaction` function signature in lib.rs to accept `&Transaction` instead of `&str`
- Update the `verify_transaction_batch` function to accept `&[Transaction]` or serialize the transactions at call sites

### 4. Unused Code and Imports

**Status**: In Progress
**Files**: Multiple files across the codebase

**Issue**: Unused imports and variables causing compiler warnings
**Solution**: Systematically clean up unused imports and variables
