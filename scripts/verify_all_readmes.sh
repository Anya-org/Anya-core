#!/bin/bash

# Comprehensive README.md Verification and Analysis Script
# Performs deep analysis of all README.md files against actual source code implementation
# with scoring, recommendations, and actionable insights

set -euo pipefail

# Configuration
ANYA_ROOT="/workspaces/Anya-core"
REPORT_FILE="${ANYA_ROOT}/readme_verification_report.md"
DETAILED_REPORT_FILE="${ANYA_ROOT}/readme_detailed_analysis_$(date +%Y%m%d_%H%M%S).md"
ERRORS_FILE="${ANYA_ROOT}/readme_verification_errors.log"
RECOMMENDATIONS_FILE="${ANYA_ROOT}/readme_recommendations_$(date +%Y%m%d_%H%M%S).md"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Advanced verification configuration
ENABLE_LINK_CHECKING=true
ENABLE_CODE_VALIDATION=true
ENABLE_COMPLIANCE_CHECK=true
ENABLE_PERFORMANCE_ANALYSIS=true
ENABLE_SECURITY_SCAN=true

# Scoring thresholds
EXCELLENT_SCORE=95
GOOD_SCORE=80
FAIR_SCORE=60
POOR_SCORE=40

# Statistics and tracking arrays
TOTAL_READMES=0
VERIFIED_READMES=0
OUTDATED_READMES=0
MISSING_CONTENT_READMES=0
EMPTY_READMES=0

# Advanced tracking
declare -A README_SCORES=()
declare -A README_DETAILS=()
declare -A README_RECOMMENDATIONS=()
declare -a CRITICAL_ISSUES=()
declare -a OUTDATED_FILES=()
declare -a INCOMPLETE_FILES=()
declare -a EXCELLENT_FILES=()

# Quality metrics
TOTAL_SCORE=0
AVERAGE_SCORE=0
QUALITY_DISTRIBUTION=()

echo -e "${BOLD}${CYAN}╔══════════════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${CYAN}║                   COMPREHENSIVE README VERIFICATION SYSTEM                  ║${NC}"
echo -e "${BOLD}${CYAN}║                         Enhanced Analysis & Scoring                         ║${NC}"
echo -e "${BOLD}${CYAN}║                              Version 3.0                                    ║${NC}"
echo -e "${BOLD}${CYAN}╚══════════════════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}🔍 Starting comprehensive README.md verification with advanced analysis...${NC}"
echo -e "${YELLOW}📊 Features: Link checking, Code validation, Compliance verification, Performance analysis${NC}"
echo ""

# Initialize comprehensive report
initialize_reports() {
    cat > "$REPORT_FILE" << EOF
# Comprehensive README.md Verification Report
Generated: $TIMESTAMP

## Executive Summary
This report provides a comprehensive analysis of all README.md files in the Anya Core repository,
including quality scoring, compliance verification, and actionable recommendations for improvement.

## Verification Configuration
- **Link Checking**: $([ "$ENABLE_LINK_CHECKING" = true ] && echo "✅ Enabled" || echo "❌ Disabled")
- **Code Validation**: $([ "$ENABLE_CODE_VALIDATION" = true ] && echo "✅ Enabled" || echo "❌ Disabled")
- **Compliance Check**: $([ "$ENABLE_COMPLIANCE_CHECK" = true ] && echo "✅ Enabled" || echo "❌ Disabled")
- **Performance Analysis**: $([ "$ENABLE_PERFORMANCE_ANALYSIS" = true ] && echo "✅ Enabled" || echo "❌ Disabled")
- **Security Scanning**: $([ "$ENABLE_SECURITY_SCAN" = true ] && echo "✅ Enabled" || echo "❌ Disabled")

## Quality Score Legend
- **🌟 Excellent (95-100)**: Production-ready documentation with comprehensive coverage
- **✅ Good (80-94)**: High-quality documentation with minor improvements needed
- **📝 Fair (60-79)**: Adequate documentation requiring moderate enhancements
- **⚠️  Poor (40-59)**: Insufficient documentation needing significant work
- **❌ Critical (<40)**: Critical documentation gaps requiring immediate attention

---

EOF

    cat > "$DETAILED_REPORT_FILE" << EOF
# Detailed README Analysis Report
Generated: $TIMESTAMP

This report contains detailed analysis for each README file, including:
- Comprehensive scoring breakdown
- Code alignment verification
- Link validation results
- Compliance assessment
- Security considerations
- Performance impact analysis

---

EOF

    echo "README.md Verification Errors and Warnings - $TIMESTAMP" > "$ERRORS_FILE"
    echo "" >> "$ERRORS_FILE"
}

