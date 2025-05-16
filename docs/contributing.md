# Contributing to Anya Core

Thank you for your interest in contributing to Anya Core! This document provides guidelines for contributing to the project.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Reporting Issues](#reporting-issues)
- [Community](#community)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo
- Git

### Setting Up the Development Environment

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/anya-core.git
   cd anya-core
   ```
3. Install dependencies:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test
   ```

## Development Workflow

1. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes
3. Run tests and linters
4. Commit your changes with a descriptive message
5. Push to your fork and open a pull request

## Code Style

### Rust

Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/).

Key points:
- 4 spaces for indentation
- Maximum line length of 100 characters
- Use `snake_case` for variable and function names
- Use `PascalCase` for types and traits

### Documentation

- Document all public APIs
- Include examples in documentation
- Keep documentation up-to-date

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

### Writing Tests

- Write unit tests for all new functionality
- Include integration tests for critical paths
- Test edge cases and error conditions

## Pull Request Process

1. Ensure tests pass
2. Update documentation as needed
3. Keep changes focused and atomic
4. Reference any related issues
5. Request reviews from maintainers

## Reporting Issues

When reporting issues, please include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details
- Any relevant logs

## Community

- Join our [Discord](https://discord.gg/anya-core)
- Follow us on [Twitter](https://twitter.com/anyacore)
- Check out our [blog](https://blog.anya.org)

## License

By contributing, you agree that your contributions will be licensed under the project's [LICENSE](LICENSE) file.
