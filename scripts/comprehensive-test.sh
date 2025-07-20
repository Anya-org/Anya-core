#!/bin/bash
# Comprehensive Anya-Core Test and Verification Script
# [AIR-3][AIS-3][BPC-3][RES-3]

set -e

echo "======================================================================"
echo "🚀 ANYA-CORE COMPREHENSIVE TEST & VERIFICATION REPORT"
echo "======================================================================" 
echo "Date: $(date)"
echo "======================================================================" 

# Function to print section headers
print_section() {
    echo ""
    echo "----------------------------------------------------------------------"
    echo "📋 $1"
    echo "----------------------------------------------------------------------"
}

# Function to run tests with timeout and capture results
run_test_section() {
    local test_name="$1"
    local test_command="$2"
    local timeout_duration="${3:-60}"
    
    echo "🧪 Running: $test_name"
    if timeout "$timeout_duration" bash -c "$test_command" 2>&1; then
        echo "✅ PASSED: $test_name"
        return 0
    else
        echo "❌ FAILED: $test_name"
        return 1
    fi
}

# Initialize counters
total_tests=0
passed_tests=0
failed_tests=0

print_section "1. LAYER 2 PROTOCOL TESTS"

# Layer 2 comprehensive tests
total_tests=$((total_tests + 1))
if run_test_section "Layer 2 Comprehensive Tests" "cd /workspaces/Anya-core && cargo test --lib layer2::comprehensive_tests --quiet"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

# Layer 2 manager tests
total_tests=$((total_tests + 1))
if run_test_section "Layer 2 Manager Tests" "cd /workspaces/Anya-core && cargo test --lib layer2::manager --quiet"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

print_section "2. BITCOIN CORE TESTS"

 # Bitcoin protocol tests (use correct path)
total_tests=$((total_tests + 1))
if run_test_section "Bitcoin Protocol Tests" "cd /workspaces/Anya-core/dependencies/anya-bitcoin && cargo test --lib --quiet"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

print_section "3. COMPILATION VERIFICATION"

# Check compilation status
total_tests=$((total_tests + 1))
if run_test_section "Core Library Compilation" "cd /workspaces/Anya-core && cargo check --lib --quiet"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

# Check Bitcoin library compilation (use correct path)
total_tests=$((total_tests + 1))
if run_test_section "Bitcoin Library Compilation" "cd /workspaces/Anya-core/dependencies/anya-bitcoin && cargo check --quiet"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

print_section "4. PROJECT STRUCTURE VERIFICATION"

# Check key files exist
check_file_exists() {
    local file_path="$1"
    local description="$2"
    
    total_tests=$((total_tests + 1))
    if [[ -f "$file_path" ]]; then
        echo "✅ EXISTS: $description ($file_path)"
        passed_tests=$((passed_tests + 1))
        return 0
    else
        echo "❌ MISSING: $description ($file_path)"
        failed_tests=$((failed_tests + 1))
        return 1
    fi
}

check_file_exists "/workspaces/Anya-core/src/layer2/mod.rs" "Layer 2 Module"
check_file_exists "/workspaces/Anya-core/src/layer2/manager.rs" "Layer 2 Manager"
check_file_exists "/workspaces/Anya-core/src/layer2/comprehensive_tests.rs" "Layer 2 Tests"
check_file_exists "/workspaces/Anya-core/anya-bitcoin/src/layer2/bob/mod.rs" "BOB Protocol"
check_file_exists "/workspaces/Anya-core/anya-bitcoin/src/layer2/liquid.rs" "Liquid Protocol"
check_file_exists "/workspaces/Anya-core/src/services/lightning-service.tsx" "Lightning React Service"
check_file_exists "/workspaces/Anya-core/src/services/rgb-service.tsx" "RGB React Service"
check_file_exists "/workspaces/Anya-core/src/services/stacks-service.tsx" "Stacks React Service"

print_section "5. REACT/TYPESCRIPT MIGRATION STATUS"

# Count Dart files remaining
dart_files=$(find /workspaces/Anya-core -name "*.dart" -not -path "*/test/*" | wc -l)
echo "📊 Dart files remaining (excluding tests): $dart_files"

# Count React/TypeScript files
tsx_files=$(find /workspaces/Anya-core/src -name "*.tsx" 2>/dev/null | wc -l || echo "0")
echo "📊 React/TypeScript files created: $tsx_files"

if [[ $tsx_files -gt 0 ]]; then
    echo "✅ React/TypeScript migration in progress"
else
    echo "⚠️  React/TypeScript migration needs attention"
fi

print_section "6. GIT STATUS CHECK"

echo "📊 Git repository status:"
cd /workspaces/Anya-core
git status --porcelain | head -10
echo ""
if git status --porcelain | grep -q .; then
    echo "📝 Changes detected - ready for commit organization"
else
    echo "✅ Working directory clean"
fi

print_section "7. DEPENDENCY AUDIT"

# Check for security advisories
total_tests=$((total_tests + 1))
if run_test_section "Security Audit" "cd /workspaces/Anya-core && cargo audit --quiet" 30; then
    passed_tests=$((passed_tests + 1))
else
    echo "⚠️  Security advisories found (may be acceptable)"
    failed_tests=$((failed_tests + 1))
fi

print_section "8. FINAL SUMMARY"

echo ""
echo "======================================================================" 
echo "🎯 TEST RESULTS SUMMARY"
echo "======================================================================" 
echo "📊 Total Tests:  $total_tests"
echo "✅ Passed Tests: $passed_tests"
echo "❌ Failed Tests: $failed_tests"
echo "📈 Success Rate: $(( (passed_tests * 100) / total_tests ))%"
echo ""

if [[ $failed_tests -eq 0 ]]; then
    echo "🎉 ALL TESTS PASSED - SYSTEM IS OPERATIONAL!"
    echo ""
    echo "✅ Layer 2 Protocols: OPERATIONAL"
    echo "✅ Bitcoin Core: FUNCTIONAL" 
    echo "✅ Compilation: SUCCESS"
    echo "✅ Project Structure: COMPLETE"
    echo ""
    echo "🚀 READY FOR PRODUCTION DEPLOYMENT"
    exit 0
else
    echo "⚠️  SOME TESTS FAILED - REVIEW REQUIRED"
    echo ""
    echo "📋 Next Steps:"
    echo "   1. Review failed test output above"
    echo "   2. Address any critical compilation issues"
    echo "   3. Complete remaining React/TypeScript migration"
    echo "   4. Re-run verification script"
    exit 1
fi
