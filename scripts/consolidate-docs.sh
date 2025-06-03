#!/bin/bash
# Documentation Structure Consolidation Script
# Removes duplicate Bitcoin documentation structure

echo "===== Documentation Structure Consolidation Script ====="
echo "Consolidating duplicate Bitcoin documentation structures..."

# Check if duplicate structures exist
if [ -d "/home/bmokoka/Anya-core/src/bitcoin/anya-bitcoin/docs" ] && [ -d "/home/bmokoka/Anya-core/anya-bitcoin/docs" ]; then
    echo "Found duplicate Bitcoin documentation structures"
    
    # Create backup
    backup_dir="/tmp/anya-docs-consolidation-backup-$(date +%Y%m%d-%H%M%S)"
    echo "Creating backup at: $backup_dir"
    mkdir -p "$backup_dir"
    
    # Backup the structure being removed
    cp -r "/home/bmokoka/Anya-core/src/bitcoin/anya-bitcoin/docs" "$backup_dir/src-bitcoin-docs"
    
    # Remove duplicate structure
    echo "Removing duplicate structure: /home/bmokoka/Anya-core/src/bitcoin/anya-bitcoin/docs"
    rm -rf "/home/bmokoka/Anya-core/src/bitcoin/anya-bitcoin/docs"
    
    # Create symbolic link to maintain compatibility
    echo "Creating symbolic link for compatibility"
    ln -sf "../../../../anya-bitcoin/docs" "/home/bmokoka/Anya-core/src/bitcoin/anya-bitcoin/docs"
    
    echo "✅ Consolidated Bitcoin documentation structures"
    echo "   Primary location: /home/bmokoka/Anya-core/anya-bitcoin/docs"
    echo "   Symlink created at: /home/bmokoka/Anya-core/src/bitcoin/anya-bitcoin/docs"
    echo "   Backup location: $backup_dir"
else
    echo "No duplicate structures found or already consolidated"
fi

echo "===== Consolidation Complete ====="
