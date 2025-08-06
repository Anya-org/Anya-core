#!/bin/bash

# Comprehensive Documentation Validation Script
# Validates documentation structure, frontmatter, and links

set -euo pipefail

DOCS_ROOT="/workspaces/Anya-core/docs"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
REPORT_FILE="/workspaces/Anya-core/docs_validation_report.md"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Display banner
echo -e "${BLUE}╔═════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║        ANYA CORE DOCUMENTATION VALIDATION SYSTEM        ║${NC}"
echo -e "${BLUE}╚═════════════════════════════════════════════════════════╝${NC}"

# Initialize report
init_report() {
    cat > "$REPORT_FILE" << EOF
# Documentation Validation Report

**Generated:** $TIMESTAMP

## Summary

EOF
}

# Validate frontmatter in markdown files
validate_frontmatter() {
    local file="$1"
    local valid=true
    local error_msg=""

    # Check if file starts with frontmatter delimiter
    if ! head -1 "$file" | grep -q "^---$"; then
        valid=false
        error_msg="Missing frontmatter delimiter"
    else
        # Check for required frontmatter fields
        if ! grep -q "^title:" "$file"; then
            valid=false
            error_msg+="Missing title field. "
        fi

        if ! grep -q "^description:" "$file"; then
            valid=false
            error_msg+="Missing description field. "
        fi
    fi

    if $valid; then
        echo -e "${GREEN}✓${NC} Frontmatter: $file"
        echo "- ✓ $file: Frontmatter valid" >> "$REPORT_FILE"
    else
        echo -e "${RED}✗${NC} Frontmatter: $file - $error_msg"
        echo "- ✗ $file: $error_msg" >> "$REPORT_FILE"
    fi

    return $([ "$valid" == "true" ] && echo 0 || echo 1)
}

# Validate internal links in markdown files
validate_links() {
    local file="$1"
    local valid=true
    local error_count=0

    # Extract all internal markdown links
    local links=$(grep -o "\[.*\](\./.*\.md)" "$file" | sed 's/.*(\(.*\))/\1/')

    # Check each link
    for link in $links; do
        # Convert relative path to absolute
        local dir=$(dirname "$file")
        local target="$dir/$(echo "$link" | sed 's|\./||')"

        if [ ! -f "$target" ]; then
            valid=false
            error_count=$((error_count + 1))
            echo -e "${RED}✗${NC} Broken link in $file: $link"
            echo "- ✗ $file: Broken link to $link" >> "$REPORT_FILE"
        fi
    done

    if $valid; then
        echo -e "${GREEN}✓${NC} Links: $file"
        echo "- ✓ $file: All links valid" >> "$REPORT_FILE"
    else
        echo -e "${RED}✗${NC} Links: $file - $error_count broken links"
    fi

    return $([ "$valid" == "true" ] && echo 0 || echo 1)
}

# Check if file follows naming conventions
validate_naming_convention() {
    local file="$1"
    local filename=$(basename "$file")
    local valid=true
    local error_msg=""

    # Check kebab-case for filenames
    if [[ "$filename" =~ [A-Z] || "$filename" =~ [_] ]]; then
        valid=false
        error_msg="Filename should use kebab-case (lowercase with hyphens)"
    fi

    if $valid; then
        echo -e "${GREEN}✓${NC} Naming: $file"
        echo "- ✓ $file: Naming convention followed" >> "$REPORT_FILE"
    else
        echo -e "${RED}✗${NC} Naming: $file - $error_msg"
        echo "- ✗ $file: $error_msg" >> "$REPORT_FILE"
    fi

    return $([ "$valid" == "true" ] && echo 0 || echo 1)
}

# Validate table of contents
validate_toc() {
    local file="$1"
    local valid=true

    # Check if file has a table of contents section
    if grep -q "^## Table of Contents" "$file" || grep -q "^## Contents" "$file"; then
        echo -e "${GREEN}✓${NC} TOC: $file"
        echo "- ✓ $file: Has table of contents" >> "$REPORT_FILE"
    else
        echo -e "${YELLOW}⚠${NC} TOC: $file - Missing table of contents"
        echo "- ⚠ $file: Missing table of contents" >> "$REPORT_FILE"
        valid=false
    fi

    return $([ "$valid" == "true" ] && echo 0 || echo 1)
}

