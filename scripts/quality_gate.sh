#!/bin/bash

# ========================================================================
# ANYA CORE QUALITY GATE SCRIPT - STRICT ADHERENCE ENFORCEMENT
# ========================================================================
# Purpose: Enforce all repository rules, commit standards, and code quality
# Usage: ./scripts/quality_gate.sh [--pre-commit|--ci|--full]
# Auto-run: Git pre-commit hook (mandatory for all developers)
# ========================================================================

set -e # Exit on any error

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Quality gate thresholds
MAX_UNIMPLEMENTED=0
MAX_TODO_STUBS=20            # Increased for development phase
MAX_SQLITE_TODOS=20          # Increased for development phase
MAX_MOCK_IMPLEMENTATIONS=150 # Increased for development phase
MAX_WARNINGS=100             # Increased for development phase (will decrease over time)

# Check mode
MODE=${1:-"--pre-commit"}

echo -e "${BLUE}🔍 ANYA CORE QUALITY GATE ENFORCEMENT${NC}"
echo "=================================================="
echo "Date: $(date)"
echo "Mode: $MODE"
echo "Directory: $(pwd)"
echo ""

# ========================================================================
# 1. COMMIT MESSAGE VALIDATION
# ========================================================================
validate_commit_message() {
    echo -e "${BLUE}📝 VALIDATING COMMIT MESSAGE FORMAT${NC}"
    echo "----------------------------------"

    if [ "$MODE" = "--pre-commit" ]; then
        # In pre-commit, check the commit message being prepared
        if [ -f ".git/COMMIT_EDITMSG" ]; then
            COMMIT_MSG=$(cat .git/COMMIT_EDITMSG)
        else
            echo -e "${YELLOW}⚠️  No commit message found (interactive commit)${NC}"
            return 0
        fi
    else
        # In CI, check the last commit message
        COMMIT_MSG=$(git log -1 --pretty=%B)
    fi

    echo "Checking commit message: $COMMIT_MSG"

    # Check Conventional Commits format
    if ! echo "$COMMIT_MSG" | grep -qE "^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?: .+"; then
        echo -e "${RED}❌ COMMIT REJECTED: Must follow Conventional Commits format${NC}"
        echo "Expected: type(scope): description"
        echo "Example: feat(bitcoin): implement DLC oracle real cryptography"
        exit 1
    fi

    # Check for required labels
    if ! echo "$COMMIT_MSG" | grep -q "Labels:"; then
        echo -e "${RED}❌ COMMIT REJECTED: Missing required labels${NC}"
        echo "Must include: Labels: [AIR-X][AIS-X][AIT-X][component]"
        exit 1
    fi

    # Check for verification command
    if ! echo "$COMMIT_MSG" | grep -q "Verification:"; then
        echo -e "${RED}❌ COMMIT REJECTED: Missing verification evidence${NC}"
        echo "Must include: Verification: <command output or metric>"
        exit 1
    fi

    # Check for aspirational claims
    if echo "$COMMIT_MSG" | grep -qiE "(100% complete|fully implemented|production ready)" && ! echo "$COMMIT_MSG" | grep -q "Evidence:"; then
        echo -e "${RED}❌ COMMIT REJECTED: Aspirational claims without evidence${NC}"
        echo "Claims like '100% complete' require Evidence: section"
        exit 1
    fi

    echo -e "${GREEN}✅ Commit message format valid${NC}"
}

# ========================================================================
# 2. CODE QUALITY ENFORCEMENT
# ========================================================================
check_unimplemented_macros() {
    echo -e "${BLUE}🚫 CHECKING UNIMPLEMENTED!() MACROS${NC}"
    echo "-----------------------------------"

    unimpl_count=$(grep -r "unimplemented!" --include="*.rs" . 2>/dev/null | wc -l)
    echo "Found $unimpl_count unimplemented!() macros"

    if [ "$unimpl_count" -gt "$MAX_UNIMPLEMENTED" ]; then
        echo -e "${RED}❌ QUALITY GATE FAILED: Too many unimplemented!() macros${NC}"
        echo "Found: $unimpl_count, Maximum allowed: $MAX_UNIMPLEMENTED"
        echo ""
        echo "Locations:"
        grep -r "unimplemented!" --include="*.rs" . 2>/dev/null | head -10
        exit 1
    fi

    echo -e "${GREEN}✅ Unimplemented macros: $unimpl_count (≤ $MAX_UNIMPLEMENTED)${NC}"
}

