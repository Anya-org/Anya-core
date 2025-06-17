#!/bin/bash
# Comprehensive Documentation Review Script
# Created: June 17, 2025
# Purpose: Perform a final review of all documentation cleanup and reindexing

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
SCRIPTS_DIR="${WORKSPACE_ROOT}/scripts"

echo -e "${BLUE}Comprehensive Documentation Review Script${NC}"
echo -e "${BLUE}=======================================${NC}"

# Verify critical files
echo -e "\n${BLUE}Verifying critical files...${NC}"

CRITICAL_FILES=(
    "${WORKSPACE_ROOT}/ROOT_INDEX.md"
    "${DOCS_DIR}/SYSTEM_MAP.md"
    "${WORKSPACE_ROOT}/DOCUMENTATION_REINDEXING_SUMMARY.md"
    "${WORKSPACE_ROOT}/REPOSITORY_REVIEW_JUNE_2025.md"
    "${WORKSPACE_ROOT}/DOCUMENTATION_CLEANUP_PLAN.md"
    "${WORKSPACE_ROOT}/DOCUMENTATION_CLEANUP_SUMMARY.md"
    "${WORKSPACE_ROOT}/DOCUMENTATION_TRUTH_REVIEW.md"
)

MISSING_FILES=0
for file in "${CRITICAL_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓ ${file} exists${NC}"
    else
        echo -e "${RED}✗ ${file} does not exist${NC}"
        MISSING_FILES=$((MISSING_FILES + 1))
    fi
done

if [ "${MISSING_FILES}" -eq 0 ]; then
    echo -e "${GREEN}All critical files exist${NC}"
else
    echo -e "${RED}${MISSING_FILES} critical files are missing${NC}"
fi

# Check for consistent dates
echo -e "\n${BLUE}Checking for consistent dates...${NC}"
CURRENT_DATE="June 17, 2025"
INCONSISTENT_DATES=0

check_date_consistency() {
    local file="$1"
    if grep -q "${CURRENT_DATE}" "$file"; then
        echo -e "${GREEN}✓ ${file} has current date (${CURRENT_DATE})${NC}"
    else
        echo -e "${RED}✗ ${file} may not have current date (${CURRENT_DATE})${NC}"
        INCONSISTENT_DATES=$((INCONSISTENT_DATES + 1))
    fi
}

for file in "${CRITICAL_FILES[@]}"; do
    if [ -f "$file" ]; then
        check_date_consistency "$file"
    fi
done

if [ "${INCONSISTENT_DATES}" -eq 0 ]; then
    echo -e "${GREEN}All critical files have consistent dates${NC}"
else
    echo -e "${RED}${INCONSISTENT_DATES} files may have inconsistent dates${NC}"
fi

# Check for production-ready claims
echo -e "\n${BLUE}Checking for misleading production-ready claims...${NC}"
PRODUCTION_CLAIMS=0

check_for_production_claims() {
    local file="$1"
    # First check if the file actually contains any production-ready terms at all
    if ! grep -q -i "production-ready\|production ready\|fully operational\|PRODUCTION STATUS ACHIEVED" "$file"; then
        echo -e "${GREEN}✓ ${file} does not contain any production-ready terms${NC}"
        return 0
    fi

    # Check if the terms are in the context of explaining what was fixed
    if grep -i "production-ready\|production ready\|fully operational\|PRODUCTION STATUS ACHIEVED" "$file" | grep -v -i "corrected\|removed\|fixed\|previous\|before\|false\|identified\|misleading\|CLAIM\|REALITY\|documentation\|found\|claims\|BEFORE\|AFTER" >/dev/null; then
        echo -e "${RED}✗ ${file} may contain misleading production-ready claims${NC}"
        PRODUCTION_CLAIMS=$((PRODUCTION_CLAIMS + 1))
    else
        echo -e "${GREEN}✓ ${file} only mentions production-ready in the context of fixes${NC}"
    fi
}

for file in "${CRITICAL_FILES[@]}"; do
    if [ -f "$file" ]; then
        check_for_production_claims "$file"
    fi
done

if [ "${PRODUCTION_CLAIMS}" -eq 0 ]; then
    echo -e "${GREEN}No misleading production-ready claims found${NC}"
