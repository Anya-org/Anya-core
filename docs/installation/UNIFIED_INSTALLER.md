# Anya-Core Unified Installer

[AIR-3][AIS-3][AIP-3][BPC-3][DAO-4]

This document describes the Anya-Core unified installer, which provides a seamless installation experience for all components of the Anya-Core platform.

## Architecture

The unified installer follows the hexagonal architecture pattern, with:

- **Core Installation Logic**: Domain-specific installation procedures
- **Input Ports**: Command-line interface, configuration file parsing
- **Output Ports**: File system operations, dependency management, service configurations
- **Adapters**: OS-specific implementations, package managers, Docker integration 