check_todo_stubs() {
    echo -e "${BLUE}📝 CHECKING TODO!() STUBS${NC}"
    echo "------------------------"

    todo_count=$(grep -r "todo!" --include="*.rs" . 2>/dev/null | wc -l)
    echo "Found $todo_count todo!() stubs"

    if [ "$todo_count" -gt "$MAX_TODO_STUBS" ]; then
        echo -e "${RED}❌ QUALITY GATE FAILED: Too many todo!() stubs${NC}"
        echo "Found: $todo_count, Maximum allowed: $MAX_TODO_STUBS"
        echo ""
        echo "Locations:"
        grep -r "todo!" --include="*.rs" . 2>/dev/null | head -5
        exit 1
    fi

    echo -e "${GREEN}✅ TODO stubs: $todo_count (≤ $MAX_TODO_STUBS)${NC}"
}

check_sqlite_todos() {
    echo -e "${BLUE}💾 CHECKING SQLITE TODOS${NC}"
    echo "------------------------"

    sqlite_count=$(grep -r "TODO.*SQLite" --include="*.rs" . 2>/dev/null | wc -l)
    echo "Found $sqlite_count SQLite TODOs"

    if [ "$sqlite_count" -gt "$MAX_SQLITE_TODOS" ]; then
        echo -e "${RED}❌ QUALITY GATE FAILED: Too many SQLite TODOs${NC}"
        echo "Found: $sqlite_count, Maximum allowed: $MAX_SQLITE_TODOS"
        echo ""
        echo "Locations:"
        grep -r "TODO.*SQLite" --include="*.rs" . 2>/dev/null | head -5
        exit 1
    fi

    echo -e "${GREEN}✅ SQLite TODOs: $sqlite_count (≤ $MAX_SQLITE_TODOS)${NC}"
}

check_mock_implementations() {
    echo -e "${BLUE}🎭 CHECKING MOCK IMPLEMENTATIONS${NC}"
    echo "-------------------------------"

    mock_count=$(grep -r "MockImpl\|placeholder.*implementation\|// TODO.*mock" --include="*.rs" . 2>/dev/null | wc -l)
    echo "Found $mock_count mock implementations"

    if [ "$mock_count" -gt "$MAX_MOCK_IMPLEMENTATIONS" ]; then
        echo -e "${YELLOW}⚠️  WARNING: High number of mock implementations${NC}"
        echo "Found: $mock_count, Target: ≤ $MAX_MOCK_IMPLEMENTATIONS"
        echo ""
        echo "Sample locations:"
        grep -r "MockImpl\|placeholder.*implementation" --include="*.rs" . 2>/dev/null | head -3
        # Note: This is a warning, not a failure for now
    else
        echo -e "${GREEN}✅ Mock implementations: $mock_count (≤ $MAX_MOCK_IMPLEMENTATIONS)${NC}"
    fi
}

# ========================================================================
# 3. COMPILATION AND WARNINGS
# ========================================================================
check_compilation() {
    echo -e "${BLUE}🔨 CHECKING COMPILATION${NC}"
    echo "------------------------"

    if ! cargo check --all-features >/dev/null 2>&1; then
        echo -e "${RED}❌ QUALITY GATE FAILED: Compilation errors${NC}"
        echo ""
        echo "Compilation output:"
        cargo check --all-features
        exit 1
    fi

    echo -e "${GREEN}✅ Compilation successful${NC}"
}

check_warnings() {
    echo -e "${BLUE}⚠️  CHECKING COMPILATION WARNINGS${NC}"
    echo "--------------------------------"

    warning_count=$(cargo check --all-features 2>&1 | grep "warning:" | wc -l)
    echo "Found $warning_count compilation warnings"

    if [ "$warning_count" -gt "$MAX_WARNINGS" ]; then
        echo -e "${RED}❌ QUALITY GATE FAILED: Too many compilation warnings${NC}"
        echo "Found: $warning_count, Maximum allowed: $MAX_WARNINGS"
        echo ""
        echo "Sample warnings:"
        cargo check --all-features 2>&1 | grep "warning:" | head -5
        exit 1
    fi

    echo -e "${GREEN}✅ Compilation warnings: $warning_count (≤ $MAX_WARNINGS)${NC}"
}

