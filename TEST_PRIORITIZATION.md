# Test Prioritization and Organization Plan

This document outlines the plan for prioritizing and organizing test fixes to systematically address the issues in the Anya-core codebase.

## Priority Order

1. **Bitcoin Core Module Tests**
   - Fixed visibility issues (all test functions now public)
   - Fixed benchmark dependency in cross_layer_tests.rs
   - Fixed TweakedPublicKey API usage to match Bitcoin v0.32.6
   - Fixed ControlBlock API usage to match Bitcoin v0.32.6
   - Next: Fix remaining import issues

2. **Layer2 Tests (RGB, Lightning)**
   - Status: Not yet addressed
   - Next: Fix imports and API compatibility

3. **DAO Tests**
   - Added dependencies to Cargo.toml
   - Status: Still has import issues
   - Next: Fix imports and add module shims

4. **Enterprise Tests**
   - Fixed variable name issues in enterprise_cluster.rs
   - Status: Still has some issues with imports
   - Next: Fix remaining imports

5. **Web5 Tests**
   - Status: Not yet addressed
   - Next: Fix imports and API compatibility

6. **Integration Tests**
   - Status: Not yet addressed
   - Next: Fix imports and cross-module dependencies

## Issues and Solutions

| Component | Issue | Solution | Status |
|-----------|-------|----------|--------|
| Bitcoin Core | Private test functions | Added `pub` to function definitions | ✅ Complete |
| Bitcoin Core | Benchmarks require nightly | Converted to regular tests | ✅ Complete |
| Bitcoin Core | TweakedPublicKey API | Updated to use dangerous_assume_tweaked | ✅ Complete |
| Bitcoin Core | ControlBlock API | Updated to use output_key_parity | ✅ Complete |
| Bitcoin Core | Import issues | Need to update module structure | 🔄 In Progress |
| Layer2 | Import issues | Need to fix | ⏱️ Not Started |
| DAO | Missing dependencies | Added to Cargo.toml | ✅ Complete |
| DAO | Import issues | Need to fix | ⏱️ Not Started |
| Enterprise | Variable name issues | Fixed underscored variables | ✅ Complete |
| Web5 | Import issues | Need to fix | ⏱️ Not Started |
| Integration | Cross-module dependencies | Need to fix | ⏱️ Not Started |

## Next Steps

1. Create module shims to fix import issues:
   - Create Bitcoin module shims for API compatibility
   - Create DAO module shims for clarity_repl and clarinet
   - Create Web5 module shims

2. Fix remaining test visibility issues:
   - Update imports to point to new module structure
   - Make test utilities consistent

3. Fix unused variable warnings:
   - Prefix unused variables with underscore
   - Remove or update dead code

4. Create test organization readme:
   - Document test structure and dependencies
   - Provide guide for adding new tests
