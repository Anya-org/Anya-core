#!/bin/bash

# Simple Documentation Link Checker
# This script checks for broken markdown links in the documentation

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

DOCS_DIR="docs"
ERRORS=0

echo "Checking documentation for broken links..."

# Find all markdown files
find "$DOCS_DIR" -name "*.md" | while read -r file; do
    # Get relative path from docs directory
    rel_path="${file#$DOCS_DIR/}"
    
    # Check each line for markdown links
    line_num=0
    while IFS= read -r line || [ -n "$line" ]; do
        ((line_num++))
        
        # Extract all markdown links [text](url)
        while [[ $line =~ \[([^]]+)\]\(([^)]+)\) ]]; do
            link="${BASH_REMATCH[2]%%\ *}"  # Get just the URL part before any space
            
            # Skip external links, mailto, and anchor-only links
            if [[ $link == http* ]] || [[ $link == mailto:* ]] || [[ $link == "#"* ]] || [[ -z $link ]]; then
                line="${line#*](${BASH_REMATCH[2]})}"
                continue
            fi
            
            # Handle relative paths
            if [[ $link == /* ]]; then
                # Remove leading slash for consistency
                link="${link:1}"
            else
                # Make path relative to the current file's directory
                link="$(dirname "$rel_path")/$link"
            fi
            
            # Check if the file exists
            if [ ! -f "$DOCS_DIR/$link" ]; then
                echo -e "${RED}Error:${NC} In $rel_path:$line_num - Broken link: ${BASH_REMATCH[2]}"
                ((ERRORS++))
            fi
            
            # Move to next link
            line="${line#*](${BASH_REMATCH[2]})}"
        done
    done < "$file"
done

if [ $ERRORS -gt 0 ]; then
    echo -e "\n${RED}Found $ERRORS broken links in documentation${NC}"
    exit 1
else
    echo -e "\n${GREEN}All documentation links are valid!${NC}"
    exit 0
fi
