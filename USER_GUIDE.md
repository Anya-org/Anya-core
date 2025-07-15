# Anya Core User Guide

Welcome to the Anya Core User Guide. This guide will help you install, configure, and use the Anya Core application.

## Table of Contents

- [Anya Core User Guide](#anya-core-user-guide)
  - [Table of Contents](#table-of-contents)
  - [1. Introduction](#1-introduction)
  - [2. Installation](#2-installation)
    - [2.1. Prerequisites](#21-prerequisites)
    - [2.2. Building from Source](#22-building-from-source)
  - [3. Configuration](#3-configuration)
    - [3.1. Environment Variables](#31-environment-variables)
  - [4. Running the Application](#4-running-the-application)
  - [5. Interacting with the API](#5-interacting-with-the-api)
    - [5.1. Health Check](#51-health-check)
    - [5.2. System Information](#52-system-information)
    - [5.3. Wallets](#53-wallets)
    - [5.4. Identities](#54-identities)
    - [5.5. DLCs](#55-dlcs)

## 1. Introduction

Anya Core is an enterprise-grade Bitcoin infrastructure platform that provides a secure and reliable way to build and manage Bitcoin applications. It offers a comprehensive set of features, including a Bitcoin wallet, decentralized identity management, and support for Discreet Log Contracts (DLCs).

This guide will walk you through the process of setting up and using Anya Core.

## 2. Installation

You can install Anya Core by building it from the source.

### 2.1. Prerequisites

Before you begin, make sure you have the following prerequisites installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Git

### 2.2. Building from Source

To build Anya Core from the source, follow these steps:

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/anya-org/anya-core.git
    cd anya-core
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

    This will create an optimized executable in the `target/release` directory.

## 3. Configuration

Anya Core can be configured using environment variables.

### 3.1. Environment Variables

The following environment variables are available for configuration:

- `ANYA_HOST`: The host address to bind the server to (default: `127.0.0.1`).
- `ANYA_PORT`: The port to listen on (default: `8080`).
- `RUST_LOG`: The logging level (e.g., `info`, `debug`, `error`).

## 4. Running the Application

To run the application, use the following command:

```bash
cargo run --release
```

The server will start and listen for incoming connections on the configured host and port.

## 5. Interacting with the API

You can interact with the application through its REST API. The base URL for the API is `http://<host>:<port>/api/v1`.

### 5.1. Health Check

To check the health of the application, send a GET request to the `/health` endpoint:

```bash
curl http://127.0.0.1:8080/api/v1/health
```

### 5.2. System Information

To get information about the system, send a GET request to the `/info` endpoint:

```bash
curl http://127.0.0.1:8080/api/v1/info
```

### 5.3. Wallets

The wallet endpoints allow you to create and manage Bitcoin wallets.

- **Create a new wallet:** `POST /wallets`
- **Get wallet details:** `GET /wallets/:id`
- **Get wallet balance:** `GET /wallets/:id/balance`
- **Generate a new address:** `POST /wallets/:id/address`
- **Send a transaction:** `POST /wallets/:id/transactions`
- **List transactions:** `GET /wallets/:id/transactions`

### 5.4. Identities

The identity endpoints allow you to create and manage decentralized identities.

- **Create a new identity:** `POST /identities`
- **Get identity details:** `GET /identities/:id`
- **Create a new credential:** `POST /credentials`
- **Get credential details:** `GET /credentials/:id`
- **Verify a credential:** `POST /credentials/verify`

### 5.5. DLCs

The DLC endpoints allow you to create and manage Discreet Log Contracts.

- **Create a new DLC:** `POST /dlc`
- **Get DLC details:** `GET /dlc/:id`
- **Accept a DLC:** `POST /dlc/:id/accept`
- **Finalize a DLC:** `POST /dlc/:id/finalize`
- **Execute a DLC:** `POST /dlc/:id/execute`