# Process a single markdown file
process_file() {
    local file="$1"
    local errors=0

    echo -e "\n${BLUE}Processing:${NC} $file"

    # Run validation checks
    validate_frontmatter "$file" || errors=$((errors + 1))
    validate_links "$file" || errors=$((errors + 1))
    validate_naming_convention "$file" || errors=$((errors + 1))
    validate_toc "$file" || errors=$((errors + 1))

    # Return error count
    return $errors
}

# Process all markdown files in a directory
process_directory() {
    local dir="$1"
    local total_files=0
    local valid_files=0

    echo -e "\n${BLUE}Scanning directory:${NC} $dir"
    echo -e "\n## Directory: $dir\n" >> "$REPORT_FILE"

    # Find all markdown files
    while IFS= read -r file; do
        total_files=$((total_files + 1))
        process_file "$file"
        if [ $? -eq 0 ]; then
            valid_files=$((valid_files + 1))
        fi
    done < <(find "$dir" -type f -name "*.md")

    # Print summary
    echo -e "\n${BLUE}Directory Summary:${NC} $dir"
    echo -e "Files checked: $total_files"
    echo -e "Valid files: $valid_files"
    echo -e "Invalid files: $((total_files - valid_files))"

    # Update report
    cat >> "$REPORT_FILE" << EOF

### Directory Summary: $dir
- Files checked: $total_files
- Valid files: $valid_files
- Invalid files: $((total_files - valid_files))
- Compliance rate: $(( (valid_files * 100) / (total_files > 0 ? total_files : 1) ))%

EOF
}

# Validate structure of documentation directories
validate_structure() {
    local required_dirs=(
        "getting-started"
        "guides"
        "reference"
        "architecture"
        "contributing"
        "specialized"
    )

    echo -e "\n${BLUE}Validating documentation structure...${NC}"
    echo -e "\n## Structure Validation\n" >> "$REPORT_FILE"

    local missing=0

    for dir in "${required_dirs[@]}"; do
        if [ -d "$DOCS_ROOT/$dir" ]; then
            echo -e "${GREEN}✓${NC} Directory exists: $dir"
            echo "- ✓ Directory exists: $dir" >> "$REPORT_FILE"
        else
            echo -e "${RED}✗${NC} Missing directory: $dir"
            echo "- ✗ Missing directory: $dir" >> "$REPORT_FILE"
            missing=$((missing + 1))
        fi
    done

    if [ $missing -eq 0 ]; then
        echo -e "\n${GREEN}Documentation structure is compliant with standards.${NC}"
        echo -e "\nDocumentation structure is compliant with standards." >> "$REPORT_FILE"
    else
        echo -e "\n${RED}Documentation structure is missing $missing required directories.${NC}"
        echo -e "\nDocumentation structure is missing $missing required directories." >> "$REPORT_FILE"
    fi
}

# Generate final report summary
generate_summary() {
    local total_files=0
    local valid_files=0

    while IFS= read -r file; do
        total_files=$((total_files + 1))
    done < <(find "$DOCS_ROOT" -type f -name "*.md")

    valid_files=$(grep -c "^- ✓" "$REPORT_FILE")
    invalid_files=$(grep -c "^- ✗" "$REPORT_FILE")
    warnings=$(grep -c "^- ⚠" "$REPORT_FILE")

    # Update report summary
    sed -i "s/^## Summary$/## Summary\n\n- Total files checked: $total_files\n- Valid files: $valid_files\n- Invalid files: $invalid_files\n- Warnings: $warnings\n- Overall compliance: $(( (valid_files * 100) / (total_files > 0 ? total_files : 1) ))%/" "$REPORT_FILE"

    echo -e "\n${BLUE}Validation complete.${NC}"
    echo -e "Report saved to: $REPORT_FILE"
}

# Main execution

# Check if docs directory exists
if [ ! -d "$DOCS_ROOT" ]; then
    echo -e "${RED}Error: Documentation directory not found: $DOCS_ROOT${NC}"
    echo -e "${YELLOW}Please ensure the documentation directory exists or update the DOCS_ROOT variable.${NC}"
    exit 1
fi

# Initialize report
init_report

# Validate structure
validate_structure

# Process all documentation directories
for dir in $(find "$DOCS_ROOT" -type d | sort); do
    # Skip the root directory itself
    if [ "$dir" == "$DOCS_ROOT" ]; then
        continue
    fi

    # Process the directory
    process_directory "$dir"
done

# Generate final summary
generate_summary

echo -e "\n${GREEN}Documentation validation completed successfully!${NC}"
echo -e "See detailed report at: ${BLUE}$REPORT_FILE${NC}\n"

exit 0