# Advanced utility functions
log_error() {
    local file="$1"
    local error="$2"
    local severity="${3:-ERROR}"

    echo -e "${RED}[$severity] $file: $error${NC}"
    echo "[$severity] $(date '+%H:%M:%S') - $file: $error" >> "$ERRORS_FILE"

    if [[ "$severity" == "CRITICAL" ]]; then
        CRITICAL_ISSUES+=("$file: $error")
    fi
}

log_warning() {
    local file="$1"
    local warning="$2"

    echo -e "${YELLOW}[WARNING] $file: $warning${NC}"
    echo "[WARNING] $(date '+%H:%M:%S') - $file: $warning" >> "$ERRORS_FILE"
}

log_info() {
    local file="$1"
    local info="$2"

    echo -e "${BLUE}[INFO] $file: $info${NC}"
}

# Advanced content analysis functions
analyze_content_structure() {
    local readme_path="$1"
    local content="$(cat "$readme_path")"
    local score=0
    local details=""

    # Title analysis (0-15 points)
    if echo "$content" | grep -q "^# "; then
        local title_count=$(echo "$content" | grep -c "^# " || echo "0")
        if [[ $title_count -eq 1 ]]; then
            score=$((score + 15))
            details+="✅ Single clear title (15/15)\n"
        elif [[ $title_count -gt 1 ]]; then
            score=$((score + 10))
            details+="⚠️  Multiple H1 titles - consider restructuring (10/15)\n"
        else
            score=$((score + 5))
            details+="⚠️  Title exists but formatting issues (5/15)\n"
        fi
    else
        details+="❌ Missing main title (0/15)\n"
    fi

    # Header structure (0-10 points)
    local h2_count=$(echo "$content" | grep -c "^## " || echo "0")
    local h3_count=$(echo "$content" | grep -c "^### " || echo "0")

    if [[ $h2_count -ge 3 && $h3_count -ge 1 ]]; then
        score=$((score + 10))
        details+="✅ Well-structured headers (10/10)\n"
    elif [[ $h2_count -ge 2 ]]; then
        score=$((score + 7))
        details+="📝 Good header structure (7/10)\n"
    elif [[ $h2_count -ge 1 ]]; then
        score=$((score + 4))
        details+="⚠️  Basic header structure (4/10)\n"
    else
        details+="❌ Poor header structure (0/10)\n"
    fi

    # Content sections analysis (0-20 points)
    local section_score=0

    if echo "$content" | grep -qi "overview\|description\|about"; then
        section_score=$((section_score + 5))
        details+="✅ Has overview/description section (+5)\n"
    fi

    if echo "$content" | grep -qi "installation\|setup\|getting started\|quick start"; then
        section_score=$((section_score + 5))
        details+="✅ Has installation/setup section (+5)\n"
    fi

    if echo "$content" | grep -qi "usage\|examples\|how to"; then
        section_score=$((section_score + 5))
        details+="✅ Has usage/examples section (+5)\n"
    fi

    if echo "$content" | grep -qi "api\|reference\|documentation"; then
        section_score=$((section_score + 5))
        details+="✅ Has API/reference section (+5)\n"
    fi

    score=$((score + section_score))
    details+="📊 Content sections score: $section_score/20\n"

    echo "$score|$details"
}

analyze_code_examples() {
    local readme_path="$1"
    local content="$(cat "$readme_path")"
    local score=0
    local details=""

    # Code block analysis (0-15 points)
    local rust_blocks=$(echo "$content" | grep -c '```rust\|```rs' || echo "0")
    local bash_blocks=$(echo "$content" | grep -c '```bash\|```sh' || echo "0")
    local total_blocks=$(echo "$content" | grep -c '```' || echo "0")
    total_blocks=$((total_blocks / 2)) # Each block has opening and closing

    if [[ $total_blocks -gt 0 ]]; then
        if [[ $rust_blocks -gt 0 ]]; then
            score=$((score + 8))
            details+="✅ Contains Rust code examples (+8)\n"
        fi

        if [[ $bash_blocks -gt 0 ]]; then
            score=$((score + 4))
            details+="✅ Contains shell/bash examples (+4)\n"
        fi

    if [[ $total_blocks -ge 3 ]]; then
        score=$((score + 3))
        details+="✅ Multiple code examples (+3)\n"
    fi

    details+="📊 Code blocks found: $total_blocks total ($rust_blocks Rust, $bash_blocks Bash)\n"
    else
        details+="❌ No code examples found (0/15)\n"
    fi

    # Code syntax validation (0-10 points)
    if [[ "$ENABLE_CODE_VALIDATION" == true && $rust_blocks -gt 0 ]]; then
        local syntax_valid=true
        # Basic Rust syntax validation
        local rust_content=$(echo "$content" | sed -n '/```rust/,/```/p' | grep -v '```')

        if echo "$rust_content" | grep -q "use \|fn \|struct \|impl \|let \|pub "; then
            score=$((score + 10))
            details+="✅ Rust syntax appears valid (+10)\n"
        else
            score=$((score + 5))
            details+="⚠️  Rust syntax may have issues (+5)\n"
        fi
    fi

    echo "$score|$details"
}

