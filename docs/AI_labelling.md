# AI Labelling Reference Guide \[AIR-3\]\[AIS-3\]

<!-- markdownlint-disable MD013 line-length -->

## Overview

This document defines standardized AI labelling tags for use in the Anya Core codebase. These tags help identify component roles, security classifications, and compliance with the Bitcoin Development Framework v2.5.

## Tag Format

AI tags follow the format: `[XXX-N]` where:

- `XXX` is a 3-letter category code
- `N` is a numeric level (1-5, with 5 being highest)

## Core Categories

### Architecture and Integration Tags [AIR]

| Tag | Level | Description |
|-----|-------|-------------|
| \[AIR-1\] | Basic | Component with simple integration requirements |
| \[AIR-2\] | Standard | Component with moderate integration complexity |
| \[AIR-3\] | Advanced | Critical component with complex integration requirements |
| \[AIR-4\] | Expert | Core system component with extensive integration requirements |
| \[AIR-5\] | Specialized | Specialized Bitcoin protocol implementation |

### Security and Privacy Tags [AIS]

| Tag | Level | Description |
|-----|-------|-------------|
| \[AIS-1\] | Basic | Standard security practices |
| \[AIS-2\] | Enhanced | Enhanced security requirements |
| \[AIS-3\] | Advanced | Advanced security controls with audit logging |
| \[AIS-4\] | Critical | Critical security component with formal verification |
| \[AIS-5\] | Military-grade | Maximum security requirements with hardware protections |

### Implementation and Testing Tags [AIT]

| Tag | Level | Description |
|-----|-------|-------------|
| \[AIT-1\] | Basic | Standard implementation with unit tests |
| \[AIT-2\] | Moderate | Comprehensive testing with integration tests |
| \[AIT-3\] | Advanced | Rigorous testing with fuzz testing and property-based tests |
| \[AIT-4\] | Enterprise | Enterprise-grade implementation with stress tests |
| \[AIT-5\] | Mission-critical | Mission-critical implementation with formal verification |

### Performance and Optimization Tags [AIP]

| Tag | Level | Description |
|-----|-------|-------------|
| \[AIP-1\] | Standard | Basic performance requirements |
| \[AIP-2\] | Optimized | Optimized for common operations |
| \[AIP-3\] | High-performance | High-performance implementation |
| \[AIP-4\] | Ultra-optimized | Ultra-optimized implementation with custom algorithms |
| \[AIP-5\] | Maximum | Maximum performance with hardware acceleration |

### Resilience and Error Handling Tags [RES]

| Tag | Level | Description |
|-----|-------|-------------|
| \[RES-1\] | Basic | Basic error handling |
| \[RES-2\] | Robust | Robust error handling with recovery mechanisms |
| \[RES-3\] | Resilient | Resilient system with comprehensive failure recovery |
| \[RES-4\] | Fault-tolerant | Fault-tolerant system with redundancy |
| \[RES-5\] | Self-healing | Self-healing system with automatic recovery |

### Scalability and Capacity Tags [SCL]

| Tag | Level | Description |
|-----|-------|-------------|
| \[SCL-1\] | Local | Designed for single-node deployment |
| \[SCL-2\] | Small-scale | Supports small-scale distributed deployment |
| \[SCL-3\] | Medium-scale | Supports medium-scale distributed deployment |
| \[SCL-4\] | Large-scale | Supports large-scale distributed deployment |
| \[SCL-5\] | Global-scale | Supports global-scale distributed deployment |

## Usage Guidelines

### When to Apply Tags

- Apply tags to all new files, classes, and significant functions/methods
- Add tags to module-level documentation
- Include tags in architectural diagrams and documentation
- Use tags in commit messages for significant changes

### How to Apply Tags

Tags should be applied as comments in code:

```rust
/// Hardware Security Module Manager for Bitcoin operations
/// \[AIR-3\]\[AIS-3\]\[AIT-3\]\[AIP-3\]\[RES-3\]
pub struct HsmManager {
    // ...
}
```

In documentation files:

```markdown
## HSM Bitcoin Integration \[AIR-3\]\[AIS-3\]\[AIT-3\]\[AIP-3\]\[RES-3\]

This document describes how the Hardware Security Module integrates with Bitcoin...
```

### Multiple Tags

Components may require multiple tags to fully describe their requirements:

- A high-security, performance-critical component: `\[AIS-4\]\[AIP-4\]`
- A core integration component with standard security: `\[AIR-3\]\[AIS-2\]`

## Tag Compliance Requirements

### \[AIR-3\] Requirements

- Must integrate with Bitcoin Core or equivalent
- Must implement required BIPs (341/342 for Taproot, 174 for PSBT)
- Must follow hexagonal architecture patterns
- Must provide comprehensive interface documentation

### \[AIS-3\] Requirements

- Must implement comprehensive access controls
- Must provide complete audit logging
- Must secure all cryptographic operations with HSM boundary protection
- Must implement secure key rotation
- Must validate all inputs against attack vectors

### \[AIT-3\] Requirements

- Must have 100% test coverage for critical paths
- Must include integration tests with multiple node types
- Must implement fuzz testing for security-critical components
- Must include property-based tests for cryptographic operations

### \[AIP-3\] Requirements

- Must optimize for Bitcoin transaction throughput
- Must minimize resource consumption for cryptographic operations
- Must provide efficient SPV verification
- Must implement parallel processing where appropriate

### \[RES-3\] Requirements

- Must gracefully handle all error conditions
- Must implement comprehensive recovery mechanisms
- Must provide detailed logging for all errors
- Must ensure no silent failures in Bitcoin operations

### \[SCL-3\] Requirements

- Must support deployment across multiple nodes
- Must support horizontal scaling of non-consensus components
- Must handle at least 1000 transactions per second
- Must implement efficient networking protocols

## Version History

- v1.0 (2024-12-15): Initial version
- v2.0 (2025-01-10): Added Bitcoin Development Framework alignment
- v2.5 (2025-02-24): Updated tags for hexagonal architecture alignment
- v2.6 (2025-03-01): Enhanced compliance requirements and added application examples
- v2.7 (2025-03-12): Updated documentation and standardized tag usage across codebase
