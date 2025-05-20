#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# script_cleanup.sh - Maintenance script for cleaning up temporary fix scripts
# Following Bitcoin Development Framework v2.5 standards
# Part of the Anya Core Hexagonal Architecture
# Date: 2025-05-20

set -e

# Error handling with line number reporting
function handle_error {
    echo "[ERROR] An error occurred at line $1"
    exit 1
}

trap 'handle_error $LINENO' ERR

# Define root directory
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[INFO] Starting script cleanup process according to Script Management Policy..."

# List of temporary fix scripts to be removed
TEMP_SCRIPTS=(
    "cleanup_script.sh"
    "final_cleanup.sh"
    "final_fix.sh"
    "fix_build_config.sh"
    "fix_error_trait.sh"
    "fix_remaining_issues.sh"
    "fix_unix_flag.sh"
    "fix_unused_variables.sh"
)

# Check and remove each temporary script
for script in "${TEMP_SCRIPTS[@]}"; do
    if [ -f "$script" ]; then
        echo "[INFO] Removing temporary script: $script"
        rm "$script"
    fi
done

# Check for any other temporary scripts with common prefixes
echo "[INFO] Checking for other temporary scripts..."
find . -maxdepth 1 -type f -name "fix_*.sh" -o -name "cleanup_*.sh" -o -name "final_*.sh" | while read -r temp_script; do
    echo "[INFO] Removing additional temporary script: $temp_script"
    rm "$temp_script"
done

echo "[INFO] Script cleanup completed successfully!"
echo "[INFO] All temporary fix scripts have been removed according to Script Management Policy."
echo "[INFO] Maintaining minimal script approach as per Bitcoin Development Framework v2.5 standards."

exit 0
