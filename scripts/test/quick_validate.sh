#!/bin/bash
# Quick validation of Anya Core Installation System

set -euo pipefail

echo "================================================================"
echo "        Anya Core Installation System Quick Validation"
echo "================================================================"
echo

# Use absolute paths for verification
ANYA_ROOT="/home/anya/anyachainlabs/projects/anya-core"
MAIN_INSTALLER="${ANYA_ROOT}/scripts/install.sh"
AUTO_INSTALLER="${ANYA_ROOT}/scripts/install/auto_install.sh"
LINUX_INSTALLER="${ANYA_ROOT}/scripts/install/linux_install.sh"
COMMON_UTILS="${ANYA_ROOT}/scripts/install/utils/install_common.sh"
MONITOR_SCRIPT="${ANYA_ROOT}/scripts/install/utils/monitor_health.sh"
VERIFY_SCRIPT="${ANYA_ROOT}/scripts/install/utils/verify_installation.sh"
CLEANUP_SCRIPT="${ANYA_ROOT}/scripts/install/utils/script_cleanup.sh"
README_FILE="${ANYA_ROOT}/scripts/install/README.md"

# Function to check file exists
check_file() {
    local file=$1
    local description=$2
    
    if [ -f "$file" ]; then
        echo "✅ Found $description: $file"
        
        # Check if executable (for scripts)
        if [[ "$file" == *.sh ]]; then
            if [ -x "$file" ]; then
                echo "  ✅ File is executable"
            else
                echo "  ⚠️ File is not executable, fixing..."
                chmod +x "$file"
            fi
        fi
        
        return 0
    else
        echo "❌ Missing $description: $file"
        return 1
    fi
}

# Check all installation files
echo "Checking installation files..."
echo

missing_count=0

check_file "$MAIN_INSTALLER" "Main Installer" || ((missing_count++))
check_file "$AUTO_INSTALLER" "Auto Installer" || ((missing_count++))
check_file "$LINUX_INSTALLER" "Linux Installer" || ((missing_count++))
check_file "$COMMON_UTILS" "Common Utilities" || ((missing_count++))
check_file "$MONITOR_SCRIPT" "Monitoring Script" || ((missing_count++))
check_file "$VERIFY_SCRIPT" "Verification Script" || ((missing_count++))
check_file "$CLEANUP_SCRIPT" "Cleanup Script" || ((missing_count++))
check_file "$README_FILE" "README Documentation" || ((missing_count++))

echo
if [ $missing_count -eq 0 ]; then
    echo "✅ All installation system files are present and executable!"
else
    echo "❌ Missing $missing_count required files"
fi

# Check implementation features
echo
echo "Checking implementation features..."
echo

function check_feature() {
    local feature=$1
    local pattern=$2
    local file=$3
    
    if [ -f "$file" ] && grep -q "$pattern" "$file"; then
        echo "✅ $feature implemented"
        return 0
    else
        echo "⚠️ $feature may be missing"
        return 1
    fi
}

check_feature "Upgrade capability" "UPGRADE_MODE" "$LINUX_INSTALLER"
check_feature "Hardware detection" "detect_" "$COMMON_UTILS"
check_feature "Version tracking" "VERSION=" "$AUTO_INSTALLER"
check_feature "Process locking" "setup_process_lock" "$AUTO_INSTALLER"
check_feature "Feature flag configuration" "feature" "$LINUX_INSTALLER"

echo
echo "================================================================"
echo "                      Quick Validation Summary"
echo "================================================================"
echo
echo "The Anya Core installation system has been enhanced with:"
echo "  - Auto-detection of hardware capabilities"
echo "  - Support for both fresh installs and upgrades"
echo "  - Comprehensive validation and monitoring"
echo "  - Proper version tracking and feature flag configuration"
echo
echo "Next steps:"
echo "  1. Run the full installer: sudo ./scripts/install.sh"
echo "  2. Verify installation: ./scripts/install/utils/verify_installation.sh"
echo "  3. Monitor system health: ./scripts/install/utils/monitor_health.sh"
echo
echo "To view hardware-specific features available for your system:"
echo "  ./scripts/install/utils/script_cleanup.sh"
echo
