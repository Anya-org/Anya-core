#!/bin/bash

# Documentation Standardization Script
# Applies naming conventions and structural standards to documentation

set -euo pipefail

DOCS_ROOT="/workspaces/Anya-core/docs"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
REPORT_FILE="/workspaces/Anya-core/docs_standardization_report.md"
TEMPLATE_FILE="/workspaces/Anya-core/docs/template.md"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Display banner
echo -e "${BLUE}╔═════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║        ANYA CORE DOCUMENTATION STANDARDIZATION          ║${NC}"
echo -e "${BLUE}╚═════════════════════════════════════════════════════════╝${NC}"

# Initialize report
init_report() {
    cat > "$REPORT_FILE" << EOF
# Documentation Standardization Report

**Generated:** $TIMESTAMP

## Summary

EOF
}

# Create or update documentation template
create_template() {
    echo -e "\n${BLUE}Creating documentation template...${NC}"

    cat > "$TEMPLATE_FILE" << EOF
---
title: "Document Title"
description: "Brief description of the document's purpose"
category: "guide|reference|architecture"
tags: ["tag1", "tag2"]
last_updated: "$(date '+%Y-%m-%d')"
compliance: ["AIR-3", "AIS-3", "BPC-3"]
---

# Document Title

## Overview
Brief overview of the document's purpose and scope.

## Table of Contents
- [Overview](#overview)
- [Section 1](#section-1)
- [Section 2](#section-2)
- [See Also](#see-also)

## Section 1
Content for section 1.

## Section 2
Content for section 2.

## See Also
- [Related Document](../path/to/doc.md)
- [External Reference](https://example.com)
EOF

    echo -e "${GREEN}✓${NC} Template created at: $TEMPLATE_FILE"
    echo "- ✓ Documentation template created at: $TEMPLATE_FILE" >> "$REPORT_FILE"
}

# Create base directory structure
create_structure() {
    echo -e "\n${BLUE}Creating documentation structure...${NC}"
    echo -e "\n## Directory Structure Creation\n" >> "$REPORT_FILE"

    local directories=(
        "getting-started"
        "guides"
        "reference"
        "architecture"
        "contributing"
        "specialized"
        "specialized/dao"
        "specialized/dependencies"
        "specialized/tools"
    )

    for dir in "${directories[@]}"; do
        if [ -d "$DOCS_ROOT/$dir" ]; then
            echo -e "${GREEN}✓${NC} Directory already exists: $dir"
            echo "- ✓ Directory already exists: $dir" >> "$REPORT_FILE"
        else
            mkdir -p "$DOCS_ROOT/$dir"
            echo -e "${GREEN}✓${NC} Created directory: $dir"
            echo "- ✓ Created directory: $dir" >> "$REPORT_FILE"

            # Create README.md in the directory
            cat > "$DOCS_ROOT/$dir/README.md" << EOF
---
title: "$(echo "$dir" | sed 's/specialized\///g' | sed 's/^./\u&/g' | sed 's/-/ /g') Documentation"
description: "$(echo "$dir" | sed 's/specialized\///g' | sed 's/^./\u&/g' | sed 's/-/ /g') documentation for Anya Core"
category: "index"
tags: ["index", "$(echo "$dir" | sed 's/specialized\///g')"]
last_updated: "$(date '+%Y-%m-%d')"
---

# $(echo "$dir" | sed 's/specialized\///g' | sed 's/^./\u&/g' | sed 's/-/ /g') Documentation

*This section contains documentation related to $(echo "$dir" | sed 's/specialized\///g' | sed 's/^./\u&/g' | sed 's/-/ /g').*

## Table of Contents

- [Overview](#overview)
- [Contents](#contents)

## Overview

Brief description of the $(echo "$dir" | sed 's/specialized\///g' | sed 's/^./\u&/g' | sed 's/-/ /g') documentation section.

## Contents

*No documents yet. This is a placeholder.*

EOF
            echo -e "${GREEN}✓${NC} Created README.md in: $dir"
            echo "- ✓ Created README.md in: $dir" >> "$REPORT_FILE"
        fi
    done
}

# Create naming conventions document
create_naming_conventions() {
    local file="$DOCS_ROOT/contributing/naming-conventions.md"

    echo -e "\n${BLUE}Creating naming conventions document...${NC}"

    mkdir -p "$(dirname "$file")"

    cat > "$file" << EOF
---
title: "Documentation Naming Conventions"
description: "Naming conventions for Anya Core documentation"
category: "contributing"
tags: ["standards", "documentation"]
last_updated: "$(date '+%Y-%m-%d')"
---

# Documentation Naming Conventions

This document defines the naming conventions for Anya Core documentation.

## Table of Contents
- [Overview](#overview)
- [File Naming](#file-naming)
- [Directory Naming](#directory-naming)
- [Link Conventions](#link-conventions)
- [Headings and Anchors](#headings-and-anchors)

## Overview

Consistent naming conventions ensure documentation is easy to navigate, reference, and maintain.

## File Naming

- Use **kebab-case** (lowercase with hyphens) for all markdown files
- Be descriptive but concise
- Use nouns or noun phrases
- Avoid acronyms unless widely recognized

**Examples:**
- ✅ \`getting-started.md\`
- ✅ \`api-reference.md\`
- ✅ \`bitcoin-integration.md\`
- ❌ \`GettingStarted.md\`
- ❌ \`getting_started.md\`
- ❌ \`getting started.md\`

## Directory Naming

- Use **lowercase** for all directories
- Use hyphens for multi-word directory names
- Prefer singular nouns for category directories

**Examples:**
- ✅ \`getting-started/\`
- ✅ \`reference/\`
- ✅ \`architecture/\`
- ❌ \`GettingStarted/\`
- ❌ \`getting_started/\`

## Link Conventions

- Use relative paths for internal links
- Include file extensions in links
- Use anchor links for sections within documents

**Examples:**
- ✅ \`[Getting Started](../getting-started/installation.md)\`
- ✅ \`[API Reference](./api-reference.md#authentication)\`
- ❌ \`[Getting Started](../getting-started/installation)\`
- ❌ \`[API Reference](https://github.com/Anya-org/Anya-core/docs/api-reference.md)\`

## Headings and Anchors

- Use sentence case for headings
- Create descriptive anchors for important sections

**Examples:**
- ✅ \`## Installation process\`
- ✅ \`## API authentication\`
- ❌ \`## INSTALLATION PROCESS\`
- ❌ \`## Installation Process\`

EOF

    echo -e "${GREEN}✓${NC} Created naming conventions document"
    echo "- ✓ Created naming conventions document: $file" >> "$REPORT_FILE"
}

# Fix filenames to match naming conventions
standardize_filenames() {
    echo -e "\n${BLUE}Standardizing filenames...${NC}"
    echo -e "\n## Filename Standardization\n" >> "$REPORT_FILE"

    local count=0

    # Find files with uppercase letters or underscores
    while IFS= read -r file; do
        local dir=$(dirname "$file")
        local filename=$(basename "$file")
        local new_filename=$(echo "$filename" | tr '[:upper:]' '[:lower:]' | tr '_' '-' | tr ' ' '-')

        # Skip if already standardized
        if [ "$filename" = "$new_filename" ]; then
            continue
        fi

        local new_file="$dir/$new_filename"

        # Rename the file
        mv "$file" "$new_file"
        count=$((count + 1))

        echo -e "${GREEN}✓${NC} Renamed: $filename → $new_filename"
        echo "- ✓ Renamed: $filename → $new_filename" >> "$REPORT_FILE"
    done < <(find "$DOCS_ROOT" -type f -name "*.md" | grep -v "README.md")

    echo -e "\nStandardized $count filenames."
    echo -e "\nStandardized $count filenames." >> "$REPORT_FILE"
}

# Add or update frontmatter in markdown files
standardize_frontmatter() {
    echo -e "\n${BLUE}Standardizing frontmatter...${NC}"
    echo -e "\n## Frontmatter Standardization\n" >> "$REPORT_FILE"

    local count=0

    # Find all markdown files
    while IFS= read -r file; do
        # Skip README files for now
        if [[ "$(basename "$file")" == "README.md" ]]; then
            continue
        fi

        local title=$(basename "$file" .md | sed 's/-/ /g' | sed 's/\b\(.\)/\u\1/g')
        local dir=$(dirname "$file" | sed "s|$DOCS_ROOT/||g")
        local category=$(echo "$dir" | cut -d'/' -f1)

        # Check if frontmatter exists
        if head -1 "$file" | grep -q "^---$"; then
            # Update existing frontmatter
            echo -e "${YELLOW}⚠${NC} Skipping existing frontmatter: $file"
            echo "- ⚠ Skipping existing frontmatter: $file" >> "$REPORT_FILE"
        else
            # Add frontmatter
            local temp_file=$(mktemp)

            cat > "$temp_file" << EOF
---
title: "$title"
description: "Documentation for $title in Anya Core"
category: "$category"
tags: ["$(echo "$dir" | tr '/' ' ')"]
last_updated: "$(date '+%Y-%m-%d')"
---

EOF
            cat "$file" >> "$temp_file"
            mv "$temp_file" "$file"
            count=$((count + 1))

            echo -e "${GREEN}✓${NC} Added frontmatter: $file"
            echo "- ✓ Added frontmatter: $file" >> "$REPORT_FILE"
        fi
    done < <(find "$DOCS_ROOT" -type f -name "*.md")

    echo -e "\nStandardized frontmatter in $count files."
    echo -e "\nStandardized frontmatter in $count files." >> "$REPORT_FILE"
}

# Update internal links to use correct paths
fix_internal_links() {
    echo -e "\n${BLUE}Fixing internal links...${NC}"
    echo -e "\n## Internal Link Fixes\n" >> "$REPORT_FILE"

    local count=0

    # Find all markdown files
    while IFS= read -r file; do
        # Create temp file for edits
        local temp_file=$(mktemp)

        # Fix links without .md extension
        sed -E 's/\[([^]]+)\]\(([^)]+)\/([^/)]+)\)/\[\1\]\(\2\/\3.md\)/g' "$file" > "$temp_file"

        # Check if any changes were made
        if ! diff -q "$file" "$temp_file" &>/dev/null; then
            mv "$temp_file" "$file"
            count=$((count + 1))
            echo -e "${GREEN}✓${NC} Fixed links in: $file"
            echo "- ✓ Fixed links in: $file" >> "$REPORT_FILE"
        else
            rm "$temp_file"
        fi
    done < <(find "$DOCS_ROOT" -type f -name "*.md")

    echo -e "\nFixed links in $count files."
    echo -e "\nFixed links in $count files." >> "$REPORT_FILE"
}

# Generate final report summary
generate_summary() {
    local total_files=0

    while IFS= read -r file; do
        total_files=$((total_files + 1))
    done < <(find "$DOCS_ROOT" -type f -name "*.md")

    # Update report summary
    sed -i "s/^## Summary$/## Summary\n\n- Documentation root: $DOCS_ROOT\n- Total files processed: $total_files\n- Template created: $([ -f "$TEMPLATE_FILE" ] && echo "Yes" || echo "No")\n- Directory structure created: $([ -d "$DOCS_ROOT/getting-started" ] && echo "Yes" || echo "No")/" "$REPORT_FILE"

    echo -e "\n${BLUE}Standardization complete.${NC}"
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

# Create documentation template
create_template

# Create/update directory structure
create_structure

# Create naming conventions document
create_naming_conventions

# Standardize filenames
standardize_filenames

# Standardize frontmatter
standardize_frontmatter

# Fix internal links
fix_internal_links

# Generate final summary
generate_summary

echo -e "\n${GREEN}Documentation standardization completed successfully!${NC}"
echo -e "See detailed report at: ${BLUE}$REPORT_FILE${NC}\n"

exit 0
