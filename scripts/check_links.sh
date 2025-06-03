#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Link Checker
# This script checks for broken links in the Anya Core documentation

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if markdown-link-check is installed
if ! command -v markdown-link-check &> /dev/null; then
    echo -e "${YELLOW}markdown-link-check not found. Installing...${NC}"
    npm install -g markdown-link-check
fi

# Check if fd is installed (for finding markdown files)
if ! command -v fd &> /dev/null; then
    echo -e "${YELLOW}fd (find command) not found. Installing...${NC}
    Please install fd (https://github.com/sharkdp/fd) and try again."
    exit 1
fi

# Configuration
DOCS_DIR="$(pwd)/docs"
CONFIG_FILE="$(pwd)/.markdown-link-check.json"

# Create config file if it doesn't exist
if [ ! -f "$CONFIG_FILE" ]; then
    cat > "$CONFIG_FILE" <<EOL
{
    "ignorePatterns": [
        "^https?://localhost",
        "^https?://127.0.0.1",
        "^#",
        "^mailto:",
        "^ftp:",
        "^/",
        ".git"
    ],
    "replacementPatterns": [
        {
            "pattern": "^/",
            "replacement": "file://$(pwd)/"
        },
        {
            "pattern": "^\\.\\./",
            "replacement": "file://$(pwd)/"
        }
    ]
}
EOL
    echo -e "${GREEN}✓ Created link check config: $CONFIG_FILE${NC}"
fi

# Function to check links in a single file
check_links_in_file() {
    local file="$1"
    local relative_path="${file#$PWD/}"
    
    echo -e "${YELLOW}🔍 Checking links in: $relative_path${NC}"
    
    # Run markdown-link-check
    if ! markdown-link-check -c "$CONFIG_FILE" "$file"; then
        echo -e "${RED}❌ Error checking links in: $relative_path${NC}"
        return 1
    fi
    
    return 0
}

# Main function
main() {
    local has_errors=0
    local processed=0
    
    # Find all markdown files in the docs directory
    while IFS= read -r -d '' file; do
        check_links_in_file "$file" || has_errors=1
        ((processed++))
    done < <(fd -e md -e markdown -0 "$DOCS_DIR")
    
    # Also check the root README.md
    if [ -f "README.md" ]; then
        check_links_in_file "README.md" || has_errors=1
        ((processed++))
    fi
    
    # Print summary
    echo -e "\n${GREEN}✅ Checked $processed files${NC}"
    
    if [ $has_errors -eq 0 ]; then
        echo -e "${GREEN}✅ No broken links found!${NC}"
        return 0
    else
        echo -e "${RED}❌ Some broken links were found. Please fix them.${NC}"
        return 1
    fi
}

# Run the main function
main

# Exit with appropriate status
exit $?
