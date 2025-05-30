---
title: "Contributing"
description: "Documentation for Contributing"
---

[AIR-3][AIS-3][BPC-3][RES-3]


# Contributing Guide \[AIR-1\]\[AIT-2\]

## Overview

Add a brief overview of this document here.

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


<!-- markdownlint-disable MD013 line-length -->

This document provides guidelines for contributing to the Anya Core project. Please read these guidelines before submitting any contributions.

## Code of Conduct

All contributors are expected to adhere to our Code of Conduct. Please read it before participating.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/anya-core.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Set up your development environment following the [Development Setup](./dev_setup.md) guide

## Development Workflow

### Branch Naming Convention

- `feature/` - for new features
- `fix/` - for bug fixes
- `docs/` - for documentation changes
- `refactor/` - for code refactoring
- `test/` - for adding or modifying tests

### Commit Message Guidelines

Follow these guidelines for commit messages:

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line
- Consider starting the commit message with an applicable emoji:
  - ✨ `:sparkles:` when adding a new feature
  - 🐛 `:bug:` when fixing a bug
  - 📚 `:books:` when adding or updating documentation
  - ♻️ `:recycle:` when refactoring code
  - 🧪 `:test_tube:` when adding tests

### Pull Request Process

1. Update the README.md or documentation with details of changes if applicable
2. Update the CHANGELOG.md with details of changes
3. The PR should work for all supported platforms
4. Ensure all tests pass
5. Get approval from at least one maintainer

## Coding Standards \[AIT-2\]

### Rust Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` to format your code
- Use `clippy` to catch common mistakes
- Document all public items with rustdoc comments
- Keep functions small and focused
- Write comprehensive tests for all new functionality

### AI Labelling \[AIR-1\]

All new code must include appropriate AI labelling tags as defined in the [AI Labelling Reference Guide](./docs/standards/AI_LABELING.md). For example:

```rust
/// Redis-based cache implementation
/// \[AIR-2\]\[AIP-3\]\[RES-2\]
pub struct RedisCache {
    // Implementation
}
```

### Testing Requirements \[AIT-2\]

- Write unit tests for all new functionality
- Ensure test coverage remains high
- Include integration tests for complex features
- For Bitcoin-related functionality, include testnet validation

## Documentation

### Code Documentation

- Document all public functions, structs, and traits
- Include examples in documentation where appropriate
- Keep documentation up-to-date with code changes

### Project Documentation

- Update relevant Markdown files when making significant changes
- Follow the AI labelling guidelines for all documentation
- Keep diagrams and architecture documents current

## Bitcoin Improvement Proposals (BIPs) Compliance \[AIR-1\]

Contributions that touch Bitcoin-related functionality must comply with official Bitcoin Improvement Proposals (BIPs):

1. Ensure protocol adherence to Bitcoin's core tenets
2. Follow privacy-preserving architecture principles
3. Adhere to asset management standards
4. Implement proper security validation
5. Follow hexagonal architecture patterns

## Submitting Issues

### Bug Reports

When submitting a bug report, please include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Screenshots or code examples if applicable
- Environment information (OS, Rust version, etc.)

### Feature Requests

When submitting a feature request, please include:

- A clear, descriptive title
- A detailed description of the proposed feature
- Rationale for why this feature is needed
- Any relevant examples or mockups

## Review Process

All contributions go through a review process:

1. Automated checks (CI/CD pipeline)
2. Code review by maintainers
3. Security review for sensitive areas
4. Final approval and merge

## Getting Help

If you need help, you can:

- Open a discussion on GitHub
- Reach out on our Discord channel
- Contact the maintainers directly

## License

By contributing to Anya Core, you agree that your contributions will be licensed under the project's license.

## Last Updated

2025-03-12

## See Also

- [Related Document](#related-document)

