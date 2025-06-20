#!/bin/bash
# Link Campaign CI Script
# Created: June 17, 2025
# Purpose: Run link checks automatically in CI/CD

set -e

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Documentation Link Validation${NC}"
echo -e "${BLUE}==========================${NC}"

# Check if python is available
if ! command -v python3 &>/dev/null; then
    echo -e "${RED}Error: Python 3 is required but not found${NC}"
    exit 1
fi

# Step 1: Run basic link checking
echo -e "\n${BLUE}Step 1: Basic link checking${NC}"
python3 "${SCRIPT_DIR}/link_checker.py"
BASIC_CHECK_STATUS=$?

# Step 2: Run comprehensive link campaign
echo -e "\n${BLUE}Step 2: Comprehensive link analysis${NC}"
python3 "${SCRIPT_DIR}/link_campaign.py" --report

# Step 3: Validate GitHub Pages structure
echo -e "\n${BLUE}Step 3: GitHub Pages validation${NC}"
"${SCRIPT_DIR}/validate-gh-pages.sh"
GHPAGES_STATUS=$?

# Step 4: Summary
echo -e "\n${BLUE}Documentation Link Validation Summary${NC}"
echo -e "${BLUE}=================================${NC}"

if [ $BASIC_CHECK_STATUS -eq 0 ] && [ $GHPAGES_STATUS -eq 0 ]; then
    echo -e "${GREEN}✓ All critical link checks passed${NC}"
    exit 0
else
    echo -e "${RED}✗ Link validation failed${NC}"
    echo -e "${YELLOW}Please review the detailed reports and fix the issues${NC}"
    exit 1
fi
