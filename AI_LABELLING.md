<!-- markdownlint-disable MD013 line-length -->

> **⚠️ DEPRECATION NOTICE ⚠️**
> 
> This document is deprecated and will be removed in future versions.
> Please use the canonical AI labeling documentation at [docs/standards/AI_LABELING.md](docs/standards/AI_LABELING.md).
> The canonical document standardizes all AI labeling formats and requirements.

<!-- Original content below this line -->
# AI Labelling System for Anya Project

## Labelling Format
Changes are tagged with the following format:
[Category-Version][Component][Status]

## Categories
- AIS: AI Security (AIS-1 through AIS-3)
- BPC: Bitcoin Protocol Compliance (BPC-1 through BPC-3)
- DAO: Governance Implementation (DAO-1 through DAO-4)
- CLN: Codebase Cleanup
- DEP: Dependency Update
- FIX: Bug Fix

## Components
- BTC: Bitcoin Integration
- TRT: Taproot Implementation
- PSB: PSBT Handling
- DOC: Documentation
- CFG: Configuration

## Status Indicators
- WIP: Work in Progress
- RDY: Ready for Review
- MRG: Merged
- REV: Under Revision

## Commit Message Format
git commit -m "[BPC-3][TRT][RDY] Implement Taproot validation with BIP-341 compliance"

## Overview

This document defines the comprehensive AI labeling system used throughout the Anya Core project, following the Bitcoin Development Framework v2.5 standards. This labeling system ensures all components are properly categorized for AI readiness, security, performance, and compliance.

## Core Label Categories

### AIR - AI Readiness

AIR labels indicate how well a component is prepared for AI interaction and augmentation.

| Label | Description | Requirements |
|-------|-------------|--------------|
| AIR-0 | Not AI-Ready | No structured data, no defined interfaces |
| AIR-1 | Minimal AI-Readiness | Basic structured data, limited documentation |
| AIR-2 | Partial AI-Readiness | Structured data, partially documented interfaces |
| AIR-3 | Full AI-Readiness | Fully structured data, well-documented interfaces |

### AIS - AI Security

AIS labels indicate the level of security considerations for AI interactions.

| Label | Description | Requirements |
|-------|-------------|--------------|
| AIS-0 | No AI Security | No security considerations for AI interactions |
| AIS-1 | Basic AI Security | Basic input validation, minimal safeguards |
| AIS-2 | Enhanced AI Security | Input/output validation, security checks |
| AIS-3 | Full AI Security | Comprehensive validation, threat modeling, testing |

### AIT - AI Testing

AIT labels indicate the level of testing for AI components and interactions.

| Label | Description | Requirements |
|-------|-------------|--------------|
| AIT-0 | No AI Testing | No specific tests for AI components |
| AIT-1 | Basic AI Testing | Simple unit tests for AI components |
| AIT-2 | Enhanced AI Testing | Unit and integration tests for AI interactions |
| AIT-3 | Full AI Testing | Comprehensive testing including adversarial testing |

### AIM - AI Monitoring

AIM labels indicate the level of monitoring for AI components.

| Label | Description | Requirements |
|-------|-------------|--------------|
| AIM-0 | No AI Monitoring | No monitoring of AI components |
| AIM-1 | Basic AI Monitoring | Basic metrics collection |
| AIM-2 | Enhanced AI Monitoring | Metrics and alerting for AI components |
| AIM-3 | Full AI Monitoring | Comprehensive metrics, alerting, and analysis |

### AIP - AI Privacy

AIP labels indicate the level of privacy considerations for AI interactions.

| Label | Description | Requirements |
|-------|-------------|--------------|
| AIP-0 | No AI Privacy | No privacy considerations for AI data |
| AIP-1 | Basic AI Privacy | Basic data minimization |
| AIP-2 | Enhanced AI Privacy | Data minimization and anonymization |
| AIP-3 | Full AI Privacy | Comprehensive privacy protections, including PETs |

### AIE - AI Ethics

AIE labels indicate the level of ethical considerations for AI components.

| Label | Description | Requirements |
|-------|-------------|--------------|
| AIE-0 | No AI Ethics | No ethical considerations for AI |
| AIE-1 | Basic AI Ethics | Basic ethical guidelines |
| AIE-2 | Enhanced AI Ethics | Ethical guidelines and review process |
| AIE-3 | Full AI Ethics | Comprehensive ethical framework, review, and testing |

## Extended Label Categories

### BPC - Bitcoin Protocol Compliance

BPC labels indicate the level of compliance with Bitcoin protocol standards and best practices.

