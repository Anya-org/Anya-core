#!/bin/bash
# Smart Anya Core Quick Analysis Script
# Fast comprehensive overview with intelligent recommendations

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

echo "🚀 ANYA CORE QUICK SMART ANALYSIS"
echo "=================================="
echo "Date: $(date)"
echo "Branch: $(git branch --show-current 2>/dev/null || echo 'Unknown')"
echo ""

# =============================================================================
# CORE METRICS ANALYSIS
# =============================================================================
echo -e "${BLUE}📊 CORE SYSTEM METRICS${NC}"
echo "======================="

# Compilation check
echo -n "Compilation status: "
if cargo check --all-features >/dev/null 2>&1; then
    echo -e "${GREEN}✅ PASSING${NC}"
    compilation_ok=true
else
    echo -e "${RED}❌ FAILING${NC}"
    compilation_ok=false
fi

# Implementation completeness
unimpl_count=$(grep -r "unimplemented!" --include="*.rs" . 2>/dev/null | wc -l || echo "0")
todo_count=$(grep -r "todo!" --include="*.rs" . 2>/dev/null | wc -l || echo "0")

echo "Unimplemented functions: $unimpl_count"
echo "TODO stubs: $todo_count"

# Mock analysis
production_mocks=$(grep -r "MockImpl\|NoopAdapter\|MockService" --include="*.rs" --exclude-dir="tests" --exclude-dir="test" . 2>/dev/null | wc -l || echo "0")
test_mocks=$(grep -r "MockImpl\|NoopAdapter\|MockService" --include="*.rs" . 2>/dev/null | wc -l || echo "0")
total_mocks=$test_mocks
test_only_mocks=$((test_mocks - production_mocks))

echo "Production mocks: $production_mocks"
echo "Test-only mocks: $test_only_mocks"
echo "Total mocks: $total_mocks"

# Code metrics
rust_files=$(find . -name "*.rs" -not -path "./target/*" | wc -l)
test_files=$(find . -name "*test*.rs" -o -path "*/tests/*" | wc -l)

echo "Rust files: $rust_files"
echo "Test files: $test_files"

if [ $rust_files -gt 0 ]; then
    test_ratio=$((test_files * 100 / rust_files))
    echo "Test coverage ratio: ${test_ratio}%"
fi

# =============================================================================
# SUBSYSTEM ANALYSIS
# =============================================================================
echo ""
echo -e "${PURPLE}🔍 SUBSYSTEM ANALYSIS${NC}"
echo "====================="

subsystems=("bitcoin" "layer2" "security" "api" "ml" "dao" "web5")

for subsystem in "${subsystems[@]}"; do
    if [ -d "src/$subsystem" ] || [ -d "$subsystem" ] || find . -path "*/$subsystem/*" -type f -name "*.rs" | head -1 >/dev/null 2>&1; then
        file_count=$(find . -path "*/$subsystem/*" -name "*.rs" 2>/dev/null | wc -l)
        mock_count=$(find . -path "*/$subsystem/*" -name "*.rs" -exec grep -l "MockImpl\|NoopAdapter\|MockService" {} \; 2>/dev/null | wc -l)
        
        if [ $file_count -gt 20 ]; then
            size_indicator="${GREEN}LARGE${NC}"
        elif [ $file_count -gt 5 ]; then
            size_indicator="${YELLOW}MEDIUM${NC}"
        else
            size_indicator="${CYAN}SMALL${NC}"
        fi
        
        if [ $mock_count -gt 3 ]; then
            mock_indicator="${RED}HIGH MOCKS${NC}"
        elif [ $mock_count -gt 0 ]; then
            mock_indicator="${YELLOW}SOME MOCKS${NC}"
        else
            mock_indicator="${GREEN}NO MOCKS${NC}"
        fi
        
        echo -e "   $subsystem: $size_indicator ($file_count files) - $mock_indicator ($mock_count)"
    else
        echo -e "   $subsystem: ${RED}NOT FOUND${NC}"
    fi
done

# =============================================================================
# MISSING FUNCTIONALITY ANALYSIS
# =============================================================================
echo ""
echo -e "${CYAN}🔍 MISSING FUNCTIONALITY ANALYSIS${NC}"
echo "=================================="

# Critical missing components
missing_critical=()
missing_important=()
missing_optional=()

# Check for critical Bitcoin functionality
if ! grep -r "lightning" --include="*.rs" . >/dev/null 2>&1; then
    missing_critical+=("Lightning Network implementation")
fi

