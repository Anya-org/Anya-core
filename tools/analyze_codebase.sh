#!/bin/bash
# Codebase Analysis Script for Anya Core
# 
# This script runs various analysis tools on the Anya Core codebase
# to help identify areas for improvement and consolidation.

set -e

ANYA_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "Analyzing Anya Core at: $ANYA_ROOT"

# Ensure the tools directory is properly set up
mkdir -p "$ANYA_ROOT/tools/output"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Anya Core Codebase Analysis ===${NC}"

# 1. Build and run the code duplication finder
echo -e "\n${YELLOW}Building code duplication finder...${NC}"
cd "$ANYA_ROOT"
rustc -o "$ANYA_ROOT/tools/find_duplicates" "$ANYA_ROOT/tools/find_duplicates.rs"

echo -e "\n${YELLOW}Running code duplication analysis...${NC}"
"$ANYA_ROOT/tools/find_duplicates" > "$ANYA_ROOT/tools/output/duplicates_report.txt"
echo -e "${GREEN}Duplication analysis complete. Report saved to tools/output/duplicates_report.txt${NC}"

# 2. Count lines of code by package
echo -e "\n${YELLOW}Generating lines of code statistics...${NC}"
echo "Package Lines of Code Report" > "$ANYA_ROOT/tools/output/loc_report.txt"
echo "==========================" >> "$ANYA_ROOT/tools/output/loc_report.txt"
echo "" >> "$ANYA_ROOT/tools/output/loc_report.txt"

# Count lines in new packages
echo "New Package Structure:" >> "$ANYA_ROOT/tools/output/loc_report.txt"
for pkg in core protocol-adapters mcp-interface bitcoin-network metrics bin; do
    if [ -d "$ANYA_ROOT/packages/$pkg" ]; then
        LOC=$(find "$ANYA_ROOT/packages/$pkg" -name "*.rs" | xargs wc -l 2>/dev/null | tail -n 1 | awk '{print $1}')
        echo "- packages/$pkg: $LOC lines" >> "$ANYA_ROOT/tools/output/loc_report.txt"
    fi
done

# Count lines in old structure
echo "" >> "$ANYA_ROOT/tools/output/loc_report.txt"
echo "Legacy Code:" >> "$ANYA_ROOT/tools/output/loc_report.txt"
for dir in anya-bitcoin anyacore anya-enterprise anya-extensions core; do
    if [ -d "$ANYA_ROOT/$dir" ]; then
        LOC=$(find "$ANYA_ROOT/$dir" -name "*.rs" | xargs wc -l 2>/dev/null | tail -n 1 | awk '{print $1}')
        echo "- $dir: $LOC lines" >> "$ANYA_ROOT/tools/output/loc_report.txt"
    fi
done

echo -e "${GREEN}Lines of code analysis complete. Report saved to tools/output/loc_report.txt${NC}"

# 3. Check for BIP-342 references across the codebase
echo -e "\n${YELLOW}Analyzing BIP-342 implementation coverage...${NC}"
echo "BIP-342 Implementation References" > "$ANYA_ROOT/tools/output/bip342_report.txt"
echo "===============================" >> "$ANYA_ROOT/tools/output/bip342_report.txt"
echo "" >> "$ANYA_ROOT/tools/output/bip342_report.txt"

grep -r "BIP-342\|BIP342\|Tapscript\|tapscript" --include="*.rs" "$ANYA_ROOT" | sort | \
    grep -v "target/" >> "$ANYA_ROOT/tools/output/bip342_report.txt"

echo -e "${GREEN}BIP-342 analysis complete. Report saved to tools/output/bip342_report.txt${NC}"

# 4. Run integration tests for the MCP server fix
echo -e "\n${YELLOW}Running MCP server integration tests...${NC}"
cd "$ANYA_ROOT"
cargo test -p anya-core-mcp-interface || echo -e "${RED}Tests failed - MCP fix may need further work${NC}"

# Print summary
echo -e "\n${BLUE}=== Analysis Complete ===${NC}"
echo -e "${GREEN}All reports have been saved to the tools/output directory.${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "1. Review the duplication report and consolidate redundant code"
echo "2. Update imports in dependent modules after consolidation"
echo "3. Run the MCP server tests to verify the fix is working properly"
echo "4. Continue enhancing the BIP-342 implementation as needed"
echo ""

# Make the script executable
chmod +x "$0"
