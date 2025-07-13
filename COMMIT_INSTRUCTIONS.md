# Commit Grouping Instructions

This document provides step-by-step instructions for implementing the grouped commits in the Anya Core repository.

## Quick Start

1. Make scripts executable:
```bash
chmod +x group-commits.sh commit-safely.sh
```

2. Run the script:
```bash
./commit-safely.sh
```

## Manual Commit Instructions

If you prefer to run the commands manually, follow these steps:

### 1. Installation System
```bash
git add install-master.sh scripts/install/*.sh scripts/implement-installation.sh scripts/commit-changes.sh INSTALLATION_REVIEW.md
git commit -m "Implement system-optimized installation

- Add comprehensive auto-installation with system analysis
- Implement intelligent resource allocation based on system capabilities
- Add HSM hardware detection and configuration
- Configure resource limits in systemd service
- Create user-friendly installation wrapper script
- Support non-interactive installation with --auto-run

[AIR-3][AIS-3][AIT-3][AIP-3][RES-3]" --author="botshelomokokoka@gmail.com"
```

### 2. Documentation Updates
```bash
git add ROADMAP.md TODO.md docs/security/README.md docs/security/advanced_security.md docs/security/hsm_guide.md
git commit -m "Update documentation for security and roadmap

- Add HSM implementation guide
- Update security documentation
- Update roadmap with latest milestones
- Update TODO list with completed and new tasks

[AIR-3][AIS-3][AIT-3]" --author="botshelomokokoka@gmail.com"
```

### 3. Code Formatting
```bash
git add src/security/hsm/
git commit -m "Apply code style formatting to HSM module

- Fix whitespace and indentation throughout HSM module
- Apply consistent formatting for multi-line function signatures
- Fix trailing newlines
- Improve code readability

[AIR-3][AIS-3]" --author="botshelomokokoka@gmail.com"
```

### 4. Bitcoin Module
```bash
git add src/bitcoin/mod.rs src/bitcoin/error.rs src/bitcoin/wallet.rs
git commit -m "Implement Bitcoin module functionality

- Refactor Bitcoin module structure
- Add wallet implementation for Bitcoin
- Improve error handling
- Re-export core Bitcoin types for convenience

[AIR-3]" --author="botshelomokokoka@gmail.com"
```

### 5. Storage Module
```bash
git add src/storage/
git commit -m "Add storage module implementation

- Implement memory-based storage
- Create storage module structure
- Add storage interfaces for persistence

[AIR-3]" --author="botshelomokokoka@gmail.com"
```

### 6. API Module
```bash
git add src/api/ Cargo.toml
git commit -m "Implement API module with routes and models

- Add API routes implementation
- Create API models for request/response
- Add error handling for API
- Update dependencies in Cargo.toml

[AIR-3][AIP-3]" --author="botshelomokokoka@gmail.com"
```

### 7. Main Application
```bash
git add src/main.rs
git commit -m "Restructure main application

- Replace demo code with production API server
- Add Tokio async runtime support
- Initialize core components (storage, wallet, identity)
- Configure tracing for logging
- Set up TCP listener for API server

[AIR-3][AIP-3]" --author="botshelomokokoka@gmail.com"
```

### 8. CI/Debug
```bash
git commit -m "Add CI/Debug configuration

- Add debugging configurations
- Set up continuous integration settings
- Improve testing infrastructure

[AIR-3][AIT-3]" --author="botshelomokokoka@gmail.com"
```

## Verification
After running all commits, verify your changes:
```bash
git log --oneline -n 8
```

## Pushing Changes
Once you've verified the commits, push to the remote repository:
```bash
git push origin develop
``` 