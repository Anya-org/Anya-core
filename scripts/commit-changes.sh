#!/bin/bash
# Script to commit all installation changes
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

echo "================================================================"
echo "        Committing Anya Core Installation System Changes"
echo "================================================================"
echo

# Define directories
INSTALL_DIR="scripts/install"

# Make scripts executable
chmod +x "${INSTALL_DIR}"/*.sh
chmod +x install-master.sh
chmod +x scripts/implement-installation.sh

# Stage all changes
git add "${INSTALL_DIR}"/*.sh
git add install-master.sh
git add scripts/implement-installation.sh
git add src/security/hsm/providers/software.rs

# Commit the changes
git commit -m "Implement system-optimized installation

- Add automated system analysis for hardware capabilities
- Create intelligent resource allocation based on system specs
- Implement HSM hardware detection and configuration
- Add resource limits to systemd service configuration
- Create user-friendly installation wrapper script
- Add non-interactive installation support
- Add comprehensive all-in-one installer (install-master.sh)
- Fix trailing newline in software.rs HSM provider

[AIR-3][AIS-3][AIT-3][AIP-3][RES-3]
Author: botshelomokokoka@gmail.com"

echo
echo "================================================================"
echo "        Installation System Changes Committed"
echo "================================================================"
echo
echo "The following files have been committed:"
echo "- auto_install.sh: System analysis and automated installation"
echo "- linux_install.sh: Build with system optimization"
echo "- systemd_config.sh: Service with resource limits"
echo "- uninstall.sh: Enhanced uninstallation"
echo "- install-anya.sh: User-friendly wrapper script"
echo "- install-master.sh: All-in-one installer script"
echo "- software.rs: Fixed trailing newline"
echo
echo "================================================================" 