#!/bin/bash
# Anya Core Unified Installation Script
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
INSTALL_DIR="${PROJECT_ROOT}/scripts/install"
UTILS_DIR="${INSTALL_DIR}/utils"

# Display header 
echo "================================================================"
echo "        Anya Core Installation System (v$VERSION)"
echo "================================================================"
echo
echo "Starting installation process at $(date)"
echo

# Check for root privileges
if [ "$EUID" -ne 0 ]; then
    echo "ERROR: This script requires root privileges. Please run with sudo."
    exit 1
fi

# Check for auto_install.sh
if [ ! -f "${INSTALL_DIR}/auto_install.sh" ]; then
    echo "ERROR: Installation script not found: ${INSTALL_DIR}/auto_install.sh"
    exit 1
fi

# Make sure it's executable
chmod +x "${INSTALL_DIR}/auto_install.sh"

# Pass all arguments to the auto_install.sh script
echo "Running main installation process..."
"${INSTALL_DIR}/auto_install.sh" "$@"

# Exit with same status code as the auto_install.sh script
exit $?
