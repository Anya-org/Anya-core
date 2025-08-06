#!/bin/bash

# README.md Refactoring and Correction Script
# Fixes outdated and incomplete README files based on actual source code

set -euo pipefail

# Configuration
ANYA_ROOT="/workspaces/Anya-core"
REFACTOR_LOG="${ANYA_ROOT}/readme_refactor.log"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "README.md Refactoring Log - $TIMESTAMP" > "$REFACTOR_LOG"

echo -e "${BLUE}🔧 Starting README.md refactoring based on source code analysis...${NC}"

# Function to extract Rust API information
extract_rust_api() {
    local dir_path="$1"
    local api_info=""

    # Find all Rust files and extract public structs, enums, and functions
    find "$dir_path" -name "*.rs" -type f 2>/dev/null | while read -r rust_file; do
        if [[ -r "$rust_file" ]]; then
            # Extract public structs
            local structs=$(grep -n "^pub struct" "$rust_file" 2>/dev/null | head -10 || true)
            if [[ -n "$structs" ]]; then
                echo "**Structs:**"
                echo "$structs" | while IFS: read -r line_num content; do
                    local struct_name=$(echo "$content" | cut -d' ' -f3 | cut -d'<' -f1)
                    echo "- \`$struct_name\` - Line $line_num in $(basename "$rust_file")"
                done
                echo ""
            fi

            # Extract public functions
            local functions=$(grep -n "^pub fn" "$rust_file" 2>/dev/null | head -10 || true)
            if [[ -n "$functions" ]]; then
                echo "**Functions:**"
                echo "$functions" | while IFS: read -r line_num content; do
                    local func_name=$(echo "$content" | cut -d'(' -f1 | cut -d' ' -f3)
                    echo "- \`$func_name()\` - Line $line_num in $(basename "$rust_file")"
                done
                echo ""
            fi

            # Extract public enums
            local enums=$(grep -n "^pub enum" "$rust_file" 2>/dev/null | head -5 || true)
            if [[ -n "$enums" ]]; then
                echo "**Enums:**"
                echo "$enums" | while IFS: read -r line_num content; do
                    local enum_name=$(echo "$content" | cut -d' ' -f3 | cut -d'<' -f1)
                    echo "- \`$enum_name\` - Line $line_num in $(basename "$rust_file")"
                done
                echo ""
            fi
        fi
    done
}

# Function to extract JavaScript/TypeScript API information
extract_js_api() {
    local dir_path="$1"

    find "$dir_path" \( -name "*.js" -o -name "*.ts" \) -type f 2>/dev/null | while read -r js_file; do
        if [[ -r "$js_file" ]]; then
            # Extract exported functions
            local exports=$(grep -n "^export \|^function \|^class " "$js_file" 2>/dev/null | head -10 || true)
            if [[ -n "$exports" ]]; then
                echo "**JavaScript/TypeScript API:**"
                echo "$exports" | while IFS: read -r line_num content; do
                    local name=$(echo "$content" | sed 's/^export //' | cut -d'(' -f1 | cut -d' ' -f2)
                    echo "- \`$name\` - Line $line_num in $(basename "$js_file")"
                done
                echo ""
            fi
        fi
    done
}

