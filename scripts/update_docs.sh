#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Update Script
# This script updates documentation to ensure consistency and compliance with standards

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOCS_DIR="$(pwd)/docs"
TEMPLATE_FILE="${DOCS_DIR}/.template.md"
AI_LABEL="[AIR-3][AIS-3][BPC-3][RES-3]"
CURRENT_DATE=$(date +'%Y-%m-%d')
TIMESTAMP=$(date +'%Y-%m-%d %H:%M:%S')

# Function to print header
print_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}\n"
}

# Function to print status
print_status() {
    local status=$1
    local message=$2
    
    case $status in
        success) echo -e "${GREEN}✓ ${message}${NC}" ;;
        warning) echo -e "${YELLOW}⚠  ${message}${NC}" ;;
        error) echo -e "${RED}✗ ${message}${NC}" ;;
        *) echo -e "${message}" ;;
    esac
}

# Function to update front matter
update_front_matter() {
    local file=$1
    # Skip checkpoint files
    if [[ "$file" == *"checkpoints/"* ]]; then
        print_status "warning" "Skipping checkpoint file: $file"
        return 0
    fi
    
    local title=$(basename "$file" .md | tr '-' ' ' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2));}1')
    
    # Check if file has front matter
    if ! head -1 "$file" | grep -q '^---'; then
        # Create a temporary file for the new content
        local tempfile=$(mktemp)
        
        # Add front matter
        echo -e "---\ntitle: \"$title\"\ndescription: \"Documentation for $title\"\n---\n" > "$tempfile"
        # Append the original content
        cat "$file" >> "$tempfile"
        # Replace the original file
        mv "$tempfile" "$file"
        
        print_status "success" "Added front matter to $file"
    else
        # Create a temporary file for updates
        local tempfile=$(mktemp)
        
        # Process the file to update front matter
        local in_front_matter=0
        local title_added=0
        local desc_added=0
        local last_updated_updated=0
        
        while IFS= read -r line; do
            # Check for front matter boundaries
            if [[ "$line" == "---" ]]; then
                if [[ $in_front_matter -eq 0 ]]; then
                    in_front_matter=1
                else
                    # End of front matter, add missing fields before closing
                    if [[ $title_added -eq 0 ]]; then
                        echo "title: \"$title\"" >> "$tempfile"
                    fi
                    if [[ $desc_added -eq 0 ]]; then
                        echo "description: \"Documentation for $title\"" >> "$tempfile"
                    fi
                    if [[ $last_updated_updated -eq 0 ]]; then
                        echo "last_updated: $CURRENT_DATE" >> "$tempfile"
                    fi
                    in_front_matter=0
                fi
                echo "$line" >> "$tempfile"
                continue
            fi
            
            # Process inside front matter
            if [[ $in_front_matter -eq 1 ]]; then
                # Check for existing fields
                if [[ "$line" =~ ^title: ]]; then
                    echo "title: \"$title\"" >> "$tempfile"
                    title_added=1
                elif [[ "$line" =~ ^description: ]]; then
                    echo "description: \"Documentation for $title\"" >> "$tempfile"
                    desc_added=1
                elif [[ "$line" =~ ^last_updated: ]]; then
                    echo "last_updated: $CURRENT_DATE" >> "$tempfile"
                    last_updated_updated=1
                else
                    echo "$line" >> "$tempfile"
                fi
            else
                echo "$line" >> "$tempfile"
            fi
        done < "$file"
        
        # Replace the original file
        mv "$tempfile" "$file"
        
        print_status "success" "Updated front matter in $file"
    fi
}

