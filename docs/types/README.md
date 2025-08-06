# Types Module Documentation

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

[AIS-3]: #ais-3 "Application Integration Standard Level 3"
[RES-3]: #res-3 "Resource Efficiency Standard Level 3"

## Overview

The Types module provides common data types, enumerations, and structures used throughout the Anya Core ecosystem. This module ensures consistent type definitions and serialization formats across all components, facilitating interoperability and maintainability.

## Core Components

### Compliance Types

The `compliance` submodule defines types related to Bitcoin Improvement Proposal (BIP) compliance and protocol validation.

#### Key Types

#### ProtocolCompliance

A structure representing the compliance status of a protocol implementation.

##### Properties

- `support_level`: Level of BIP support
- `verification_status`: Current verification state
- `compliance_score`: Numeric score (0.0-1.0) indicating compliance level
- `supported_bips`: List of supported BIP identifiers
- `missing_features`: List of features not yet implemented

#### BipSupportLevel

An enumeration representing the level of BIP support:

- `Full`: Complete implementation of the BIP
- `Partial`: Implementation of core features with some limitations
- `Minimal`: Basic implementation with significant limitations
- `None`: No implementation

#### VerificationStatus

An enumeration representing the status of compliance verification:

- `Passed`: Verification tests passed
- `Failed`: Verification tests failed
- `Pending`: Verification in progress
- `Skipped`: Verification not performed

#### MilestoneStatus

An enumeration representing the status of implementation milestones:

- `Completed`: Milestone fully achieved
- `InProgress`: Work on milestone ongoing
- `Failed`: Milestone implementation failed
- `Pending`: Milestone work not yet started

#### Usage Example

```rust
use anya_core::types::{ProtocolCompliance, BipSupportLevel, VerificationStatus};

fn check_protocol_compliance() -> ProtocolCompliance {
    ProtocolCompliance {
        support_level: BipSupportLevel::Full,
        verification_status: VerificationStatus::Passed,
        compliance_score: 0.98,
        supported_bips: vec![
            "BIP-174".to_string(),
            "BIP-340".to_string(),
            "BIP-341".to_string(),
            "BIP-342".to_string(),
            "BIP-370".to_string(),
        ],
        missing_features: vec![
            "Advanced PSBT extensions".to_string(),
        ],
    }
}
```

## Type Safety and Consistency

The Types module ensures type safety and consistency through:

1. **Clear Definitions**: Well-defined types with documentation
2. **Serialization Support**: Serde integration for all types
3. **Cross-Module Compatibility**: Types designed for use across different modules
4. **Extensibility**: Types designed to accommodate future extensions

## Future Extensions

The Types module is designed to be extended with additional type definitions as needed:

- **Transaction Types**: Specific transaction format types
- **Network Types**: Network-related data structures
- **Protocol Types**: Additional protocol-specific types
- **Security Types**: Types related to security operations

## Integration Points

The Types module is used throughout the Anya Core ecosystem:

- **BIP Module**: For BIP compliance reporting
- **Network Module**: For protocol support declarations
- **Configuration Module**: For protocol configuration
- **Test Module**: For test result reporting

## Compliance Standards

### AIR-3

Availability & Integrity Requirement Level 3: The Types module ensures data integrity through well-defined, consistent type definitions with validation capabilities.

### AIS-3

Application Integration Standard Level 3: Provides comprehensive type definitions for consistent data interchange across components and with external systems.

### BPC-3

Bitcoin Protocol Compatibility Level 3: Includes specialized types for Bitcoin protocol compliance tracking and validation.

### RES-3

Resource Efficiency Standard Level 3: Implements memory-efficient type definitions with optimized serialization and deserialization.
