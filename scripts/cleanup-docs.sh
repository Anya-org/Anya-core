#!/bin/bash
# Documentation Cleanup Script
# Created: June 17, 2025
# Purpose: Automate the cleanup of documentation files in the Anya-core repository

# Set strict error handling
set -e

# Define colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create timestamp for the backup
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
BACKUP_DIR="/tmp/anya-docs-backup-${TIMESTAMP}"

echo -e "${BLUE}Anya Core Documentation Cleanup Script${NC}"
echo -e "${BLUE}======================================${NC}"

# Create backup directory
echo -e "${YELLOW}Creating backup directory: ${BACKUP_DIR}${NC}"
mkdir -p "${BACKUP_DIR}"

# Function to backup a file before removing it
backup_file() {
    if [ -f "$1" ]; then
        echo -e "${YELLOW}Backing up: $1${NC}"
        cp "$1" "${BACKUP_DIR}/$(basename "$1")"
        return 0
    else
        echo -e "${RED}File not found: $1${NC}"
        return 1
    fi
}

# Function to backup a directory before removing it
backup_dir() {
    if [ -d "$1" ]; then
        DIR_NAME=$(basename "$1")
        echo -e "${YELLOW}Backing up directory: $1${NC}"
        mkdir -p "${BACKUP_DIR}/${DIR_NAME}"
        cp -r "$1" "${BACKUP_DIR}/"
        return 0
    else
        echo -e "${RED}Directory not found: $1${NC}"
        return 1
    fi
}

# Function to remove a file
remove_file() {
    if backup_file "$1"; then
        echo -e "${RED}Removing file: $1${NC}"
        rm "$1"
        echo -e "${GREEN}File removed successfully${NC}"
    fi
}

# Function to remove a directory
remove_dir() {
    if backup_dir "$1"; then
        echo -e "${RED}Removing directory: $1${NC}"
        rm -rf "$1"
        echo -e "${GREEN}Directory removed successfully${NC}"
    fi
}

# Step 1: Remove redundant index files
echo -e "\n${BLUE}Step 1: Removing redundant index files${NC}"
remove_file "/workspaces/Anya-core/INDEX.md"
remove_file "/workspaces/Anya-core/INDEX_ORIGINAL.md"

# Step 2: Remove files with false production claims
echo -e "\n${BLUE}Step 2: Removing files with false production claims${NC}"
remove_file "/workspaces/Anya-core/COMPILATION_STATUS_v1.2.0.md"
remove_file "/workspaces/Anya-core/LAYER2_COMPLETION_REPORT.md"

# Step 3: Remove duplicate documentation
echo -e "\n${BLUE}Step 3: Removing duplicate documentation${NC}"
remove_dir "/workspaces/Anya-core/src/bitcoin/anya-bitcoin/docs/"

# Step 4: Find and remove backup files
echo -e "\n${BLUE}Step 4: Removing backup files${NC}"
find /workspaces/Anya-core -name "*.backup" -type f -print0 | while IFS= read -r -d '' file; do
    remove_file "$file"
done

# Step 5: Find and remove .disabled files
echo -e "\n${BLUE}Step 5: Removing disabled files${NC}"
find /workspaces/Anya-core -name "*.disabled" -type f -print0 | while IFS= read -r -d '' file; do
    remove_file "$file"
done

# Step 6: Create a symbolic link for documentation consistency
echo -e "\n${BLUE}Step 6: Creating symbolic links for documentation consistency${NC}"
if [ -d "/workspaces/Anya-core/src/bitcoin/anya-bitcoin" ] && [ ! -d "/workspaces/Anya-core/src/bitcoin/anya-bitcoin/docs" ]; then
    echo -e "${YELLOW}Creating symbolic link for Bitcoin documentation${NC}"
    ln -sf "/workspaces/Anya-core/anya-bitcoin/docs" "/workspaces/Anya-core/src/bitcoin/anya-bitcoin/docs"
    echo -e "${GREEN}Symbolic link created successfully${NC}"
fi

# Step 7: Update timestamps in critical files
echo -e "\n${BLUE}Step 7: Updating timestamps in critical files${NC}"
CURRENT_DATE="June 17, 2025"
FILES_TO_UPDATE=(
    "/workspaces/Anya-core/docs/SYSTEM_MAP.md"
    "/workspaces/Anya-core/ROOT_INDEX.md"
)

for file in "${FILES_TO_UPDATE[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${YELLOW}Updating timestamps in: $file${NC}"
        sed -i "s/[Jj]une [0-9]\+, 2025/${CURRENT_DATE}/g" "$file"
        echo -e "${GREEN}Timestamps updated successfully${NC}"
    else
        echo -e "${RED}File not found: $file${NC}"
    fi
done

echo -e "\n${GREEN}Documentation cleanup completed successfully!${NC}"
echo -e "${YELLOW}Backup created at: ${BACKUP_DIR}${NC}"
echo -e "${BLUE}Please verify the changes and run appropriate tests to ensure everything works correctly.${NC}"
