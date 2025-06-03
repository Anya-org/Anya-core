#!/bin/bash
# Group commits by logical changes
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

echo "====== Grouping commits by logical changes ======"

# Group 1: Installation System
echo "Committing installation system changes..."
git add install-master.sh scripts/install/*.sh scripts/implement-installation.sh scripts/commit-changes.sh INSTALLATION_REVIEW.md
git commit -m "Implement system-optimized installation

- Add comprehensive auto-installation with system analysis
- Implement intelligent resource allocation based on system capabilities
- Add HSM hardware detection and configuration
- Configure resource limits in systemd service
- Create user-friendly installation wrapper script
- Support non-interactive installation with --auto-run

[AIR-3][AIS-3][AIT-3][AIP-3][RES-3]" --author="botshelomokokoka@gmail.com"

# Group 2: Documentation Updates
echo "Committing documentation updates..."
git add ROADMAP.md TODO.md docs/security/README.md docs/security/advanced_security.md docs/security/hsm_guide.md
git commit -m "Update documentation for security and roadmap

- Add HSM implementation guide
- Update security documentation
- Update roadmap with latest milestones
- Update TODO list with completed and new tasks

[AIR-3][AIS-3][AIT-3]" --author="botshelomokokoka@gmail.com"

# Group 3: Code Formatting
echo "Committing code formatting changes..."
git add src/security/hsm/
git commit -m "Apply code style formatting to HSM module

- Fix whitespace and indentation throughout HSM module
- Apply consistent formatting for multi-line function signatures
- Fix trailing newlines
- Improve code readability

[AIR-3][AIS-3]" --author="botshelomokokoka@gmail.com"

# Group 4: Bitcoin Module
echo "Committing Bitcoin module changes..."
git add src/bitcoin/mod.rs src/bitcoin/error.rs src/bitcoin/wallet.rs
git commit -m "Implement Bitcoin module functionality

- Refactor Bitcoin module structure
- Add wallet implementation for Bitcoin
- Improve error handling
- Re-export core Bitcoin types for convenience

[AIR-3]" --author="botshelomokokoka@gmail.com"

# Group 5: Storage Module
echo "Committing storage module implementation..."
git add src/storage/
git commit -m "Add storage module implementation

- Implement memory-based storage
- Create storage module structure
- Add storage interfaces for persistence

[AIR-3]" --author="botshelomokokoka@gmail.com"

# Group 6: API Module
echo "Committing API module implementation..."
git add src/api/ Cargo.toml
git commit -m "Implement API module with routes and models

- Add API routes implementation
- Create API models for request/response
- Add error handling for API
- Update dependencies in Cargo.toml

[AIR-3][AIP-3]" --author="botshelomokokoka@gmail.com"

# Group 7: Main Application
echo "Committing main application changes..."
git add src/main.rs
git commit -m "Restructure main application

- Replace demo code with production API server
- Add Tokio async runtime support
- Initialize core components (storage, wallet, identity)
- Configure tracing for logging
- Set up TCP listener for API server

[AIR-3][AIP-3]" --author="botshelomokokoka@gmail.com"

# Group 8: CI/Debug
echo "Committing CI/Debug configuration..."
git add anya-core-ci-debug/
git commit -m "Add CI/Debug configuration

- Add debugging configurations
- Set up continuous integration settings
- Improve testing infrastructure

[AIR-3][AIT-3]" --author="botshelomokokoka@gmail.com"

echo "====== All changes committed successfully ======" 