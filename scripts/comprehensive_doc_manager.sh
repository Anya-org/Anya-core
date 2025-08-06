#!/bin/bash

# Comprehensive Documentation Management System
# Master script for all documentation operations in Anya Core

set -euo pipefail

ANYA_ROOT="/workspaces/Anya-core"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Display banner
echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                    ANYA CORE DOCUMENTATION MANAGER                       ║${NC}"
echo -e "${CYAN}║                        Truth-Aligned Documentation                       ║${NC}"
echo -e "${CYAN}║                            Version 2.0                                   ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Function to show help
show_help() {
    echo -e "${BLUE}📚 Anya Core Documentation Manager${NC}"
    echo -e "Comprehensive documentation management aligned with source code truth"
    echo ""
    echo -e "${YELLOW}USAGE:${NC}"
    echo -e "  $0 [COMMAND] [OPTIONS]"
    echo ""
    echo -e "${YELLOW}COMMANDS:${NC}"
    echo -e "  ${GREEN}verify${NC}      - Verify all README.md files against source code"
    echo -e "  ${GREEN}refactor${NC}    - Refactor outdated/incomplete README files"
    echo -e "  ${GREEN}analyze${NC}     - Analyze repository documentation structure"
    echo -e "  ${GREEN}align${NC}       - Create aligned documentation structure"
    echo -e "  ${GREEN}validate${NC}    - Validate aligned documentation"
    echo -e "  ${GREEN}sync${NC}        - Synchronize documentation with source changes"
    echo -e "  ${GREEN}report${NC}      - Generate comprehensive documentation report"
    echo -e "  ${GREEN}all${NC}         - Execute complete documentation workflow"
    echo -e "  ${GREEN}help${NC}        - Show this help message"
    echo ""
    echo -e "${YELLOW}EXAMPLES:${NC}"
    echo -e "  $0 verify           # Verify all README files"
    echo -e "  $0 all              # Complete documentation workflow"
    echo -e "  $0 report           # Generate status report"
    echo ""
}

# Function to verify README files
verify_readmes() {
    echo -e "${BLUE}📋 Verifying README.md files...${NC}"
    if [[ -x "${ANYA_ROOT}/scripts/verify_all_readmes.sh" ]]; then
        "${ANYA_ROOT}/scripts/verify_all_readmes.sh"
    else
        echo -e "${RED}❌ Verification script not found or not executable${NC}"
        return 1
    fi
}

# Function to refactor README files
refactor_readmes() {
    echo -e "${YELLOW}🔧 Refactoring README.md files...${NC}"
    if [[ -x "${ANYA_ROOT}/scripts/refactor_readme_files.sh" ]]; then
        "${ANYA_ROOT}/scripts/refactor_readme_files.sh"
    else
        echo -e "${RED}❌ Refactoring script not found or not executable${NC}"
        return 1
    fi
}

# Function to analyze repository
analyze_repository() {
    echo -e "${CYAN}🔍 Analyzing repository structure...${NC}"
    if [[ -x "${ANYA_ROOT}/scripts/simple_doc_analysis.sh" ]]; then
        "${ANYA_ROOT}/scripts/simple_doc_analysis.sh"
    else
        echo -e "${RED}❌ Analysis script not found${NC}"
        return 1
    fi
}

# Function to create aligned documentation
align_documentation() {
    echo -e "${MAGENTA}📝 Creating aligned documentation...${NC}"
    if [[ -x "${ANYA_ROOT}/scripts/create_aligned_docs.sh" ]]; then
        "${ANYA_ROOT}/scripts/create_aligned_docs.sh"
    else
        echo -e "${RED}❌ Alignment script not found${NC}"
        return 1
    fi
}

# Function to validate aligned documentation
validate_documentation() {
    echo -e "${GREEN}✅ Validating aligned documentation...${NC}"
    if [[ -x "${ANYA_ROOT}/scripts/validate_aligned_docs.sh" ]]; then
        "${ANYA_ROOT}/scripts/validate_aligned_docs.sh"
    else
        echo -e "${RED}❌ Validation script not found${NC}"
        return 1
    fi
}

# Function to synchronize documentation
sync_documentation() {
    echo -e "${BLUE}🔄 Synchronizing documentation...${NC}"
    if [[ -x "${ANYA_ROOT}/scripts/manage_docs.sh" ]]; then
        "${ANYA_ROOT}/scripts/manage_docs.sh" sync
    else
        echo -e "${RED}❌ Management script not found${NC}"
        return 1
    fi
}

