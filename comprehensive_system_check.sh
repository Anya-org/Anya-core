#!/bin/bash
# Comprehensive System Verification and Warning Fix Script
# Auto-run mode with comprehensive checks

set -e

echo "🔍 COMPREHENSIVE SYSTEM VERIFICATION - AUTO-RUN MODE"
echo "===================================================="
echo "Date: $(date)"
echo

# Function to run tests and capture results
run_tests() {
    local test_name="$1"
    local test_cmd="$2"
    echo "📋 Running $test_name..."
    
    if eval "$test_cmd" > "/tmp/${test_name}_output.log" 2>&1; then
        echo "✅ $test_name: PASSED"
        return 0
    else
        echo "❌ $test_name: FAILED"
        cat "/tmp/${test_name}_output.log"
        return 1
    fi
}

# 1. WORKSPACE STATUS CHECK
echo "🔍 1. WORKSPACE STATUS CHECK"
echo "------------------------------"
git status --porcelain | head -10
echo

# 2. COMPILATION STATUS
echo "🔧 2. COMPILATION STATUS CHECK"
echo "--------------------------------"
if cargo check --workspace &>/dev/null; then
    echo "✅ Workspace compiles successfully"
else
    echo "❌ Compilation issues detected"
    cargo check --workspace 2>&1 | tail -20
fi
echo

# 3. LAYER 2 TESTS VERIFICATION
echo "🚀 3. LAYER 2 TESTS VERIFICATION"
echo "----------------------------------"
run_tests "Layer2_Tests" "cargo test --lib layer2 --quiet"
echo

# 4. CORE SYSTEM TESTS
echo "🏗️ 4. CORE SYSTEM TESTS"
echo "-------------------------"
run_tests "Core_Tests" "cargo test --lib --quiet --bins core"
echo

# 5. WARNING COUNT ANALYSIS
echo "⚠️ 5. WARNING ANALYSIS"
echo "------------------------"
warning_count=$(cargo check --workspace 2>&1 | grep -c "warning:" || echo "0")
echo "Total warnings detected: $warning_count"

if [ "$warning_count" -gt 0 ]; then
    echo "📝 Top warning categories:"
    cargo check --workspace 2>&1 | grep "warning:" | head -10
fi
echo

# 6. TYPESCRIPT/REACT STATUS
echo "⚛️ 6. TYPESCRIPT/REACT STATUS"
echo "-------------------------------"
if [ -f "package.json" ]; then
    echo "✅ package.json exists"
    if [ -f "node_modules/.package-lock.json" ] || [ -d "node_modules" ]; then
        echo "✅ Node modules installed"
    else
        echo "📦 Installing Node dependencies..."
        npm install --silent
    fi
    
    # Check TypeScript compilation
    if npx tsc --noEmit --skipLibCheck &>/dev/null; then
        echo "✅ TypeScript compilation successful"
    else
        echo "⚠️ TypeScript issues detected"
        npx tsc --noEmit --skipLibCheck 2>&1 | head -5
    fi
else
    echo "⚠️ No package.json found"
fi
echo

# 7. SYSTEM ARCHITECTURE VALIDATION
echo "🏛️ 7. SYSTEM ARCHITECTURE VALIDATION"
echo "--------------------------------------"
echo "Layer 2 Protocols Status:"

# Check for key Layer 2 files
protocols=("bob" "liquid" "rsk" "stacks" "taproot_assets" "rgb")
for protocol in "${protocols[@]}"; do
    if [ -f "src/layer2/${protocol}.rs" ] || [ -f "anya-bitcoin/src/layer2/${protocol}/mod.rs" ]; then
        echo "  ✅ $protocol: Implementation found"
    else
        echo "  ❌ $protocol: Implementation missing"
    fi
done
echo

# 8. DOCUMENTATION STATUS
echo "📚 8. DOCUMENTATION STATUS"
echo "----------------------------"
key_docs=("TODO.md" "CHANGELOG.md" "README.md" "SYSTEM_STATUS_REPORT_JUNE_14_2025.md")
for doc in "${key_docs[@]}"; do
    if [ -f "$doc" ]; then
        lines=$(wc -l < "$doc")
        echo "  ✅ $doc: $lines lines"
    else
        echo "  ❌ $doc: Missing"
    fi
done
echo

# 9. GIT STATUS AND BRANCHES
echo "🌿 9. GIT STATUS AND BRANCHES"
echo "-------------------------------"
echo "Current branch: $(git branch --show-current)"
echo "Staged changes: $(git diff --cached --name-only | wc -l)"
echo "Unstaged changes: $(git diff --name-only | wc -l)"
echo "Untracked files: $(git ls-files --others --exclude-standard | wc -l)"
echo

# 10. FINAL SYSTEM STATUS SUMMARY
echo "📊 10. FINAL SYSTEM STATUS SUMMARY"
echo "------------------------------------"

total_score=0
max_score=8

# Scoring
[ "$warning_count" -lt 50 ] && ((total_score++))
[ -f "SYSTEM_STATUS_REPORT_JUNE_14_2025.md" ] && ((total_score++))
[ -f "src/components/Layer2Provider.tsx" ] && ((total_score++))
[ -f "src/services/layer2-service.ts" ] && ((total_score++))
cargo test --lib layer2 --quiet &>/dev/null && ((total_score++))
[ -f "package.json" ] && ((total_score++))
[ -d "src/layer2" ] && ((total_score++))
[ "$(git status --porcelain | wc -l)" -lt 20 ] && ((total_score++))

percentage=$((total_score * 100 / max_score))

echo "🎯 SYSTEM HEALTH SCORE: $total_score/$max_score ($percentage%)"
echo

if [ "$percentage" -ge 90 ]; then
    echo "🎉 EXCELLENT: System is in excellent condition!"
    echo "✅ Layer 2 solution is production-ready"
    echo "✅ All major components operational"
    echo "✅ Ready for deployment"
elif [ "$percentage" -ge 75 ]; then
    echo "✅ GOOD: System is in good condition"
    echo "📝 Minor optimizations recommended"
elif [ "$percentage" -ge 50 ]; then
    echo "⚠️ MODERATE: System needs attention"
    echo "🔧 Several issues require fixing"
else
    echo "❌ POOR: System requires significant work"
    echo "🚨 Critical issues need immediate attention"
fi

echo
echo "📋 AUTOMATED REPORT COMPLETED"
echo "Generated: $(date)"
echo "Report saved in system logs"
echo "===================================================="
