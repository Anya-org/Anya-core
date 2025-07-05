#!/bin/bash
# Canonical Work Item Tracking and Source of Truth Registry Validator
# This script enforces the PRD requirements for checkin work tracking

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Registry paths
REGISTRY_PATH="$WORKSPACE_ROOT/.source_of_truth_registry"
WORK_ITEMS_PATH="$REGISTRY_PATH/work_items"
CANONICAL_DOCS_PATH="$REGISTRY_PATH/canonical_documents"

echo -e "${BLUE}🔍 Canonical Source of Truth & Work Item Validation${NC}"
echo "=================================================="

# Initialize registry if it doesn't exist
initialize_registry() {
    if [[ ! -d "$REGISTRY_PATH" ]]; then
        echo -e "${YELLOW}📁 Initializing Source of Truth Registry...${NC}"
        mkdir -p "$WORK_ITEMS_PATH"
        mkdir -p "$CANONICAL_DOCS_PATH"

        # Create registry metadata
        cat >"$REGISTRY_PATH/registry_metadata.json" <<EOF
{
    "version": "1.0.0",
    "created": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "last_updated": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "total_work_items": 0,
    "total_canonical_documents": 0
}
EOF
        echo -e "${GREEN}✅ Registry initialized${NC}"
    fi
}

# Validate work item ID format
validate_work_item_id() {
    local work_id="$1"

    if [[ ! "$work_id" =~ ^WI-[0-9]{4}-[0-9]{2}-[0-9]{2}-[0-9]+$ ]]; then
        echo -e "${RED}❌ Invalid work item ID format: $work_id${NC}"
        echo "   Expected format: WI-YYYY-MM-DD-###"
        return 1
    fi

    return 0
}

# Check if work item exists in registry
registry_contains_work_item() {
    local work_id="$1"
    local work_item_file="$WORK_ITEMS_PATH/${work_id}.json"

    if [[ -f "$work_item_file" ]]; then
        return 0
    else
        return 1
    fi
}

