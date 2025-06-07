# Anya Core Refactor & Auto-Run Implementation Plan

*Generated: June 7, 2025*

## Current State Analysis

### ✅ Completed (Core Library)
- Main `anya-core` library compiles successfully with 0 errors
- Only warnings remain (unused imports/variables - non-blocking)
- All Bitcoin, Layer2, P2P, mempool modules operational
- Security framework and error handling complete

### ❌ Remaining Issues (Binary Files)
- 32+ compilation errors across 10+ binary files
- Import path issues (`anya_core::` vs `crate::`)
- Missing type definitions
- Missing manager method implementations
- Outdated dependency usage patterns

## Refactor Strategy

### Phase 1: Critical Type Definitions
- [ ] Create missing types: `ProtocolCompliance`, `BipSupportLevel`, `VerificationStatus`, `MilestoneStatus`
- [ ] Add missing manager methods to dummy implementations
- [ ] Fix AnyaCore field access issues

### Phase 2: Import Path Standardization
- [ ] Update all binary files to use correct import paths
- [ ] Fix `crate::` vs `anya::` vs `anya_core::` inconsistencies
- [ ] Standardize module access patterns

### Phase 3: Binary File Fixes
- [ ] `api_server.rs` - Fix manager implementations and field access
- [ ] `anya_installer.rs` - Fix missing types and method calls
- [ ] `lightning_demo.rs` - Fix module imports and config access
- [ ] `perf_test.rs` - Fix structopt to clap migration
- [ ] `run_protocol_tests.rs` - Fix return types and error handling
- [ ] `anya_validator.rs` - Fix missing imports and types
- [ ] `doc_validator.rs` - Fix module paths
- [ ] `installer.rs` - Fix system API calls and clap usage

### Phase 4: Auto-Run Implementation
- [ ] Update auto-run scripts for real implementation
- [ ] Add automated compilation checking
- [ ] Implement continuous validation pipeline
- [ ] Add real-time error reporting

### Phase 5: TODO Modernization
- [ ] Update TODO.md with current completion state
- [ ] Add new implementation tasks
- [ ] Clean up completed items
- [ ] Add auto-run integration tasks

## Implementation Priorities

1. **HIGH**: Core type definitions and manager methods
2. **HIGH**: Binary file compilation fixes
3. **MEDIUM**: Auto-run script improvements
4. **MEDIUM**: TODO file updates
5. **LOW**: Warning cleanup and optimization

## Success Metrics

- [ ] 0 compilation errors across all binary files
- [ ] All auto-run scripts functional
- [ ] Updated TODO files reflect real state
- [ ] Comprehensive test coverage
- [ ] Production-ready deployment capability
