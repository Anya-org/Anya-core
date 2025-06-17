---
title: "Dev_setup"
description: "Documentation for Dev_setup"
---

[AIR-3][AIS-3][BPC-3][RES-3]


# Development Environment Setup

## Overview

Add a brief overview of this document here.


This guide will help you set up your development environment for Anya Core.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Environment Setup](#environment-setup)
- [IDE Configuration](#ide-configuration)
- [Development Tools](#development-tools)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### System Requirements

- **Operating System**: Linux/macOS (Windows with WSL2 recommended)
- **CPU**: x86_64 or ARM64
- **Memory**: 8GB RAM minimum, 16GB recommended
- **Storage**: 20GB free space

### Required Software

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Git](https://git-scm.com/)
- [Docker](https://www.docker.com/) (optional, for containerized development)
- [Node.js](https://nodejs.org/) (for web components)

## Environment Setup

### 1. Clone the Repository

```bash
git clone https://github.com/anya-org/anya-core.git
cd anya-core
```

### 2. Install Dependencies

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y build-essential cmake pkg-config libssl-dev
```

#### macOS

```bash
brew install cmake pkg-config openssl
```

### 3. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup update
```

### 4. Install Development Dependencies

```bash
# Install Rust toolchain
rustup toolchain install stable
rustup default stable

# Install Rust components
rustup component add rustfmt clippy

# Install cargo tools
cargo install cargo-watch cargo-udeps cargo-audit
```

## IDE Configuration

### VS Code Setup

1. Install the following extensions:
   - Rust Analyzer
   - Better TOML
   - crates
   - CodeLLDB

2. Recommended settings (`.vscode/settings.json`):

```json
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.checkOnSave": true,
    "rust-analyzer.cargo.allFeatures": true,
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.codeActionsOnSave": {
        "source.organizeImports": true
    }
}
```

## Development Tools

### Useful Scripts

- `scripts/format.sh` - Format code
- `scripts/lint.sh` - Run linters
- `scripts/test.sh` - Run tests
- `scripts/coverage.sh` - Generate test coverage

### Git Hooks

Pre-commit hooks are set up to ensure code quality:

```bash
# Install pre-commit hook
ln -s ../../scripts/pre-commit .git/hooks/pre-commit
```

## Containerized Development

A `Dockerfile` and `docker-compose.yml` are provided for containerized development:

```bash
# Build the development image
docker-compose build

# Start the development environment
docker-compose up -d

# Attach to the container
docker-compose exec anya-core bash
```

## Troubleshooting

### Common Issues

#### 1. Linker Errors

```
= note: /usr/bin/ld: cannot find -lssl
```

**Solution**: Install OpenSSL development libraries:

```bash
# Ubuntu/Debian
sudo apt install libssl-dev

# Fedora
sudo dnf install openssl-devel

# macOS
brew install openssl@1.1
```

#### 2. Permission Denied

```
Error: Permission denied (os error 13)
```

**Solution**: Ensure your user has proper permissions or use `sudo` (not recommended for development).

#### 3. Outdated Dependencies

```
error: no matching package named `xyz` found
```

**Solution**: Update your dependencies:

```bash
cargo update
```

## Getting Help

If you encounter any issues, please:
1. Check the [Troubleshooting](#troubleshooting) section
2. Search the [issue tracker](https://github.com/anya-org/anya-core/issues)
3. Open a new issue if needed

## Next Steps

- [Contribution Guidelines](CONTRIBUTING.md)
- [Code of Conduct](../CODE_OF_CONDUCT.md)
- [API Documentation](https://docs.anya.org/api)

## See Also

- [Related Document](#related-document)

