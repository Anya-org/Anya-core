#!/bin/bash
# GitHub Pages Validation Script
# Created: June 17, 2025
# Purpose: Validate GitHub Pages configuration and documentation structure

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
DOCS_DIR="${WORKSPACE_ROOT}/docs"
GITHUB_DIR="${WORKSPACE_ROOT}/.github"
WORKFLOWS_DIR="${GITHUB_DIR}/workflows"

echo -e "${BLUE}GitHub Pages Validation Script${NC}"
echo -e "${BLUE}=============================${NC}"

# Check if required directories exist
if [ ! -d "${DOCS_DIR}" ]; then
    echo -e "${RED}ERROR: Docs directory not found at ${DOCS_DIR}${NC}"
    exit 1
fi

if [ ! -d "${GITHUB_DIR}" ]; then
    echo -e "${RED}ERROR: .github directory not found at ${GITHUB_DIR}${NC}"
    exit 1
fi

if [ ! -d "${WORKFLOWS_DIR}" ]; then
    echo -e "${RED}ERROR: Workflows directory not found at ${WORKFLOWS_DIR}${NC}"
    exit 1
fi

# Check for necessary files
echo -e "\n${BLUE}Checking for required files...${NC}"

# Check for GitHub workflow file
GH_PAGES_WORKFLOW="${WORKFLOWS_DIR}/gh-pages.yml"
if [ -f "${GH_PAGES_WORKFLOW}" ]; then
    echo -e "${GREEN}✓ GitHub Pages workflow found at ${GH_PAGES_WORKFLOW}${NC}"
else
    echo -e "${RED}✗ GitHub Pages workflow not found at ${GH_PAGES_WORKFLOW}${NC}"
    exit 1
fi

# Check for MkDocs configuration
MKDOCS_CONFIG="${WORKSPACE_ROOT}/mkdocs.yml"
if [ -f "${MKDOCS_CONFIG}" ]; then
    echo -e "${GREEN}✓ MkDocs configuration found at ${MKDOCS_CONFIG}${NC}"
else
    echo -e "${RED}✗ MkDocs configuration not found at ${MKDOCS_CONFIG}${NC}"
    exit 1
fi

# Validate GitHub workflow file
echo -e "\n${BLUE}Validating GitHub workflow file...${NC}"
if grep -q "mkdocs gh-deploy" "${GH_PAGES_WORKFLOW}"; then
    echo -e "${GREEN}✓ GitHub workflow contains 'mkdocs gh-deploy' command${NC}"
else
    echo -e "${RED}✗ GitHub workflow does not contain 'mkdocs gh-deploy' command${NC}"
fi

# Validate MkDocs configuration
echo -e "\n${BLUE}Validating MkDocs configuration...${NC}"
if grep -q "site_name:" "${MKDOCS_CONFIG}"; then
    echo -e "${GREEN}✓ MkDocs configuration contains site_name${NC}"
else
    echo -e "${RED}✗ MkDocs configuration does not contain site_name${NC}"
fi

# Check documentation structure
echo -e "\n${BLUE}Checking documentation structure...${NC}"

# Check for index file in docs directory
INDEX_FILES=("${DOCS_DIR}/index.md" "${WORKSPACE_ROOT}/ROOT_INDEX.md")
INDEX_FOUND=false

for INDEX_FILE in "${INDEX_FILES[@]}"; do
    if [ -f "${INDEX_FILE}" ]; then
        echo -e "${GREEN}✓ Index file found at ${INDEX_FILE}${NC}"
        INDEX_FOUND=true
        break
    fi
done

if [ "${INDEX_FOUND}" = false ]; then
    echo -e "${RED}✗ No index file found in docs directory or root${NC}"
fi

# Validate links in markdown files
echo -e "\n${BLUE}Validating internal links in documentation...${NC}"
BROKEN_LINKS=0

check_links() {
    local file="$1"
    local dir=$(dirname "$file")

    echo -e "${YELLOW}Checking links in ${file}${NC}"

    # Find all markdown links in the file [text](url)
    grep -o '\[.*\](.*\.md)' "$file" | grep -o '(.*\.md)' | tr -d '()' | while read -r link; do
        # Skip external links
        if [[ "$link" =~ ^https?:// ]]; then
            continue
        fi

        # Handle relative links
        if [[ "$link" = /* ]]; then
            # Absolute path within the repository
            target="${WORKSPACE_ROOT}${link}"
        else
            # Relative path
            target="${dir}/${link}"
        fi

        # Normalize the path (resolve ../ etc)
        target=$(realpath --relative-to="${WORKSPACE_ROOT}" "$target" 2>/dev/null || echo "$target")
        target="${WORKSPACE_ROOT}/${target}"

        if [ ! -f "$target" ]; then
            echo -e "${RED}  ✗ Broken link in $file: $link (target not found: $target)${NC}"
            BROKEN_LINKS=$((BROKEN_LINKS + 1))
        fi
    done
}

# Check all markdown files in docs directory
find "${DOCS_DIR}" -name "*.md" -type f -print0 | while IFS= read -r -d '' file; do
    check_links "$file"
done

# Check root markdown files
find "${WORKSPACE_ROOT}" -maxdepth 1 -name "*.md" -type f -print0 | while IFS= read -r -d '' file; do
    check_links "$file"
done

if [ "${BROKEN_LINKS}" -eq 0 ]; then
    echo -e "${GREEN}✓ No broken links found in documentation${NC}"
else
    echo -e "${RED}✗ Found ${BROKEN_LINKS} broken links in documentation${NC}"
fi

# Check for required assets
echo -e "\n${BLUE}Checking for required assets...${NC}"
ASSETS_DIR="${DOCS_DIR}/assets"
if [ -d "${ASSETS_DIR}" ]; then
    echo -e "${GREEN}✓ Assets directory found at ${ASSETS_DIR}${NC}"
    # Check for any image files
    if find "${ASSETS_DIR}" -type f \( -name "*.png" -o -name "*.jpg" -o -name "*.svg" \) | grep -q .; then
        echo -e "${GREEN}✓ Image files found in assets directory${NC}"
    else
        echo -e "${YELLOW}⚠ No image files found in assets directory${NC}"
    fi
else
    echo -e "${YELLOW}⚠ Assets directory not found at ${ASSETS_DIR}${NC}"
fi

echo -e "\n${GREEN}GitHub Pages validation completed!${NC}"
echo -e "${BLUE}Please address any issues before deploying to GitHub Pages.${NC}"