if ! grep -r "taproot\|bip341" --include="*.rs" . >/dev/null 2>&1; then
    missing_critical+=("Taproot/BIP341 support")
fi

# Check for important security features
if ! grep -r "hsm.*provider" --include="*.rs" . >/dev/null 2>&1; then
    missing_important+=("HSM provider implementations")
fi

if ! grep -r "audit.*log" --include="*.rs" . >/dev/null 2>&1; then
    missing_important+=("Comprehensive audit logging")
fi

# Check for optional enterprise features
if ! grep -r "monitoring\|metrics" --include="*.rs" . >/dev/null 2>&1; then
    missing_optional+=("Advanced monitoring/metrics")
fi

if ! grep -r "rate.*limit" --include="*.rs" . >/dev/null 2>&1; then
    missing_optional+=("API rate limiting")
fi

# Output missing functionality
if [ ${#missing_critical[@]} -gt 0 ]; then
    echo -e "${RED}🚨 CRITICAL MISSING:${NC}"
    for item in "${missing_critical[@]}"; do
        echo "   • $item"
    done
fi

if [ ${#missing_important[@]} -gt 0 ]; then
    echo -e "${YELLOW}⚠️  IMPORTANT MISSING:${NC}"
    for item in "${missing_important[@]}"; do
        echo "   • $item"
    done
fi

if [ ${#missing_optional[@]} -gt 0 ]; then
    echo -e "${CYAN}💡 OPTIONAL ENHANCEMENTS:${NC}"
    for item in "${missing_optional[@]}"; do
        echo "   • $item"
    done
fi

if [ ${#missing_critical[@]} -eq 0 ] && [ ${#missing_important[@]} -eq 0 ]; then
    echo -e "${GREEN}✅ No critical functionality missing!${NC}"
fi

# =============================================================================
# PRODUCTION READINESS SCORE
# =============================================================================
echo ""
echo -e "${GREEN}🎯 PRODUCTION READINESS ASSESSMENT${NC}"
echo "=================================="

score=100

# Critical factors
if [ "$compilation_ok" = false ]; then
    score=$((score - 50))
    echo -e "${RED}▼ Compilation failures: -50 points${NC}"
fi

if [ $unimpl_count -gt 0 ]; then
    deduction=$((unimpl_count * 10))
    if [ $deduction -gt 40 ]; then deduction=40; fi
    score=$((score - deduction))
    echo -e "${RED}▼ Unimplemented functions: -$deduction points${NC}"
fi

if [ $production_mocks -gt 20 ]; then
    deduction=$(((production_mocks - 20) / 2))
    if [ $deduction -gt 25 ]; then deduction=25; fi
    score=$((score - deduction))
    echo -e "${YELLOW}▼ Production mocks: -$deduction points${NC}"
fi

if [ ${#missing_critical[@]} -gt 0 ]; then
    deduction=$((${#missing_critical[@]} * 15))
    score=$((score - deduction))
    echo -e "${RED}▼ Critical missing features: -$deduction points${NC}"
fi

# Positive factors
if [ $test_ratio -gt 25 ]; then
    bonus=5
    score=$((score + bonus))
    echo -e "${GREEN}▲ Good test coverage: +$bonus points${NC}"
fi

if [ $score -lt 0 ]; then score=0; fi

echo ""
echo "🏆 OVERALL SCORE: $score/100"

if [ $score -ge 90 ]; then
    echo -e "${GREEN}✅ PRODUCTION READY: Excellent - Deploy with confidence${NC}"
    status="PRODUCTION_READY"
elif [ $score -ge 75 ]; then
    echo -e "${YELLOW}⚠️  NEAR PRODUCTION: Good - Minor fixes needed${NC}"
    status="NEAR_PRODUCTION"
elif [ $score -ge 50 ]; then
    echo -e "${YELLOW}🔧 DEVELOPMENT: Fair - Significant work required${NC}"
    status="DEVELOPMENT"
else
    echo -e "${RED}❌ EARLY STAGE: Poor - Major development needed${NC}"
    status="EARLY_STAGE"
fi

# =============================================================================
# SMART RECOMMENDATIONS
# =============================================================================
echo ""
echo -e "${PURPLE}🧠 SMART RECOMMENDATIONS${NC}"
echo "========================"

recommendations=()

if [ "$compilation_ok" = false ]; then
    recommendations+=("🚨 CRITICAL: Fix compilation errors immediately")
fi

if [ $unimpl_count -gt 0 ]; then
    recommendations+=("🔴 HIGH: Complete $unimpl_count unimplemented!() functions")
fi

if [ $production_mocks -gt 30 ]; then
    recommendations+=("🟡 MEDIUM: Reduce production mocks from $production_mocks")
fi

for missing in "${missing_critical[@]}"; do
    recommendations+=("🔴 HIGH: Implement $missing")
done

for missing in "${missing_important[@]}"; do
    recommendations+=("🟡 MEDIUM: Add $missing")
done

if [ ${#recommendations[@]} -eq 0 ]; then
    echo -e "${GREEN}🎉 EXCELLENT: System is production-ready!${NC}"
    echo ""
    echo "🚀 NEXT STEPS FOR OPTIMIZATION:"
    echo "• Performance benchmarking and optimization"
    echo "• Security audit and penetration testing"
    echo "• Load testing and scaling preparation"
    echo "• Advanced monitoring and alerting setup"
else
    echo "📋 PRIORITY ACTIONS:"
    for rec in "${recommendations[@]}"; do
        echo "   $rec"
    done
fi

# =============================================================================
# INTRALAYER CONTRACT READINESS
# =============================================================================
echo ""
echo -e "${BLUE}🔗 INTRALAYER CONTRACT (DEV TESTNET) READINESS${NC}"
echo "=============================================="

# Check for required components
contract_components=()
contract_missing=()

if grep -r "stacks\|clarity" --include="*.rs" . >/dev/null 2>&1; then
    contract_components+=("✅ Stacks/Clarity integration")
else
    contract_missing+=("❌ Stacks/Clarity integration")
fi

if grep -r "multi.*sig\|multisig" --include="*.rs" . >/dev/null 2>&1; then
    contract_components+=("✅ Multi-signature support")
else
    contract_missing+=("❌ Multi-signature support")
fi

if grep -r "dao\|governance" --include="*.rs" . >/dev/null 2>&1; then
    contract_components+=("✅ DAO governance framework")
else
    contract_missing+=("❌ DAO governance framework")
fi

if grep -r "cross.*chain\|bridge" --include="*.rs" . >/dev/null 2>&1; then
    contract_components+=("✅ Cross-chain capabilities")
else
    contract_missing+=("❌ Cross-chain capabilities")
fi

echo "🏗️  AVAILABLE COMPONENTS:"
for component in "${contract_components[@]}"; do
    echo "   $component"
done

if [ ${#contract_missing[@]} -gt 0 ]; then
    echo ""
    echo "🚧 MISSING FOR FULL INTRALAYER SUPPORT:"
    for missing in "${contract_missing[@]}"; do
        echo "   $missing"
    done
fi

# Contract readiness assessment
contract_readiness=$((${#contract_components[@]} * 100 / 4))
echo ""
echo "📊 INTRALAYER CONTRACT READINESS: ${contract_readiness}%"

if [ $contract_readiness -ge 75 ]; then
    echo -e "${GREEN}✅ READY for dev testnet deployment${NC}"
elif [ $contract_readiness -ge 50 ]; then
    echo -e "${YELLOW}⚠️  PARTIAL readiness - implement missing components${NC}"
else
    echo -e "${RED}❌ NOT READY - significant development required${NC}"
fi

# =============================================================================
# SUMMARY
# =============================================================================
echo ""
echo -e "${CYAN}📋 EXECUTIVE SUMMARY${NC}"
echo "===================="
echo "Production Readiness: $status ($score/100)"
echo "Contract Readiness: ${contract_readiness}%"
echo "Compilation: $([ "$compilation_ok" = true ] && echo "PASSING" || echo "FAILING")"
echo "Critical Issues: ${#missing_critical[@]}"
echo "Production Mocks: $production_mocks"
echo "Implementation Gaps: $unimpl_count unimplemented + $todo_count TODOs"

# Save summary
cat > "quick_verification_summary.json" << EOF
{
  "timestamp": "$(date -Iseconds)",
  "production_readiness_score": $score,
  "production_readiness_status": "$status",
  "contract_readiness_percent": $contract_readiness,
  "compilation_ok": $compilation_ok,
  "unimplemented_count": $unimpl_count,
  "todo_count": $todo_count,
  "production_mocks": $production_mocks,
  "test_files": $test_files,
  "rust_files": $rust_files,
  "critical_missing": ${#missing_critical[@]},
  "recommendations_count": ${#recommendations[@]}
}
EOF

echo ""
echo "💾 Summary saved to: quick_verification_summary.json"
echo -e "${GREEN}🎉 Quick smart analysis complete!${NC}"
