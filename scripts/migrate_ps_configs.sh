#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m'

# Directories to check for old configs
DIRS_TO_CHECK="config install installer Modules scripts"

echo -e "${YELLOW}Scanning for PowerShell configurations...${NC}"

# Find and process PowerShell configurations
for dir in $DIRS_TO_CHECK; do
    if [ -d "$dir" ]; then
        echo -e "\nChecking $dir..."
        
        # Find any remaining .ps1 configs
        find "$dir" -type f -name "*.ps1" | while read -r file; do
            echo -e "${YELLOW}Found PowerShell config: $file${NC}"
            
            # Extract config values
            if grep -q "network_type" "$file"; then
                network=$(grep "network_type" "$file" | cut -d'"' -f2)
                echo "- Migrating network configuration: $network"
                ./target/release/anya_tools module bitcoin --network "$network"
            fi
            
            if grep -q "log_level" "$file"; then
                level=$(grep "log_level" "$file" | cut -d'"' -f2)
                echo "- Migrating logging configuration: $level"
                ./target/release/anya_tools module logging --level "$level"
            fi

            if grep -q "deployment_target" "$file"; then
                target=$(grep "deployment_target" "$file" | cut -d'"' -f2)
                echo "- Migrating deployment configuration: $target"
                ./target/release/anya_tools module deployment --target "$target"
            fi
            
            echo -e "${GREEN}Migrated configuration from $file${NC}"
        done
    fi
done

echo -e "\n${GREEN}Configuration migration complete!${NC}"