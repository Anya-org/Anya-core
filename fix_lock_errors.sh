#!/bin/bash

# Script to fix the lock()? pattern across multiple files
# Replaces lock()? with lock().map_err(|e| format!("Mutex lock error: {}", e))?

# Get list of files containing lock()?
FILES=$(find /home/anya/anyachainlabs/projects/anya-core/src -name "*.rs" -exec grep -l "lock()?" {} \;)

# Fix each file
for file in $FILES; do
  echo "Fixing file: $file"
  # Replace lock()? with map_err version
  sed -i 's/\.lock()?/\.lock().map_err(|e| format!("Mutex lock error: {}", e))?/g' "$file"
done

echo "Done fixing lock()? errors."