# Function to add AI label
add_ai_label() {
    local file=$1
    
    # Skip checkpoint files
    if [[ "$file" == *"checkpoints/"* ]]; then
        print_status "warning" "Skipping AI label for checkpoint file: $file"
        return 0
    fi
    
    # Skip if file doesn't exist or is empty
    if [ ! -f "$file" ] || [ ! -s "$file" ]; then
        print_status "warning" "Skipping empty or non-existent file: $file"
        return 0
    fi
    
    # Check if file already has AI label
    if LC_ALL=C grep -q '\[AIR-3\]' "$file"; then
        return 0
    fi
    
    # Create a temporary file
    local tempfile=$(mktemp) || {
        print_status "error" "Failed to create temporary file for $file"
        return 1
    }
    
    # Handle files with front matter
    if LC_ALL=C head -1 "$file" | grep -q '^---'; then
        # Find the end of front matter
        local end_line=$(LC_ALL=C grep -n '^---' "$file" | tail -1 | cut -d: -f1)
        
        if [ -z "$end_line" ]; then
            print_status "error" "Malformed front matter in $file"
            rm -f "$tempfile"
            return 1
        fi
        
        # Copy front matter
        if ! LC_ALL=C head -n "$end_line" "$file" > "$tempfile" 2>/dev/null; then
            print_status "error" "Failed to read front matter from $file"
            rm -f "$tempfile"
            return 1
        fi
        
        # Add AI label after front matter
        echo -e "\n$AI_LABEL" >> "$tempfile"
        
        # Copy the rest of the file if there is any
        if [ $(wc -l < "$file") -gt $end_line ]; then
            echo "" >> "$tempfile"  # Add a blank line before the rest of the content
            LC_ALL=C tail -n +"$((end_line + 1))" "$file" >> "$tempfile" 2>/dev/null || {
                print_status "error" "Failed to read content after front matter in $file"
                rm -f "$tempfile"
                return 1
            }
        fi
    else
        # Add AI label at the beginning for files without front matter
        echo -e "$AI_LABEL\n" > "$tempfile"
        LC_ALL=C cat "$file" >> "$tempfile" 2>/dev/null || {
            print_status "error" "Failed to read content of $file"
            rm -f "$tempfile"
            return 1
        }
    fi
    
    # Replace the original file if the temp file is not empty
    if [ -s "$tempfile" ]; then
        # Create a backup of the original file
        cp "$file" "${file}.bak" 2>/dev/null
        
        # Replace the file atomically
        if mv -f "$tempfile" "$file" 2>/dev/null; then
            print_status "success" "Added AI label to $file"
            rm -f "${file}.bak" 2>/dev/null
            return 0
        else
            print_status "error" "Failed to update $file"
            # Try to restore from backup if it exists
            [ -f "${file}.bak" ] && mv -f "${file}.bak" "$file" 2>/dev/null
            rm -f "$tempfile" 2>/dev/null
            return 1
        fi
    else
        print_status "error" "Failed to process $file - empty result"
        rm -f "$tempfile" 2>/dev/null
        return 1
    fi
}

# Function to safely add content to a file
safe_add_content() {
    local file=$1
    local content=$2
    
    # Create a temporary file
    local temp_file=$(mktemp)
    
    # Add content to temp file
    echo -e "$content" > "$temp_file"
    
    # If the file exists, append to it
    if [ -f "$file" ]; then
        cat "$file" >> "$temp_file"
    fi
    
    # Replace the original file
    mv "$temp_file" "$file"
}

