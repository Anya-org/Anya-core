---
title: "Hsm_rc_requirements"
description: "Documentation for Hsm_rc_requirements"
---

[AIR-3][AIS-3][BPC-3][RES-3]


# HSM Module RC Requirements and Validation

## Overview

Add a brief overview of this document here.

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


## RC Requirements

1. **Software HSM Provider Only**
   - For the Release Candidate (v0.2.0-rc1), only the Software HSM provider should be used
   - Other providers (Hardware, TPM, Cloud, etc.) should be disabled or redirected to the Software provider
   - This simplifies testing and ensures consistent behavior across test environments

2. **User Activation Required**
   - The HSM module must require explicit user activation after successful testing
   - This prevents unauthorized or accidental use of cryptographic operations
   - All operations should fail with an appropriate error if HSM is not enabled by the user

## Implementation Guidelines

### HSM Manager Structure

```rust
pub struct HsmManager {
    // Configuration
    config: HsmConfig,
    
    // Provider implementation
    provider: Box<dyn HsmProvider>,
    
    // Activation state
    enabled: bool,
    
    // Other fields...
}
```

### HSM Status Enum

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HsmStatus {
    Initializing,
    Ready,
    Error(String),
    Disconnected,
    ShuttingDown,
    Disabled,  // New state for user-controlled activation
}
```

### Provider Selection Logic

```rust
// In HsmManager::new()
let provider: Box<dyn HsmProvider> = match config.provider_type {
    // For RC, only use Software provider regardless of configuration
    _ => {
        log::warn!("Using Software HSM provider for RC testing");
        Box::new(SoftwareHsmProvider::new(&config.software)?)
    }
};

// Initialize in disabled state
let manager = Self {
    config,
    provider,
    enabled: false,  // Disabled by default
    // Other fields...
};
```

### Enable/Disable Methods

```rust
impl HsmManager {
    // Enable the HSM (user activation)
    pub async fn enable(&mut self) -> Result<(), HsmError> {
        // Validation logic...
        self.enabled = true;
        Ok(())
    }
    
    // Disable the HSM
    pub async fn disable(&mut self) -> Result<(), HsmError> {
        self.enabled = false;
        Ok(())
    }
    
    // Check enabled state
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
```

### Operation Guard

All HSM operations should first check if the module is enabled:

```rust
impl HsmManager {
    pub async fn sign(&self, msg: &[u8], key_path: &HsmKeyPath) -> Result<Signature, HsmError> {
        // Check if HSM is enabled
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }
        
        // Proceed with operation...
    }
    
    // Similar checks for all other operations...
}
```

## Validation Steps

For RC testing, the HSM module should be validated as follows:

1. **Configuration Test**
   - Initialize the HSM manager with default configuration
   - Verify it's created in the disabled state

2. **Operation Guard Test**
   - Attempt operations before enabling
   - Verify all operations return `HsmError::Disabled`

3. **Activation Test**
   - Call `hsm_manager.enable()`
   - Verify operations now succeed

4. **Deactivation Test**
   - Call `hsm_manager.disable()`
   - Verify operations fail again

5. **Provider Override Test**
   - Configure with non-Software provider
   - Verify Software provider is still used

## RC Test Matrix

| Test Case | Expected Result | RC Validation |
|-----------|-----------------|---------------|
| Initialize HSM | Created in disabled state | ✓ Required |
| Operations before enable | Return HsmError::Disabled | ✓ Required |
| Enable HSM | Operations succeed | ✓ Required |
| Disable HSM | Operations fail | ✓ Required |
| Configure with Hardware provider | Uses Software provider | ✓ Required |

## Post-RC Implementation Plan

After RC validation, the following changes will be implemented for the final release:

1. Allow proper selection of all HSM provider types
2. Add comprehensive error handling for all providers
3. Improve performance with caching and optimized cryptography
4. Fix all deprecated base64 usage and clean up unused imports
5. Maintain the user activation requirement as a security feature

## Validation Approval

The RC should only be approved when the Software HSM provider works correctly with the user activation workflow described above.

## See Also

- [Related Document](#related-document)

