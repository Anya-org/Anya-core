#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Alignment Validator

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SRC_ROOT="$WORKSPACE_ROOT/src"
DOCS_ROOT="$WORKSPACE_ROOT/docs_aligned"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🔍 DOCUMENTATION ALIGNMENT VALIDATION${NC}"
echo "===================================="

errors=0

# Check that each source module has documentation
echo -e "\n${BLUE}📋 Checking module coverage...${NC}"
while IFS= read -r -d '' src_dir; do
    module=$(basename "$src_dir")
    if [[ "$module" != "src" && -d "$src_dir" ]]; then
        rust_files=$(find "$src_dir" -name "*.rs" 2>/dev/null | wc -l)
        if [[ $rust_files -gt 0 ]]; then
            if [[ -f "$DOCS_ROOT/$module/README.md" ]]; then
                echo -e "${GREEN}✅ $module${NC}: Documentation exists"
            else
                echo -e "${RED}❌ $module${NC}: Missing documentation"
                ((errors++))
            fi
        fi
    fi
done < <(find "$SRC_ROOT" -maxdepth 1 -type d -print0)

# Check for orphaned docs
echo -e "\n${BLUE}🏚️  Checking for orphaned documentation...${NC}"
if [[ -d "$DOCS_ROOT" ]]; then
    while IFS= read -r -d '' doc_dir; do
        module=$(basename "$doc_dir")
        if [[ "$module" != "docs_aligned" && "$module" != "api" && "$module" != "getting-started" && -d "$doc_dir" ]]; then
            if [[ ! -d "$SRC_ROOT/$module" ]]; then
                echo -e "${YELLOW}⚠️  $module${NC}: Documentation exists but no source module"
            fi
        fi
    done < <(find "$DOCS_ROOT" -maxdepth 1 -type d -print0)
fi

echo -e "\n${BLUE}📊 VALIDATION RESULTS${NC}"
echo "==================="
if [[ $errors -eq 0 ]]; then
    echo -e "${GREEN}✅ All validations passed!${NC}"
    echo -e "Documentation is perfectly aligned with source code."
    exit 0
else
    echo -e "${RED}❌ $errors alignment issues found${NC}"
    echo -e "Please fix the issues and run again."
    exit 1
fi
