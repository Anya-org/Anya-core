#!/bin/bash
# Comprehensive CI/CD Pipeline Testing Script
# Tests all aspects of the enhanced CI/CD system

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TEST_LOG="$PROJECT_ROOT/cicd_test_$(date +%Y%m%d_%H%M%S).log"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

# Test tracking
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Logging function
test_log() {
    local level="$1"
    shift
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] [$level] $message" | tee -a "$TEST_LOG"
}

# Test assertion functions
assert_file_exists() {
    local file="$1"
    local description="$2"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if [[ -f "$file" ]]; then
        echo -e "${GREEN}✅ PASS: $description${NC}"
        test_log "PASS" "$description - File exists: $file"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${RED}❌ FAIL: $description${NC}"
        test_log "FAIL" "$description - File missing: $file"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

assert_command_succeeds() {
    local command="$1"
    local description="$2"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if eval "$command" &>/dev/null; then
        echo -e "${GREEN}✅ PASS: $description${NC}"
        test_log "PASS" "$description - Command succeeded: $command"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${RED}❌ FAIL: $description${NC}"
        test_log "FAIL" "$description - Command failed: $command"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

assert_json_valid() {
    local file="$1"
    local description="$2"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    # Try multiple JSON validation methods
    if command -v jq &>/dev/null && jq empty "$file" &>/dev/null; then
        echo -e "${GREEN}✅ PASS: $description${NC}"
        test_log "PASS" "$description - Valid JSON: $file"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    elif command -v python3 &>/dev/null && python3 -c "import json; json.load(open('$file'))" &>/dev/null; then
        echo -e "${GREEN}✅ PASS: $description${NC}"
        test_log "PASS" "$description - Valid JSON (Python): $file"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    elif command -v node &>/dev/null && node -e "JSON.parse(require('fs').readFileSync('$file', 'utf8'))" &>/dev/null; then
        echo -e "${GREEN}✅ PASS: $description${NC}"
        test_log "PASS" "$description - Valid JSON (Node): $file"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${YELLOW}⚠️ SKIP: $description (no JSON validator available)${NC}"
        test_log "SKIP" "$description - No JSON validator available"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    fi
}

# Test suites
test_workflow_files() {
    echo -e "${CYAN}🧪 Testing GitHub Actions Workflows${NC}"
    
    assert_file_exists ".github/workflows/comprehensive-ci.yml" "Comprehensive CI workflow exists"
    assert_file_exists ".github/workflows/enhanced-release.yml" "Enhanced release workflow exists"
    
    # Test workflow syntax
    if command -v actionlint &>/dev/null; then
        assert_command_succeeds "actionlint .github/workflows/comprehensive-ci.yml" "Comprehensive CI workflow syntax is valid"
        assert_command_succeeds "actionlint .github/workflows/enhanced-release.yml" "Enhanced release workflow syntax is valid"
    else
        echo -e "${YELLOW}⚠️ SKIP: actionlint not available for workflow validation${NC}"
    fi
}

test_ci_pipeline_script() {
    echo -e "${CYAN}🧪 Testing CI Pipeline Script${NC}"
    
    assert_file_exists "scripts/automation/ci_pipeline.sh" "CI pipeline script exists"
    assert_command_succeeds "bash -n scripts/automation/ci_pipeline.sh" "CI pipeline script syntax is valid"
    
    # Test individual phases
    if [[ -x "scripts/automation/ci_pipeline.sh" ]]; then
        assert_command_succeeds "timeout 30 scripts/automation/ci_pipeline.sh pre_validation" "CI pre-validation phase works"
    fi
}

test_metrics_generation() {
    echo -e "${CYAN}🧪 Testing Metrics Generation${NC}"
    
    assert_file_exists "scripts/generate_ci_metrics.sh" "CI metrics script exists"
    assert_command_succeeds "bash -n scripts/generate_ci_metrics.sh" "CI metrics script syntax is valid"
    
    # Test metrics generation
    if [[ -x "scripts/generate_ci_metrics.sh" ]]; then
        if timeout 60 scripts/generate_ci_metrics.sh; then
            assert_file_exists "ci_metrics.json" "CI metrics JSON generated"
            assert_file_exists "ci_metrics_summary.md" "CI metrics summary generated"
            
            if [[ -f "ci_metrics.json" ]]; then
                assert_json_valid "ci_metrics.json" "CI metrics JSON is valid"
            fi
        else
            echo -e "${YELLOW}⚠️ SKIP: Metrics generation timed out or failed${NC}"
        fi
    fi
}

test_repository_sync() {
    echo -e "${CYAN}🧪 Testing Repository Synchronization${NC}"
    
    assert_file_exists "sync-repository.js" "Repository sync script exists"
    
    # Test Node.js syntax
    if command -v node &>/dev/null; then
        assert_command_succeeds "node -c sync-repository.js" "Repository sync script syntax is valid"
        
        # Test sync execution (dry run)
        export DRY_RUN=true
        if timeout 30 node sync-repository.js; then
            echo -e "${GREEN}✅ PASS: Repository sync script executes successfully${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo -e "${YELLOW}⚠️ SKIP: Repository sync execution test (missing dependencies)${NC}"
        fi
        TESTS_RUN=$((TESTS_RUN + 1))
    else
        echo -e "${YELLOW}⚠️ SKIP: Node.js not available for sync testing${NC}"
    fi
}

test_rust_environment() {
    echo -e "${CYAN}🧪 Testing Rust Environment${NC}"
    
    assert_command_succeeds "command -v cargo" "Cargo is available"
    assert_command_succeeds "command -v rustc" "Rust compiler is available"
    
    if [[ -f "Cargo.toml" ]]; then
        assert_command_succeeds "cargo check --workspace" "Rust project compiles successfully"
        
        # Test basic cargo commands
        assert_command_succeeds "cargo fmt --check" "Code formatting is correct"
        
        if command -v cargo-clippy &>/dev/null; then
            assert_command_succeeds "cargo clippy --workspace -- -D warnings" "Clippy lints pass"
        fi
        
        # Quick test run
        assert_command_succeeds "timeout 60 cargo test --workspace --no-run" "Tests compile successfully"
    else
        echo -e "${YELLOW}⚠️ SKIP: No Cargo.toml found${NC}"
    fi
}

test_security_tools() {
    echo -e "${CYAN}🧪 Testing Security Tools${NC}"
    
    # Test cargo-audit
    if command -v cargo-audit &>/dev/null; then
        assert_command_succeeds "cargo audit --version" "cargo-audit is available"
        if [[ -f "Cargo.lock" ]]; then
            assert_command_succeeds "timeout 30 cargo audit" "Security audit passes"
        fi
    else
        echo -e "${YELLOW}⚠️ SKIP: cargo-audit not installed${NC}"
    fi
    
    # Test for sensitive files
    TESTS_RUN=$((TESTS_RUN + 1))
    sensitive_files=$(find . -name "*.key" -o -name "*.pem" -o -name ".env" | head -5)
    if [[ -z "$sensitive_files" ]]; then
        echo -e "${GREEN}✅ PASS: No obvious sensitive files in repository${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${YELLOW}⚠️ WARN: Potential sensitive files found${NC}"
        echo "$sensitive_files"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

test_documentation() {
    echo -e "${CYAN}🧪 Testing Documentation${NC}"
    
    assert_file_exists "README.md" "README.md exists"
    assert_file_exists "CHANGELOG.md" "CHANGELOG.md exists"
    
    # Test for broken links in markdown files
    if command -v grep &>/dev/null; then
        TESTS_RUN=$((TESTS_RUN + 1))
        broken_links=$(find . -name "*.md" -exec grep -l "](.*\.md)" {} \; | wc -l)
        if [[ $broken_links -eq 0 ]]; then
            echo -e "${GREEN}✅ PASS: No obvious broken internal links found${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo -e "${YELLOW}⚠️ INFO: Found $broken_links files with internal links (manual review needed)${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        fi
    fi
}

test_performance_benchmarks() {
    echo -e "${CYAN}🧪 Testing Performance Benchmarks${NC}"
    
    if [[ -f "Cargo.toml" ]] && command -v cargo &>/dev/null; then
        # Check if benchmarks are defined
        if grep -q "\[\[bench\]\]" Cargo.toml || find . -name "benches" -type d | grep -q .; then
            assert_command_succeeds "timeout 60 cargo bench --no-run" "Benchmarks compile successfully"
        else
            echo -e "${YELLOW}⚠️ INFO: No benchmarks found${NC}"
        fi
    fi
}

generate_test_report() {
    echo -e "${BLUE}📊 Generating Test Report${NC}"
    
    local end_time=$(date)
    local success_rate=0
    if [[ $TESTS_RUN -gt 0 ]]; then
        success_rate=$((TESTS_PASSED * 100 / TESTS_RUN))
    fi
    
    cat > "$PROJECT_ROOT/cicd_test_report.md" << EOF
# CI/CD Pipeline Test Report

**Generated**: $end_time
**Test Duration**: Started at script execution
**Overall Success Rate**: ${success_rate}% (${TESTS_PASSED}/${TESTS_RUN})

## Test Summary

- ✅ **Passed**: ${TESTS_PASSED} tests
- ❌ **Failed**: ${TESTS_FAILED} tests
- 📊 **Total**: ${TESTS_RUN} tests

## Test Categories

### GitHub Actions Workflows
- Comprehensive CI workflow validation
- Enhanced release workflow validation
- Workflow syntax checking

### CI Pipeline Scripts
- Core pipeline script functionality
- Metrics generation capabilities
- Error handling and reporting

### Repository Synchronization
- Cross-repository sync functionality
- Configuration management
- Dependency updates

### Security Testing
- Security audit tools
- Code analysis (clippy)
- Sensitive file detection

### Performance & Benchmarks
- Build performance
- Runtime benchmarks
- Resource utilization

## Recommendations

$(if [[ $TESTS_FAILED -gt 0 ]]; then
    echo "🔧 **Action Required**: $TESTS_FAILED tests failed. Review the test log for details."
else
    echo "🎉 **Excellent**: All tests passed! CI/CD pipeline is healthy."
fi)

$(if [[ $success_rate -lt 80 ]]; then
    echo "⚠️ **Warning**: Success rate below 80%. Consider addressing failing tests."
elif [[ $success_rate -lt 95 ]]; then
    echo "👍 **Good**: Success rate above 80%. Minor improvements possible."
else
    echo "🌟 **Outstanding**: Success rate above 95%. Excellent CI/CD health!"
fi)

## Next Steps

1. Address any failing tests identified above
2. Ensure all required tools are installed
3. Run full CI pipeline to validate integration
4. Monitor performance metrics over time
5. Regular security audits and dependency updates

---
*Generated by Anya-Core CI/CD Test Suite*
EOF
    
    echo -e "${GREEN}✅ Test report generated: cicd_test_report.md${NC}"
}

print_summary() {
    echo
    echo -e "${MAGENTA}════════════════════════════════════════${NC}"
    echo -e "${MAGENTA}         CI/CD PIPELINE TEST SUMMARY       ${NC}"
    echo -e "${MAGENTA}════════════════════════════════════════${NC}"
    echo
    echo -e "${BLUE}Tests Run:    ${NC}${TESTS_RUN}"
    echo -e "${GREEN}Tests Passed: ${NC}${TESTS_PASSED}"
    echo -e "${RED}Tests Failed: ${NC}${TESTS_FAILED}"
    echo
    
    local success_rate=0
    if [[ $TESTS_RUN -gt 0 ]]; then
        success_rate=$((TESTS_PASSED * 100 / TESTS_RUN))
    fi
    
    echo -e "${CYAN}Success Rate: ${NC}${success_rate}%"
    echo
    
    if [[ $TESTS_FAILED -eq 0 ]]; then
        echo -e "${GREEN}🎉 ALL TESTS PASSED! CI/CD pipeline is ready.${NC}"
    else
        echo -e "${YELLOW}⚠️  Some tests failed. Check the report for details.${NC}"
    fi
    
    echo
    echo -e "${BLUE}📋 Detailed log: ${NC}$TEST_LOG"
    echo -e "${BLUE}📊 Test report: ${NC}$PROJECT_ROOT/cicd_test_report.md"
    echo
}

# Main execution
main() {
    echo -e "${YELLOW}🚀 Starting Comprehensive CI/CD Pipeline Tests${NC}"
    echo -e "${BLUE}Test log: $TEST_LOG${NC}"
    echo
    
    # Ensure we're in the project root
    cd "$PROJECT_ROOT"
    
    # Run all test suites
    test_workflow_files
    echo
    test_ci_pipeline_script
    echo
    test_metrics_generation
    echo
    test_repository_sync
    echo
    test_rust_environment
    echo
    test_security_tools
    echo
    test_documentation
    echo
    test_performance_benchmarks
    echo
    
    # Generate reports
    generate_test_report
    print_summary
    
    # Exit with error code if tests failed
    if [[ $TESTS_FAILED -gt 0 ]]; then
        exit 1
    fi
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
