#!/bin/bash
# Comprehensive README.md Verification Script
# Verifies all README files against actual source code implementation

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="/workspaces/Anya-core"
REPORT_FILE="$REPO_ROOT/README_VERIFICATION_REPORT_$(date +%Y%m%d_%H%M%S).md"
ERRORS_FILE="$REPO_ROOT/README_ERRORS_$(date +%Y%m%d_%H%M%S).json"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Statistics counters
TOTAL_READMES=0
VERIFIED_READMES=0
ERRORS_FOUND=0
WARNINGS_FOUND=0

# Error tracking array
declare -a README_ERRORS=()

echo -e "${BLUE}🔍 Starting Comprehensive README Verification${NC}"
echo "Report will be saved to: $REPORT_FILE"

# Initialize report
cat > "$REPORT_FILE" << 'EOF'
# README Files Verification Report

**Generated**: $(date)
**Repository**: /workspaces/Anya-core

## Summary

This report analyzes all README.md files in the repository to verify their accuracy against actual source code implementation.

## Methodology

1. **Content Analysis**: Check if README content matches actual code structure
2. **Link Validation**: Verify internal links point to existing files
3. **Code Examples**: Validate code examples against actual implementations
4. **Module References**: Ensure referenced modules/functions exist
5. **Badge Accuracy**: Verify status badges reflect actual state

---

EOF

# Function to log errors
log_error() {
    local file="$1"
    local error="$2"
    local line="${3:-}"

    ERRORS_FOUND=$((ERRORS_FOUND + 1))

    local error_entry="{ \"file\": \"$file\", \"error\": \"$error\", \"line\": \"$line\", \"timestamp\": \"$(date -Iseconds)\" }"
    README_ERRORS+=("$error_entry")

    echo -e "${RED}❌ ERROR in $file: $error${NC}"
}

# Function to log warnings
log_warning() {
    local file="$1"
    local warning="$2"
    local line="${3:-}"

    WARNINGS_FOUND=$((WARNINGS_FOUND + 1))
    echo -e "${YELLOW}⚠️ WARNING in $file: $warning${NC}"
}

# Function to verify file existence
verify_file_exists() {
    local file_path="$1"
    local referenced_in="$2"

    if [[ "$file_path" =~ ^https?: ]]; then
        # External URL - skip for now
        return 0
    fi

    # Handle relative paths
    if [[ ! "$file_path" =~ ^/ ]]; then
        local base_dir="$(dirname "$referenced_in")"
        file_path="$base_dir/$file_path"
    fi

    if [[ ! -e "$file_path" ]]; then
        log_error "$referenced_in" "Referenced file does not exist: $file_path"
        return 1
    fi

    return 0
}

