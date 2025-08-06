#!/bin/bash

# Cross-Reference Integrity Checking Script
# Validates all internal links in documentation

set -euo pipefail

DOCS_ROOT="/workspaces/Anya-core/docs"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
REPORT_FILE="/workspaces/Anya-core/cross_reference_report.md"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Display banner
echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     ANYA CORE DOCUMENTATION CROSS-REFERENCE CHECKER        ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"

# Initialize report
init_report() {
    cat > "$REPORT_FILE" << EOF
# Documentation Cross-Reference Report

**Generated:** $TIMESTAMP

## Summary

