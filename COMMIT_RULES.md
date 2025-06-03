# Commit Rules

## Overview

This document outlines the rules for creating commits in the Anya Core repository, including comprehensive AI and component labeling requirements based on official Bitcoin Improvement Proposals (BIPs).

## Commit Message Format

All commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **build**: Changes that affect the build system or external dependencies
- **ci**: Changes to our CI configuration files and scripts
- **chore**: Other changes that don't modify src or test files
- **revert**: Reverts a previous commit

### Scopes

Optional scopes should indicate the component affected:

- **bitcoin**: Bitcoin-related code
- **web5**: Web5-related code
- **ml**: Machine learning components
- **dao**: DAO governance components
- **core**: Core system components
- **config**: Configuration-related changes
- **security**: Security-related changes
- **ui**: User interface components
- **api**: API components
- **perf**: Performance-related changes
- **scale**: Scalability-related changes
- **did**: Decentralized identity components
- **ln**: Lightning Network components

## Comprehensive Labeling Integration

All commits that affect components must include the relevant labels from both Core and Extended categories in the commit message footer:

```
feat(ml): add new model versioning system

Implements a new versioning system for ML models with enhanced tracking

Labels: [AIR-3][AIS-2][AIT-3][AIM-2][PFM-2][SCL-2][RES-1]
```

### Required Labels by Component Type

Each commit must include appropriate labels based on the component type being modified:

#### Bitcoin Components

Must include: AIR, AIS, AIT, BPC
Should include where relevant: PFM, SCL, RES

#### Web5 Components

Must include: AIR, AIS, AIT, W5C, DID
Should include where relevant: PFM, SCL, RES

#### ML Components

Must include: AIR, AIS, AIT, AIM, AIP, AIE
Should include where relevant: PFM, SCL, RES

#### UI Components

Must include: AIR, UXA
Should include where relevant: PFM, AIP

#### API Components

Must include: AIR, AIS, AIP
Should include where relevant: PFM, SCL, RES

#### Core Components

Must include: AIR, AIS, AIT, PFM, RES, SCL

#### DAO Components

Must include: AIR, AIS, AIT, DAO
Should include where relevant: PFM, RES, SCL

## Branch Strategy

1. **main**: The main development branch containing stable code
2. **develop**: Active development branch
3. **feature/xxx**: Feature branches for new features
4. **fix/xxx**: Fix branches for bug fixes
5. **release/x.y.z**: Release branches for releases

All changes should be made on feature or fix branches, then merged to develop or main after review.

## Pull Requests

All pull requests must:

1. Include comprehensive descriptions
2. Reference related issues
3. Pass all CI checks
4. Include appropriate labels for all modified components
5. Be reviewed by at least one maintainer
6. Pass automated label compliance checks

## Label Compliance Requirements

### Adding or Modifying Components

When adding or modifying components, you must:

1. Add appropriate labels from both Core and Extended categories to the component documentation
2. Update relevant labels in README.md
3. Include label changes in commit messages
4. Ensure the component meets the requirements for its assigned labels
5. Document how the component meets each label requirement

### Documentation Updates

When updating documentation:

1. Add or update labels as needed
2. Follow the [](docs/standards/AI_LABELING.md) guidelines
3. Include `docs` type in commit messages
4. Indicate label changes in commit message footers

### Checkpoint Integration

Label updates must align with the checkpoint system:

#### Development stage (60%)

- **Core Categories**: Minimum AIR-1, AIS-1, AIT-1
- **Extended Categories**: Minimum BPC-1 (for Bitcoin components), W5C-1 (for Web5 components)

#### Production stage (90%)

- **Core Categories**: Minimum AIR-2, AIS-2, AIT-2, AIM-1, AIP-1
- **Extended Categories**: Minimum BPC-2, PFM-1, RES-1, SCL-1

#### Release stage (99%)

- **Core Categories**: Minimum AIR-3, AIS-3, AIT-3, AIM-2, AIP-2, AIE-2
- **Extended Categories**: Minimum BPC-3, PFM-2, RES-2, SCL-2, UXA-2

## Example Commits

### Bitcoin Component with Comprehensive Labels

```
feat(bitcoin): implement Taproot signing support

Add support for Taproot signing with BIP 341/342 compliance

Labels: [AIR-3][AIS-3][AIT-3][BPC-3][PFM-2][RES-2][SCL-1]
```

### ML Component with Comprehensive Labels

```
feat(ml): implement federated learning system

Add support for federated learning with privacy-preserving aggregation

Labels: [AIR-3][AIS-2][AIT-2][AIM-2][AIP-3][AIE-2][PFM-2][SCL-2][RES-1]
```

### Web5 Component with Comprehensive Labels

```
feat(web5): enhance DWN protocol support

Improve DWN protocol support with advanced record types

Labels: [AIR-3][AIS-2][AIT-2][W5C-3][DID-2][PFM-1][SCL-2]
```

### Documentation Update with Comprehensive Labels

```
docs: update ML system documentation

Improve ML system documentation and add comprehensive labeling

Labels: [AIR-3][AIT-2][AIM-2][AIE-2]
```

### Bug Fix with Comprehensive Labels

```
fix(ml): resolve model drift in prediction pipeline

Fix issue with model drift in the prediction pipeline

Labels: [AIR-2][AIT-3][AIM-3][PFM-2][RES-2]
```

## Validation

All commits are validated by the CI/CD pipeline, which checks:

1. Commit message format
2. Code quality (linting, formatting)
3. Test coverage
4. Label compliance for all modified components
5. Documentation completeness

### Automated Label Validation

The CI pipeline includes automated checks that:

1. Verify all modified components have appropriate labels
2. Check that labels meet minimum requirements for the component's checkpoint stage
3. Validate that components meet the requirements for their assigned labels
4. Generate reports on label compliance

## Special Component Requirements

### DLC Oracle Components

Commits affecting DLC oracles must include:

- Minimum AIS-3, AIP-2, AIT-3, BPC-3, RES-2 ratings
- Documentation of non-interactive pattern implementation
- Verification function implementation

### Lightning Network Components

Commits affecting Lightning Network components must include:

- Minimum AIS-3, PFM-2, RES-3, BPC-2 ratings
- Documentation of BOLT compliance
- Testing coverage for edge cases

### Cross-Chain Components

Commits affecting cross-chain operations must include:

- Minimum AIS-3, AIP-2, AIT-3, RES-3 ratings
- Documentation of security measures
- Test coverage for all supported chains

## Label Audit Requirements

All components must undergo regular label audits:

1. Quarterly comprehensive audits
2. Pre-release full compliance checks
3. Automated weekly compliance reports
4. CI-triggered compliance checks on modified components

## Bitcoin Development Framework Compliance

All commits must maintain compliance with official Bitcoin Improvement Proposals (BIPs), including:

1. Protocol adherence for Bitcoin operations
2. Privacy-preserving architecture for DLCs
3. Taproot-enabled protocols for asset issuance
4. Security validation for all transactions
5. AI system governance according to the Agent Decision Matrix

## Last Updated

2025-02-24
