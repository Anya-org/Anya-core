#!/bin/bash
# Documentation Timestamp Update Script
# Updates all instances of "2024-12-07" to "2025-06-02" across markdown files

echo "===== Documentation Timestamp Update Script ====="
echo "Updating timestamps from 2024-12-07 to 2025-06-02..."

# Count files to be updated
echo "Counting files with outdated timestamps..."
file_count=$(find /home/bmokoka/Anya-core -name "*.md" -exec grep -l "2024-12-07" {} \; | wc -l)
echo "Found $file_count files with outdated timestamps"

# Create backup of current state
backup_dir="/tmp/anya-docs-backup-$(date +%Y%m%d-%H%M%S)"
echo "Creating backup at: $backup_dir"
mkdir -p "$backup_dir"

# List of files to update
files_to_update=$(find /home/bmokoka/Anya-core -name "*.md" -exec grep -l "2024-12-07" {} \;)

# Backup files before modification
echo "Backing up files to be modified..."
for file in $files_to_update; do
    rel_path=${file#/home/bmokoka/Anya-core/}
    backup_file="$backup_dir/$rel_path"
    mkdir -p "$(dirname "$backup_file")"
    cp "$file" "$backup_file"
done

# Update timestamps
echo "Updating timestamps..."
updated_count=0
for file in $files_to_update; do
    if sed -i 's/2024-12-07/2025-06-02/g' "$file"; then
        echo "Updated: $file"
        ((updated_count++))
    else
        echo "Failed to update: $file"
    fi
done

echo "===== Update Complete ====="
echo "Files processed: $updated_count"
echo "Backup location: $backup_dir"

# Verify updates
echo "Verifying updates..."
remaining=$(find /home/bmokoka/Anya-core -name "*.md" -exec grep -l "2024-12-07" {} \; | wc -l)
echo "Files still containing old timestamp: $remaining"

if [ "$remaining" -eq 0 ]; then
    echo "✅ All timestamps successfully updated!"
else
    echo "⚠️  Some files may need manual review"
    find /home/bmokoka/Anya-core -name "*.md" -exec grep -l "2024-12-07" {} \;
fi

echo "===== Script Complete ====="
