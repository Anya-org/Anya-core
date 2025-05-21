#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Status Report
# This script generates a status report for the Anya Core documentation

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOCS_DIR="$(pwd)/docs"
REPORT_FILE="${DOCS_DIR}/status/REPORT_$(date +%Y-%m-%d).md"
STATUS_DIR="${DOCS_DIR}/status"

# Create status directory if it doesn't exist
mkdir -p "$STATUS_DIR"

# Get total word count for documentation
get_word_count() {
    find "$DOCS_DIR" -name "*.md" -type f -exec cat {} \; | wc -w
}

# Count files by type
count_files() {
    local ext="$1"
    find "$DOCS_DIR" -name "*.$ext" -type f | wc -l
}

# Check for TODOs and FIXMEs
check_todos() {
    local count=0
    while IFS= read -r file; do
        count=$((count + $(grep -i -E 'TODO|FIXME' "$file" | wc -l)))
    done < <(find "$DOCS_DIR" -name "*.md" -type f)
    echo "$count"
}

# Check for broken links
check_links() {
    if [ -f "scripts/check_links.sh" ]; then
        ./scripts/check_links.sh 2>&1 | grep -c "ERROR:" || echo "0"
    else
        echo "N/A"
    fi
}

# Generate the report
generate_report() {
    local total_files
    local md_files
    local total_words
    local todos
    local broken_links
    
    echo -e "# Documentation Status Report\n"
    echo -e "**Generated:** $(date '+%Y-%m-%d %H:%M:%S')"
    echo -e "**Documentation Directory:** ${DOCS_DIR/$PWD/.}\n"
    
    # Count files
    total_files=$(find "$DOCS_DIR" -type f | wc -l)
    md_files=$(count_files "md")
    
    echo -e "## 📊 Statistics\n"
    echo -e "- **Total Files:** $total_files"
    echo -e "- **Markdown Files:** $md_files"
    
    # Word count
    total_words=$(get_word_count)
    echo -e "- **Total Words:** $total_words"
    
    # TODOs and FIXMEs
    todos=$(check_todos)
    echo -e "- **TODOs/FIXMEs:** $todos"
    
    # Broken links
    echo -e "\n## 🔗 Link Status\n"
    broken_links=$(check_links)
    if [ "$broken_links" = "N/A" ]; then
        echo -e "- **Broken Links:** ${YELLOW}Link checker not available${NC}"
    elif [ "$broken_links" -eq 0 ]; then
        echo -e "- **Broken Links:** ${GREEN}None found${NC}"
    else
        echo -e "- **Broken Links:** ${RED}$broken_links found${NC}"
    fi
    
    # Recent changes
    echo -e "\n## 📝 Recent Changes\n"
    echo -e '```bash'
    git -C "$DOCS_DIR" log --since="1 week ago" --pretty=format:"%h - %an, %ar : %s" --name-status
    echo -e '```'
    
    # Recommendations
    echo -e "\n## 📋 Recommendations\n"
    if [ "$todos" -gt 0 ]; then
        echo -e "- ${YELLOW}Address $todos TODO/FIXME comments in the documentation${NC}"
    fi
    
    if [ "$broken_links" != "N/A" ] && [ "$broken_links" -gt 0 ]; then
        echo -e "- ${RED}Fix $broken_links broken links in the documentation${NC}"
    fi
    
    echo -e "- ${BLUE}Review recent changes for accuracy and completeness${NC}"
}

# Main function
main() {
    # Generate the report
    echo -e "${YELLOW}📊 Generating documentation status report...${NC}"
    generate_report > "$REPORT_FILE"
    
    # Print summary
    echo -e "\n${GREEN}✅ Documentation status report generated:${NC}"
    echo -e "   ${REPORT_FILE/$PWD/.}"
    
    # Show a preview of the report
    echo -e "\n${BLUE}--- Report Preview ---${NC}"
    head -n 20 "$REPORT_FILE"
    echo -e "${BLUE}----------------------${NC}"
    
    # Open the report if on macOS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        open "$REPORT_FILE"
    fi
}

# Run the main function
main

exit 0
