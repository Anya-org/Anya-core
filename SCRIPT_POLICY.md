# [AIR-3][AIS-3][BPC-3][RES-3] Script Management Policy

## Bitcoin Improvement Proposals (BIPs) Script Standards
*Last Updated: 2025-05-20*

This document defines the script management policy for the Anya Core project, following official Bitcoin Improvement Proposals (BIPs) with hexagonal architecture principles.

## Core Principles

1. **Minimal Script Approach**
   - Scripts should be consolidated into core utilities
   - Each script must serve a specific, well-defined purpose
   - No duplication of functionality across multiple scripts

2. **Hexagonal Architecture Compliance**
   - Scripts must respect the separation between core domain and adapters
   - No scripts should bypass the defined ports and adapters
   - All scripts must follow the defined system boundaries

3. **Standardized Structure**
   - All scripts must be placed in the appropriate category directory
   - Scripts must follow the standard header format with AI labeling
   - All scripts must implement proper error handling

## Approved Script Categories

1. **Core Operations** (`/scripts/core/`)
   - Essential build, test, and deployment operations
   - Core Bitcoin protocol interactions
   - System initialization and configuration

2. **Maintenance** (`/scripts/maintenance/`)
   - Data management and pruning
   - Backup and recovery operations
   - Health monitoring and reporting

3. **Security** (`/scripts/security/`)
   - Permission management
   - Cryptographic operations
   - Audit and compliance verification

4. **Development** (`/scripts/dev/`)
   - Development workflow utilities
   - Testing frameworks
   - Documentation generators

## Script Creation Process

1. Before creating a new script, developers must:
   - Verify no existing script provides the functionality
   - Obtain approval through the standard review process
   - Document the script purpose and integration points

2. All new scripts must:
   - Include standard [AIR-3][AIS-3][BPC-3][RES-3] labeling
   - Implement the error handling framework
   - Follow the hexagonal architecture principles
   - Include comprehensive documentation

## Enforcement

This policy is enforced through:
1. Automated CI/CD checks for script compliance
2. Code review requirements for script additions
3. Regular audits of the script directory
4. Dependency analysis to prevent script proliferation

## Exceptions

Exceptions to this policy require:
1. Formal documentation of the requirement
2. Approval from the architecture team
3. Implementation of compensating controls
4. Regular review for potential consolidation

---

By adhering to this policy, we ensure our script ecosystem remains maintainable, secure, and aligned with official Bitcoin Improvement Proposals (BIPs) and hexagonal architecture principles.