| Label | Description | Requirements |
|-------|-------------|--------------|
| BPC-0 | No Bitcoin Compliance | No compliance with Bitcoin protocols |
| BPC-1 | Basic Bitcoin Compliance | Basic implementation of Bitcoin protocols |
| BPC-2 | Enhanced Bitcoin Compliance | Implementation of advanced Bitcoin features (Taproot, Schnorr) |
| BPC-3 | Full Bitcoin Compliance | Complete compliance with all relevant BIPs, comprehensive testing |

### AIP - Application Interoperability

AIP labels indicate the level of interoperability with other systems and protocols.

| Label | Description | Requirements |
|-------|-------------|--------------|
| AIP-0 | No Interoperability | Isolated system with no external connections |
| AIP-1 | Basic Interoperability | Simple API for external systems |
| AIP-2 | Enhanced Interoperability | Comprehensive APIs and protocol support |
| AIP-3 | Full Interoperability | Complete interoperability with all relevant systems |

### PFM - Performance

PFM labels indicate the level of performance optimization and efficiency.

| Label | Description | Requirements |
|-------|-------------|--------------|
| PFM-0 | No Performance Optimization | No specific performance considerations |
| PFM-1 | Basic Performance | Basic performance optimizations |
| PFM-2 | Enhanced Performance | Comprehensive performance optimizations, benchmarking |
| PFM-3 | Full Performance | Advanced optimizations, continuous performance monitoring |

### SCL - Scalability

SCL labels indicate how well a component can scale with increased load.

| Label | Description | Requirements |
|-------|-------------|--------------|
| SCL-0 | Not Scalable | Cannot handle increased load |
| SCL-1 | Minimally Scalable | Basic vertical scaling capabilities |
| SCL-2 | Moderately Scalable | Horizontal and vertical scaling support |
| SCL-3 | Highly Scalable | Advanced scaling, automatic resource management |

### RES - Resilience

RES labels indicate how resilient a component is to failures and attacks.

| Label | Description | Requirements |
|-------|-------------|--------------|
| RES-0 | Not Resilient | No resilience mechanisms |
| RES-1 | Minimally Resilient | Basic error handling and recovery |
| RES-2 | Moderately Resilient | Comprehensive error handling, failover mechanisms |
| RES-3 | Highly Resilient | Advanced resilience, self-healing capabilities |

### DID - Decentralized Identity

DID labels indicate the level of decentralized identity integration.

| Label | Description | Requirements |
|-------|-------------|--------------|
| DID-0 | No DID Support | No support for decentralized identities |
| DID-1 | Basic DID Support | Basic DID resolution and verification |
| DID-2 | Enhanced DID Support | Comprehensive DID operations, credential management |
| DID-3 | Full DID Support | Complete W3C DID standard compliance, advanced features |

### W5C - Web5 Compliance

W5C labels indicate the level of compliance with Web5 standards.

| Label | Description | Requirements |
|-------|-------------|--------------|
| W5C-0 | No Web5 Compliance | No support for Web5 protocols |
| W5C-1 | Basic Web5 Compliance | Basic DWN integration |
| W5C-2 | Enhanced Web5 Compliance | Comprehensive DWN support, protocol implementation |
| W5C-3 | Full Web5 Compliance | Complete Web5 stack implementation, advanced features |

### UXA - User Experience & Accessibility

UXA labels indicate the level of user experience and accessibility considerations.

| Label | Description | Requirements |
|-------|-------------|--------------|
| UXA-0 | No UX/Accessibility | No specific UX or accessibility considerations |
| UXA-1 | Basic UX/Accessibility | Basic usability and accessibility features |
| UXA-2 | Enhanced UX/Accessibility | Comprehensive usability, WCAG A compliance |
| UXA-3 | Full UX/Accessibility | Advanced UX, WCAG AAA compliance |

### DAO - DAO Governance Integration (v2.2)

| Label | Requirements | Bitcoin Compliance |
|-------|--------------|---------------------|
| DAO-4 | Multi-chain PSBT, Legal Wrappers | BPC-3 Required |

## Implementation Rules

1. **Application**: All components must be labeled with appropriate categories from both Core and Extended label sets.

2. **Documentation**: Labels must be documented in component headers, README files, and design documents.

3. **Versioning**: Labels must be updated when components are modified.

4. **Compliance**: Components must meet the requirements for their assigned labels.

5. **Review**: Labels must be reviewed during code reviews.

6. **Completeness**: All components must have labels for each relevant category.

7. **Consistency**: Label applications must be consistent across similar components.

## Label Format

Labels are applied in documentation using the following format:

```
[AIR-2][AIS-1][AIT-2][AIM-1][AIP-2][AIE-1][BPC-2][SCL-2][DID-1]
```

Include only relevant labels for each component. Not all components require all label categories.

## Checkpoint Integration

Labels are integrated with the checkpoint system:

### Development Stage (60%)

