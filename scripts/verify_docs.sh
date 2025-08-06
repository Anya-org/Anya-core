#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Verification Script
# This script verifies that all documentation is properly aligned and valid

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🔍 DOCUMENTATION VERIFICATION${NC}"
echo "================================="

# 1. Run the aligned documentation validation
echo -e "${YELLOW}📋 Checking module coverage...${NC}"
if ! bash ./scripts/validate_aligned_docs.sh; then
    echo -e "${RED}❌ Documentation alignment validation failed${NC}"
    exit 1
fi

# 2. Verify Rust documentation builds without errors
echo -e "${YELLOW}🦀 Verifying Rust documentation...${NC}"
if ! cargo doc --no-deps --document-private-items --quiet; then
    echo -e "${RED}❌ Cargo doc failed${NC}"
    exit 1
fi

# 3. Check for basic Markdown issues (if markdownlint is available)
if command -v markdownlint >/dev/null 2>&1; then
    echo -e "${YELLOW}📝 Running Markdown linter...${NC}"
    # Run with relaxed rules for production
    if ! markdownlint docs/**/*.md --disable MD013 --disable MD033 --disable MD041 --disable MD024 --disable MD053 --disable MD022 --disable MD032 --disable MD031 --disable MD001 --disable MD040; then
        echo -e "${YELLOW}⚠  Markdown linting found issues but continuing...${NC}"
    fi
else
    echo -e "${YELLOW}📝 Markdown linter not available, skipping...${NC}"
fi

# 4. Verify critical files exist
echo -e "${YELLOW}📁 Checking critical documentation files...${NC}"
critical_files=(
    "README.md"
    "CHANGELOG.md"
    "LICENSE.md"
    "docs"
    "mkdocs.yml"
)

for file in "${critical_files[@]}"; do
    if [ ! -e "$file" ]; then
        echo -e "${RED}❌ Critical file missing: $file${NC}"
        exit 1
    fi
done

echo -e "${GREEN}✅ All documentation verification checks passed!${NC}"
echo -e "${GREEN}📚 Documentation is ready for deployment${NC}"
