#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# [AIR-3][AIS-3][AIT-3]
# Anya Core AI Label Enforcement Script
# This script checks and enforces the proper AI labeling across the codebase

set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Color codes for output
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color
BOLD="\033[1m"

# Label patterns to check for
REQUIRED_LABELS=("AIR" "AIS" "BPC")
EXTENDED_LABELS=("AIT" "AIM" "AIP" "AIE" "RES" "DAO" "SCL" "PFM" "W5C")

# Track statistics
TOTAL_FILES=0
LABELED_FILES=0
UNLABELED_FILES=0
PARTIALLY_LABELED_FILES=0
AUTO_FIXED_FILES=0

# Log file for results
LOG_DIR="${PROJECT_ROOT}/logs"
mkdir -p "$LOG_DIR"
LOG_FILE="${LOG_DIR}/ai_label_check_$(date +%Y%m%d-%H%M%S).log"

# Functions
log() {
    local level=$1
    local message=$2
    echo -e "[$level] $message"
    echo "[$level] $message" >> "$LOG_FILE"
}

check_file_labels() {
    local file=$1
    local has_all_required=true
    local has_some_required=false
    local missing_labels=()
    
    # Check for each required label
    for label in "${REQUIRED_LABELS[@]}"; do
        if ! grep -q "\[$label-[0-3]\]" "$file"; then
            has_all_required=false
            missing_labels+=("$label")
        else
            has_some_required=true
        fi
    done
    
    # Categorize the file
    if $has_all_required; then
        ((LABELED_FILES++))
        log "INFO" "✅ Fully labeled: $file"
    elif $has_some_required; then
        ((PARTIALLY_LABELED_FILES++))
        log "WARN" "⚠️ Partially labeled: $file (missing: ${missing_labels[*]})"
        
        # Try to auto-fix if it has some labels already
        if [[ "$AUTO_FIX" == "true" ]]; then
            auto_fix_labels "$file" "${missing_labels[@]}"
        fi
    else
        ((UNLABELED_FILES++))
        log "ERROR" "❌ Unlabeled: $file"
        
        # Try to auto-fix by adding all required labels
        if [[ "$AUTO_FIX" == "true" ]]; then
            auto_fix_labels "$file" "${REQUIRED_LABELS[@]}"
        fi
    fi
}

auto_fix_labels() {
    local file=$1
    shift
    local missing_labels=("$@")
    local file_type=$(basename "$file" | cut -d. -f2)
    local label_line=""
    local default_level=3
    
    # Create the label line to add
    for label in "${missing_labels[@]}"; do
        label_line+="[$label-$default_level]"
    done
    
    # Check if file already has some labels to append to
    if grep -q "\[[A-Z]\{2,3\}-[0-3]\]" "$file"; then
        # Find existing label line and append
        sed -i "s/\(\[[A-Z]\{2,3\}-[0-3]\]\)/\1$label_line/g" "$file"
    else
        # Add new label line based on file type
        case "$file_type" in
            rs)
                # For Rust files, add after first doc comment or at top
                if grep -q "^//!" "$file"; then
                    sed -i "s|^//!.*$|&\n//! $label_line|" "$file"
                else
                    sed -i "1i//! $label_line\n" "$file"
                fi
                ;;
            *)
                # For other files, add at the top with appropriate comment style
                sed -i "1i// $label_line\n" "$file"
                ;;
        esac
    fi
    
    log "INFO" "🔧 Auto-fixed labels in: $file"
    ((AUTO_FIXED_FILES++))
}

# Main execution
echo -e "${BOLD}Anya Core AI Label Enforcement${NC}"
echo -e "Checking AI labels compliance across the codebase..."
echo "Results will be logged to: $LOG_FILE"

# Parse arguments
AUTO_FIX=false
if [[ "$1" == "--auto-fix" ]]; then
    AUTO_FIX=true
    echo -e "${YELLOW}Auto-fix mode enabled. Missing labels will be added automatically.${NC}"
fi

# Find all source files
echo -e "${BLUE}Scanning source files...${NC}"

# Find Rust files and check labels
find "$PROJECT_ROOT/src" -name "*.rs" -type f | sort | while read -r file; do
    ((TOTAL_FILES++))
    check_file_labels "$file"
done

# Print summary
echo -e "\n${BOLD}${BLUE}Summary:${NC}"
echo -e "${GREEN}Fully labeled files:${NC} $LABELED_FILES"
echo -e "${YELLOW}Partially labeled files:${NC} $PARTIALLY_LABELED_FILES"
echo -e "${RED}Unlabeled files:${NC} $UNLABELED_FILES"
if [[ "$AUTO_FIX" == "true" ]]; then
    echo -e "${BLUE}Auto-fixed files:${NC} $AUTO_FIXED_FILES"
fi
echo -e "${BLUE}Total files scanned:${NC} $TOTAL_FILES"

# Store summary in log
echo "=== SUMMARY ===" >> "$LOG_FILE"
echo "Fully labeled files: $LABELED_FILES" >> "$LOG_FILE"
echo "Partially labeled files: $PARTIALLY_LABELED_FILES" >> "$LOG_FILE"
echo "Unlabeled files: $UNLABELED_FILES" >> "$LOG_FILE"
if [[ "$AUTO_FIX" == "true" ]]; then
    echo "Auto-fixed files: $AUTO_FIXED_FILES" >> "$LOG_FILE"
fi
echo "Total files scanned: $TOTAL_FILES" >> "$LOG_FILE"

# Exit with appropriate status
if [[ "$UNLABELED_FILES" -gt 0 || "$PARTIALLY_LABELED_FILES" -gt 0 ]]; then
    if [[ "$AUTO_FIX" == "true" ]]; then
        echo -e "${YELLOW}${BOLD}Some files were auto-fixed. Please review changes.${NC}"
        exit 0
    else
        echo -e "${RED}${BOLD}Some files are missing required AI labels.${NC}"
        echo -e "Run with --auto-fix to automatically add missing labels."
        exit 1
    fi
else
    echo -e "${GREEN}${BOLD}All files have proper AI labels!${NC}"
    exit 0
fi
