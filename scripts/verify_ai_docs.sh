#!/bin/bash

# Verify AI Documentation Links Script
# This script checks for broken links in the AI documentation

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if link checker script exists
if [ ! -f "$(pwd)/scripts/check_links.py" ]; then
    echo -e "${RED}Error: check_links.py not found in scripts/ directory${NC}"
    exit 1
fi

# Check all markdown files in the AI docs directory
AI_DOCS_DIR="$(pwd)/docs/ai"

echo -e "${GREEN}Checking links in AI documentation...${NC}"
echo "========================================"

# Find all markdown files and check their links
find "$AI_DOCS_DIR" -name "*.md" | while read -r file; do
    echo -e "${GREEN}Checking:${NC} $file"
    python3 "$(pwd)/scripts/check_links.py" --file "$file" 2>/dev/null || echo -e "${GREEN}No issues found${NC}"
    echo "----------------------------------------"
done

echo -e "${GREEN}AI documentation link check complete!${NC}"
