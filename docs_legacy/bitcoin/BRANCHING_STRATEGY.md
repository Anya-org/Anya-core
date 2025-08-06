# Bitcoin Module Branching Strategy
[AIR-3][AIS-3][BPC-3][AIT-3][RES-3]

This document outlines the branching strategy for Bitcoin-related changes in the Anya-Core project, following the Bitcoin Development Framework v2.5.

## Branch Structure

Our branch structure for Bitcoin module development follows a hierarchical model:

```
main
├── dev
│   ├── feature/bitcoin-core
│   ├── feature/bitcoin-implementation
│   ├── feature/bitcoin-layer2
│   ├── feature/bitcoin-testing
│   └── feature/bitcoin-hexagonal-architecture
└── new-release-candidate-1.0
```

## Branch Descriptions

- **main**: Stable production branch with released code only
- **dev**: Main development branch with integrated features
- **feature/bitcoin-core**: Core Bitcoin protocol implementations
- **feature/bitcoin-implementation**: Implementation-specific features
- **feature/bitcoin-layer2**: Layer 2 solutions (Lightning, RGB, etc.)
- **feature/bitcoin-testing**: Testing infrastructure for Bitcoin modules
- **feature/bitcoin-hexagonal-architecture**: Hexagonal architecture implementation for Bitcoin modules

## Workflow for Bitcoin Changes

1. **Feature Development**
   - Create a feature branch from the appropriate parent branch
   - Branch naming should follow the pattern: `feature/bitcoin-{component}-{description}`
   - Example: `feature/bitcoin-taproot-verification`

2. **Commit Standards**
   - All commits must follow the AI labeling standards
   - Format: `[AIR-3][AIS-3][BPC-3] Brief description`
   - AIR-3: AI Requirements
   - AIS-3: AI Security
   - BPC-3: Bitcoin Protocol Compliance

3. **Pull Request Process**
   - Create PR targeting the appropriate integration branch
   - PR title must follow the AI labeling format
   - PR description must include Bitcoin Development Framework compliance checklist
   - Require reviews from at least one Bitcoin protocol expert

4. **Testing Requirements**
   - Unit tests: 100% coverage for consensus-critical code
   - Integration tests: Testnet simulations with 3+ node types
   - Security tests: Fuzz testing for critical components

5. **Merging Strategy**
   - Squash and merge for small, contained changes
   - Merge commit for large feature integrations
   - Always maintain a clean commit history

6. **Branch Lifecycle**
   - Feature branches are deleted after successful merge
   - Integration branches are periodically synced with their parent branches
   - Release branches are created from integration branches when ready

## Compliance Requirements

All branches must adhere to:

- BIP standards compliance
- Hexagonal architecture principles
- Security validation requirements
- Documentation standards

This branching strategy ensures orderly development, clear tracking of changes, and maintenance of Bitcoin protocol compliance throughout the development lifecycle. 