# Function to add required sections
add_required_sections() {
    local file=$1
    
    # Skip checkpoint files
    if [[ "$file" == *"checkpoints/"* ]]; then
        print_status "warning" "Skipping required sections for checkpoint file: $file"
        return 0
    fi
    
    # Skip files that don't exist
    if [ ! -f "$file" ]; then
        print_status "error" "File not found: $file"
        return 1
    fi
    
    # Check if file is empty
    if [ ! -s "$file" ]; then
        print_status "warning" "File is empty: $file"
        return 0
    fi
    
    local needs_toc=0
    local needs_overview=0
    local needs_see_also=0
    
    # Check for required sections using grep -q for better performance
    if ! LC_ALL=C grep -q '^## Table of Contents' "$file" 2>/dev/null; then
        needs_toc=1
    fi
    
    if ! LC_ALL=C grep -q '^## Overview' "$file" 2>/dev/null; then
        needs_overview=1
    fi
    
    if ! LC_ALL=C grep -q '^## See Also' "$file" 2>/dev/null; then
        needs_see_also=1
    fi
    
    # If no sections are needed, return early
    if [ $needs_toc -eq 0 ] && [ $needs_overview -eq 0 ] && [ $needs_see_also -eq 0 ]; then
        return 0
    fi
    
    # Create a backup of the original file
    local backup_file="${file}.bak"
    cp "$file" "$backup_file"
    
    # Create a temporary file for the new content
    local temp_file=$(mktemp)
    
    # Find the first header
    local first_header_line=$(LC_ALL=C grep -n '^#' "$file" | head -1 | cut -d: -f1 2>/dev/null || echo 0)
    
    # If no header was found, add to the beginning of the file
    if [ -z "$first_header_line" ] || [ "$first_header_line" -eq 0 ]; then
        # Add to the beginning of the file
        {
            # Add missing sections first
            if [ $needs_overview -eq 1 ]; then
                echo -e "## Overview\n"
                echo -e "Add a brief overview of this document here.\n"
                print_status "warning" "Added missing Overview section to $file"
            fi
            
            if [ $needs_toc -eq 1 ]; then
                echo -e "## Table of Contents\n"
                echo -e "- [Section 1](#section-1)"
                echo -e "- [Section 2](#section-2)\n"
                print_status "warning" "Added missing Table of Contents to $file"
            fi
            
            if [ $needs_see_also -eq 1 ]; then
                echo -e "## See Also\n"
                echo -e "- [Related Document](#related-document)\n"
                print_status "warning" "Added missing See Also section to $file"
            fi
            
            # Then add the original content
            cat "$backup_file"
        } > "$temp_file"
    else
        # Copy content before first header
        head -n $((first_header_line - 1)) "$backup_file" > "$temp_file"
        
        # Add first header
        head -n $first_header_line "$backup_file" | tail -1 >> "$temp_file"
        echo "" >> "$temp_file"
        
        # Add missing sections
        if [ $needs_overview -eq 1 ]; then
            echo -e "## Overview\n" >> "$temp_file"
            echo -e "Add a brief overview of this document here.\n" >> "$temp_file"
            print_status "warning" "Added missing Overview section to $file"
        fi
        
        if [ $needs_toc -eq 1 ]; then
            echo -e "## Table of Contents\n" >> "$temp_file"
            echo -e "- [Section 1](#section-1)" >> "$temp_file"
            echo -e "- [Section 2](#section-2)\n" >> "$temp_file"
            print_status "warning" "Added missing Table of Contents to $file"
        fi
        
        # Add the rest of the content
        tail -n +$((first_header_line + 1)) "$backup_file" >> "$temp_file"
        
        # Add See Also at the end if needed
        if [ $needs_see_also -eq 1 ]; then
            echo -e "\n## See Also\n" >> "$temp_file"
            echo -e "- [Related Document](#related-document)\n" >> "$temp_file"
            print_status "warning" "Added missing See Also section to $file"
        fi
    fi
    
    # Replace the original file if the temp file is not empty
    if [ -s "$temp_file" ]; then
        mv "$temp_file" "$file"
        # Remove the backup file if the update was successful
        rm -f "$backup_file"
    else
        print_status "error" "Failed to update file: $file"
        # Restore from backup if the temp file is empty
        mv "$backup_file" "$file"
        return 1
    fi
    
    return 0
}

# Function to update a single file
update_file() {
    local file=$1
    
    # Skip if not a markdown file or if it's in the site/ directory
    if [[ "$file" != *.md && "$file" != *.markdown ]] || [[ "$file" == *"site/"* ]]; then
        return 0
    fi
    
    echo -e "\n${BLUE}Processing: $file${NC}"
    
    # Create backup
    cp "$file" "${file}.bak"
    
    # Apply updates
    update_front_matter "$file"
    add_ai_label "$file"
    add_required_sections "$file"
    
    # Clean up backup if no changes
    if diff -q "$file" "${file}.bak" > /dev/null; then
        rm "${file}.bak"
    else
        print_status "success" "Updated $file"
    fi
}

# Export functions and variables for parallel processing
export -f update_file update_front_matter add_ai_label add_required_sections print_status
export GREEN YELLOW RED BLUE NC AI_LABEL CURRENT_DATE

# Main script
echo -e "${YELLOW}🚀 Updating Anya Core Documentation${NC}"

print_header "Starting Documentation Update"

# Check if parallel is available for faster processing
if command -v parallel > /dev/null; then
    echo -e "${GREEN}Using parallel processing for faster updates${NC}"
    find "$DOCS_DIR" -name "*.md" -o -name "*.markdown" | parallel --will-cite -j 4 update_file {}
else
    echo -e "${YELLOW}GNU parallel not found, processing sequentially (install parallel for faster updates)${NC}"
    find "$DOCS_DIR" -name "*.md" -o -name "*.markdown" | while read -r file; do
        update_file "$file"
    done
fi

# Update the main README if it exists
if [ -f "README.md" ]; then
    echo -e "\n${BLUE}Updating main README.md${NC}"
    update_file "README.md"
fi

print_header "Documentation Update Complete"
echo -e "${GREEN}✓ All documentation files have been processed${NC}"
echo -e "\nNext steps:"
echo -e "1. Review the changes with: git diff"
echo -e "2. Commit the updates: git add . && git commit -m 'docs: update documentation structure and metadata'"
echo -e "3. Run documentation checks: ./scripts/verify_docs.sh\n"
echo -e "\nTo view the updated documentation, run: ${YELLOW}./scripts/serve_docs.sh${NC}\n"
