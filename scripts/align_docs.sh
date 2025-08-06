#!/bin/bash

# Documentation Alignment Script
# Aligns Rust code documentation with Markdown documentation

# Don't exit on errors to make the script more resilient
set -uo pipefail

SRC_ROOT="/workspaces/Anya-core/src"
DOCS_ROOT="/workspaces/Anya-core/docs"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
REPORT_FILE="/workspaces/Anya-core/docs_alignment_report.md"
TEMP_RUST_DOCS="/tmp/anya_rust_docs.md"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Display banner
echo -e "${BLUE}╔═════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     ANYA CORE DOCUMENTATION ALIGNMENT SYSTEM            ║${NC}"
echo -e "${BLUE}╚═════════════════════════════════════════════════════════╝${NC}"

# Initialize report
init_report() {
    cat > "$REPORT_FILE" << EOF
# Documentation Alignment Report

**Generated:** $TIMESTAMP

## Summary

EOF
}

# Extract doc comments from Rust files
extract_rust_docs() {
    local module="$1"
    local module_path="${SRC_ROOT}/${module}"
    local docs_path="${DOCS_ROOT}/reference/${module}"

    echo -e "\n${BLUE}Extracting documentation from:${NC} $module"

    # Create reference directory if it doesn't exist
    mkdir -p "$docs_path"

    # Find mod.rs or lib.rs
    local mod_file=""
    if [[ -f "${module_path}/mod.rs" ]]; then
        mod_file="${module_path}/mod.rs"
    elif [[ -f "${module_path}" && $(basename "${module_path}") == *".rs" ]]; then
        mod_file="${module_path}"
    else
        echo -e "${YELLOW}⚠${NC} No module file found for: $module"
        return
    fi

    # Extract doc comments
    echo -e "\n${BLUE}Extracting from:${NC} $mod_file"

    # Create temporary file for extracted docs
    local temp_file="${TEMP_RUST_DOCS}_${module//\//_}"

    # Extract module-level doc comments
    if grep -q '^//!' "$mod_file"; then
        grep -A 100 '^//!' "$mod_file" |
            grep '^//!' |
            sed 's/^\/\/\![ ]*//' > "$temp_file"
    else
        echo "// No documentation comments found" > "$temp_file"
        echo -e "${YELLOW}⚠${NC} No doc comments found in: $mod_file"
    fi

    # Create reference markdown file
    local ref_md_file="${docs_path}/index.md"

    if [[ ! -f "$ref_md_file" || $(stat -c %s "$ref_md_file") -eq 0 ]]; then
        # Create new file with proper frontmatter
        cat > "$ref_md_file" << EOF
---
title: "$(echo "$module" | sed 's/\//-/g') API Reference"
description: "API reference for the $(echo "$module" | sed 's/\//-/g') module"
category: "reference"
tags: ["api", "reference", "$(echo "$module" | sed 's/\//-/g')"]
last_updated: "$(date '+%Y-%m-%d')"
---

# $(echo "$module" | sed 's/\//-/g' | sed 's/^./\U&/g') Module

EOF

        # Append extracted doc comments
        cat "$temp_file" >> "$ref_md_file"

        # Add table of contents
        cat >> "$ref_md_file" << EOF

## Table of Contents

- [Overview](#overview)
- [Functions](#functions)
- [Types](#types)
- [Usage](#usage)

## Functions

*This section is auto-generated from Rust doc comments*

EOF

        echo -e "${GREEN}✓${NC} Created reference doc: $ref_md_file"
        echo "- ✓ Created reference doc: $ref_md_file" >> "$REPORT_FILE"
    else
        echo -e "${YELLOW}⚠${NC} Reference file already exists: $ref_md_file"
        echo "- ⚠ Reference file already exists: $ref_md_file" >> "$REPORT_FILE"
    fi

    # Clean up temp file
    rm -f "$temp_file"
}

# Find all Rust module paths
find_modules() {
    echo -e "\n${BLUE}Finding Rust modules...${NC}"
    echo -e "\n## Rust Modules Found\n" >> "$REPORT_FILE"

    # Find direct modules in src/ directory
    for file in $(find "$SRC_ROOT" -maxdepth 1 -type f -name "*.rs" ! -name "lib.rs" ! -name "main.rs"); do
        local module=$(basename "$file" .rs)
        echo "- Module: $module"
        echo "- Module: $module" >> "$REPORT_FILE"
        extract_rust_docs "$module"
    done

    # Find directories with mod.rs
    for dir in $(find "$SRC_ROOT" -type d); do
        # Skip src root
        if [[ "$dir" == "$SRC_ROOT" ]]; then
            continue
        fi

        # Check if mod.rs exists
        if [[ -f "$dir/mod.rs" ]]; then
            local module=${dir#"$SRC_ROOT/"}
            echo "- Module: $module"
            echo "- Module: $module" >> "$REPORT_FILE"
            extract_rust_docs "$module"
        fi
    done
}

# Generate cargo doc and ensure README.md files exist for each module
generate_cargo_doc_readmes() {
    echo -e "\n${BLUE}Generating module README.md files for cargo doc...${NC}"
    echo -e "\n## Module README Files\n" >> "$REPORT_FILE"

    for dir in $(find "$SRC_ROOT" -type d); do
        # Skip src root
        if [[ "$dir" == "$SRC_ROOT" ]]; then
            continue
        fi

        # Check if mod.rs exists
        if [[ -f "$dir/mod.rs" ]]; then
            local module=${dir#"$SRC_ROOT/"}
            local readme_file="$dir/README.md"

            if [[ ! -f "$readme_file" ]]; then
                # Extract module doc comments for the README
                local module_docs=$(grep -A 100 '^//!' "$dir/mod.rs" |
                    grep '^//!' |
                    sed 's/^\/\/\![ ]*//')

                # Create README.md file
                cat > "$readme_file" << EOF
# ${module} Module

$(echo "$module_docs" || echo "Documentation for the ${module} module.")

## Overview

This module is part of the Anya Core system.

## For more information

See the comprehensive documentation in the [docs/](../../../docs/) directory.
EOF

                echo -e "${GREEN}✓${NC} Created README.md: $readme_file"
                echo "- ✓ Created README.md: $readme_file" >> "$REPORT_FILE"
            else
                echo -e "${YELLOW}⚠${NC} README.md already exists: $readme_file"
                echo "- ⚠ README.md already exists: $readme_file" >> "$REPORT_FILE"
            fi
        fi
    done
}

# Create API reference index
create_api_reference_index() {
    local api_index="${DOCS_ROOT}/reference/README.md"

    echo -e "\n${BLUE}Creating API reference index...${NC}"

    # Create reference directory if it doesn't exist
    mkdir -p "${DOCS_ROOT}/reference"

    # Create API reference index file
    cat > "$api_index" << EOF
---
title: "API Reference"
description: "API reference documentation for Anya Core"
category: "reference"
tags: ["api", "reference", "index"]
last_updated: "$(date '+%Y-%m-%d')"
---

# API Reference

This section contains API reference documentation for Anya Core modules.

## Table of Contents

- [Overview](#overview)
- [Modules](#modules)
- [Usage](#usage)

## Overview

The API reference documentation is generated from the Rust source code comments
and organized by module.

## Modules

EOF

    # Add module links
    for dir in $(find "$DOCS_ROOT/reference" -mindepth 1 -maxdepth 1 -type d | sort); do
        local module=$(basename "$dir")
        echo "- [${module}](./${module}/)" >> "$api_index"
    done

    # Add usage section
    cat >> "$api_index" << EOF

## Usage

To generate the complete API documentation with cargo:

```bash
cargo doc --open --document-private-items
```

EOF

    echo -e "${GREEN}✓${NC} Created API reference index: $api_index"
    echo "- ✓ Created API reference index: $api_index" >> "$REPORT_FILE"
}

# Generate final report summary
generate_summary() {
    local modules_processed=$(grep -c "^- Module:" "$REPORT_FILE")
    local files_created=$(grep -c "^- ✓ Created" "$REPORT_FILE")
    local files_skipped=$(grep -c "^- ⚠" "$REPORT_FILE")

    # Update report summary
    sed -i "s/^## Summary$/## Summary\n\n- Source code root: $SRC_ROOT\n- Documentation root: $DOCS_ROOT\n- Modules processed: $modules_processed\n- Reference files created: $files_created\n- Files skipped (already exist): $files_skipped/" "$REPORT_FILE"

    echo -e "\n${BLUE}Documentation alignment complete.${NC}"
    echo -e "Report saved to: $REPORT_FILE"
}

# Main execution

# Check if source directory exists
if [ ! -d "$SRC_ROOT" ]; then
    echo -e "${RED}Error: Source directory not found: $SRC_ROOT${NC}"
    echo -e "${YELLOW}Please ensure the source directory exists or update the SRC_ROOT variable.${NC}"
    exit 1
fi

# Check if docs directory exists
if [ ! -d "$DOCS_ROOT" ]; then
    echo -e "${RED}Error: Documentation directory not found: $DOCS_ROOT${NC}"
    echo -e "${YELLOW}Please ensure the documentation directory exists or update the DOCS_ROOT variable.${NC}"
    exit 1
fi

# Initialize report
init_report

# Find and process modules
find_modules

# Generate README.md files for cargo doc
generate_cargo_doc_readmes

# Create API reference index
create_api_reference_index

# Generate final summary
generate_summary

echo -e "\n${GREEN}Documentation alignment completed successfully!${NC}"
echo -e "See detailed report at: ${BLUE}$REPORT_FILE${NC}\n"
echo -e "${YELLOW}To generate complete API docs, run: ${BLUE}cargo doc --open --document-private-items${NC}\n"

exit 0