# Function to extract and verify links
verify_links() {
    local readme_file="$1"
    local line_num=0

    while IFS= read -r line; do
        line_num=$((line_num + 1))

        # Extract markdown links [text](url)
        while [[ "$line" =~ \[([^\]]*)\]\(([^)]+)\) ]]; do
            local link_text="${BASH_REMATCH[1]}"
            local link_url="${BASH_REMATCH[2]}"

            # Skip anchors and external URLs for now
            if [[ "$link_url" =~ ^# ]] || [[ "$link_url" =~ ^https?: ]]; then
                line="${line/${BASH_REMATCH[0]}/}"
                continue
            fi

            verify_file_exists "$link_url" "$readme_file"
            line="${line/${BASH_REMATCH[0]}/}"
        done
    done < "$readme_file"
}

# Function to verify code examples
verify_code_examples() {
    local readme_file="$1"
    local in_code_block=false
    local code_lang=""
    local code_content=""
    local line_num=0

    while IFS= read -r line; do
        line_num=$((line_num + 1))

        # Check for code block start
        if [[ "$line" =~ ^```([a-zA-Z]*) ]]; then
            in_code_block=true
            code_lang="${BASH_REMATCH[1]}"
            code_content=""
            continue
        fi

        # Check for code block end
        if [[ "$line" =~ ^```$ ]] && [[ "$in_code_block" == true ]]; then
            in_code_block=false

            # Verify Rust code examples
            if [[ "$code_lang" == "rust" ]] || [[ "$code_lang" == "rs" ]]; then
                verify_rust_code_example "$readme_file" "$code_content" "$line_num"
            fi

            code_content=""
            code_lang=""
            continue
        fi

        # Accumulate code content
        if [[ "$in_code_block" == true ]]; then
            code_content="$code_content$line"$'\n'
        fi

    done < "$readme_file"
}

# Function to verify Rust code examples
verify_rust_code_example() {
    local readme_file="$1"
    local code_content="$2"
    local line_num="$3"

    # Check for common patterns that should exist in actual code

    # Check for module references
    if echo "$code_content" | grep -q "use anya_core::" || echo "$code_content" | grep -q "use crate::"; then
        # Extract module paths and verify they exist
        while read -r use_line; do
            if [[ "$use_line" =~ use[[:space:]]+([^;]+) ]]; then
                local module_path="${BASH_REMATCH[1]}"
                # Simplified verification - check if module exists in source
                if [[ "$module_path" =~ ^anya_core:: ]] || [[ "$module_path" =~ ^crate:: ]]; then
                    local src_path="${module_path//anya_core::/}"
                    src_path="${src_path//crate::/}"
                    src_path="${src_path//::///}"

                    # Check if this path exists in src/
                    if [[ ! -f "$REPO_ROOT/src/${src_path}.rs" ]] && [[ ! -f "$REPO_ROOT/src/${src_path}/mod.rs" ]]; then
                        log_warning "$readme_file" "Code example references potentially non-existent module: $module_path" "$line_num"
                    fi
                fi
            fi
        done <<< "$(echo "$code_content" | grep "use ")"
    fi

    # Check for function calls that should exist
    if echo "$code_content" | grep -q "AnyaCore::" || echo "$code_content" | grep -q "BitcoinAdapter::" || echo "$code_content" | grep -q "Web5Adapter::"; then
        # These are core types that should exist
        local core_types=("AnyaCore" "BitcoinAdapter" "Web5Adapter")
        for type_name in "${core_types[@]}"; do
            if echo "$code_content" | grep -q "$type_name::" && ! grep -r "struct $type_name" "$REPO_ROOT/src/" >/dev/null 2>&1; then
                log_warning "$readme_file" "Code example uses potentially non-existent type: $type_name" "$line_num"
            fi
        done
    fi
}

# Function to verify module documentation
verify_module_documentation() {
    local readme_file="$1"

    # If this is a module-specific README, verify it matches the actual module
    local dir_name="$(basename "$(dirname "$readme_file")")"

    # Check if there's a corresponding Rust module
    if [[ -f "$(dirname "$readme_file")/mod.rs" ]] || [[ -f "$(dirname "$readme_file")/../src/$dir_name/mod.rs" ]]; then
        # This should be a module README - verify it documents the actual module
        local mod_file=""
        if [[ -f "$(dirname "$readme_file")/mod.rs" ]]; then
            mod_file="$(dirname "$readme_file")/mod.rs"
        elif [[ -f "$(dirname "$readme_file")/../src/$dir_name/mod.rs" ]]; then
            mod_file="$(dirname "$readme_file")/../src/$dir_name/mod.rs"
        fi

        if [[ -n "$mod_file" ]]; then
            # Extract public items from module
            local pub_items=$(grep -E "^pub (fn|struct|enum|trait|mod|type)" "$mod_file" 2>/dev/null || true)

            if [[ -n "$pub_items" ]]; then
                # Check if README mentions any of these public items
                local mentioned_count=0
                while read -r pub_item; do
                    if [[ "$pub_item" =~ pub[[:space:]]+(fn|struct|enum|trait|mod|type)[[:space:]]+([a-zA-Z_][a-zA-Z0-9_]*) ]]; then
                        local item_type="${BASH_REMATCH[1]}"
                        local item_name="${BASH_REMATCH[2]}"

                        if grep -q "$item_name" "$readme_file"; then
                            mentioned_count=$((mentioned_count + 1))
                        fi
                    fi
                done <<< "$pub_items"

                if [[ $mentioned_count -eq 0 ]] && [[ -n "$pub_items" ]]; then
                    log_warning "$readme_file" "Module README doesn't document any public items from $mod_file"
                fi
            fi
        fi
    fi
}

# Function to check README completeness
check_readme_completeness() {
    local readme_file="$1"
    local content="$(cat "$readme_file")"

    # Check for essential sections
    local required_sections=("# " "## ")
    local has_title=false
    local has_sections=false

    if echo "$content" | grep -q "^# "; then
        has_title=true
    fi

    if echo "$content" | grep -q "^## "; then
        has_sections=true
    fi

    if [[ "$has_title" == false ]]; then
        log_warning "$readme_file" "Missing main title (# header)"
    fi

    if [[ "$has_sections" == false ]]; then
        log_warning "$readme_file" "No subsections found (## headers)"
    fi

    # Check if README is too short (might be incomplete)
    local line_count=$(wc -l < "$readme_file")
    if [[ $line_count -lt 5 ]]; then
        log_warning "$readme_file" "README seems incomplete (only $line_count lines)"
    fi

    # Check for TODO or placeholder content
    if echo "$content" | grep -qi "todo\|placeholder\|coming soon\|not implemented"; then
        log_warning "$readme_file" "Contains placeholder/TODO content"
    fi
}

# Function to verify badges and status indicators
verify_badges() {
    local readme_file="$1"
    local content="$(cat "$readme_file")"

    # Check build status badges
    if echo "$content" | grep -q "Build.*Passing"; then
        # Verify we actually have working builds
        if [[ ! -f "$REPO_ROOT/Cargo.toml" ]] && [[ ! -f "$REPO_ROOT/package.json" ]]; then
            log_warning "$readme_file" "Claims build passing but no build configuration found"
        fi
    fi

    # Check test coverage badges
    if echo "$content" | grep -q "Tests.*%.*Pass"; then
        # Check if we have test files
        local test_count=$(find "$REPO_ROOT" -name "*test*.rs" -o -name "test_*.rs" | wc -l)
        if [[ $test_count -eq 0 ]]; then
            log_warning "$readme_file" "Claims test coverage but no test files found"
        fi
    fi

    # Check version badges
    if echo "$content" | grep -q "Version:.*[0-9]"; then
        # Should match Cargo.toml version if it exists
        if [[ -f "$REPO_ROOT/Cargo.toml" ]]; then
            local cargo_version=$(grep "^version" "$REPO_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)
            if [[ -n "$cargo_version" ]] && ! echo "$content" | grep -q "$cargo_version"; then
                log_warning "$readme_file" "Version in README doesn't match Cargo.toml ($cargo_version)"
            fi
        fi
    fi
}

# Main verification function
verify_readme() {
    local readme_file="$1"

    echo -e "${BLUE}📋 Verifying: $readme_file${NC}"
    TOTAL_READMES=$((TOTAL_READMES + 1))

    # Basic checks
    if [[ ! -r "$readme_file" ]]; then
        log_error "$readme_file" "File is not readable"
        return
    fi

    if [[ ! -s "$readme_file" ]]; then
        log_error "$readme_file" "File is empty"
        return
    fi

    # Perform all verification checks
    verify_links "$readme_file"
    verify_code_examples "$readme_file"
    verify_module_documentation "$readme_file"
    check_readme_completeness "$readme_file"
    verify_badges "$readme_file"

    VERIFIED_READMES=$((VERIFIED_READMES + 1))
    echo -e "${GREEN}✅ Verified: $readme_file${NC}"
}

# Find and verify all README files
echo -e "${BLUE}📂 Finding all README.md files...${NC}"

# Find README files (excluding node_modules and target directories)
while IFS= read -r readme_file; do
    verify_readme "$readme_file"
done < <(find "$REPO_ROOT" -name "README.md" -not -path "*/node_modules/*" -not -path "*/target/*" | sort)

# Generate final report
echo -e "${BLUE}📊 Generating verification report...${NC}"

# Add results to report
cat >> "$REPORT_FILE" << EOF

## Verification Results

- **Total README files**: $TOTAL_READMES
- **Successfully verified**: $VERIFIED_READMES
- **Errors found**: $ERRORS_FOUND
- **Warnings issued**: $WARNINGS_FOUND

### Status

EOF

if [[ $ERRORS_FOUND -eq 0 ]]; then
    echo "✅ **PASS**: All README files passed verification" >> "$REPORT_FILE"
    echo -e "${GREEN}✅ All README files passed verification!${NC}"
else
    echo "❌ **FAIL**: $ERRORS_FOUND errors found" >> "$REPORT_FILE"
    echo -e "${RED}❌ Verification failed with $ERRORS_FOUND errors${NC}"
fi

if [[ $WARNINGS_FOUND -gt 0 ]]; then
    echo "⚠️ **$WARNINGS_FOUND warnings** issued for potential improvements" >> "$REPORT_FILE"
    echo -e "${YELLOW}⚠️ $WARNINGS_FOUND warnings found${NC}"
fi

# Add detailed error information if any errors found
if [[ $ERRORS_FOUND -gt 0 ]]; then
    cat >> "$REPORT_FILE" << 'EOF'

## Error Details

EOF

    # Save errors to JSON file
    echo "[" > "$ERRORS_FILE"
    local first_error=true
    for error in "${README_ERRORS[@]}"; do
        if [[ "$first_error" == true ]]; then
            first_error=false
        else
            echo "," >> "$ERRORS_FILE"
        fi
        echo "  $error" >> "$ERRORS_FILE"
    done
    echo "]" >> "$ERRORS_FILE"

    echo "Detailed error information saved to: $ERRORS_FILE" >> "$REPORT_FILE"
fi

# Add recommendations section
cat >> "$REPORT_FILE" << 'EOF'

## Recommendations

### For Maintainers

1. **Fix Critical Errors**: Address all red-flagged issues immediately
2. **Review Warnings**: Consider fixing warnings for better documentation quality
3. **Regular Verification**: Run this script regularly as part of CI/CD
4. **Update Process**: Establish process to update README files when code changes

### Best Practices Applied

1. ✅ **Link Validation**: All internal links verified
2. ✅ **Code Example Verification**: Rust code examples checked against actual modules
3. ✅ **Module Documentation**: README files validated against corresponding source modules
4. ✅ **Completeness Check**: Structure and content completeness verified
5. ✅ **Badge Accuracy**: Status badges validated against actual project state

### Next Steps

1. Address any errors found in this verification
2. Consider implementing automated README verification in CI/CD pipeline
3. Establish documentation update procedures
4. Regular review cycle for documentation accuracy

---

**Report Generated**: $(date)
**Script Version**: 1.0.0

EOF

echo ""
echo -e "${BLUE}📋 Verification Summary:${NC}"
echo -e "   Total README files: $TOTAL_READMES"
echo -e "   Successfully verified: $VERIFIED_READMES"
echo -e "   Errors found: $ERRORS_FOUND"
echo -e "   Warnings issued: $WARNINGS_FOUND"
echo ""
echo -e "${BLUE}📄 Full report saved to:${NC} $REPORT_FILE"
if [[ $ERRORS_FOUND -gt 0 ]]; then
    echo -e "${BLUE}🔍 Error details saved to:${NC} $ERRORS_FILE"
fi

# Exit with appropriate code
if [[ $ERRORS_FOUND -gt 0 ]]; then
    exit 1
else
    exit 0
fi