- **Core Categories**: Minimum AIR-1, AIS-1, AIT-1
- **Extended Categories**: Minimum BPC-1 (for Bitcoin components), W5C-1 (for Web5 components)

### Production Stage (90%)

- **Core Categories**: Minimum AIR-2, AIS-2, AIT-2, AIM-1, AIP-1
- **Extended Categories**: Minimum BPC-2, PFM-1, RES-1, SCL-1

### Release Stage (99%)

- **Core Categories**: Minimum AIR-3, AIS-3, AIT-3, AIM-2, AIP-2, AIE-2
- **Extended Categories**: Minimum BPC-3, PFM-2, RES-2, SCL-2, UXA-2

## Special Integration Requirements

### DLC Oracle Integration

For components that act as oracles for Discrete Log Contracts (DLCs), additional requirements apply:

1. **Non-interactive Pattern**: Must follow the non-interactive oracle pattern
2. **Verification**: Must implement `verify_bitcoin_payment(proof: BitcoinSPV) -> bool`
3. **Security**: Must achieve minimum AIS-3 rating
4. **Privacy**: Must achieve minimum AIP-2 rating
5. **Testing**: Must achieve minimum AIT-3 rating
6. **Bitcoin Compliance**: Must achieve minimum BPC-3 rating
7. **Resilience**: Must achieve minimum RES-2 rating

### DWN Integration

For Web5 Decentralized Web Node (DWN) components, additional requirements apply:

1. **Identity**: Must properly handle DIDs and identity verification
2. **Storage**: Must implement encrypted storage
3. **Privacy**: Must achieve minimum AIP-2 rating
4. **Web5 Compliance**: Must achieve minimum W5C-2 rating
5. **Decentralized Identity**: Must achieve minimum DID-2 rating

### Lightning Network Integration

For Lightning Network components, additional requirements apply:

1. **BOLT Compliance**: Must comply with relevant BOLT specifications
2. **Security**: Must achieve minimum AIS-3 rating
3. **Performance**: Must achieve minimum PFM-2 rating
4. **Resilience**: Must achieve minimum RES-3 rating
5. **Bitcoin Compliance**: Must achieve minimum BPC-2 rating

### Cross-Chain Operations

For components that interact with multiple blockchains, additional requirements apply:

1. **Security**: Must achieve minimum AIS-3 rating
2. **Interoperability**: Must achieve minimum AIP-2 rating
3. **Testing**: Must achieve minimum AIT-3 rating
4. **Resilience**: Must achieve minimum RES-3 rating

## Component-Specific Labeling Requirements

### Core System Components

All core system components must have:

- Minimum AIR-2, AIS-2, AIT-2
- Minimum PFM-2, RES-2, SCL-2

### UI Components

All UI components must have:

- Minimum UXA-2 rating
- Documented accessibility features

### API Components

All API components must have:

- Minimum AIP-2 rating
- Documented interface specifications
- Security and authentication documentation

### Machine Learning Components

All ML components must have:

- Complete Core category ratings
- Minimum PFM-2, RES-2 ratings
- Documented training and inference procedures
- Model versioning information

## Auditing and Compliance

1. **Regular Audits**: All components must be audited quarterly for label compliance
2. **Automated Checks**: CI/CD pipelines must include automated label compliance checks
3. **Documentation**: All label audits must be documented and issues tracked
4. **Remediation**: Non-compliant components must be updated to meet requirements

## Last Updated

2025-02-24

## Bitcoin Protocol Compliance (BPC)
| Level | Requirements | Verification |
|-------|--------------|--------------|
| BPC-3 | BIP-341/342, Bitcoin Anchoring | SPV Proof Validation |

## DAO Governance (DAO)
| Level | Requirements | Bitcoin Alignment |
|-------|--------------|-------------------|
| DAO-4 | Institutional, Multi-chain | BPC-3 Compliant Proofs |

## Test Coverage Labels

### Bitcoin Protocol Tests [BPC-3] 

| Test Type | Implementation | No-Build Required |
|-----------|----------------|-------------------|
| BIP-341 (Taproot) | Mock + Testnet | ✅ |
| BIP-174 (PSBT) | Mock + Testnet | ✅ |
| BIP-370 (PSBT v2) | Mock Only | ✅ |

### Testnet Integration [BTC-TST]

| Component | Test Implementation | CI Integration |
|-----------|---------------------|---------------|
| Transaction Validation | ✅ | ✅ |
| Block Validation | ✅ | ✅ |
| Address Generation | ✅ | ✅ |

## Test Tracking Format

Tests are tracked with the following format:
[Category-Version][Component][TestType]

### Test Types
- UNT: Unit Test
- INT: Integration Test
- TST: Testnet Test
- MOK: Mock Test

