---
title: "Installation"
description: "Comprehensive guide to installing Anya Core"
---

[AIR-3][AIS-3][BPC-3][RES-3]

# Anya Core Installation Guide

This guide provides comprehensive instructions for installing Anya Core. Choose the installation method that best suits your needs:

*   **Interactive Installation:** A user-friendly, guided installation process that helps you configure your system.
*   **Non-Interactive Installation:** A scriptable installer for automated deployments.

## Prerequisites

Before you begin, ensure your system meets the following minimum requirements:

*   **Operating System:** Linux (Ubuntu/Debian recommended) or macOS. Windows users should use WSL2.
*   **CPU:** 2 cores
*   **Memory:** 4 GB RAM
*   **Disk Space:** 100 GB
*   **Software:**
    *   Rust 1.70.0 or later
    *   Docker
    *   Git

## Interactive Installation

The interactive installer guides you through the installation process, helping you choose the right profile for your needs and verifying your system's compatibility.

### 1. Run the Installer

To start the interactive installer, run the following command from the root of the repository:

```bash
cargo run --bin anya_installer
```

### 2. Follow the Prompts

The installer will prompt you for the following information:

*   **Installation Directory:** The directory where Anya Core will be installed. The default is `/opt/anya-core`.
*   **Verbose Output:** Whether to display detailed output during installation.
*   **Installation Profile:** The type of installation you want to perform.

### 3. Choose an Installation Profile

The installer offers several profiles, each tailored to a specific use case:

*   **Auto-Configure (Recommended):** Automatically detects your hardware and selects the best profile for your system.
*   **Minimal Node:** A lightweight installation with minimal features, suitable for resource-constrained environments.
*   **Standard Node:** A balanced installation with a standard set of features.
*   **Full Archive Node:** A complete installation with all features and a full copy of the blockchain.
*   **Enterprise Cluster:** A high-performance installation for enterprise use cases.

### 4. Complete the Installation

Once you've answered all the prompts, the installer will perform the following steps:

1.  **Check System Dependencies:** Verifies that all required system packages and Rust crates are installed, and installs any missing dependencies.
2.  **Verify System Requirements:** Ensures your system meets the minimum hardware requirements.
3.  **Apply Hardware-Optimized Configuration:** Configures the system based on your hardware for optimal performance.
4.  **Generate and Validate Bitcoin Configuration:** Creates a `bitcoin.conf` file with the appropriate settings for your chosen profile.
5.  **Validate BIP Compliance:** Verifies that your system is compliant with the required Bitcoin Improvement Proposals (BIPs).
6.  **Run Security Audit:** Performs a security audit of your system.
7.  **Set up Monitoring and Services:** Configures systemd services and log rotation.
8.  **Generate Audit Log:** Creates a detailed audit log of the installation process.

## Non-Interactive Installation

The non-interactive installer is designed for automated deployments and scripting.

### 1. Run the Installer

To use the non-interactive installer, run the following command from the root of the repository:

```bash
cargo run --bin anya_installer -- --non-interactive install --profile <profile>
```

Replace `<profile>` with one of the following: `minimal`, `standard`, `full-node`, `enterprise`, or `auto`.

### 2. Command-Line Options

You can customize the installation with the following command-line options:

*   `--install-dir <path>`: Sets the installation directory.
*   `--non-interactive`: Skips all interactive prompts.
*   `--verbose`: Enables verbose output.

## Verifying the Installation

After the installation is complete, you can verify it by running the following command:

```bash
anya-cli status
```

## Checking System Requirements

You can check if your system meets the requirements without running the full installer:

```bash
cargo run --bin anya_installer check
```

## Uninstalling Anya Core

To uninstall Anya Core, run the following command:

```bash
cargo run --bin anya_installer uninstall
```

**Note:** The uninstall feature is not yet implemented.

## Updating an Existing Installation

To update an existing installation, run the following command:

```bash
cargo run --bin anya_installer update
```

**Note:** The update feature is not yet implemented.

## The `anya-install` Binary

The `anya-install` binary provides a non-interactive, component-based installation process. It is suitable for advanced users and automated deployments that require fine-grained control over the installed components.

### 1. Run the Installer

To use the `anya-install` binary, run the following command from the root of the repository:

```bash
cargo run --bin anya-install -- --config <config_file> --modules <components>
```

### 2. Command-Line Options

*   `-c, --config <FILE>`: Specifies a custom configuration file.
*   `-m, --modules <COMPONENTS>`: A comma-separated list of components to install (e.g., `core,bitcoin,dao`).
*   `-n, --network <NETWORK>`: The Bitcoin network type (`mainnet`, `testnet`, `regtest`).
*   `--rpc-endpoint <URL>`: A custom Bitcoin RPC endpoint.
*   `-v, --verify`: Verifies the installation after it's complete.

### 3. Installation Process

The `anya-install` binary performs the following steps:

1.  **Verifies System Requirements:** Checks for the required software and hardware.
2.  **Loads Configuration:** Loads the specified configuration file.
3.  **Installs Components:** Installs the specified components.
4.  **Verifies Installation:** If the `--verify` flag is present, it verifies the installation.
5.  **Generates Deployment Configuration:** Creates a `docker-compose.yml` and `.env` file for deployment.
