# HSM Feature Fix Plan

## Issues and Fixes

### 1. `software.rs` Issues

- Error: `await` is only allowed inside `async` functions and blocks (line 320)
  - Fix: Check if the function containing this call is async or not

- Error: struct `RotateKeyParams` has no field named `id` (line 733)
  - Fix: Change `params.id` to the correct field name, likely `params.key_id`

- Error: mismatched types `Ok(HsmResponse::success(...))` expected `()`, found `HsmResponse`
  - Fix: Return the correct type from function

- Error: no method named `as_str` found for struct `SecureString` (line 339)
  - Fix: Implement `as_str` method for SecureString struct

- Error: no method named `lock` found for SecureString (line 537)
  - Fix: SecureString doesn't have lock method, fix implementation

### 2. `mod.rs` Issues

- Error: cannot find value `success` in scope (lines 292-293)
  - Fix: Define the missing variable

- Error: no variant `ProviderNotFound` for enum `HsmError` (line 147)
  - Fix: Use correct variant from HsmError enum

- Error: no variant `HealthCheck` for enum `AuditEventType` (line 291)
  - Fix: Use correct variant from AuditEventType enum

- Error: mismatched types expected `OperationResponse`, found `HsmResponse` (line 448)
  - Fix: Adjust return type

### 3. Other Issues

- Error: The `?` operator can't convert errors in several places
  - Fix: Implement required From traits or handle errors differently

- Error: incorrect function signatures in several provider implementations
  - Fix: Correct function signatures to match trait requirements

## Action Plan

1. Start with fixing `software.rs` issues
2. Fix `mod.rs` issues
3. Fix `audit.rs` issues
4. Fix provider interface mismatches
5. Fix bitcoin provider issues
6. Fix hardware provider issues
7. Address remaining errors
