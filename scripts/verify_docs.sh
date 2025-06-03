#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Verification Script
# This script verifies that all documentation files follow the project standards

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Required AI labels
REQUIRED_LABELS=(
    "\[AIR-3\]"
    "\[AIS-3\]"
    "\[BPC-3\]"
    "\[RES-3\]"
)

# Files to check
DOC_FILES=(
    "README.md"
    "CHANGELOG.md"
    "CONTRIBUTING.md"
    "docs/"
)

# Check if a file has all required AI labels
check_ai_labels() {
    local file=$1
    local missing_labels=()
    
    for label in "${REQUIRED_LABELS[@]}"; do
        if ! grep -q "$label" "$file"; then
            missing_labels+=("$label")
        fi
    done
    
    if [ ${#missing_labels[@]} -gt 0 ]; then
        echo -e "${RED}✗ Missing AI labels in $file: ${missing_labels[*]}${NC}"
        return 1
    else
        echo -e "${GREEN}✓ All AI labels present in $file${NC}"
        return 0
    fi
}

# Check markdown formatting
check_markdown() {
    local file=$1
    local has_errors=0
    
    # Check for trailing spaces
    if grep -q ' $' "$file"; then
        echo -e "${YELLOW}⚠  Trailing spaces found in $file${NC}"
        has_errors=1
    fi
    
    # Check for proper line endings
    if file "$file" | grep -q 'with CRLF line terminators'; then
        echo -e "${YELLOW}⚠  CRLF line endings found in $file (use LF)${NC}"
        has_errors=1
    fi
    
    # Check for proper YAML front matter in markdown files
    if [[ "$file" == *.md ]] && ! head -n 1 "$file" | grep -q '^---$'; then
        echo -e "${YELLOW}⚠  Missing YAML front matter in $file${NC}"
        has_errors=1
    fi
    
    if [ $has_errors -eq 0 ]; then
        echo -e "${GREEN}✓ Markdown formatting in $file is good${NC}"
    fi
    
    return $has_errors
}

# Main verification function
verify_docs() {
    local has_errors=0
    
    echo -e "${YELLOW}🚀 Verifying documentation files...${NC}\n"
    
    # Process each file or directory
    for item in "${DOC_FILES[@]}"; do
        if [ -d "$item" ]; then
            # Process all markdown files in directory
            while IFS= read -r -d '' file; do
                check_ai_labels "$file" || has_errors=1
                check_markdown "$file" || has_errors=1
                echo ""
            done < <(find "$item" -name '*.md' -print0)
        elif [ -f "$item" ]; then
            check_ai_labels "$item" || has_errors=1
            check_markdown "$item" || has_errors=1
            echo ""
        fi
    done
    
    if [ $has_errors -eq 0 ]; then
        echo -e "${GREEN}✅ All documentation files passed verification!${NC}"
        return 0
    else
        echo -e "${RED}❌ Some documentation files have issues that need to be addressed.${NC}"
        return 1
    fi
}

# Run the verification
verify_docs

# Exit with appropriate status
exit $?
