#!/bin/bash
# This script adds the standard AI label block to all markdown files that are missing it.

# The standard AI label block
LABEL_BLOCK="[AIR-3][AIS-3][BPC-3][RES-3]"

# Find all markdown files in the docs directory that are tracked by git
git ls-files -z -- 'docs/*.md' | while IFS= read -r -d $'\0' file; do
    # Check if the file already contains the label block
    if ! grep -q "$LABEL_BLOCK" "$file"; then
        # If the label block is not found, add it to the beginning of the file
        echo "Adding label to $file"
        echo -e "$LABEL_BLOCK\n$(cat "$file")" > "$file"
    fi
done
