---
title: "Branch_structure"
description: "Documentation for Branch_structure"
---

# Branch Structure and Repository Organization [AIR-3][AIS-3][BPC-3]

## Overview

Add a brief overview of this document here.

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


This document defines the standardized branch structure for the Anya Core repository, in accordance with official Bitcoin Improvement Proposals (BIPs) and our hexagonal architecture.

## Branch Naming Convention

All branches must follow a consistent naming convention:

```
<type>/<feature-or-component>[-<subcomponent>]
```

Where:

- `<type>` is one of:
  - `feature`: New functionality
  - `fix`: Bug fixes
  - `docs`: Documentation-only changes
  - `refactor`: Code changes that neither fix bugs nor add features
  - `security`: Security-related changes
  - `bitcoin`: Bitcoin protocol-specific implementations
  - `web5`: Web5-related functionality

- `<feature-or-component>` should be a short, descriptive name of the feature or component
- `<subcomponent>` (optional) further specifies the scope

## Main Branches

| Branch | Purpose | Protection Rules |
|--------|---------|------------------|
| `main` | Production-ready code | Requires PR and approvals |
| `develop` | Integration branch for features | No direct commits |

## Feature Development Workflow

1. Create feature branch from `main`: `feature/<feature-name>`
2. Develop and test feature
3. Create PR to merge into `main`
4. After code review and approval, merge with `--no-ff` flag

## Security-Related Branches

Security branches follow stricter guidelines:

1. Name with `security/` prefix
2. Reference BIPs where applicable: `security/bip341-taproot`
3. Include thorough testing for all security components
4. Require security team approval before merging

## Documentation Branches

Documentation branches should be prefixed with `docs/`:

```
docs/hexagonal-architecture-update
docs/standards
```

## Bitcoin Protocol Branches

Bitcoin protocol implementations should be prefixed with `bitcoin/`:

```
bitcoin/bip341-implementation
bitcoin/transaction-validation
```

## Branch Lifecycle

1. **Creation**: Branch created from `main`
2. **Development**: Active work
3. **Testing**: Verification phase
4. **Review**: PR submitted
5. **Merge**: Merged into `main` with `--no-ff`
6. **Tagging**: Version tag applied to main after significant merges
7. **Cleanup**: Branch deleted after successful merge

## Tags

Version tags follow semantic versioning:

```
v<major>.<minor>.<patch>[-<prerelease>]
```

Example: `v1.3.0-rc1`

## Current Repository Structure

### Main Branches

- `main` - Production-ready code, latest version v1.3.0-rc1

### Feature Branches

- `feature/ai-validation-system` - AI validation for code quality
- `feature/enhanced-bitcoin-compliance` - Enhanced Bitcoin protocol compliance
- `feature/enterprise-wallet` - Enterprise wallet features
- `feature/web5-wallet` - Web5 wallet features
- `feature/consolidated-updates` - Consolidated feature updates

### Documentation Branches

- `docs/hexagonal-architecture-update` - Updates to architecture docs

### Specialty Branches

- `AIP-001-read-first-implementation` - Implementation of AIP-001
- `spv-security-enhancements` - SPV security improvements
- `new-feature-branch` - Repository cleanup and restructuring

## Hexagonal Architecture Considerations

Branch structure should respect the hexagonal architecture:

1. **Core Domain** branches should focus on business logic
2. **Adapter** branches should handle integration with external systems
3. **Port** branches should define interfaces
4. **Infrastructure** branches handle technical concerns

When creating branches, consider which layer of the hexagonal architecture is being modified.

## Version History

- v1.0.0: Initial branch structure
- v1.1.0: Added BDF-compliance considerations
- v1.2.0: Updated with AI labeling integration
- v1.3.0: Enhanced with SPV security 

## See Also

- [Related Document](#related-document)

