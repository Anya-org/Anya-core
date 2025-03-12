<!-- markdownlint-disable MD013 line-length -->

# OPSource-dev Environment Rules

This document outlines the development environment guidelines as per OPSource-dev standards.

## Core Principles

- Maintain security, stability, and reproducibility in the development environment.
- Use layered configuration: global defaults can be overridden by local settings.
- Manage environment variables and secrets securely.
- Utilize containerization and consistent build tools.
- Ensure logging and monitoring are set up for development diagnostics.

## Environment Configuration

- **Configuration Files:** Place all environment-specific configuration files in the `config` directory.
- **Default Settings:** Use the provided `config/default.yaml` as baseline.
- **Development Overrides:** Merge `config/development.yaml` over default settings.
- **Containerization:** Development container configuration in `.devcontainer/` should follow best practices for reproducibility.

## Dependency Management

- Use consistent versioning for all dependencies.
- Ensure that your local environment is reproducible using the provided setup scripts (e.g., `install_dependencies.sh`).

## Testing and Deployment

- Run local tests using provided scripts (e.g., `scripts/run_tests.sh`).
- Use CI/CD pipelines as defined in `.github/workflows/` to ensure consistency across environments.

## Additional Guidelines

- Follow security recommendations and best practices documented in the corresponding security files.
- Regularly update the environment configuration files to reflect changes and improvements in the development process.
