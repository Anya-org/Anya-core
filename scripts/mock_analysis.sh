#!/bin/bash
# Mock Implementation Analysis Script
# Analyzes mock usage per module and suggests refactoring

set -e

echo "🔍 MOCK IMPLEMENTATION ANALYSIS"
echo "================================"
echo "Date: $(date)"
echo "Target: Maximum 3 mocks per module"
echo ""

# Function to analyze mocks in a directory
analyze_module_mocks() {
    local module_path="$1"
    local module_name=$(basename "$module_path")
    
    if [[ -d "$module_path" ]]; then
        local mock_count=$(grep -r "mock\|Mock" "$module_path" --include="*.rs" 2>/dev/null | grep -v "test\|Test" | wc -l)
        
        if [[ $mock_count -gt 0 ]]; then
            echo "📦 Module: $module_name"
            echo "   Path: $module_path"
            echo "   Mock count: $mock_count"
            
            if [[ $mock_count -gt 3 ]]; then
                echo "   ❌ EXCEEDS LIMIT (max 3)"
                echo "   Mock locations:"
                grep -r "mock\|Mock" "$module_path" --include="*.rs" 2>/dev/null | grep -v "test\|Test" | head -5 | sed 's/^/      /'
                if [[ $mock_count -gt 5 ]]; then
                    echo "      ... and $((mock_count - 5)) more"
                fi
            else
                echo "   ✅ Within limit"
            fi
            echo ""
        fi
    fi
}

echo "📊 MOCK ANALYSIS BY MODULE:"
echo "============================"

# Analyze each major module
for module in src/layer2 src/bitcoin src/security src/ml src/storage src/dao src/compliance src/infrastructure; do
    if [[ -d "$module" ]]; then
        analyze_module_mocks "$module"
    fi
done

# Analyze specific problematic modules
echo "🎯 DETAILED ANALYSIS OF HIGH-MOCK MODULES:"
echo "=========================================="

echo "Layer2 Module Breakdown:"
for submodule in src/layer2/*; do
    if [[ -d "$submodule" ]]; then
        analyze_module_mocks "$submodule"
    fi
done

echo "📋 SUMMARY AND RECOMMENDATIONS:"
echo "==============================="

# Count total production mocks (excluding tests)
total_prod_mocks=$(grep -r "mock\|Mock" src/ --include="*.rs" 2>/dev/null | grep -v "test\|Test" | wc -l)
total_test_mocks=$(grep -r "mock\|Mock" src/ --include="*.rs" 2>/dev/null | grep "test\|Test" | wc -l)

echo "Total production mocks: $total_prod_mocks"
echo "Total test mocks: $total_test_mocks"
echo "Combined total: $((total_prod_mocks + total_test_mocks))"
echo ""

echo "🎯 REFACTORING STRATEGY:"
echo "========================"
echo "1. Replace inline mock values with proper implementations"
echo "2. Consolidate mock utilities into dedicated test modules"
echo "3. Use dependency injection for testable real implementations"
echo "4. Remove production mock implementations"
echo "5. Keep only essential mocks for testing framework"

echo ""
echo "📝 PRIORITY ACTIONS:"
echo "==================="
echo "1. Layer2 protocols: Replace mock responses with real logic"
echo "2. Bitcoin adapters: Implement actual RPC/protocol communication"
echo "3. Storage layer: Replace mock databases with real implementations"
echo "4. ML agents: Implement actual inference logic"
echo "5. Security HSM: Use software implementations instead of mocks"
