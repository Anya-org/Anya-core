# Anya Core Developer Guide

Welcome to the Anya Core Developer Guide. This guide provides a technical overview of the project and is intended for developers who want to contribute to the project or build applications on top of it.

## Table of Contents

- [Anya Core Developer Guide](#anya-core-developer-guide)
  - [Table of Contents](#table-of-contents)
  - [1. Introduction](#1-introduction)
  - [2. Architecture](#2-architecture)
    - [2.1. Overview](#21-overview)
    - [2.2. Core Components](#22-core-components)
    - [2.3. Directory Structure](#23-directory-structure)
  - [3. Getting Started](#3-getting-started)
    - [3.1. Prerequisites](#31-prerequisites)
    - [3.2. Development Environment](#32-development-environment)
  - [4. Running Tests](#4-running-tests)
  - [5. Contributing](#5-contributing)
    - [5.1. Code Style](#51-code-style)
    - [5.2. Commit Messages](#52-commit-messages)
    - [5.3. Pull Requests](#53-pull-requests)

## 1. Introduction

Anya Core is an enterprise-grade Bitcoin infrastructure platform that provides a secure and reliable way to build and manage Bitcoin applications. It is written in Rust and is designed to be modular, extensible, and easy to use.

This guide provides a technical overview of the project, including its architecture, how to set up a development environment, and how to contribute to the project.

## 2. Architecture

### 2.1. Overview

Anya Core follows a hexagonal architecture (also known as ports and adapters) to ensure a clear separation of concerns and to make the application easier to test and maintain. The core business logic is isolated from the external dependencies, such as the web server, database, and other services.

### 2.2. Core Components

- **`anya-core`**: The central crate that contains the core business logic of the application.
- **`anya-bitcoin`**: A library that provides a high-level API for interacting with the Bitcoin network.
- **`anya-web5`**: A library that provides a set of tools for building decentralized applications with Web5.
- **`axum`**: The web framework used to build the REST API.
- **`tokio`**: The asynchronous runtime used to power the application.

### 2.3. Directory Structure

The project is organized into the following directories:

- **`src`**: Contains the source code for the `anya-core` crate.
- **`src/api`**: Contains the API handlers, routes, and server configuration.
- **`src/bitcoin`**: Contains the Bitcoin-related functionality.
- **`src/web5`**: Contains the Web5-related functionality.
- **`tests`**: Contains the integration tests for the application.
- **`docs`**: Contains the documentation for the project.

## 3. Getting Started

### 3.1. Prerequisites

Before you begin, make sure you have the following prerequisites installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Git

### 3.2. Development Environment

To set up a development environment, follow these steps:

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/anya-org/anya-core.git
    cd anya-core
    ```

2.  **Install the dependencies:**

    ```bash
    cargo build
    ```

## 4. Running Tests

To run the tests, use the following command:

```bash
cargo test
```

## 5. Contributing

We welcome contributions from the community. If you would like to contribute to the project, please follow these guidelines.

### 5.1. Code Style

We follow the official Rust style guidelines. Please run `cargo fmt` to format your code before submitting a pull request.

### 5.2. Commit Messages

We follow the Conventional Commits specification for our commit messages. This makes it easier to track changes and to automatically generate release notes.

### 5.3. Pull Requests

When you are ready to submit a pull request, please make sure that your code is well-tested and that you have added any necessary documentation.
