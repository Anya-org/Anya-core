#!/bin/bash
# fix_cargo_edition.sh - Cross-platform version for Linux/macOS
# Script to fix edition inheritance in Cargo.toml files

echo "Starting Cargo.toml fix..."

# Fix count tracking
edition_fixes=0
workspace_fixes=0

find . -name "Cargo.toml" -not -path "*/\.*" | while read file; do
  if [ "$file" = "./Cargo.toml" ]; then
    echo "Skipping root Cargo.toml"
    continue
  fi
  
  modified=0
  
  # Fix edition inheritance
  if grep -q "edition\.workspace = true" "$file"; then
    sed -i.bak 's/edition\.workspace = true/edition = "2021"/g' "$file"
    edition_fixes=$((edition_fixes + 1))
    modified=1
  fi
  
  # Remove conflicting workspace sections
  if grep -q "\[workspace\]" "$file"; then
    # This approach works on both GNU and BSD (macOS) sed
    sed -i.bak '/\[workspace\]/,/\[.*\]/{s/\[workspace\]/# REMOVED CONFLICTING WORKSPACE SECTION/;t;b}' "$file"
    workspace_fixes=$((workspace_fixes + 1))
    modified=1
  fi
  
  # Clean up backup files (macOS/BSD sed creates them)
  rm -f "${file}.bak"
  
  if [ $modified -eq 1 ]; then
    echo "Updated $file"
  else
    echo "No changes needed for $file"
  fi
done

echo "Fixed $edition_fixes edition inheritance issues"
echo "Fixed $workspace_fixes conflicting workspace sections"
echo "Run 'cargo check' to verify the changes."

# Make script executable
chmod +x "$0" 