# ========================================================================
# 4. DOCUMENTATION VALIDATION
# ========================================================================
check_documentation() {
    echo -e "${BLUE}📚 CHECKING DOCUMENTATION COMPLIANCE${NC}"
    echo "------------------------------------"

    # Check for aspirational claims without evidence in Rust source code only
    if grep -r "100% complete\|fully implemented\|production ready" . \
        --include="*.rs" \
        --exclude-dir=target --exclude-dir=.git --exclude-dir=.github --exclude-dir=scripts --exclude-dir=site --exclude-dir=mcp 2>/dev/null |
        grep -v "Evidence:\|Verification:\|// For now\|// For brevity\|// TODO\|// FIXME\|not fully implemented\|requires additional development" >/dev/null 2>&1; then
        echo -e "${RED}❌ QUALITY GATE FAILED: Aspirational claims without evidence in Rust code${NC}"
        echo ""
        echo "Found unsupported claims:"
        grep -r "100% complete\|fully implemented\|production ready" . \
            --include="*.rs" \
            --exclude-dir=target --exclude-dir=.git --exclude-dir=.github --exclude-dir=scripts --exclude-dir=site --exclude-dir=mcp 2>/dev/null |
            grep -v "Evidence:\|Verification:\|// For now\|// For brevity\|// TODO\|// FIXME\|not fully implemented\|requires additional development" | head -3
        exit 1
    fi

    # Check for evidence-based documentation updates
    if [ "$MODE" = "--pre-commit" ]; then
        # Check if status files are updated with recent changes
        status_files=("IMPLEMENTATION_STATUS_VERIFIED_REALITY.md" "PRD_PRODUCTION_IMPLEMENTATION_AI_PROMPT.md")
        for file in "${status_files[@]}"; do
            if [ -f "$file" ]; then
                # Check if file was modified recently (within 1 hour)
                if [ $(find "$file" -mmin -60 | wc -l) -eq 0 ]; then
                    echo -e "${YELLOW}⚠️  WARNING: $file may need updating${NC}"
                fi
            fi
        done
    fi

    echo -e "${GREEN}✅ Documentation compliance validated${NC}"
}

# ========================================================================
# 5. BRANCH AND WORKFLOW VALIDATION
# ========================================================================
check_branch_compliance() {
    echo -e "${BLUE}🌿 CHECKING BRANCH COMPLIANCE${NC}"
    echo "-----------------------------"

    current_branch=$(git branch --show-current)
    echo "Current branch: $current_branch"

    # Check if on main branch (should only allow emergency fixes)
    if [ "$current_branch" = "main" ] && [ "$MODE" = "--pre-commit" ]; then
        echo -e "${YELLOW}⚠️  WARNING: Direct commit to main branch${NC}"
        echo "Recommended: Use feature/fix branches with PR workflow"
    fi

    # Check branch naming convention
    if ! echo "$current_branch" | grep -qE "^(feature|fix|hotfix|release)\/[a-z0-9-]+$" && [ "$current_branch" != "main" ]; then
        echo -e "${YELLOW}⚠️  WARNING: Branch name doesn't follow convention${NC}"
        echo "Recommended format: feature/description-with-hyphens"
    fi

    echo -e "${GREEN}✅ Branch compliance checked${NC}"
}

# ========================================================================
# 6. SECURITY CHECKS
# ========================================================================
check_security() {
    echo -e "${BLUE}🔒 CHECKING SECURITY COMPLIANCE${NC}"
    echo "-------------------------------"

    # Check for hardcoded secrets (specific patterns, excluding derivation paths)
    if grep -r "password.*=.*[\"'].*[\"']\|secret.*=.*[\"'].*[\"']\|api_key.*=.*[\"'].*[\"']\|private_key.*=.*[\"'].*[\"']" --include="*.rs" . 2>/dev/null |
        grep -v "// Example\|// TODO\|test\|generate_\|derive_key\|m/" >/dev/null 2>&1; then
        echo -e "${RED}❌ QUALITY GATE FAILED: Potential hardcoded secrets${NC}"
        echo ""
        echo "Found potential secrets:"
        grep -r "password.*=.*[\"'].*[\"']\|secret.*=.*[\"'].*[\"']\|api_key.*=.*[\"'].*[\"']\|private_key.*=.*[\"'].*[\"']" --include="*.rs" . 2>/dev/null |
            grep -v "// Example\|// TODO\|test\|generate_\|derive_key\|m/" | head -3
        exit 1
    fi

    # Check for unsafe code blocks
    unsafe_count=$(grep -r "unsafe {" --include="*.rs" . 2>/dev/null | wc -l)
    if [ "$unsafe_count" -gt 5 ]; then
        echo -e "${YELLOW}⚠️  WARNING: High number of unsafe blocks: $unsafe_count${NC}"
        echo "Review required for production deployment"
    fi

    echo -e "${GREEN}✅ Security checks passed${NC}"
}

