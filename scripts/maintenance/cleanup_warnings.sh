#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Log functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to update deprecated base64 functions
update_base64_usage() {
    log_info "Updating deprecated base64 functions..."
    
    # Find all files containing deprecated base64 usage
    FILES=$(grep -r "base64::encode\|base64::decode" --include="*.rs" /home/anya/anyachainlabs/projects/anya-core/src)
    
    if [ -z "$FILES" ]; then
        log_info "No deprecated base64 usage found."
        return
    fi
    
    # Update each file
    echo "$FILES" | cut -d: -f1 | sort | uniq | while read -r file; do
        log_info "Updating $file..."
        
        # Replace base64::encode with base64::engine::general_purpose::STANDARD.encode
        sed -i 's/base64::encode/base64::engine::general_purpose::STANDARD.encode/g' "$file"
        
        # Replace base64::decode with base64::engine::general_purpose::STANDARD.decode
        sed -i 's/base64::decode/base64::engine::general_purpose::STANDARD.decode/g' "$file"
    done
    
    log_success "Updated base64 usage in all files."
}

# Function to remove unused imports
cleanup_unused_imports() {
    log_info "Cleaning up unused imports..."
    
    # Run cargo check with the -W unused-imports flag to identify unused imports
    cargo check -W unused-imports > unused_imports.log
    
    log_info "Unused imports identified. You can review them in unused_imports.log"
    log_info "For automatic cleanup, consider using 'cargo fix --allow-dirty' or a Rust formatter like rustfmt"
    
    log_success "Unused imports check completed."
}

# Main execution
echo "==================================="
log_info "WARNINGS CLEANUP"
echo "==================================="
echo ""

# Create scripts directory if it doesn't exist
mkdir -p "$(dirname "$0")"

# Execute each cleanup task
update_base64_usage
echo ""

cleanup_unused_imports
echo ""

log_success "Warnings cleanup completed."
log_info "Run 'cargo check' to verify the changes."
