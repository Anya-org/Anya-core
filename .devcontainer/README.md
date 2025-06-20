# Development Container for Anya Core

This directory contains the configuration for the VS Code Development Container used for Anya Core development.

## Features

- **Up-to-date Git** - Built from source as needed
- **Complete Rust Development Environment**
  - Latest stable Rust with common utilities
  - Cargo tools for testing, code coverage, and publishing
  - WASM target support
  - Cross-compilation support
- **Bitcoin Development Tools**
  - Bitcoin Core development dependencies
  - Lightning Network development support
- **ML/AI Support**
  - Python with data science libraries
  - JupyterLab for notebooks
- **Web Development**
  - Node.js for web components
  - TypeScript support
- **Docker-in-Docker**
  - For container-based testing and deployment

## Getting Started

1. Open this repository in VS Code
2. When prompted, click "Reopen in Container"
3. Alternatively, press F1, then select "Dev Containers: Rebuild and Reopen in Container"
4. Run `.devcontainer/verify-setup.sh` to confirm everything is installed correctly

## VS Code Task Support

The following tasks are available from the VS Code Terminal menu:

- **Build Debug**: Build the project in debug mode
- **Build Release**: Build the project in release mode
- **Run**: Run the project
- **Test**: Run all tests
- **Check and Clippy**: Run Clippy with warnings as errors
- **Coverage**: Generate test coverage report
- **Verify DevContainer Setup**: Verify all tools are correctly installed
- **Generate Documentation**: Generate and open Rust documentation

## Debugging Support

Launch configurations are provided for:

- **Debug executable**: Debug the main executable
- **Debug unit tests**: Debug unit tests
- **Debug with features**: Debug with specific features enabled
- **Debug integration tests**: Debug integration tests
- **Debug benchmark**: Debug benchmarks

## Customization

If you need to install additional tools:

1. Edit `.devcontainer/devcontainer.json` to add features
2. Edit `.devcontainer/install-tools.sh` to add custom installation steps
3. Rebuild the container (F1 > Dev Containers: Rebuild Container)

## Persistent Storage

The following directories use Docker volumes for persistence:

- `/usr/local/cargo`: Cargo cache
- `/usr/local/rustup`: Rustup toolchain
- `${containerWorkspaceFolder}/node_modules`: Node.js modules

## Ports

The following ports are forwarded to the host:

- 3000: Web frontend
- 8000: API server
- 8080: Alternative web server
- 8332: Bitcoin RPC
- 8333: Bitcoin P2P
- 9735: Lightning Network
- 8888: Jupyter notebook
- 9999: Debug port
