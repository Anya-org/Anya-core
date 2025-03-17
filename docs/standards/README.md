# Standards Documentation [AIR-3][AIS-3][BPC-3]

This directory contains canonical documentation of standards, architecture, and conventions used in the Anya Core project.

## Purpose

The standards documentation serves as the authoritative reference for:

1. Code conventions and style guides
2. Architecture patterns and principles
3. System organization and design
4. Versioning and labeling systems

## Index of Documents

### Core Standards

- [AI Labeling System](AI_LABELING.md) - Official AI labeling conventions and guidelines
- [Branch Structure](BRANCH_STRUCTURE.md) - Repository branch organization and conventions
- [Hexagonal Architecture](HEXAGONAL_ARCHITECTURE.md) - Architecture principles

### Bitcoin Compliance

- [BIP Standards](BIP_STANDARDS.md) - Compliance with Bitcoin Improvement Proposals
- [Security Model](SECURITY_MODEL.md) - Security principles and validation

## Version History

- v1.0 (2025-02-15): Initial standards documentation
- v1.1 (2025-03-01): Added Hexagonal Architecture documentation
- v1.2 (2025-03-20): Added AI Labeling System
- v1.3 (2025-03-25): Added Branch Structure documentation

## Usage Guidelines

All team members must adhere to these standards. When developing new features or modifying existing code:

1. Review relevant standards documents
2. Apply conventions consistently
3. Update standards documentation if necessary

## Proposing Changes

To propose changes to standards:

1. Create a `docs/` branch from `main`
2. Make your changes to the relevant document(s)
3. Submit a PR with thorough explanation
4. Obtain approval from at least two senior team members

## Standards Implementation Scripts

The following scripts help with standards implementation:

| Script | Description |
|--------|-------------|
| `scripts/validate_ai_labels.ps1` | Validates AI labels in code and documentation |
| `scripts/install_hooks.ps1` | Installs Git hooks for standards validation |
| `scripts/update_ai_labeling_docs.ps1` | Updates deprecated documentation with notices | 