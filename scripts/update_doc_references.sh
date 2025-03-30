#!/bin/bash
# Script to update references to deprecated documentation files
# [AIR-3][AIS-3][BPC-3]

set -e

echo "Updating documentation references..."

# Define mappings (old path → new path)
declare -A doc_mappings=(
  ["docs/AI_LABELING.md"]="docs/standards/AI_LABELING.md"
  ["docs/SYSTEM_MAP.md"]="docs/architecture/SYSTEM_MAP.md"
  ["docs/LABELLING_SYSTEM.md"]="docs/standards/AI_LABELING.md"
  ["docs/API.md"]="docs/api/api-reference.md"
)

# Find all markdown and text files that may contain references
find_cmd="find . -type f -not -path \"*/\.*\" -not -path \"*/target/*\" -not -path \"*/node_modules/*\""
for ext in md txt rs js py toml; do
  files="$files $(eval "$find_cmd -name \"*.$ext\"")"
done

# Process each file
for file in $files; do
  needs_update=0
  
  # Check if any deprecated path exists in the file
  for old_path in "${!doc_mappings[@]}"; do
    if grep -q "$old_path" "$file"; then
      needs_update=1
      echo "Found reference to $old_path in $file"
      break
    fi
  done
  
  # Update the file if needed
  if [ $needs_update -eq 1 ]; then
    echo "Updating references in $file"
    
    # Create a temporary file
    temp_file=$(mktemp)
    
    # Replace all deprecated references
    cat "$file" > "$temp_file"
    for old_path in "${!doc_mappings[@]}"; do
      new_path=${doc_mappings[$old_path]}
      echo "  Replacing $old_path with $new_path"
      sed -i "s|$old_path|$new_path|g" "$temp_file"
    done
    
    # Move the temporary file back
    mv "$temp_file" "$file"
  fi
done

echo "Documentation reference update complete!"
echo "Added a reference to the new locations in /docs/REMOVED.md"