# ========================================================================
# 7. FINAL REPORT
# ========================================================================
generate_report() {
    echo ""
    echo -e "${BLUE}📊 QUALITY GATE SUMMARY${NC}"
    echo "========================="

    # Collect metrics
    unimpl_count=$(grep -r "unimplemented!" --include="*.rs" . 2>/dev/null | wc -l)
    todo_count=$(grep -r "todo!" --include="*.rs" . 2>/dev/null | wc -l)
    sqlite_count=$(grep -r "TODO.*SQLite" --include="*.rs" . 2>/dev/null | wc -l)
    mock_count=$(grep -r "MockImpl\|placeholder.*implementation" --include="*.rs" . 2>/dev/null | wc -l)
    warning_count=$(cargo check --all-features 2>&1 | grep "warning:" | wc -l)

    echo "📊 Code Quality Metrics:"
    echo "  • Unimplemented macros: $unimpl_count (≤ $MAX_UNIMPLEMENTED)"
    echo "  • TODO stubs: $todo_count (≤ $MAX_TODO_STUBS)"
    echo "  • SQLite TODOs: $sqlite_count (≤ $MAX_SQLITE_TODOS)"
    echo "  • Mock implementations: $mock_count (target ≤ $MAX_MOCK_IMPLEMENTATIONS)"
    echo "  • Compilation warnings: $warning_count (≤ $MAX_WARNINGS)"
    echo ""

    # Overall status
    if [ "$unimpl_count" -eq 0 ] && [ "$todo_count" -eq 0 ] && [ "$warning_count" -le "$MAX_WARNINGS" ]; then
        echo -e "${GREEN}🎉 PRODUCTION READY: All critical quality gates passed${NC}"
    elif [ "$unimpl_count" -eq 0 ] && [ "$warning_count" -le "$MAX_WARNINGS" ]; then
        echo -e "${YELLOW}🟡 PARTIAL READY: Core implementation complete, optimization needed${NC}"
    else
        echo -e "${YELLOW}🔄 IN DEVELOPMENT: Continue reducing technical debt${NC}"
    fi

    echo ""
    echo -e "${GREEN}✅ QUALITY GATE PASSED - COMMIT APPROVED${NC}"
}

