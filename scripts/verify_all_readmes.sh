#!/bin/bash

# README.md Verification and Alignment Script
# Analyzes all README.md files against actual source code implementation

set -euo pipefail

# Configuration
ANYA_ROOT="/workspaces/Anya-core"
REPORT_FILE="${ANYA_ROOT}/readme_verification_report.md"
ERRORS_FILE="${ANYA_ROOT}/readme_verification_errors.log"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Initialize report
cat > "$REPORT_FILE" << EOF
# README.md Verification Report
Generated: $TIMESTAMP

## Executive Summary
This report analyzes all README.md files in the Anya Core repository against actual source code implementation to ensure documentation truth alignment.

## Verification Results
EOF

# Initialize error log
echo "README.md Verification Errors - $TIMESTAMP" > "$ERRORS_FILE"

# Statistics
TOTAL_READMES=0
VERIFIED_READMES=0
OUTDATED_READMES=0
MISSING_CONTENT_READMES=0
EMPTY_READMES=0

echo -e "${BLUE}🔍 Starting comprehensive README.md verification...${NC}"

# Function to analyze a single README file
verify_readme() {
    local readme_path="$1"
    local relative_path="${readme_path#$ANYA_ROOT/}"
    local dir_path=$(dirname "$readme_path")
    local module_name=$(basename "$dir_path")

    TOTAL_READMES=$((TOTAL_READMES + 1))

    echo -e "${YELLOW}📋 Analyzing: $relative_path${NC}"

    # Check if README exists and is readable
    if [[ ! -f "$readme_path" || ! -r "$readme_path" ]]; then
        echo "❌ ERROR: Cannot read $relative_path" | tee -a "$ERRORS_FILE"
        return 1
    fi

    # Get README content
    local readme_size=$(wc -c < "$readme_path" 2>/dev/null || echo "0")
    local readme_lines=$(wc -l < "$readme_path" 2>/dev/null || echo "0")

    # Check if README is empty or too small
    if [[ $readme_size -lt 50 ]]; then
        EMPTY_READMES=$((EMPTY_READMES + 1))
        echo "⚠️  WARNING: $relative_path is too small ($readme_size bytes)" | tee -a "$ERRORS_FILE"

        cat >> "$REPORT_FILE" << EOF

### ❌ $relative_path
- **Status**: EMPTY/MINIMAL
- **Size**: $readme_size bytes, $readme_lines lines
- **Issue**: README file is too small to be meaningful
- **Action Required**: Add comprehensive documentation

EOF
        return 0
    fi

    # Analyze README content
    local has_title=$(grep -i "^#" "$readme_path" | head -1 | wc -l)
    local has_description=$(grep -i "description\|overview\|about" "$readme_path" | wc -l)
    local has_usage=$(grep -i "usage\|example\|how to\|getting started" "$readme_path" | wc -l)
    local has_api=$(grep -i "api\|function\|method\|struct\|trait\|interface" "$readme_path" | wc -l)
    local has_install=$(grep -i "install\|setup\|build\|compile" "$readme_path" | wc -l)

    # Look for corresponding source files
    local source_files=()
    local rust_files=0
    local js_files=0
    local py_files=0
    local other_files=0

    if [[ -d "$dir_path" ]]; then
        # Count different types of source files
        rust_files=$(find "$dir_path" -name "*.rs" -type f 2>/dev/null | wc -l)
        js_files=$(find "$dir_path" -name "*.js" -o -name "*.ts" -type f 2>/dev/null | wc -l)
        py_files=$(find "$dir_path" -name "*.py" -type f 2>/dev/null | wc -l)
        other_files=$(find "$dir_path" -type f \( -name "*.toml" -o -name "*.yaml" -o -name "*.yml" -o -name "*.json" \) 2>/dev/null | wc -l)

        # Get actual source files for content analysis
        while IFS= read -r -d '' file; do
            source_files+=("$file")
        done < <(find "$dir_path" -name "*.rs" -o -name "*.js" -o -name "*.ts" -o -name "*.py" -type f -print0 2>/dev/null | head -20)
    fi

    # Determine verification status
    local status="UNKNOWN"
    local issues=()
    local recommendations=()

    if [[ $has_title -eq 0 ]]; then
        issues+=("Missing title/header")
    fi

    if [[ $has_description -eq 0 && ${#source_files[@]} -gt 0 ]]; then
        issues+=("Missing description for module with source code")
    fi

    if [[ $rust_files -gt 0 && $has_api -eq 0 ]]; then
        issues+=("Rust module without API documentation")
        recommendations+=("Add Rust structs, traits, and function documentation")
    fi

    if [[ ${#source_files[@]} -gt 0 && $has_usage -eq 0 ]]; then
        issues+=("Missing usage examples")
        recommendations+=("Add code examples and usage patterns")
    fi

    # Check if README mentions actual structs/functions from source
    local mentions_actual_code=false
    if [[ ${#source_files[@]} -gt 0 ]]; then
        # Look for struct/function names in README
        for source_file in "${source_files[@]}"; do
            if [[ -f "$source_file" && "$source_file" == *.rs ]]; then
                # Extract struct and function names from Rust files
                local structs=$(grep -o "^pub struct [A-Za-z0-9_]*" "$source_file" 2>/dev/null | cut -d' ' -f3 || true)
                local functions=$(grep -o "^pub fn [A-Za-z0-9_]*" "$source_file" 2>/dev/null | cut -d' ' -f3 || true)

                for item in $structs $functions; do
                    if grep -q "$item" "$readme_path" 2>/dev/null; then
                        mentions_actual_code=true
                        break 2
                    fi
                done
            fi
        done
    fi

    # Final status determination
    if [[ ${#issues[@]} -eq 0 && $mentions_actual_code == true ]]; then
        status="✅ VERIFIED"
        VERIFIED_READMES=$((VERIFIED_READMES + 1))
    elif [[ ${#issues[@]} -gt 0 ]]; then
        if [[ $mentions_actual_code == false && ${#source_files[@]} -gt 0 ]]; then
            status="❌ OUTDATED"
            OUTDATED_READMES=$((OUTDATED_READMES + 1))
            issues+=("Does not reference actual code implementation")
        else
            status="⚠️  INCOMPLETE"
            MISSING_CONTENT_READMES=$((MISSING_CONTENT_READMES + 1))
        fi
    else
        status="✅ BASIC"
        VERIFIED_READMES=$((VERIFIED_READMES + 1))
    fi

    # Generate report entry
    cat >> "$REPORT_FILE" << EOF

### $status $relative_path
- **Module**: $module_name
- **Size**: $readme_size bytes, $readme_lines lines
- **Source Files**: Rust($rust_files) JS/TS($js_files) Python($py_files) Config($other_files)
- **Content Analysis**:
  - Title/Header: $([ $has_title -gt 0 ] && echo "✅" || echo "❌")
  - Description: $([ $has_description -gt 0 ] && echo "✅" || echo "❌")
  - Usage Examples: $([ $has_usage -gt 0 ] && echo "✅" || echo "❌")
  - API Documentation: $([ $has_api -gt 0 ] && echo "✅" || echo "❌")
  - Installation Info: $([ $has_install -gt 0 ] && echo "✅" || echo "❌")
  - References Real Code: $([ $mentions_actual_code == true ] && echo "✅" || echo "❌")

EOF

    if [[ ${#issues[@]} -gt 0 ]]; then
        cat >> "$REPORT_FILE" << EOF
- **Issues Found**:
EOF
        for issue in "${issues[@]}"; do
            echo "  - $issue" >> "$REPORT_FILE"
        done
    fi

    if [[ ${#recommendations[@]} -gt 0 ]]; then
        cat >> "$REPORT_FILE" << EOF
- **Recommendations**:
EOF
        for rec in "${recommendations[@]}"; do
            echo "  - $rec" >> "$REPORT_FILE"
        done
    fi

    echo "   Status: $status"
}

# Main verification loop
echo -e "${BLUE}📚 Discovering all README.md files...${NC}"

# Get all README files (excluding docs_legacy as those are legacy files)
mapfile -t readme_files < <(find "$ANYA_ROOT" -name "README.md" -type f -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/docs_legacy/*" | sort)

echo -e "${GREEN}Found ${#readme_files[@]} README.md files to verify${NC}"

# Verify each README
for readme_file in "${readme_files[@]}"; do
    verify_readme "$readme_file"
done

# Generate summary statistics
cat >> "$REPORT_FILE" << EOF

## Summary Statistics

- **Total README files analyzed**: $TOTAL_READMES
- **✅ Verified/Good**: $VERIFIED_READMES ($(( VERIFIED_READMES * 100 / TOTAL_READMES ))%)
- **❌ Outdated/Incorrect**: $OUTDATED_READMES ($(( OUTDATED_READMES * 100 / TOTAL_READMES ))%)
- **⚠️  Incomplete/Missing Content**: $MISSING_CONTENT_READMES ($(( MISSING_CONTENT_READMES * 100 / TOTAL_READMES ))%)
- **📄 Empty/Minimal**: $EMPTY_READMES ($(( EMPTY_READMES * 100 / TOTAL_READMES ))%)

## Priority Actions Required

### High Priority (Outdated Documentation)
EOF

# Add specific recommendations for outdated READMEs
if [[ $OUTDATED_READMES -gt 0 ]]; then
    echo "These README files need immediate attention as they don't reflect actual code:" >> "$REPORT_FILE"
    # We'll populate this in the next phase
fi

cat >> "$REPORT_FILE" << EOF

### Medium Priority (Incomplete Documentation)
README files that exist but need enhancement with usage examples, API docs, or better structure.

### Low Priority (Empty/Minimal Documentation)
README files that are too small and need complete rewriting.

## Verification Completed
Report generated: $TIMESTAMP
Log file: $ERRORS_FILE
EOF

echo -e "${GREEN}✅ README verification complete!${NC}"
echo -e "${BLUE}📊 Statistics:${NC}"
echo -e "   Total: $TOTAL_READMES"
echo -e "   ✅ Verified: $VERIFIED_READMES"
echo -e "   ❌ Outdated: $OUTDATED_READMES"
echo -e "   ⚠️  Incomplete: $MISSING_CONTENT_READMES"
echo -e "   📄 Empty: $EMPTY_READMES"
echo -e "${YELLOW}📋 Full report: $REPORT_FILE${NC}"
echo -e "${YELLOW}🚨 Error log: $ERRORS_FILE${NC}"