else
    echo -e "${RED}${PRODUCTION_CLAIMS} files may contain misleading production-ready claims${NC}"
fi

# Check for empty/redundant index files
echo -e "\n${BLUE}Checking for empty or redundant index files...${NC}"
EMPTY_FILES=0

if [ -f "${WORKSPACE_ROOT}/INDEX.md" ]; then
    if [ ! -s "${WORKSPACE_ROOT}/INDEX.md" ] || [ $(wc -l <"${WORKSPACE_ROOT}/INDEX.md") -lt 5 ]; then
        echo -e "${RED}✗ ${WORKSPACE_ROOT}/INDEX.md appears to be empty or nearly empty${NC}"
        EMPTY_FILES=$((EMPTY_FILES + 1))
    else
        echo -e "${YELLOW}⚠ ${WORKSPACE_ROOT}/INDEX.md still exists and has content${NC}"
    fi
fi

if [ -f "${WORKSPACE_ROOT}/INDEX_ORIGINAL.md" ]; then
    echo -e "${RED}✗ ${WORKSPACE_ROOT}/INDEX_ORIGINAL.md still exists (should be removed)${NC}"
    EMPTY_FILES=$((EMPTY_FILES + 1))
fi

if [ "${EMPTY_FILES}" -eq 0 ]; then
    echo -e "${GREEN}No empty or redundant index files found${NC}"
else
    echo -e "${RED}${EMPTY_FILES} empty or redundant index files found${NC}"
fi

# Verify scripts
echo -e "\n${BLUE}Verifying documentation scripts...${NC}"

SCRIPTS=(
    "${SCRIPTS_DIR}/cleanup-docs.sh"
    "${SCRIPTS_DIR}/validate-gh-pages.sh"
    "${SCRIPTS_DIR}/cleanup-gh-pages-test.sh"
)

MISSING_SCRIPTS=0
for script in "${SCRIPTS[@]}"; do
    if [ -f "$script" ]; then
        echo -e "${GREEN}✓ ${script} exists${NC}"

        # Check if executable
        if [ -x "$script" ]; then
            echo -e "${GREEN}  ✓ ${script} is executable${NC}"
        else
            echo -e "${YELLOW}  ⚠ ${script} is not executable${NC}"
            chmod +x "$script"
            echo -e "${GREEN}  ✓ Made ${script} executable${NC}"
        fi
    else
        echo -e "${RED}✗ ${script} does not exist${NC}"
        MISSING_SCRIPTS=$((MISSING_SCRIPTS + 1))
    fi
done

if [ "${MISSING_SCRIPTS}" -eq 0 ]; then
    echo -e "${GREEN}All documentation scripts exist${NC}"
else
    echo -e "${RED}${MISSING_SCRIPTS} documentation scripts are missing${NC}"
fi

# Summary
echo -e "\n${BLUE}Documentation Review Summary${NC}"
echo -e "${BLUE}============================${NC}"
echo -e "${GREEN}Critical files verified: $((${#CRITICAL_FILES[@]} - ${MISSING_FILES})) of ${#CRITICAL_FILES[@]}${NC}"
echo -e "${GREEN}Date consistency: $((${#CRITICAL_FILES[@]} - ${INCONSISTENT_DATES})) of ${#CRITICAL_FILES[@]}${NC}"
echo -e "${GREEN}No production claims: $((${#CRITICAL_FILES[@]} - ${PRODUCTION_CLAIMS})) of ${#CRITICAL_FILES[@]}${NC}"
echo -e "${GREEN}No redundant files: ${EMPTY_FILES} found${NC}"
echo -e "${GREEN}Documentation scripts: $((${#SCRIPTS[@]} - ${MISSING_SCRIPTS})) of ${#SCRIPTS[@]}${NC}"

echo -e "\n${GREEN}Documentation review completed!${NC}"
echo -e "${BLUE}Please address any issues highlighted above.${NC}"

# Make all scripts executable
echo -e "\n${BLUE}Making all documentation scripts executable...${NC}"
chmod +x "${SCRIPTS_DIR}/cleanup-docs.sh" "${SCRIPTS_DIR}/validate-gh-pages.sh" "${SCRIPTS_DIR}/cleanup-gh-pages-test.sh"
echo -e "${GREEN}All scripts are now executable!${NC}"