# Create new work item
create_work_item() {
    local title="$1"
    local component="$2"

    # Generate work item ID
    local date_prefix=$(date +"%Y-%m-%d")
    local sequence_num=1

    # Find next available sequence number
    while [[ -f "$WORK_ITEMS_PATH/WI-${date_prefix}-${sequence_num}.json" ]]; do
        ((sequence_num++))
    done

    local work_id="WI-${date_prefix}-${sequence_num}"
    local work_item_file="$WORK_ITEMS_PATH/${work_id}.json"

    # Create work item JSON
    cat >"$work_item_file" <<EOF
{
    "id": "$work_id",
    "title": "$title",
    "status": "Planning",
    "component": "$component",
    "files_modified": [],
    "duplication_check": "NotChecked",
    "source_of_truth_updated": false,
    "verification_hash": "",
    "completion_timestamp": null,
    "evidence_link": "",
    "dependencies": [],
    "blockers": [],
    "created": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "last_updated": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF

    echo -e "${GREEN}✅ Created work item: $work_id${NC}"
    echo "   Title: $title"
    echo "   Component: $component"
    echo "   File: $work_item_file"

    # Update registry metadata
    update_registry_metadata

    return 0
}

# Update work item status
update_work_item_status() {
    local work_id="$1"
    local new_status="$2"
    local work_item_file="$WORK_ITEMS_PATH/${work_id}.json"

    if [[ ! -f "$work_item_file" ]]; then
        echo -e "${RED}❌ Work item not found: $work_id${NC}"
        return 1
    fi

    # Update the JSON file (simple sed replacement for demo)
    sed -i "s/\"status\": \"[^\"]*\"/\"status\": \"$new_status\"/" "$work_item_file"
    sed -i "s/\"last_updated\": \"[^\"]*\"/\"last_updated\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"/" "$work_item_file"

    echo -e "${GREEN}✅ Updated work item $work_id status: $new_status${NC}"

    return 0
}

# Check for code duplication
check_comprehensive_duplication() {
    echo -e "${BLUE}🔍 Running comprehensive duplication check...${NC}"

    local duplication_found=false

    # Check for duplicate function signatures
    echo "   Checking function signature duplicates..."
    if grep -r "fn check_transaction_status" --include="*.rs" "$WORKSPACE_ROOT/src" | wc -l | awk '{if($1 > 3) exit 1}'; then
        echo -e "${YELLOW}   ⚠️  Multiple check_transaction_status implementations found${NC}"
        duplication_found=true
    fi

    # Check for duplicate documentation patterns
    echo "   Checking documentation duplicates..."
    if find "$WORKSPACE_ROOT" -name "*.md" -exec grep -l "Single source of truth" {} \; | wc -l | awk '{if($1 > 5) exit 1}'; then
        echo -e "${YELLOW}   ⚠️  Multiple 'Single source of truth' documentation found${NC}"
        duplication_found=true
    fi

    # Check for duplicate test utilities
    echo "   Checking test utility duplicates..."
    if find "$WORKSPACE_ROOT/tests" -name "*.rs" -exec grep -l "create_dummy_transaction" {} \; | wc -l | awk '{if($1 > 1) exit 1}'; then
        echo -e "${YELLOW}   ⚠️  Duplicate test utility functions found${NC}"
        duplication_found=true
    fi

    if [[ "$duplication_found" == "true" ]]; then
        echo -e "${RED}❌ Duplication detected - must resolve before proceeding${NC}"
        return 1
    else
        echo -e "${GREEN}✅ No duplication detected${NC}"
        return 0
    fi
}

# Validate source of truth compliance
validate_source_of_truth_compliance() {
    echo -e "${BLUE}🔍 Validating Source of Truth compliance...${NC}"

    # Check that canonical documents exist and are updated
    local canonical_files=(
        "PRD_SYSTEM_INDEX_DUPLICATION_ELIMINATION.md"
        "MASTER_IMPLEMENTATION_PLAN_CANONICAL.md"
        "COMPREHENSIVE_ALIGNMENT_REVIEW.md"
    )

    for file in "${canonical_files[@]}"; do
        local file_path="$WORKSPACE_ROOT/$file"
        if [[ ! -f "$file_path" ]]; then
            echo -e "${RED}❌ Canonical document missing: $file${NC}"
            return 1
        fi

        # Check if file has been updated recently (within last 24 hours)
        local file_age=$(find "$file_path" -mtime -1 | wc -l)
        if [[ "$file_age" -eq 0 ]]; then
            echo -e "${YELLOW}⚠️  Canonical document may be stale: $file${NC}"
        fi
    done

    echo -e "${GREEN}✅ Source of Truth compliance validated${NC}"
    return 0
}

# Update registry metadata
update_registry_metadata() {
    local work_item_count=$(find "$WORK_ITEMS_PATH" -name "*.json" | wc -l)
    local canonical_doc_count=$(find "$CANONICAL_DOCS_PATH" -name "*.json" | wc -l)

    cat >"$REGISTRY_PATH/registry_metadata.json" <<EOF
{
    "version": "1.0.0",
    "created": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "last_updated": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "total_work_items": $work_item_count,
    "total_canonical_documents": $canonical_doc_count
}
EOF
}

# Generate verification report
generate_verification_report() {
    echo -e "${BLUE}📊 Generating verification report...${NC}"

    local report_file="$WORKSPACE_ROOT/VERIFICATION_REPORT_$(date +%Y%m%d_%H%M%S).md"

    cat >"$report_file" <<EOF
# Source of Truth & Work Item Verification Report

**Generated**: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
**Registry Path**: $REGISTRY_PATH

## Registry Status

- **Total Work Items**: $(find "$WORK_ITEMS_PATH" -name "*.json" 2>/dev/null | wc -l)
- **Active Work Items**: $(grep -l '"status": "InProgress"' "$WORK_ITEMS_PATH"/*.json 2>/dev/null | wc -l)
- **Completed Work Items**: $(grep -l '"status": "Completed"' "$WORK_ITEMS_PATH"/*.json 2>/dev/null | wc -l)

## Recent Work Items

$(ls -t "$WORK_ITEMS_PATH"/*.json 2>/dev/null | head -5 | while read file; do
        work_id=$(basename "$file" .json)
        title=$(grep '"title"' "$file" | cut -d'"' -f4)
        status=$(grep '"status"' "$file" | cut -d'"' -f4)
        echo "- **$work_id**: $title [$status]"
    done)

## Duplication Check Status

$(if check_comprehensive_duplication >/dev/null 2>&1; then
        echo "✅ **PASSED** - No duplication detected"
    else
        echo "❌ **FAILED** - Duplication detected"
    fi)

## Source of Truth Compliance

$(if validate_source_of_truth_compliance >/dev/null 2>&1; then
        echo "✅ **COMPLIANT** - All canonical documents present and valid"
    else
        echo "❌ **NON-COMPLIANT** - Issues with canonical documents"
    fi)

---
*This report was generated by the Canonical Source of Truth Validator*
EOF

    echo -e "${GREEN}✅ Verification report generated: $report_file${NC}"
}

# Main execution
main() {
    local command="${1:-validate}"

    case "$command" in
    "init")
        initialize_registry
        ;;
    "create")
        local title="${2:-}"
        local component="${3:-}"
        if [[ -z "$title" || -z "$component" ]]; then
            echo -e "${RED}❌ Usage: $0 create 'Work item title' 'component_name'${NC}"
            exit 1
        fi
        initialize_registry
        create_work_item "$title" "$component"
        ;;
    "update")
        local work_id="${2:-}"
        local status="${3:-}"
        if [[ -z "$work_id" || -z "$status" ]]; then
            echo -e "${RED}❌ Usage: $0 update WI-YYYY-MM-DD-### status${NC}"
            exit 1
        fi
        update_work_item_status "$work_id" "$status"
        ;;
    "validate")
        initialize_registry
        echo -e "${BLUE}🔍 Running full canonical validation...${NC}"

        local validation_passed=true

        if ! check_comprehensive_duplication; then
            validation_passed=false
        fi

        if ! validate_source_of_truth_compliance; then
            validation_passed=false
        fi

        if [[ "$validation_passed" == "true" ]]; then
            echo -e "${GREEN}✅ ALL VALIDATIONS PASSED${NC}"
            generate_verification_report
            exit 0
        else
            echo -e "${RED}❌ VALIDATION FAILED${NC}"
            exit 1
        fi
        ;;
    "report")
        initialize_registry
        generate_verification_report
        ;;
    *)
        echo "Usage: $0 {init|create|update|validate|report}"
        echo ""
        echo "Commands:"
        echo "  init                           - Initialize registry"
        echo "  create 'title' 'component'     - Create new work item"
        echo "  update WI-YYYY-MM-DD-### status - Update work item status"
        echo "  validate                       - Run full validation"
        echo "  report                         - Generate verification report"
        exit 1
        ;;
    esac
}

main "$@"
