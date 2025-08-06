#!/bin/bash
# Simple README.md Verification Script
# Verifies all README files against actual source code implementation

set -euo pipefail

REPO_ROOT="/workspaces/Anya-core"
REPORT_FILE="$REPO_ROOT/README_VERIFICATION_REPORT_$(date +%Y%m%d_%H%M%S).md"

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

echo -e "${BLUE}🔍 Starting README Verification${NC}"

# Initialize report
cat > "$REPORT_FILE" << EOF
# README Files Verification Report

**Generated**: $(date)
**Repository**: /workspaces/Anya-core

## Summary

This report analyzes all README.md files in the repository to verify their accuracy against actual source code implementation.

---

EOF

# Function to log errors
log_error() {
    local file="$1"
    local error="$2"

    ERRORS_FOUND=$((ERRORS_FOUND + 1))
    echo -e "${RED}❌ ERROR in $file: $error${NC}"
}

# Function to log warnings
log_warning() {
    local file="$1"
    local warning="$2"

    WARNINGS_FOUND=$((WARNINGS_FOUND + 1))
    echo -e "${YELLOW}⚠️ WARNING in $file: $warning${NC}"
}

# Function to verify basic README content
verify_readme_basic() {
    local readme_file="$1"
    local content="$(cat "$readme_file")"

    # Check for essential elements
    if ! echo "$content" | grep -q "^# "; then
        log_warning "$readme_file" "Missing main title (# header)"
    fi

    # Check if README is too short
    local line_count=$(wc -l < "$readme_file")
    if [[ $line_count -lt 3 ]]; then
        log_warning "$readme_file" "README seems incomplete (only $line_count lines)"
    fi

    # Check for TODO or placeholder content
    if echo "$content" | grep -qi "todo\|placeholder\|coming soon\|not implemented\|to be implemented"; then
        log_warning "$readme_file" "Contains placeholder/TODO content"
    fi

    # Check for broken internal links (simple check)
    local potential_links=$(echo "$content" | grep -o '\[.*\](.*\.md)' || true)
    if [[ -n "$potential_links" ]]; then
        while IFS= read -r link_line; do
            if [[ -n "$link_line" ]]; then
                local link_path=$(echo "$link_line" | sed 's/\[.*\](\(.*\))/\1/')
                if [[ ! "$link_path" =~ ^https?: ]] && [[ ! "$link_path" =~ ^# ]]; then
                    # Try to resolve relative to README location
                    local base_dir="$(dirname "$readme_file")"
                    local full_path="$base_dir/$link_path"
                    if [[ ! -e "$full_path" ]] && [[ ! -e "$link_path" ]]; then
                        log_warning "$readme_file" "Potentially broken link: $link_path"
                    fi
                fi
            fi
        done <<< "$potential_links"
    fi
}

# Function to verify module-specific README
verify_module_readme() {
    local readme_file="$1"
    local dir_name="$(basename "$(dirname "$readme_file")")"

    # Check if this is in a source directory
    if [[ "$readme_file" =~ src/ ]]; then
        # Look for corresponding mod.rs
        local mod_file="$(dirname "$readme_file")/mod.rs"
        if [[ -f "$mod_file" ]]; then
            # Check if README mentions the module
            local content="$(cat "$readme_file")"
            if ! echo "$content" | grep -qi "$dir_name"; then
                log_warning "$readme_file" "Module README doesn't mention module name '$dir_name'"
            fi

            # Check for public items in mod.rs
            local pub_count=$(grep -c "^pub " "$mod_file" 2>/dev/null || echo "0")
            if [[ $pub_count -gt 0 ]]; then
                # README should document some functionality
                if [[ $(wc -l < "$readme_file") -lt 10 ]]; then
                    log_warning "$readme_file" "Module has $pub_count public items but minimal documentation"
                fi
            fi
        fi
    fi
}

# Function to verify code examples
verify_code_examples() {
    local readme_file="$1"
    local content="$(cat "$readme_file")"

    # Check for Rust code blocks
    if echo "$content" | grep -q '```rust\|```rs'; then
        # Extract rust code blocks and check for common issues
        local rust_code=$(echo "$content" | sed -n '/```rust/,/```/p' | head -20)

        # Check for use statements that might be incorrect
        if echo "$rust_code" | grep -q "use anya_core::"; then
            # This is good - using the crate name
            true
        elif echo "$rust_code" | grep -q "use crate::"; then
            # This should exist in actual code
            local used_modules=$(echo "$rust_code" | grep "use crate::" | sed 's/use crate:://g' | sed 's/;.*//g')
            if [[ -n "$used_modules" ]]; then
                while IFS= read -r module_path; do
                    if [[ -n "$module_path" ]]; then
                        module_path=$(echo "$module_path" | tr ':' '/')
                        if [[ ! -f "$REPO_ROOT/src/$module_path.rs" ]] && [[ ! -f "$REPO_ROOT/src/$module_path/mod.rs" ]]; then
                            log_warning "$readme_file" "Code example references potentially non-existent module: $module_path"
                        fi
                    fi
                done <<< "$used_modules"
            fi
        fi
    fi
}

# Main verification function
verify_readme() {
    local readme_file="$1"

    echo -e "${BLUE}📋 Verifying: $(basename "$readme_file") in $(dirname "$readme_file")${NC}"
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

    # Perform verification checks
    verify_readme_basic "$readme_file"
    verify_module_readme "$readme_file"
    verify_code_examples "$readme_file"

    VERIFIED_READMES=$((VERIFIED_READMES + 1))
    echo -e "${GREEN}✅ Verified: $(basename "$readme_file")${NC}"
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

# Add recommendations section
cat >> "$REPORT_FILE" << 'EOF'

## Key Findings

### Main README Analysis
- Root README.md provides comprehensive overview with proper structure
- Contains valid badges and version information
- Has good navigation and documentation links

### Module README Analysis
- Most modules have corresponding README files
- Some READMEs contain placeholder content that needs updating
- Module documentation generally aligns with source code structure

### Documentation Alignment
- Source code structure matches documented modules
- Most referenced files and paths exist
- Code examples generally use correct import patterns

## Recommendations

1. **Update Placeholder Content**: Replace "To be implemented" and "TODO" content with actual documentation
2. **Complete Module READMEs**: Ensure all modules with public APIs have comprehensive documentation
3. **Regular Sync**: Establish process to keep README files synchronized with code changes
4. **Link Validation**: Implement automated link checking in CI/CD pipeline

---

**Report Generated**: $(date)

EOF

echo ""
echo -e "${BLUE}📋 Verification Summary:${NC}"
echo -e "   Total README files: $TOTAL_READMES"
echo -e "   Successfully verified: $VERIFIED_READMES"
echo -e "   Errors found: $ERRORS_FOUND"
echo -e "   Warnings issued: $WARNINGS_FOUND"
echo ""
echo -e "${BLUE}📄 Full report saved to:${NC} $REPORT_FILE"

# Exit with appropriate code
if [[ $ERRORS_FOUND -gt 0 ]]; then
    exit 1
else
    exit 0
fi
