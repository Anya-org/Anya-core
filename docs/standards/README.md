# Anya Core Standards [AIR-3][AIS-3][BPC-3]

<!-- markdownlint-disable MD013 line-length -->

This directory contains the canonical documentation for all standards used in the Anya Core project.

## Standards Document Index

| Document | Description | Status |
|----------|-------------|--------|
| [AI_LABELING.md](AI_LABELING.md) | Canonical AI labeling system definition | Active |

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2025-03-20 | Initial standards directory created |

## Usage Guidelines

### Standards Compliance

All code and documentation in the Anya Core project must comply with these standards. Enforcement is done through:

1. **Automated Validation**: Scripts in the `scripts/` directory validate compliance
2. **Git Hooks**: Pre-commit and commit-msg hooks enforce standards
3. **CI/CD**: GitHub Actions workflows validate compliance in PRs
4. **Code Reviews**: Reviewers ensure standards compliance

### Proposing Standards Changes

To propose changes to these standards:

1. Create a new branch with the format `standards/[standard-name]`
2. Make your changes
3. Submit a PR with the appropriate AI labels
4. Get approval from at least two senior contributors

## Relationship to Other Documentation

These standards take precedence over other documentation files. When conflicts exist, the documents in this directory are the source of truth.

## Standards Implementation Scripts

The following scripts help with standards implementation:

| Script | Description |
|--------|-------------|
| `scripts/validate_ai_labels.ps1` | Validates AI labels in code and documentation |
| `scripts/install_hooks.ps1` | Installs Git hooks for standards validation |
| `scripts/update_ai_labeling_docs.ps1` | Updates deprecated documentation with notices | 