# Function to refactor a single README
refactor_readme() {
    local readme_path="$1"
    local relative_path="${readme_path#$ANYA_ROOT/}"
    local dir_path=$(dirname "$readme_path")
    local module_name=$(basename "$dir_path")

    echo -e "${YELLOW}🔧 Refactoring: $relative_path${NC}"
    echo "Refactoring: $relative_path" >> "$REFACTOR_LOG"

    # Backup original
    cp "$readme_path" "$readme_path.backup"

    # Count source files
    local rust_files=$(find "$dir_path" -name "*.rs" -type f 2>/dev/null | wc -l)
    local js_files=$(find "$dir_path" -name "*.js" -o -name "*.ts" -type f 2>/dev/null | wc -l)
    local py_files=$(find "$dir_path" -name "*.py" -type f 2>/dev/null | wc -l)

    # Create enhanced README content
    local new_readme_content=""

    # Generate header
    new_readme_content+="# $module_name Module

*Auto-generated documentation based on source code analysis*
*Last updated: $TIMESTAMP*

## Overview

"

    # Add module description based on source files
    if [[ $rust_files -gt 0 ]]; then
        new_readme_content+="This module contains $rust_files Rust source files providing core functionality for the $module_name component of Anya Core.

"
    fi

    if [[ $js_files -gt 0 ]]; then
        new_readme_content+="This module includes $js_files JavaScript/TypeScript files for frontend/web functionality.

"
    fi

    if [[ $py_files -gt 0 ]]; then
        new_readme_content+="This module contains $py_files Python files for analysis, tooling, or integration purposes.

"
    fi

    # Add source file structure
    new_readme_content+="## Source Structure

"

    local all_files=$(find "$dir_path" -type f \( -name "*.rs" -o -name "*.js" -o -name "*.ts" -o -name "*.py" \) 2>/dev/null | head -20)
    if [[ -n "$all_files" ]]; then
        new_readme_content+="\`\`\`
Directory: $relative_path
├── Source Files:
"
        echo "$all_files" | while read -r file; do
            local rel_file="${file#$dir_path/}"
            new_readme_content+="│   ├── $rel_file
"
        done
        new_readme_content+="\`\`\`

"
    fi

    # Add API documentation if Rust files exist
    if [[ $rust_files -gt 0 ]]; then
        new_readme_content+="## API Documentation

"
        local api_info=$(extract_rust_api "$dir_path")
        if [[ -n "$api_info" ]]; then
            new_readme_content+="$api_info"
        else
            new_readme_content+="*API documentation will be generated from source code analysis*

"
        fi
    fi

    # Add JavaScript API if relevant
    if [[ $js_files -gt 0 ]]; then
        local js_api=$(extract_js_api "$dir_path")
        if [[ -n "$js_api" ]]; then
            new_readme_content+="$js_api"
        fi
    fi

    # Add usage examples section
    new_readme_content+="## Usage

\`\`\`rust
// Example usage for $module_name module
// TODO: Add specific examples based on actual API
\`\`\`

"

    # Add implementation notes
    new_readme_content+="## Implementation Notes

- Module location: \`$relative_path\`
- Rust files: $rust_files
- JavaScript/TypeScript files: $js_files
- Python files: $py_files
- Last analyzed: $TIMESTAMP

## Dependencies

This module may depend on other Anya Core modules. Check \`Cargo.toml\` or relevant configuration files for specific dependencies.

## Testing

Related tests can be found in:
- Unit tests: Check for \`#[cfg(test)]\` modules in Rust files
- Integration tests: Look for corresponding files in \`tests/\` directory

## Contributing

When contributing to this module:
1. Ensure all public APIs are documented
2. Add appropriate tests for new functionality
3. Update this README when adding new public interfaces
4. Follow the project's coding standards

---
*This README was automatically generated and enhanced based on source code analysis.*
*For the most up-to-date information, refer to the actual source code.*
"

    # Write the new content to file
    echo "$new_readme_content" > "$readme_path"

    echo "   ✅ Refactored successfully"
    echo "   📄 Original backed up as: $readme_path.backup"
}

# Priority 1: Fix outdated README files
echo -e "${RED}🚨 Fixing OUTDATED README files...${NC}"

outdated_readmes=(
    "/workspaces/Anya-core/dependencies/README.md"
    "/workspaces/Anya-core/scripts/README.md"
)

for readme in "${outdated_readmes[@]}"; do
    if [[ -f "$readme" ]]; then
        refactor_readme "$readme"
    fi
done

# Priority 2: Fix incomplete README files with source code
echo -e "${YELLOW}⚠️  Fixing INCOMPLETE README files...${NC}"

incomplete_readmes=(
    "/workspaces/Anya-core/consolidated/bitcoin/layer2/lightning/README.md"
    "/workspaces/Anya-core/consolidated/bitcoin/layer2/rgb/README.md"
    "/workspaces/Anya-core/core/src/enterprise/README.md"
    "/workspaces/Anya-core/core/src/ml/README.md"
    "/workspaces/Anya-core/core/src/protocol/README.md"
    "/workspaces/Anya-core/src/ml/agents/README.md"
)

for readme in "${incomplete_readmes[@]}"; do
    if [[ -f "$readme" ]]; then
        refactor_readme "$readme"
    fi
done

echo -e "${GREEN}✅ README refactoring completed!${NC}"
echo -e "${BLUE}📋 Log file: $REFACTOR_LOG${NC}"
echo -e "${YELLOW}💾 All original files backed up with .backup extension${NC}"

# Show summary
echo -e "${BLUE}📊 Refactoring Summary:${NC}"
echo -e "   🚨 Outdated files fixed: ${#outdated_readmes[@]}"
echo -e "   ⚠️  Incomplete files enhanced: ${#incomplete_readmes[@]}"
echo -e "   📄 Total files refactored: $((${#outdated_readmes[@]} + ${#incomplete_readmes[@]}))"
