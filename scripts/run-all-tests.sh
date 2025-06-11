#!/bin/bash

# Comprehensive test runner for Anya Core
# Runs tests in organized categories

set -e

echo "🧪 Running Anya Core Test Suite..."

PROJECT_ROOT="/workspaces/Anya-core"
cd "$PROJECT_ROOT"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Test categories
declare -a TEST_CATEGORIES=(
    "Unit Tests:tests/unit_tests/"
    "Bitcoin Tests:tests/bitcoin/"
    "Hardware Tests:tests/hardware/"
    "DAO Tests:tests/dao/"
    "Layer2 Tests:tests/layer2/"
    "Web5 Tests:tests/web5/"
)

TOTAL_PASSED=0
TOTAL_FAILED=0

echo -e "${YELLOW}Running tests by category...${NC}"

for category in "${TEST_CATEGORIES[@]}"; do
    IFS=':' read -r name path <<< "$category"
    
    if [ -d "$path" ]; then
        echo -e "${YELLOW}📂 Running $name${NC}"
        
        if cargo test --test-threads=1 --manifest-path="$path/Cargo.toml" 2>/dev/null; then
            echo -e "${GREEN}✅ $name - PASSED${NC}"
            ((TOTAL_PASSED++))
        else
            echo -e "${RED}❌ $name - FAILED${NC}"
            ((TOTAL_FAILED++))
        fi
        echo
    fi
done

# Run all tests
echo -e "${YELLOW}🚀 Running full test suite...${NC}"
if cargo test --all; then
    echo -e "${GREEN}✅ Full test suite completed successfully${NC}"
else
    echo -e "${RED}❌ Some tests failed in full suite${NC}"
fi

echo
echo -e "${YELLOW}📊 Test Summary:${NC}"
echo -e "  Categories Passed: ${GREEN}$TOTAL_PASSED${NC}"
echo -e "  Categories Failed: ${RED}$TOTAL_FAILED${NC}"
echo
echo -e "${GREEN}🎉 Test execution complete!${NC}"
