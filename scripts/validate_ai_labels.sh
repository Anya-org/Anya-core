#!/bin/bash
# [AIS-3] AI label validation script
set -euo pipefail

source "$(dirname "$0")/common/utils.sh"

# Required labels and their patterns
declare -A REQUIRED_LABELS=(
    ["AIR"]="AIR-[0-3]"
    ["AIS"]="AIS-[0-3]"
    ["BPC"]="BPC-[0-3]"
)

# Files to check based on extension
VALID_EXTENSIONS=("rs" "md" "toml" "sh")

validate_labels() {
    local file="$1"
    local missing_labels=()
    local invalid_labels=()

    # Check each required label
    for label in "${!REQUIRED_LABELS[@]}"; do
        pattern="${REQUIRED_LABELS[$label]}"
        if ! grep -q "\[$pattern\]" "$file"; then
            missing_labels+=("$label")
        fi
    done

    # Check for invalid label formats
    if grep -o "\[[A-Z]\{3\}-[0-9]\]" "$file" | grep -qv "^\[(AIR\|AIS\|BPC\)-[0-3]\]$"; then
        invalid_labels+=("$(grep -o "\[[A-Z]\{3\}-[0-9]\]" "$file" | grep -v "^\[(AIR\|AIS\|BPC\)-[0-3]\]$")")
    fi

    # Report issues
    if [[ ${#missing_labels[@]} -gt 0 ]]; then
        log_error "Missing labels in $file: ${missing_labels[*]}"
        return 1
    fi

    if [[ ${#invalid_labels[@]} -gt 0 ]]; then
        log_error "Invalid labels in $file: ${invalid_labels[*]}"
        return 1
    fi

    return 0
}

main() {
    local exit_code=0
    local checked_files=0

    # Process changed files
    while IFS= read -r file; do
        # Check if file extension is valid
        if [[ ! " ${VALID_EXTENSIONS[@]} " =~ " ${file##*.} " ]]; then
            continue
        fi

        ((checked_files++))
        log_info "Checking $file..."
        
        if ! validate_labels "$file"; then
            exit_code=1
        fi
    done < <(git diff --cached --name-only)

    if [[ $checked_files -eq 0 ]]; then
        log_warn "No files checked"
        exit 0
    fi

    if [[ $exit_code -eq 0 ]]; then
        log_success "AI label validation passed"
    else
        log_error "AI label validation failed"
    fi

    exit $exit_code
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main
fi
