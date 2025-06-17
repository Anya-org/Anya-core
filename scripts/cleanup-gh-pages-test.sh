#!/bin/bash
# Cleanup Temporary GitHub Pages Test File
# Created: June 17, 2025
# Purpose: Remove the temporary test file created for GitHub Pages validation

# Set strict error handling
set -e

# Define colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Paths
WORKSPACE_ROOT="/workspaces/Anya-core"
TEST_FILE="${WORKSPACE_ROOT}/docs/gh-pages-test.md"

echo -e "${BLUE}Cleanup Temporary GitHub Pages Test File${NC}"
echo -e "${BLUE}=====================================${NC}"

# Check if the file exists
if [ -f "${TEST_FILE}" ]; then
    echo -e "${YELLOW}Temporary GitHub Pages test file found at ${TEST_FILE}${NC}"

    # Create a backup
    BACKUP_DIR="/tmp/anya-docs-backup-$(date +%Y%m%d-%H%M%S)"
    mkdir -p "${BACKUP_DIR}"
    echo -e "${YELLOW}Creating backup at ${BACKUP_DIR}${NC}"
    cp "${TEST_FILE}" "${BACKUP_DIR}/$(basename ${TEST_FILE})"

    # Remove the file
    echo -e "${YELLOW}Removing temporary GitHub Pages test file...${NC}"
    rm "${TEST_FILE}"

    echo -e "${GREEN}Temporary GitHub Pages test file successfully removed!${NC}"
    echo -e "${YELLOW}A backup was created at ${BACKUP_DIR}/$(basename ${TEST_FILE})${NC}"
else
    echo -e "${GREEN}No temporary GitHub Pages test file found. Nothing to do.${NC}"
fi

echo -e "\n${GREEN}GitHub Pages cleanup completed!${NC}"
