#!/bin/bash
# Mock Implementation Enforcement Script
# Enforces max 3 mocks per production module

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🎭 MOCK IMPLEMENTATION ENFORCEMENT${NC}"
echo "================================="
echo "Purpose: Enforce max 3 mocks per production module"
echo ""

# Function to analyze mock usage in a file
analyze_file_mocks() {
    local file="$1"
    local module=$(dirname "$file" | sed 's|.*src/||')
    local mock_count=$(grep -c "mock\|Mock" "$file" 2>/dev/null | grep -v "test\|Test" || echo "0")
    
    echo "File: $file"
    echo "Module: $module"
    echo "Mock count: $mock_count"
    
    if [ "$mock_count" -gt 3 ]; then
        echo -e "${RED}❌ VIOLATION: $mock_count mocks (limit: 3)${NC}"
        return 1
    elif [ "$mock_count" -eq 3 ]; then
        echo -e "${YELLOW}⚠️ AT LIMIT: $mock_count mocks${NC}"
        return 0
    else
        echo -e "${GREEN}✅ COMPLIANT: $mock_count mocks${NC}"
        return 0
    fi
}

# Function to refactor high-mock modules
refactor_module() {
    local file="$1"
    local module=$(dirname "$file" | sed 's|.*src/||')
    
    echo -e "${BLUE}Refactoring $module to reduce mocks...${NC}"
    
    # Create a dedicated mock module for this component
    local mock_dir=$(dirname "$file")/mocks
    mkdir -p "$mock_dir"
    
    # Extract mock definitions to separate file
    local mock_file="$mock_dir/mod.rs"
    
    cat > "$mock_file" << 'EOF'
//! Mock implementations for testing
//! This module centralizes mocks to enforce the 3-per-module limit

#[cfg(test)]
pub mod test_mocks {
    use super::*;
    use mockall::*;
    
    // Centralized mock implementations
    // TODO: Extract actual mocks from parent module
}
EOF
    
    echo "Created mock module: $mock_file"
    echo "Manual refactoring required:"
    echo "1. Move mock definitions from $file to $mock_file"
    echo "2. Update imports in $file to use $mock_file"
    echo "3. Reduce direct mock usage in production code"
}

# Main analysis
echo "Analyzing production mock usage..."
echo ""

violation_count=0
total_files=0

# Get files with mock usage (excluding tests)
mock_files=$(grep -r "mock\|Mock" --include="*.rs" src/ 2>/dev/null | grep -v "test\|Test\|#\[cfg(test)\]" | cut -d: -f1 | sort | uniq)

for file in $mock_files; do
    if [[ -f "$file" ]]; then
        total_files=$((total_files + 1))
        echo "----------------------------------------"
        
        if ! analyze_file_mocks "$file"; then
            violation_count=$((violation_count + 1))
            
            # Offer to refactor automatically
            echo ""
            echo -e "${YELLOW}Auto-refactor this module? (y/n)${NC}"
            read -r response
            if [[ "$response" == "y" || "$response" == "Y" ]]; then
                refactor_module "$file"
            fi
        fi
        echo ""
    fi
done

echo "========================================"
echo -e "${BLUE}SUMMARY:${NC}"
echo "Total files analyzed: $total_files"
echo "Violations found: $violation_count"

if [ $violation_count -eq 0 ]; then
    echo -e "${GREEN}✅ ALL MODULES COMPLIANT${NC}"
    echo "Maximum 3 mocks per module enforced successfully"
    exit 0
else
    echo -e "${RED}❌ VIOLATIONS FOUND${NC}"
    echo "Modules exceeding 3-mock limit: $violation_count"
    echo ""
    echo "Recommended actions:"
    echo "1. Move mocks to dedicated test modules"
    echo "2. Use dependency injection instead of mocks"
    echo "3. Create mock factories for reusable test objects"
    echo "4. Consider integration tests over unit tests with mocks"
    exit 1
fi
