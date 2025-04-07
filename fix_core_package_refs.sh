#!/bin/bash
# fix_core_package_refs.sh - Cross-platform version for Linux/macOS
# Script to fix references to anya-core-core in Cargo.toml files

echo "Starting package reference fix..."

fix_count=0

find . -name "Cargo.toml" -not -path "*/\.*" | while read file; do
  if [ "$file" = "./Cargo.toml" ]; then
    echo "Skipping root Cargo.toml"
    continue
  fi
  
  if grep -q "anya-core-core" "$file"; then
    sed -i.bak 's/anya-core-core/anya-core-lib/g' "$file"
    echo "Updated $file"
    fix_count=$((fix_count + 1))
    # Clean up backup files
    rm -f "${file}.bak"
  else
    echo "No changes needed for $file"
  fi
done

echo "Fixed $fix_count Cargo.toml files"
echo "Run 'cargo check' to verify the changes."

# Make script executable
chmod +x "$0" 