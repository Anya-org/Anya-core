---
title: "Post_rc_fixes"
description: "Documentation for Post_rc_fixes"
---

[AIR-3][AIS-3][BPC-3][RES-3]


# Post-RC Compilation Fixes

## Overview

Add a brief overview of this document here.

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


This document outlines the specific fixes needed after the RC validation to resolve the compilation errors. These fixes are meant to be applied after the RC testing is complete, as they don't affect the functional validation of the RC candidate.

## HSM Module Errors

The primary issues in the HSM module are:

1. **Missing Type Definitions**: Several types referenced in `src/security/hsm/mod.rs` need to be properly defined

2. **Incorrect Imports**: Some imports are missing or duplicated

3. **Structural Issues**: There are references to types that need to be properly structured

## Required Fixes

### 1. Create HSM Types File

Create a new file at `src/security/hsm/types.rs` with the following core type definitions:

```rust
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::error::Error;
use std::fmt;

// Key type definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    // Implementation details
}

// KeyInfo structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    // Implementation details
}

// HSM Audit Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmAuditEvent {
    // Implementation details
}

// Various operation parameter structs
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateKeyParams {
    // Implementation details
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignParams {
    // Implementation details
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyParams {
    // Implementation details
}

// Other required type definitions
```

### 2. Update HSM Provider Status

Ensure the `HsmProviderStatus` enum is properly defined in `src/security/hsm/provider.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmProviderStatus {
    Ready,
    Initializing,
    Error(String),
    // Other states
}
```

### 3. Update Bitcoin Imports

Fix the bitcoin-related imports by ensuring the proper types are imported:

```rust
use bitcoin::{Script, ScriptBuf, XOnlyPublicKey, Txid, Psbt};
use bitcoin::taproot::TaprootBuilder;
use bitcoin::bip32::ExtendedPrivKey;
```

## Implementation Plan

1. Complete RC validation with the current code (ignoring compilation warnings)
2. Create a branch for post-RC fixes
3. Implement the type definitions and import fixes
4. Run comprehensive tests to ensure the fixes don't alter functionality
5. Merge the fixes for the final release

## Note on Base64 Warnings

The deprecated base64 functions should be addressed separately as they're just warnings and don't affect compilation. Follow the cleanup script created earlier to address these warnings after the RC process.

## See Also

- [Related Document](#related-document)

