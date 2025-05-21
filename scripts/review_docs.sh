#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Review Script
# This script performs a comprehensive review of all documentation files

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOCS_DIR="$(pwd)/docs"
REPORT_FILE="${DOCS_DIR}/REVIEW_REPORT_$(date +%Y%m%d_%H%M%S).md"
TEMP_FILE=$(mktemp)

# Initialize counters
TOTAL_FILES=0
GOOD_FILES=0
WARNING_FILES=0
ERROR_FILES=0

# Header for the report
echo -e "# 📝 Documentation Review Report\n" > "$REPORT_FILE"
echo -e "**Generated:** $(date '+%Y-%m-%d %H:%M:%S')" >> "$REPORT_FILE"
echo -e "**Documentation Directory:** ${DOCS_DIR}\n" >> "$REPORT_FILE"

# Function to check AI labels
check_ai_labels() {
    local file="$1"
    local has_ai_labels=0
    
    if grep -q '\[AIR-3\]' "$file" && \
       grep -q '\[AIS-3\]' "$file" && \
       grep -q '\[BPC-3\]' "$file" && \
       grep -q '\[RES-3\]' "$file"; then
        has_ai_labels=1
    fi
    
    echo $has_ai_labels
}

# Function to check for required sections
check_required_sections() {
    local file="$1"
    local missing_sections=()
    
    # List of required sections
    local required_sections=(
        "## Overview"
        "## Table of Contents"
        "## See Also"
    )
    
    for section in "${required_sections[@]}"; do
        if ! grep -q "^${section}$" "$file"; then
            missing_sections+=("${section#*## }")
        fi
    done
    
    if [ ${#missing_sections[@]} -gt 0 ]; then
        echo "Missing sections: ${missing_sections[*]}"
    else
        echo "OK"
    fi
}

# Function to check file naming
check_file_naming() {
    local file="$1"
    local filename=$(basename "$file")
    
    if [[ "$filename" =~ [A-Z] ]]; then
        echo "Warning: File contains uppercase letters (use lowercase with hyphens)"
    elif [[ "$filename" =~ _ ]]; then
        echo "Warning: File contains underscores (use hyphens instead)"
    else
        echo "OK"
    fi
}

# Function to check for TODOs and FIXMEs
check_todos() {
    local file="$1"
    local todos=$(grep -i -E 'TODO|FIXME' "$file" | wc -l)
    
    if [ "$todos" -gt 0 ]; then
        echo "$todos TODOs/FIXMEs found"
    else
        echo "OK"
    fi
}

# Main function to process files
process_file() {
    local file="$1"
    local relative_path="${file#$PWD/}"
    local status=0
    
    echo -e "\n## ${BLUE}$relative_path${NC}" >> "$REPORT_FILE"
    
    # Check AI labels
    if [ $(check_ai_labels "$file") -eq 1 ]; then
        echo -e "- [x] AI Labels: ${GREEN}Present${NC}" >> "$REPORT_FILE"
    else
        echo -e "- [ ] AI Labels: ${RED}Missing${NC}" >> "$REPORT_FILE"
        status=1
    fi
    
    # Check required sections
    local sections_result=$(check_required_sections "$file")
    if [ "$sections_result" = "OK" ]; then
        echo -e "- [x] Required Sections: ${GREEN}Complete${NC}" >> "$REPORT_FILE"
    else
        echo -e "- [ ] Required Sections: ${YELLOW}$sections_result${NC}" >> "$REPORT_FILE"
        [ $status -eq 0 ] && status=2
    fi
    
    # Check file naming
    local naming_result=$(check_file_naming "$file")
    if [ "$naming_result" = "OK" ]; then
        echo -e "- [x] File Naming: ${GREEN}OK${NC}" >> "$REPORT_FILE"
    else
        echo -e "- [ ] File Naming: ${YELLOW}$naming_result${NC}" >> "$REPORT_FILE"
        [ $status -eq 0 ] && status=2
    fi
    
    # Check TODOs
    local todos_result=$(check_todos "$file")
    if [ "$todos_result" = "OK" ]; then
        echo -e "- [x] TODOs/FIXMEs: ${GREEN}None${NC}" >> "$REPORT_FILE"
    else
        echo -e "- [ ] TODOs/FIXMEs: ${YELLOW}$todos_result${NC}" >> "$REPORT_FILE"
        [ $status -eq 0 ] && status=2
    fi
    
    # Update counters
    ((TOTAL_FILES++))
    case $status in
        0) ((GOOD_FILES++)) ;;
        1) ((ERROR_FILES++)) ;;
        2) ((WARNING_FILES++)) ;;
    esac
    
    return $status
}

# Find and process all markdown files
echo -e "${BLUE}🔍 Scanning documentation files...${NC}"
find "$DOCS_DIR" -name "*.md" -type f | sort | while read -r file; do
    process_file "$file"
done

# Generate summary
echo -e "\n## 📊 Summary\n" >> "$REPORT_FILE"
echo -e "- **Total Files Reviewed:** $TOTAL_FILES" >> "$REPORT_FILE"
echo -e "- "✅ ${GREEN}Good:" $GOOD_FILES" >> "$REPORT_FILE"
echo -e "- "⚠️  ${YELLOW}Warnings:" $WARNING_FILES" >> "$REPORT_FILE"
echo -e "- "❌ ${RED}Errors:" $ERROR_FILES" >> "$REPORT_FILE"

# Print report location
echo -e "\n${GREEN}✅ Documentation review complete!${NC}"
echo -e "Report generated: ${REPORT_FILE}"

# Show summary
echo -e "\n${BLUE}--- Summary ---${NC}"
echo -e "✅ ${GREEN}Good: $GOOD_FILES"
echo -e "⚠️  ${YELLOW}Warnings: $WARNING_FILES"
echo -e "❌ ${RED}Errors: $ERROR_FILES${NC}"

# Exit with error if there are any errors
if [ $ERROR_FILES -gt 0 ]; then
    exit 1
fi

exit 0
