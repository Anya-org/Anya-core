#!/bin/bash
# Anya Core Installation Scripts Cleanup Utility
# This script analyzes existing installation scripts and identifies redundant ones
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$INSTALL_DIR/../.." && pwd)"
UTILS_DIR="${SCRIPT_DIR}"

# Source common utilities
if [ -f "${UTILS_DIR}/install_common.sh" ]; then
    source "${UTILS_DIR}/install_common.sh"
else
    echo "ERROR: Common utilities not found: ${UTILS_DIR}/install_common.sh"
    exit 1
fi

# Set up process lock
if ! setup_process_lock "script_cleanup"; then
    exit 1
fi

# Display header
print_header "Script Cleanup Utility" "$VERSION"

# Essential scripts that should NOT be removed
ESSENTIAL_SCRIPTS=(
    "${PROJECT_ROOT}/scripts/install.sh"
    "${INSTALL_DIR}/auto_install.sh"
    "${INSTALL_DIR}/linux_install.sh"
    "${INSTALL_DIR}/systemd_config.sh"
    "${INSTALL_DIR}/uninstall.sh"
    "${UTILS_DIR}/install_common.sh"
    "${UTILS_DIR}/script_cleanup.sh"
)

# Find all installation-related scripts
log INFO "Scanning for installation-related scripts..."
INSTALL_SCRIPTS=()
mapfile -t INSTALL_SCRIPTS < <(find "${PROJECT_ROOT}/scripts" -name "*install*.sh" -type f)

# Count scripts before cleanup
TOTAL_SCRIPTS=${#INSTALL_SCRIPTS[@]}
log INFO "Found $TOTAL_SCRIPTS installation-related scripts"

# List all installation scripts
echo "================================================================"
echo "Installation Scripts Analysis"
echo "================================================================"
echo
echo "Essential scripts (will be preserved):"
for script in "${ESSENTIAL_SCRIPTS[@]}"; do
    if [ -f "$script" ]; then
        echo "✅ $(readlink -f "$script")"
    else
        echo "❌ $script (missing)"
    fi
done
echo

# Check redundant scripts
REDUNDANT_SCRIPTS=()
echo "Potentially redundant scripts:"
for script in "${INSTALL_SCRIPTS[@]}"; do
    # Skip essential scripts
    is_essential=false
    for essential in "${ESSENTIAL_SCRIPTS[@]}"; do
        if [ "$(readlink -f "$script")" = "$(readlink -f "$essential")" ]; then
            is_essential=true
            break
        fi
    done
    
    if [ "$is_essential" = false ]; then
        # Check if script is small (likely a wrapper) or special case
        if [ -f "$script" ]; then
            size=$(wc -l < "$script")
            if [ $size -lt 100 ]; then
                echo "- $script ($size lines - likely a wrapper)"
            else
                echo "+ $script ($size lines - likely contains logic)"
            fi
        else
            echo "? $script (cannot access)"
        fi
        REDUNDANT_SCRIPTS+=("$script")
    fi
done

# Count redundant scripts
REDUNDANT_COUNT=${#REDUNDANT_SCRIPTS[@]}
log INFO "Found $REDUNDANT_COUNT potentially redundant scripts"

if [ $REDUNDANT_COUNT -gt 0 ]; then
    echo
    echo "Recommendations:"
    echo
    echo "The following scripts appear to be redundant and could be deprecated"
    echo "in favor of the new unified installation system:"
    echo
    for script in "${REDUNDANT_SCRIPTS[@]}"; do
        script_file=$(basename "$script")
        echo "1. $script_file - Replace with: ./scripts/install.sh"
    done
    echo
    echo "The new installation system provides:"
    echo "- Hardware auto-detection and feature flag configuration"
    echo "- Unified experience across different environments"
    echo "- Support for clean installs and upgrades"
    echo "- Proper version tracking and configuration management"
    echo
    echo "To clean up redundant scripts, run:"
    echo "  $0 --clean"
    echo
else
    log INFO "No redundant scripts found!"
fi

# Clean up if requested
if [ $# -gt 0 ] && [ "$1" = "--clean" ]; then
    log INFO "Cleaning up redundant scripts..."
    
    # Create backup directory
    BACKUP_DIR="${PROJECT_ROOT}/scripts/install/utils/backup_$(date +%Y%m%d-%H%M%S)"
    mkdir -p "$BACKUP_DIR"
    
    # Move redundant scripts to backup directory
    for script in "${REDUNDANT_SCRIPTS[@]}"; do
        if [ -f "$script" ]; then
            script_name=$(basename "$script")
            log INFO "Moving $script to backup"
            cp "$script" "${BACKUP_DIR}/${script_name}"
            # Create a symbolic link to the new unified script
            ln -sf "${PROJECT_ROOT}/scripts/install.sh" "$script"
            log INFO "Created symbolic link from $script to unified installer"
        fi
    done
    
    log INFO "Cleanup complete. Redundant scripts backed up to $BACKUP_DIR"
fi

# Feature flag detection for Anya Core components
echo
echo "================================================================"
echo "Feature Flag Component Support"
echo "================================================================"
echo
log INFO "Analyzing feature flag support for Anya Core components..."

# Function to check if a component is feature-flagged
check_component_feature_flag() {
    local component=$1
    local flag_name=$2
    local src_dir="${PROJECT_ROOT}/src/${component}"
    
    if [ -d "$src_dir" ]; then
        # Check for cfg attributes in Rust files
        if grep -r --include="*.rs" "#\[cfg(feature\s*=\s*\"$flag_name\"" "$src_dir" >/dev/null 2>&1; then
            echo "✅ $component: Feature flagged with '$flag_name'"
            return 0
        else
            echo "❌ $component: No feature flag found"
            return 1
        fi
    else
        echo "⚠️ $component: Directory not found"
        return 2
    fi
}

# Check core components
check_component_feature_flag "security/hsm" "hsm"
check_component_feature_flag "bitcoin" "bitcoin_integration"
check_component_feature_flag "layer2/lightning" "lightning"
check_component_feature_flag "dao" "dao"

echo
log INFO "Script analysis completed"