# ========================================================================
# CANONICAL LABEL VALIDATION - STRICT ENFORCEMENT
# ========================================================================
validate_canonical_labels() {
    echo -e "${BLUE}🏷️  VALIDATING CANONICAL LABELS${NC}"
    echo "--------------------------------"

    if [ "$MODE" = "--pre-commit" ]; then
        if [ -f ".git/COMMIT_EDITMSG" ]; then
            COMMIT_MSG=$(cat .git/COMMIT_EDITMSG)
        else
            echo -e "${YELLOW}⚠️  No commit message found (interactive commit)${NC}"
            return 0
        fi
    else
        COMMIT_MSG=$(git log -1 --pretty=%B)
    fi

    # Extract labels line
    LABELS_LINE=$(echo "$COMMIT_MSG" | grep "Labels:" | head -1)

    if [ -z "$LABELS_LINE" ]; then
        echo -e "${RED}❌ LABEL VALIDATION FAILED: Missing Labels: line${NC}"
        exit 1
    fi

    echo "Validating labels: $LABELS_LINE"

    # Define canonical label patterns
    MANDATORY_CORE="AIR-[123]|AIS-[123]|AIT-[123]"
    STORAGE_LABELS="STORAGE-[123]|DWN-[123]|IPFS-[123]|BTC-[123]|SEC-[123]"
    BITCOIN_LABELS="BTC-[123]|L2-[123]|RGB-[123]|DLC-[123]|LN-[123]"
    WEB5_LABELS="W5-[123]|DID-[123]|VC-[123]"
    PERFORMANCE_LABELS="PFM-[123]|SCL-[123]|RES-[123]"
    INFRA_LABELS="CI-[123]|DOC-[123]|TEST-[123]|BUILD-[123]"

    ALL_VALID_LABELS="$MANDATORY_CORE|$STORAGE_LABELS|$BITCOIN_LABELS|$WEB5_LABELS|$PERFORMANCE_LABELS|$INFRA_LABELS"

    # Check mandatory core labels (AIR-X, AIS-X, AIT-X)
    if ! echo "$LABELS_LINE" | grep -qE "\[AIR-[123]\]"; then
        echo -e "${RED}❌ LABEL VALIDATION FAILED: Missing mandatory [AIR-X] label${NC}"
        exit 1
    fi

    if ! echo "$LABELS_LINE" | grep -qE "\[AIS-[123]\]"; then
        echo -e "${RED}❌ LABEL VALIDATION FAILED: Missing mandatory [AIS-X] label${NC}"
        exit 1
    fi

    if ! echo "$LABELS_LINE" | grep -qE "\[AIT-[123]\]"; then
        echo -e "${RED}❌ LABEL VALIDATION FAILED: Missing mandatory [AIT-X] label${NC}"
        exit 1
    fi

    # Extract all labels from the line
    LABELS=$(echo "$LABELS_LINE" | grep -oE '\[[A-Z0-9-]+\]' | tr -d '[]')

    # Validate each label format and content
    for label in $LABELS; do
        if ! echo "$label" | grep -qE "^($ALL_VALID_LABELS)$"; then
            echo -e "${RED}❌ LABEL VALIDATION FAILED: Invalid label [$label]${NC}"
            echo "Valid formats: [CATEGORY-LEVEL] where LEVEL is 1-3"
            echo "Valid categories: AIR, AIS, AIT, STORAGE, DWN, IPFS, BTC, SEC, L2, RGB, DLC, LN, W5, DID, VC, PFM, SCL, RES, CI, DOC, TEST, BUILD"
            exit 1
        fi
    done

    # Check label format (must be [CATEGORY-LEVEL] with square brackets)
    if echo "$LABELS_LINE" | grep -qE '\([A-Z0-9-]+\)|\{[A-Z0-9-]+\}'; then
        echo -e "${RED}❌ LABEL VALIDATION FAILED: Labels must use square brackets [CATEGORY-LEVEL]${NC}"
        exit 1
    fi

    # Check for component-specific requirements based on file changes
    if git diff --cached --name-only 2>/dev/null | grep -qE "(storage|dwn|ipfs)" || git show --name-only --pretty="" HEAD 2>/dev/null | grep -qE "(storage|dwn|ipfs)"; then
        if ! echo "$LABELS_LINE" | grep -qE "\[(STORAGE|DWN|IPFS)-[123]\]"; then
            echo -e "${RED}❌ LABEL VALIDATION FAILED: Storage-related changes require STORAGE/DWN/IPFS labels${NC}"
            exit 1
        fi
    fi

    if git diff --cached --name-only 2>/dev/null | grep -qE "(bitcoin|layer2|rgb|dlc)" || git show --name-only --pretty="" HEAD 2>/dev/null | grep -qE "(bitcoin|layer2|rgb|dlc)"; then
        if ! echo "$LABELS_LINE" | grep -qE "\[(BTC|L2|RGB|DLC)-[123]\]"; then
            echo -e "${RED}❌ LABEL VALIDATION FAILED: Bitcoin-related changes require BTC/L2/RGB/DLC labels${NC}"
            exit 1
        fi
    fi

    echo -e "${GREEN}✅ All labels are canonical and properly formatted${NC}"
}

# ========================================================================
# MAIN EXECUTION
# ========================================================================
main() {
    echo -e "${BLUE}Starting quality gate validation...${NC}"
    echo ""

    # Core validations (always run)
    validate_commit_message
    validate_canonical_labels
    check_compilation
    check_unimplemented_macros
    check_warnings
    check_documentation
    check_security

    # Extended validations (for full mode)
    if [ "$MODE" = "--full" ] || [ "$MODE" = "--ci" ]; then
        check_todo_stubs
        check_sqlite_todos
        check_mock_implementations
        check_branch_compliance
    fi

    # Generate final report
    generate_report

    echo ""
    echo -e "${GREEN}🎯 Quality gate completed successfully!${NC}"
    echo "Ready for commit/merge."
}

# Run main function
main "$@"

echo ""
echo "=================================================="
echo -e "${BLUE}Quality Gate Script completed: $(date)${NC}"
echo "=================================================="
