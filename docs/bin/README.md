---
title: "Binary Executables"
description: "Command-line interfaces and executables for Anya Core"
status: "active"
last_updated: "2025-08-06"
---

# Binary Executables [AIR-3][AIS-3][BPC-3][RES-3]

This module contains the command-line interfaces and executable binaries for Anya Core.

## Table of Contents

- [Overview](#overview)
- [Available Binaries](#available-binaries)
- [Usage Examples](#usage-examples)

## Overview

Anya Core provides several binary executables for different purposes, from the main application server to utility tools for validation and installation.

## Available Binaries

- **main.rs**: The main Anya Core server executable
- **anya_installer.rs**: Installation utility for Anya Core
- **anya_validator.rs**: Validation tool for Anya Core configuration
- **bip_health.rs**: Tool to check BIP compliance and health
- **doc_scanner.rs**: Documentation scanning and validation utility
- **lightning_demo.rs**: Demonstration of Lightning Network functionality
- **verify_bip_modules.rs**: Verification tool for BIP-compliant modules

## Usage Examples

### Main Server

```bash
# Start the Anya Core server on default port 8080
anya-core start

# Start the server on a custom port
anya-core start --port 9000

# Start with a specific configuration file
anya-core start --config /path/to/config.yaml
```

### Documentation Tools

```bash
# Scan and validate documentation
anya-doc-scanner --path /path/to/docs

# Generate documentation report
anya-doc-scanner --report --output report.md
```

### BIP Validation

```bash
# Check BIP compliance
anya-bip-health --check-all

# Verify specific BIP implementation
anya-bip-health --verify-bip 341
```
