# Anya Core Workflow Architecture

This document outlines the GitHub Actions workflow architecture implemented for the Anya Core project. The architecture is designed to adhere to Bitcoin Core principles while optimizing resource usage and ensuring comprehensive validation.

## Core Principles

All workflows in Anya Core adhere to the following Bitcoin Core principles:

- **Decentralization**: No central dependencies or services that could represent single points of failure.
- **Security**: Comprehensive validation with multiple security layers.
- **Immutability**: Deterministic builds with environment hashing.
- **Transparency**: All validation steps are documented and results are published as artifacts.

## Architecture Overview

The workflow architecture follows a three-tiered structure:

1. **Base Layer**: Reusable workflow components that set up the environment.
2. **Layer 2**: Protocol-specific validation workflows.
3. **Application Layer**: Integration workflows and summaries.

### Branch-Based Optimization Strategy

Workflows are optimized based on branch type:

- **Main Branch**: Full validation with local Bitcoin Core node and hardware acceleration.
- **Development/Feature/RC Branches**: Streamlined validation using public RPCs to conserve resources.

## Reusable Workflow Components

### `.github/workflows/reusable/bitcoin-setup.yml`

Sets up a Bitcoin validation environment with the following features:

- Configurable Bitcoin Core version
- Local or public RPC endpoints based on branch
- Deterministic environment setup

### `.github/workflows/reusable/rust-bitcoin.yml`

Configures Rust toolchain with Bitcoin-specific components:

- Rust stable or nightly based on requirements
- Bitcoin-specific crates and dependencies
- Taproot support configuration

### `.github/workflows/reusable/bip-validation.yml`

Implements comprehensive BIP standards validation:

- BIP compliance testing
- Cryptographic validation
- Standardized reporting

## Main Workflows

### Bitcoin Core Workflow (`.github/workflows/bitcoin-core.yml`)

Consolidated workflow for Bitcoin Core validation:

- Replaces previous `bitcoin-validation.yml` and `bitcoin-combined.yml`
- Conditional execution based on branch
- Full validation for main branch, streamlined for other branches

### Layer 2 Protocols Workflow (`.github/workflows/layer2-protocols.yml`)

Validates all Layer 2 technologies on Bitcoin:

- Lightning Network validation
- Discrete Log Contracts (DLCs) validation
- RGB Smart Contracts validation
- Stacks blockchain validation
- RSK (Rootstock) sidechain validation
- Taproot Assets validation
- Cross-protocol integration tests

### Web5 Components Workflow (`.github/workflows/web5-components.yml`)

Validates Web5 components:

- Decentralized Identifiers (DIDs) validation
- Handshake protocol validation
- Web5 API conformance tests
- Mobile integration tests for React Native

## Resource Optimization

The workflows implement resource optimization strategies:

1. **Conditional Execution**: Only run comprehensive tests on main branch
2. **Public RPC Usage**: Use public RPC endpoints for non-main branches
3. **Artifact Retention**: Only retain essential artifacts
4. **Parallel Execution**: Run independent validation steps in parallel

## GitHub MCP Integration

The workflows are designed to leverage GitHub MCP tools for:

- Issue and PR management
- Repository operations
- Code search and retrieval
- Branch management

## Environment Variables

The following environment variables control workflow behavior:

- `MCP_GITHUB_USERNAME`: GitHub username for MCP operations
- `MCP_GITHUB_EMAIL`: GitHub email for MCP operations
- `MCP_GITHUB_DEFAULT_OWNER`: Default repository owner
- `MCP_GITHUB_DEFAULT_REPO`: Default repository name
- `BRANCH_TYPE`: Automatically determined branch type for optimization

## Validation Reports

Each workflow generates structured validation reports as JSON artifacts:

- Protocol-specific reports
- Integration reports
- Summary reports

These reports are uploaded as artifacts and can be used for compliance verification and audit trails.

## Running Workflows

### Manual Execution

Workflows can be triggered manually with the following parameters:

- `validation_level`: Standard or extended validation
- `specific_protocol/component`: Focus validation on a specific protocol or component

### Automated Execution

Workflows run automatically on:

- Push to main branch
- Push to development, feature, or release candidate branches
- Pull requests against any of these branches

## Future Enhancements

Planned workflow enhancements:

1. **AI Agent Integration**: Incorporate AI-driven validation and testing
2. **Federated Testing**: Implement federated testing across multiple environments
3. **Enhanced Mobile Testing**: Add more comprehensive mobile platform testing
4. **Cross-Repository Integration**: Test integration with other Bitcoin projects

## Troubleshooting

Common workflow issues and solutions:

1. **RPC Connection Failures**: Check if the required Bitcoin Core node is running
2. **Rust Build Failures**: Verify Rust toolchain configuration
3. **Test Failures**: Check test logs for specific error messages
4. **GitHub Action Limits**: Be aware of GitHub Actions usage limits

## Conclusion

This workflow architecture ensures that Anya Core remains aligned with Bitcoin Core principles while optimizing GitHub Actions resource usage. By implementing branch-specific validation strategies and reusable components, we maintain security and reliability while improving development efficiency.