# Function to generate comprehensive report
generate_report() {
    echo -e "${CYAN}📊 Generating comprehensive documentation report...${NC}"

    local report_file="${ANYA_ROOT}/COMPREHENSIVE_DOCUMENTATION_REPORT.md"

    cat > "$report_file" << EOF
# Comprehensive Documentation Report
Generated: $TIMESTAMP

## Executive Summary

This report provides a complete analysis of the Anya Core documentation ecosystem,
including verification results, alignment status, and recommendations for maintaining
documentation truth alignment with source code.

## Repository Statistics

### Source Code Analysis
EOF

    # Add source statistics
    local total_rust_files=$(find "$ANYA_ROOT/src" -name "*.rs" -type f 2>/dev/null | wc -l)
    local total_js_files=$(find "$ANYA_ROOT" -name "*.js" -o -name "*.ts" -type f -not -path "*/node_modules/*" 2>/dev/null | wc -l)
    local total_py_files=$(find "$ANYA_ROOT" -name "*.py" -type f -not -path "*/node_modules/*" 2>/dev/null | wc -l)
    local total_readmes=$(find "$ANYA_ROOT" -name "README.md" -type f -not -path "*/node_modules/*" -not -path "*/.git/*" 2>/dev/null | wc -l)

    cat >> "$report_file" << EOF
- **Rust Files**: $total_rust_files
- **JavaScript/TypeScript Files**: $total_js_files
- **Python Files**: $total_py_files
- **README.md Files**: $total_readmes

### Documentation Verification Status
EOF

    # Include latest verification results if available
    if [[ -f "${ANYA_ROOT}/readme_verification_report.md" ]]; then
        echo "- **Latest Verification**: $(date -r "${ANYA_ROOT}/readme_verification_report.md" '+%Y-%m-%d %H:%M:%S')" >> "$report_file"

        # Extract key metrics from verification report
        local verified_count=$(grep -c "✅ VERIFIED\|✅ BASIC" "${ANYA_ROOT}/readme_verification_report.md" || echo "0")
        local outdated_count=$(grep -c "❌ OUTDATED" "${ANYA_ROOT}/readme_verification_report.md" || echo "0")
        local incomplete_count=$(grep -c "⚠️  INCOMPLETE" "${ANYA_ROOT}/readme_verification_report.md" || echo "0")

        cat >> "$report_file" << EOF
- **Verified READMEs**: $verified_count
- **Outdated READMEs**: $outdated_count
- **Incomplete READMEs**: $incomplete_count
EOF
    fi

    cat >> "$report_file" << EOF

### Aligned Documentation Status
EOF

    # Check aligned docs status
    if [[ -d "${ANYA_ROOT}/docs_aligned" ]]; then
        local aligned_modules=$(find "${ANYA_ROOT}/docs_aligned" -maxdepth 1 -type d | wc -l)
        echo "- **Aligned Documentation Modules**: $((aligned_modules - 1))" >> "$report_file"
        echo "- **Aligned Documentation Status**: ✅ Active" >> "$report_file"
    else
        echo "- **Aligned Documentation Status**: ❌ Not Created" >> "$report_file"
    fi

    cat >> "$report_file" << EOF

## Available Tools

### Documentation Management Scripts
- **verify_all_readmes.sh**: Comprehensive README verification against source code
- **refactor_readme_files.sh**: Automated refactoring of outdated documentation
- **create_aligned_docs.sh**: Creates 1:1 aligned documentation structure
- **validate_aligned_docs.sh**: Validates aligned documentation integrity
- **manage_docs.sh**: Ongoing documentation management operations
- **comprehensive_doc_manager.sh**: Master documentation management script (this script)

### Key Features
1. **Truth Alignment**: All documentation verified against actual source code
2. **Automated Refactoring**: Outdated documentation automatically updated
3. **Comprehensive Analysis**: Deep analysis of documentation vs implementation
4. **Continuous Validation**: Ongoing verification of documentation accuracy
5. **Structured Reporting**: Detailed reports on documentation health

## Recommendations

### For Ongoing Maintenance
1. Run verification before each release
2. Update documentation when adding new public APIs
3. Use aligned documentation structure for new modules
4. Regular synchronization with source code changes

### For Development Workflow
1. Include documentation updates in pull requests
2. Verify documentation accuracy in CI/CD pipeline
3. Use automated tools for initial documentation generation
4. Manual review for accuracy and completeness

## Report Generated
- **Timestamp**: $TIMESTAMP
- **Tool Version**: Anya Core Documentation Manager v2.0
- **Repository**: Anya Core
- **Analysis Scope**: Complete repository

---
*This report was automatically generated by the Anya Core Documentation Management System*
EOF

    echo -e "${GREEN}✅ Comprehensive report generated: $report_file${NC}"
}

# Function to execute complete workflow
complete_workflow() {
    echo -e "${MAGENTA}🚀 Executing complete documentation workflow...${NC}"
    echo ""

    analyze_repository
    echo ""

    align_documentation
    echo ""

    validate_documentation
    echo ""

    verify_readmes
    echo ""

    sync_documentation
    echo ""

    generate_report
    echo ""

    echo -e "${GREEN}✅ Complete documentation workflow finished!${NC}"
}

# Main command processing
case "${1:-help}" in
    verify)
        verify_readmes
        ;;
    refactor)
        refactor_readmes
        ;;
    analyze)
        analyze_repository
        ;;
    align)
        align_documentation
        ;;
    validate)
        validate_documentation
        ;;
    sync)
        sync_documentation
        ;;
    report)
        generate_report
        ;;
    all)
        complete_workflow
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo -e "${RED}❌ Unknown command: $1${NC}"
        echo ""
        show_help
        exit 1
        ;;
esac

echo ""
echo -e "${CYAN}📚 Anya Core Documentation Manager - Operation Complete${NC}"
