#!/bin/bash
# Documentation Cleanup Completion Script
# Created: June 17, 2025
# Purpose: Run all documentation cleanup, validation, and review scripts in sequence

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
SCRIPTS_DIR="${WORKSPACE_ROOT}/scripts"

echo -e "${BLUE}Documentation Cleanup Completion Script${NC}"
echo -e "${BLUE}=====================================${NC}"
echo -e "${BLUE}This script will run all documentation cleanup, validation, and review scripts in sequence.${NC}"
echo -e "${YELLOW}Ensure you have committed any important changes before running this script.${NC}"

# Confirm before proceeding
read -p "Do you want to proceed with the documentation cleanup? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Cleanup aborted.${NC}"
    exit 0
fi

# Make all scripts executable
echo -e "\n${BLUE}Making all documentation scripts executable...${NC}"
chmod +x "${SCRIPTS_DIR}/cleanup-docs.sh" "${SCRIPTS_DIR}/validate-gh-pages.sh" "${SCRIPTS_DIR}/cleanup-gh-pages-test.sh" "${SCRIPTS_DIR}/final-documentation-review.sh"
echo -e "${GREEN}All scripts are now executable!${NC}"

# Step 1: Run the documentation cleanup script
echo -e "\n${BLUE}Step 1: Running documentation cleanup script...${NC}"
"${SCRIPTS_DIR}/cleanup-docs.sh"
echo -e "${GREEN}Documentation cleanup completed!${NC}"

# Step 2: Validate GitHub Pages
echo -e "\n${BLUE}Step 2: Validating GitHub Pages...${NC}"
"${SCRIPTS_DIR}/validate-gh-pages.sh"
echo -e "${GREEN}GitHub Pages validation completed!${NC}"

# Step 3: Remove the temporary GitHub Pages test file
echo -e "\n${BLUE}Step 3: Removing temporary GitHub Pages test file...${NC}"
"${SCRIPTS_DIR}/cleanup-gh-pages-test.sh"
echo -e "${GREEN}Temporary GitHub Pages test file removed!${NC}"

# Step 4: Perform a final comprehensive documentation review
echo -e "\n${BLUE}Step 4: Performing final documentation review...${NC}"
"${SCRIPTS_DIR}/final-documentation-review.sh"
echo -e "${GREEN}Final documentation review completed!${NC}"

# Summary
echo -e "\n${GREEN}Documentation cleanup process completed successfully!${NC}"
echo -e "${BLUE}Next steps:${NC}"
echo -e "1. Review any issues highlighted during the process"
echo -e "2. Commit and push the changes to the repository"
echo -e "3. Verify GitHub Pages builds correctly"
echo -e "4. Submit a pull request to merge the changes into the main branch"

# Display the status of the repository
echo -e "\n${BLUE}Repository status:${NC}"
cd "${WORKSPACE_ROOT}" && git status
