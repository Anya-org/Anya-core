---
title: "Vs_ai_rules"
description: "Documentation for Vs_ai_rules"
last_updated: 2025-05-30
---
[AIR-3][AIS-3][BPC-3][RES-3]


<!-- markdownlint-disable MD013 line-length -->

# Visual Studio AI IDE Rules

## Overview

Add a brief overview of this document here.

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


This document outlines the rules and best practices for using Visual Studio with AI-assisted development in our project.

## Core Guidelines

- **Unified Project Structure:**
  - Ensure that all source code, modules, and dependencies are organized in a clean, modular structure.
  - Utilize our unified folders (e.g., the `modules` directory) so that the IDE's navigation, refactoring, and search tools work efficiently.

- **Source Control Integration:**
  - Leverage Git and related extensions within Visual Studio for source control management. Regularly commit changes with clear messages.
  - Follow our git submodule structure for managing different modules of the project.

- **Environment Consistency:**
  - Adhere to the OPSource-dev environment rules outlined in `docs/OPSource-dev.md` to maintain consistency between development, testing, and production environments.
  - Use containerization and standardized build tools supported by Visual Studio tasks and launch configurations.

- **AI-Assisted Features:**
  - Enable AI-assisted code analysis, debugging, and testing features to improve code quality and productivity.
  - Regularly update the IDE extensions and plugins to benefit from the latest AI capabilities, including intelligent code completion and automated refactoring.

- **Security and Compliance:**
  - Follow strict security practices, including regular security scans and adherence to best practices documented in our security files (e.g., `SECURITY.md`).
  - Integrate automated security and vulnerability checks directly into your VS development workflow.

- **Documentation and Collaboration:**
  - Maintain clear and updated documentation within the repository. Use the integrated documentation tools in Visual Studio to navigate and update guidelines.
  - Use collaboration tools (e.g., Live Share) integrated with Visual Studio to facilitate effective teamwork and real-time code reviews.

## Additional Best Practices

- Regularly sync your local repository with the remote to take advantage of automated CI/CD pipelines in our workflows.
- Leverage task runners and scripts (e.g., `scripts/run_tests.sh`) integrated into Visual Studio for a streamlined development process.
- Ensure that environment variables and configurations are managed securely, using provided configuration files and secret management systems.

By following these rules, the development experience on Visual Studio will be optimized with AI-driven features, ensuring rapid development cycles, high code quality, and consistent deployment practices.

## See Also

- [Related Document 1](../INSTALLATION.md)
- [Related Document 2](../INSTALLATION_REVIEW.md)
