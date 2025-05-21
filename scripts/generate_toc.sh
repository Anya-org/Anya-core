#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Table of Contents Generator
# This script generates or updates a table of contents for markdown files

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if a file was provided
if [ $# -eq 0 ]; then
    echo -e "${YELLOW}Usage: $0 <markdown-file> [--in-place]${NC}"
    echo -e "  --in-place  Update the file in place (default: print to stdout)"
    exit 1
fi

FILE="$1"
IN_PLACE=0

# Check for --in-place flag
if [ "$1" = "--in-place" ]; then
    IN_PLACE=1
    shift
    FILE="$1"
fi

# Check if file exists
if [ ! -f "$FILE" ]; then
    echo -e "${RED}Error: File '$FILE' not found${NC}"
    exit 1
fi

# Check if file is markdown
if [[ "$FILE" != *.md ]] && [[ "$FILE" != *.markdown ]]; then
    echo -e "${YELLOW}Warning: '$FILE' may not be a markdown file${NC}"
fi

# Extract the TOC
TOC=""
IN_TOC=0
HAS_TOC=0

# Process the file line by line
while IFS= read -r line; do
    # Check if we're in the TOC section
    if [[ "$line" == "## Table of Contents"* ]]; then
        IN_TOC=1
        HAS_TOC=1
        TOC="$line\n\n"
        continue
    fi
    
    # If we're in the TOC, skip until we hit the next section
    if [ $IN_TOC -eq 1 ]; then
        # Check if we've hit the next section
        if [[ "$line" == "## "* ]]; then
            IN_TOC=0
        else
            continue
        fi
    fi
    
    # Process headers for TOC
    if [[ "$line" == "## "* ]]; then
        # Skip h1 headers (single #)
        if [[ "$line" != "# "* ]]; then
            # Get the header level and text
            level=$(echo "$line" | grep -o '^#\+ ' | wc -c)
            level=$((level - 2)) # Account for the space
            indent=$(( (level - 2) * 2 ))
            text=$(echo "$line" | sed -E 's/^#+ //')
            
            # Create anchor (lowercase, replace spaces with dashes, remove special chars)
            anchor=$(echo "$text" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | tr -cd '[:alnum:]-')
            
            # Add to TOC
            TOC+="$(printf '%*s' $indent)- [$text](#$anchor)\n"
        fi
    fi
done < "$FILE"

# Add TOC to the file
if [ $IN_PLACE -eq 1 ]; then
    # Create a temporary file
    TMP_FILE=$(mktemp)
    
    # Process the file and insert TOC
    IN_HEADER=1
    IN_TOC_SECTION=0
    TOC_ADDED=0
    
    while IFS= read -r line; do
        # Check if we're in the YAML front matter
        if [ $IN_HEADER -eq 1 ]; then
            if [[ "$line" == "---" ]]; then
                IN_HEADER=0
            fi
            echo "$line" >> "$TMP_FILE"
            continue
        fi
        
        # Check for existing TOC
        if [[ "$line" == "## Table of Contents"* ]]; then
            IN_TOC_SECTION=1
            echo -e "## Table of Contents\n" >> "$TMP_FILE"
            echo -e "$TOC" | grep -v '^## Table of Contents' >> "$TMP_FILE"
            TOC_ADDED=1
            continue
        fi
        
        # Skip lines in the old TOC
        if [ $IN_TOC_SECTION -eq 1 ]; then
            if [[ "$line" == "## "* ]]; then
                IN_TOC_SECTION=0
                echo "$line" >> "$TMP_FILE"
            fi
            continue
        fi
        
        # Insert TOC after the first header if it doesn't exist
        if [ $TOC_ADDED -eq 0 ] && [[ "$line" == "# "* ]]; then
            echo -e "$line\n\n## Table of Contents\n\n$TOC" >> "$TMP_FILE"
            TOC_ADDED=1
        else
            echo "$line" >> "$TMP_FILE"
        fi
    done < "$FILE"
    
    # If we never added the TOC, add it at the end
    if [ $TOC_ADDED -eq 0 ]; then
        echo -e "\n## Table of Contents\n\n$TOC" >> "$TMP_FILE"
    fi
    
    # Replace the original file
    mv "$TMP_FILE" "$FILE"
    
    echo -e "${GREEN}✓ Updated TOC in $FILE${NC}"
else
    # Just print the TOC
    if [ $HAS_TOC -eq 1 ]; then
        echo -e "## Table of Contents\n"
    fi
    echo -e "$TOC"
fi

exit 0