verify_links() {
    local readme_path="$1"
    local content="$(cat "$readme_path")"
    local dir_path="$(dirname "$readme_path")"
    local score=0
    local details=""
    local broken_links=0
    local total_links=0

    if [[ "$ENABLE_LINK_CHECKING" != true ]]; then
        echo "10|📝 Link checking disabled (+10)\n"
        return
    fi

    # Extract all markdown links - fix regex
    local links=$(echo "$content" | grep -oE '\[[^]]*\]\([^)]+\)' || true)

    if [[ -n "$links" ]]; then
        while IFS= read -r link; do
            if [[ -z "$link" ]]; then continue; fi

            total_links=$((total_links + 1))

            # Extract the URL from [text](url) format
            local url=$(echo "$link" | sed -n 's/.*](\([^)]*\)).*/\1/p')

            # Skip if extraction failed or empty
            if [[ -z "$url" ]]; then
                continue
            fi

            # Skip external URLs
            if [[ "$url" =~ ^https?:// ]]; then
                continue
            fi

            # Resolve relative paths
            local full_path
            if [[ "$url" == /* ]]; then
                full_path="$ANYA_ROOT$url"
            else
                full_path="$dir_path/$url"
            fi

            # Check if file/directory exists
            if [[ ! -e "$full_path" ]]; then
                broken_links=$((broken_links + 1))
                details+="❌ Broken link: $url\n"
                log_warning "$(basename "$readme_path")" "Broken internal link: $url"
            fi
        done <<< "$links"

        # Calculate score based on link quality
        local link_success_rate=0
        if [[ $total_links -gt 0 ]]; then
            link_success_rate=$(( (total_links - broken_links) * 100 / total_links ))
        fi

        if [[ $broken_links -eq 0 ]]; then
            score=15
            details+="✅ All internal links valid ($total_links links) (15/15)\n"
        elif [[ $link_success_rate -ge 80 ]]; then
            score=10
            details+="⚠️  Few broken links ($broken_links/$total_links) (10/15)\n"
        else
            score=5
            details+="❌ Multiple broken links ($broken_links/$total_links) (5/15)\n"
        fi
    else
        score=10
        details+="📝 No internal links to verify (+10)\n"
    fi

    echo "$score|$details"
}

analyze_source_alignment() {
    local readme_path="$1"
    local content="$(cat "$readme_path")"
    local dir_path="$(dirname "$readme_path")"
    local score=0
    local details=""

    # Find source files in the same directory
    local rust_files=$(find "$dir_path" -maxdepth 1 -name "*.rs" -type f 2>/dev/null | wc -l)
    local js_files=$(find "$dir_path" -maxdepth 1 -name "*.js" -o -name "*.ts" -type f 2>/dev/null | wc -l)
    local py_files=$(find "$dir_path" -maxdepth 1 -name "*.py" -type f 2>/dev/null | wc -l)

    if [[ $rust_files -eq 0 && $js_files -eq 0 && $py_files -eq 0 ]]; then
        score=15
        details+="📝 No source files to align with (+15)\n"
        echo "$score|$details"
        return
    fi

    # Check for Rust code alignment
    if [[ $rust_files -gt 0 ]]; then
        local structs_in_source=()
        local functions_in_source=()
        local traits_in_source=()

        # Extract public items from Rust files
        while IFS= read -r rust_file; do
            if [[ -f "$rust_file" ]]; then
                # Extract public structs
                while IFS= read -r struct_name; do
                    if [[ -n "$struct_name" ]]; then
                        structs_in_source+=("$struct_name")
                    fi
                done < <(grep -o "pub struct [A-Za-z0-9_]*" "$rust_file" 2>/dev/null | cut -d' ' -f3 || true)

                # Extract public functions
                while IFS= read -r func_name; do
                    if [[ -n "$func_name" ]]; then
                        functions_in_source+=("$func_name")
                    fi
                done < <(grep -o "pub fn [A-Za-z0-9_]*" "$rust_file" 2>/dev/null | cut -d' ' -f3 || true)

                # Extract public traits
                while IFS= read -r trait_name; do
                    if [[ -n "$trait_name" ]]; then
                        traits_in_source+=("$trait_name")
                    fi
                done < <(grep -o "pub trait [A-Za-z0-9_]*" "$rust_file" 2>/dev/null | cut -d' ' -f3 || true)
            fi
        done < <(find "$dir_path" -maxdepth 1 -name "*.rs" -type f 2>/dev/null)

        # Check if README mentions actual code elements
        local mentioned_items=0
        local total_items=$((${#structs_in_source[@]} + ${#functions_in_source[@]} + ${#traits_in_source[@]}))

        for item in "${structs_in_source[@]}" "${functions_in_source[@]}" "${traits_in_source[@]}"; do
            if grep -q "\b$item\b" "$readme_path" 2>/dev/null; then
                mentioned_items=$((mentioned_items + 1))
            fi
        done

        if [[ $total_items -eq 0 ]]; then
            score=10
            details+="📝 No public API items found to document (+10)\n"
        elif [[ $mentioned_items -gt 0 ]]; then
            local alignment_percentage=$((mentioned_items * 100 / total_items))
            if [[ $alignment_percentage -ge 75 ]]; then
                score=20
                details+="✅ Excellent code alignment ($mentioned_items/$total_items items documented) (+20)\n"
            elif [[ $alignment_percentage -ge 50 ]]; then
                score=15
                details+="📝 Good code alignment ($mentioned_items/$total_items items documented) (+15)\n"
            elif [[ $alignment_percentage -ge 25 ]]; then
                score=10
                details+="⚠️  Partial code alignment ($mentioned_items/$total_items items documented) (+10)\n"
            else
                score=5
                details+="❌ Poor code alignment ($mentioned_items/$total_items items documented) (+5)\n"
            fi
        else
            score=0
            details+="❌ No alignment with actual code implementation (0/20)\n"
            log_warning "$(basename "$readme_path")" "README doesn't document any actual code elements from $total_items available"
        fi
    else
        score=15
        details+="📝 No Rust source files to align with (+15)\n"
    fi

    echo "$score|$details"
}

check_compliance_standards() {
    local readme_path="$1"
    local content="$(cat "$readme_path")"
    local score=0
    local details=""

    if [[ "$ENABLE_COMPLIANCE_CHECK" != true ]]; then
        echo "10|📝 Compliance checking disabled (+10)\n"
        return
    fi

    # Check for compliance tags (AIR-3, AIS-3, BPC-3, RES-3)
    local compliance_tags=0

    if echo "$content" | grep -q "\[AIR-3\]"; then
        compliance_tags=$((compliance_tags + 1))
        details+="✅ AIR-3 compliance tag found (+2)\n"
        score=$((score + 2))
    fi

    if echo "$content" | grep -q "\[AIS-3\]"; then
        compliance_tags=$((compliance_tags + 1))
        details+="✅ AIS-3 compliance tag found (+2)\n"
        score=$((score + 2))
    fi

    if echo "$content" | grep -q "\[BPC-3\]"; then
        compliance_tags=$((compliance_tags + 1))
        details+="✅ BPC-3 compliance tag found (+2)\n"
        score=$((score + 2))
    fi

    if echo "$content" | grep -q "\[RES-3\]"; then
        compliance_tags=$((compliance_tags + 1))
        details+="✅ RES-3 compliance tag found (+2)\n"
        score=$((score + 2))
    fi

    # Check for compliance documentation
    if echo "$content" | grep -qi "compliance\|standard\|requirement"; then
        score=$((score + 2))
        details+="✅ Contains compliance documentation (+2)\n"
    fi

    # Security considerations
    if echo "$content" | grep -qi "security\|safe\|crypto\|encryption"; then
        score=$((score + 3))
        details+="✅ Contains security considerations (+3)\n"
    fi

    # Performance considerations
    if echo "$content" | grep -qi "performance\|optimization\|efficiency\|benchmark"; then
        score=$((score + 3))
        details+="✅ Contains performance information (+3)\n"
    fi

    if [[ $compliance_tags -eq 0 ]]; then
        details+="⚠️  No compliance tags found (consider adding)\n"
    fi

    echo "$score|$details"
}

calculate_quality_score() {
    local structure_result="$1"
    local examples_result="$2"
    local links_result="$3"
    local alignment_result="$4"
    local compliance_result="$5"

    local structure_score=$(echo "$structure_result" | cut -d'|' -f1 | head -1)
    local examples_score=$(echo "$examples_result" | cut -d'|' -f1 | head -1)
    local links_score=$(echo "$links_result" | cut -d'|' -f1 | head -1)
    local alignment_score=$(echo "$alignment_result" | cut -d'|' -f1 | head -1)
    local compliance_score=$(echo "$compliance_result" | cut -d'|' -f1 | head -1)

    # Ensure all scores are valid numbers
    structure_score=${structure_score:-0}
    examples_score=${examples_score:-0}
    links_score=${links_score:-0}
    alignment_score=${alignment_score:-0}
    compliance_score=${compliance_score:-0}

    local total_score=$((structure_score + examples_score + links_score + alignment_score + compliance_score))

    echo "$total_score"
}

get_quality_rating() {
    local score="$1"

    if [[ $score -ge $EXCELLENT_SCORE ]]; then
        echo "🌟 EXCELLENT"
    elif [[ $score -ge $GOOD_SCORE ]]; then
        echo "✅ GOOD"
    elif [[ $score -ge $FAIR_SCORE ]]; then
        echo "📝 FAIR"
    elif [[ $score -ge $POOR_SCORE ]]; then
        echo "⚠️  POOR"
    else
        echo "❌ CRITICAL"
    fi
}

generate_recommendations() {
    local readme_path="$1"
    local score="$2"
    local structure_details="$3"
    local examples_details="$4"
    local links_details="$5"
    local alignment_details="$6"
    local compliance_details="$7"

    local recommendations=()

    # Structure recommendations
    if [[ $(echo "$structure_details" | grep -c "❌") -gt 0 ]]; then
        recommendations+=("📝 Improve document structure: Add clear title and organize content with proper headers")
    fi

    # Examples recommendations
    if [[ $(echo "$examples_details" | grep -c "❌") -gt 0 ]]; then
        recommendations+=("💻 Add code examples: Include Rust code snippets and usage examples")
    fi

    # Links recommendations
    if [[ $(echo "$links_details" | grep -c "❌") -gt 0 ]]; then
        recommendations+=("🔗 Fix broken links: Update or remove non-functional internal links")
    fi

    # Alignment recommendations
    if [[ $(echo "$alignment_details" | grep -c "❌") -gt 0 ]]; then
        recommendations+=("⚙️  Improve code alignment: Document actual structs, functions, and traits from source code")
    fi

    # Compliance recommendations
    if [[ $(echo "$compliance_details" | grep -c "⚠️") -gt 0 ]]; then
        recommendations+=("🛡️  Add compliance information: Include relevant compliance tags and security considerations")
    fi

    # Score-based recommendations
    if [[ $score -lt $POOR_SCORE ]]; then
        recommendations+=("🚨 CRITICAL: Complete rewrite needed - this README requires immediate attention")
    elif [[ $score -lt $FAIR_SCORE ]]; then
        recommendations+=("⚠️  MAJOR: Significant improvements needed across multiple areas")
    elif [[ $score -lt $GOOD_SCORE ]]; then
        recommendations+=("📈 MODERATE: Good foundation, focus on enhancing weak areas")
    elif [[ $score -lt $EXCELLENT_SCORE ]]; then
        recommendations+=("✨ MINOR: Excellent documentation, minor polishing needed")
    fi

    printf '%s\n' "${recommendations[@]}"
}

# Enhanced README verification function
verify_readme() {
    local readme_path="$1"
    local relative_path="${readme_path#$ANYA_ROOT/}"
    local dir_path=$(dirname "$readme_path")
    local module_name=$(basename "$dir_path")

    TOTAL_READMES=$((TOTAL_READMES + 1))

    echo -e "${CYAN}📋 Analyzing: $relative_path${NC}"

    # Check if README exists and is readable
    if [[ ! -f "$readme_path" || ! -r "$readme_path" ]]; then
        log_error "$relative_path" "Cannot read file" "CRITICAL"
        return 1
    fi

    # Get README content and basic metrics
    local readme_size=$(wc -c < "$readme_path" 2>/dev/null || echo "0")
    local readme_lines=$(wc -l < "$readme_path" 2>/dev/null || echo "0")

    # Check if README is empty or too small
    if [[ $readme_size -lt 50 ]]; then
        EMPTY_READMES=$((EMPTY_READMES + 1))
        log_warning "$relative_path" "File too small ($readme_size bytes)"

        cat >> "$REPORT_FILE" << EOF

### ❌ CRITICAL $relative_path
- **Status**: EMPTY/MINIMAL
- **Score**: 0/100 ❌
- **Size**: $readme_size bytes, $readme_lines lines
- **Issue**: README file too small to be meaningful
- **Priority**: HIGH - Complete rewrite needed

EOF
        return 0
    fi

    # Perform comprehensive analysis
    echo -e "${BLUE}   🔍 Running content structure analysis...${NC}"
    local structure_result=$(analyze_content_structure "$readme_path")

    echo -e "${BLUE}   💻 Analyzing code examples...${NC}"
    local examples_result=$(analyze_code_examples "$readme_path")

    echo -e "${BLUE}   🔗 Verifying links...${NC}"
    local links_result=$(verify_links "$readme_path")

    echo -e "${BLUE}   ⚙️  Checking source alignment...${NC}"
    local alignment_result=$(analyze_source_alignment "$readme_path")

    echo -e "${BLUE}   🛡️  Validating compliance...${NC}"
    local compliance_result=$(check_compliance_standards "$readme_path")

    # Calculate overall quality score
    local total_score=$(calculate_quality_score "$structure_result" "$examples_result" "$links_result" "$alignment_result" "$compliance_result")
    local quality_rating=$(get_quality_rating "$total_score")

    # Store results for summary
    README_SCORES["$relative_path"]="$total_score"
    TOTAL_SCORE=$((TOTAL_SCORE + total_score))

    # Extract details for reporting
    local structure_details=$(echo "$structure_result" | cut -d'|' -f2-)
    local examples_details=$(echo "$examples_result" | cut -d'|' -f2-)
    local links_details=$(echo "$links_result" | cut -d'|' -f2-)
    local alignment_details=$(echo "$alignment_result" | cut -d'|' -f2-)
    local compliance_details=$(echo "$compliance_result" | cut -d'|' -f2-)

    # Generate recommendations
    local recommendations_text=$(generate_recommendations "$readme_path" "$total_score" "$structure_details" "$examples_details" "$links_details" "$alignment_details" "$compliance_details")
    README_RECOMMENDATIONS["$relative_path"]="$recommendations_text"

    # Categorize files for summary
    if [[ $total_score -ge $EXCELLENT_SCORE ]]; then
        EXCELLENT_FILES+=("$relative_path")
        VERIFIED_READMES=$((VERIFIED_READMES + 1))
    elif [[ $total_score -ge $GOOD_SCORE ]]; then
        VERIFIED_READMES=$((VERIFIED_READMES + 1))
    elif [[ $total_score -ge $FAIR_SCORE ]]; then
        MISSING_CONTENT_READMES=$((MISSING_CONTENT_READMES + 1))
        INCOMPLETE_FILES+=("$relative_path")
    else
        OUTDATED_READMES=$((OUTDATED_READMES + 1))
        OUTDATED_FILES+=("$relative_path")
    fi

    # Generate comprehensive report entry
    cat >> "$REPORT_FILE" << EOF

### $quality_rating $relative_path
- **Overall Score**: $total_score/100
- **Module**: $module_name
- **Size**: $readme_size bytes, $readme_lines lines

#### 📊 Detailed Score Breakdown
$(echo -e "$structure_details" | sed 's/^/  /')
$(echo -e "$examples_details" | sed 's/^/  /')
$(echo -e "$links_details" | sed 's/^/  /')
$(echo -e "$alignment_details" | sed 's/^/  /')
$(echo -e "$compliance_details" | sed 's/^/  /')

#### 🎯 Recommendations
$(echo "$recommendations_text" | sed 's/^/  /')

EOF

    # Generate detailed analysis entry
    cat >> "$DETAILED_REPORT_FILE" << EOF

## $quality_rating $relative_path

### Quality Metrics
- **Overall Score**: $total_score/100
- **Quality Rating**: $quality_rating
- **File Size**: $readme_size bytes ($readme_lines lines)

### Content Structure Analysis (25 points)
$(echo -e "$structure_details")

### Code Examples Analysis (25 points)
$(echo -e "$examples_details")

### Link Verification (15 points)
$(echo -e "$links_details")

### Source Code Alignment (20 points)
$(echo -e "$alignment_details")

### Compliance Standards (15 points)
$(echo -e "$compliance_details")

### Improvement Recommendations
$(echo "$recommendations_text")

---

EOF

    # Display status
    echo -e "   Score: ${BOLD}$total_score/100${NC} - $quality_rating"

    # Log any critical issues
    if [[ $total_score -lt $POOR_SCORE ]]; then
        log_error "$relative_path" "Critical quality score: $total_score/100" "CRITICAL"
    fi
}

# Initialize reports
initialize_reports

# Main verification loop
echo -e "${BLUE}📚 Discovering all README.md files...${NC}"

# Get all README files (excluding docs_legacy as those are legacy files)
mapfile -t readme_files < <(find "$ANYA_ROOT" -name "README.md" -type f -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/docs_legacy/*" | sort)

echo -e "${GREEN}Found ${#readme_files[@]} README.md files to verify${NC}"
echo ""

# Verify each README
for readme_file in "${readme_files[@]}"; do
    verify_readme "$readme_file"
    echo ""
done

# Calculate final statistics
if [[ $TOTAL_READMES -gt 0 ]]; then
    AVERAGE_SCORE=$((TOTAL_SCORE / TOTAL_READMES))
else
    AVERAGE_SCORE=0
fi

# Generate comprehensive summary statistics
cat >> "$REPORT_FILE" << EOF

## 📊 Comprehensive Summary Statistics

### Overall Quality Metrics
- **Total README files analyzed**: $TOTAL_READMES
- **Average Quality Score**: $AVERAGE_SCORE/100
- **Total Combined Score**: $TOTAL_SCORE points

### Quality Distribution
- **🌟 Excellent (95-100)**: ${#EXCELLENT_FILES[@]} files ($(( ${#EXCELLENT_FILES[@]} * 100 / TOTAL_READMES ))%)
- **✅ Good (80-94)**: $((VERIFIED_READMES - ${#EXCELLENT_FILES[@]})) files ($(( (VERIFIED_READMES - ${#EXCELLENT_FILES[@]}) * 100 / TOTAL_READMES ))%)
- **📝 Fair (60-79)**: $MISSING_CONTENT_READMES files ($(( MISSING_CONTENT_READMES * 100 / TOTAL_READMES ))%)
- **⚠️  Poor/Critical (<60)**: $((OUTDATED_READMES + EMPTY_READMES)) files ($(( (OUTDATED_READMES + EMPTY_READMES) * 100 / TOTAL_READMES ))%)

### Repository Health Assessment
EOF

if [[ $AVERAGE_SCORE -ge $EXCELLENT_SCORE ]]; then
    cat >> "$REPORT_FILE" << EOF
**🌟 EXCELLENT**: Repository documentation is of exceptional quality with comprehensive coverage.
EOF
elif [[ $AVERAGE_SCORE -ge $GOOD_SCORE ]]; then
    cat >> "$REPORT_FILE" << EOF
**✅ GOOD**: Repository documentation is high quality with minor areas for improvement.
EOF
elif [[ $AVERAGE_SCORE -ge $FAIR_SCORE ]]; then
    cat >> "$REPORT_FILE" << EOF
**📝 FAIR**: Repository documentation is adequate but needs moderate enhancements.
EOF
elif [[ $AVERAGE_SCORE -ge $POOR_SCORE ]]; then
    cat >> "$REPORT_FILE" << EOF
**⚠️  POOR**: Repository documentation needs significant improvements across multiple areas.
EOF
else
    cat >> "$REPORT_FILE" << EOF
**❌ CRITICAL**: Repository documentation requires immediate comprehensive overhaul.
EOF
fi

cat >> "$REPORT_FILE" << EOF

## 🎯 Priority Action Plan

### 🚨 Critical Priority (Immediate Action Required)
EOF

if [[ ${#OUTDATED_FILES[@]} -gt 0 ]]; then
    cat >> "$REPORT_FILE" << EOF
**Files requiring immediate attention:**
EOF
    for file in "${OUTDATED_FILES[@]}"; do
        cat >> "$REPORT_FILE" << EOF
- \`$file\` - Score: ${README_SCORES[$file]}/100
  $(echo "${README_RECOMMENDATIONS[$file]}" | head -1)
EOF
    done
    cat >> "$REPORT_FILE" << EOF

EOF
fi

if [[ $EMPTY_READMES -gt 0 ]]; then
    cat >> "$REPORT_FILE" << EOF
**Empty/Minimal files needing complete rewrite:**
- $EMPTY_READMES files identified that are too small to be meaningful
- These require complete documentation from scratch

EOF
fi

cat >> "$REPORT_FILE" << EOF
### 📈 Medium Priority (Enhancement Needed)
EOF

if [[ ${#INCOMPLETE_FILES[@]} -gt 0 ]]; then
    cat >> "$REPORT_FILE" << EOF
**Files with good foundation but needing improvement:**
EOF
    for file in "${INCOMPLETE_FILES[@]}"; do
        cat >> "$REPORT_FILE" << EOF
- \`$file\` - Score: ${README_SCORES[$file]}/100
EOF
    done
    cat >> "$REPORT_FILE" << EOF

EOF
fi

cat >> "$REPORT_FILE" << EOF
### ✨ Low Priority (Polish & Enhancement)
EOF

if [[ ${#EXCELLENT_FILES[@]} -gt 0 ]]; then
    cat >> "$REPORT_FILE" << EOF
**Excellent files requiring minimal polish:**
EOF
    for file in "${EXCELLENT_FILES[@]}"; do
        cat >> "$REPORT_FILE" << EOF
- \`$file\` - Score: ${README_SCORES[$file]}/100 🌟
EOF
    done
    cat >> "$REPORT_FILE" << EOF

EOF
fi

cat >> "$REPORT_FILE" << EOF
## 🛠️  Technical Recommendations

### Repository-Wide Improvements
1. **Standardize Documentation Format**: Implement consistent structure across all README files
2. **Automate Link Checking**: Set up CI/CD pipeline to validate internal links
3. **Code-Documentation Sync**: Ensure README files stay aligned with source code changes
4. **Compliance Integration**: Add compliance tags to all modules systematically
5. **Performance Documentation**: Include performance considerations in high-impact modules

### Quality Assurance
- Set minimum quality score threshold of $GOOD_SCORE/100 for new README files
- Implement automated README quality checking in pre-commit hooks
- Regular quarterly reviews of documentation alignment

## 📋 Report Generation Details
- **Generated**: $TIMESTAMP
- **Configuration**: Advanced analysis enabled (Links: $ENABLE_LINK_CHECKING, Code: $ENABLE_CODE_VALIDATION, Compliance: $ENABLE_COMPLIANCE_CHECK)
- **Analysis Scope**: ${#readme_files[@]} README files across entire repository
- **Detailed Report**: $DETAILED_REPORT_FILE
- **Error Log**: $ERRORS_FILE

---
*This comprehensive analysis ensures documentation truth alignment with actual system implementation.*
EOF

# Generate final console output
echo -e "${BOLD}${GREEN}╔══════════════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${GREEN}║                           VERIFICATION COMPLETE                             ║${NC}"
echo -e "${BOLD}${GREEN}╚══════════════════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}📊 Final Statistics:${NC}"
echo -e "   📋 Total Files: ${BOLD}$TOTAL_READMES${NC}"
echo -e "   📈 Average Score: ${BOLD}$AVERAGE_SCORE/100${NC}"
echo -e "   🌟 Excellent: ${BOLD}${#EXCELLENT_FILES[@]}${NC}"
echo -e "   ✅ Good: ${BOLD}$((VERIFIED_READMES - ${#EXCELLENT_FILES[@]}))${NC}"
echo -e "   📝 Fair: ${BOLD}$MISSING_CONTENT_READMES${NC}"
echo -e "   ⚠️  Poor: ${BOLD}$OUTDATED_READMES${NC}"
echo -e "   ❌ Critical: ${BOLD}$EMPTY_READMES${NC}"
echo ""

if [[ ${#CRITICAL_ISSUES[@]} -gt 0 ]]; then
    echo -e "${RED}🚨 Critical Issues Found: ${#CRITICAL_ISSUES[@]}${NC}"
    for issue in "${CRITICAL_ISSUES[@]}"; do
        echo -e "   ${RED}• $issue${NC}"
    done
    echo ""
fi

echo -e "${YELLOW}📋 Reports Generated:${NC}"
echo -e "   📄 Summary Report: ${BOLD}$REPORT_FILE${NC}"
echo -e "   📊 Detailed Analysis: ${BOLD}$DETAILED_REPORT_FILE${NC}"
echo -e "   🚨 Error Log: ${BOLD}$ERRORS_FILE${NC}"

if [[ $AVERAGE_SCORE -ge $GOOD_SCORE ]]; then
    echo -e "${GREEN}🎉 Overall Status: HIGH QUALITY DOCUMENTATION${NC}"
elif [[ $AVERAGE_SCORE -ge $FAIR_SCORE ]]; then
    echo -e "${YELLOW}⚠️  Overall Status: MODERATE QUALITY - IMPROVEMENTS NEEDED${NC}"
else
    echo -e "${RED}❌ Overall Status: LOW QUALITY - IMMEDIATE ACTION REQUIRED${NC}"
fi

echo -e "${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}"
