#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# script_manager.sh - Comprehensive script management utility
# Following official Bitcoin Improvement Proposals (BIPs)
# Part of the Anya Core Hexagonal Architecture
# Date: 2025-05-20

set -e

# Error handling with line number reporting
function handle_error {
    echo "[ERROR] An error occurred at line $1"
    exit 1
}

trap 'handle_error $LINENO' ERR

# Define root directory
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

# Color codes for prettier output
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color
BOLD="\033[1m"

# Function to log messages
function log {
    local level=$1
    local message=$2
    local timestamp=$(date "+%Y-%m-%d %H:%M:%S")
    
    case $level in
        INFO)
            echo -e "${BLUE}[INFO]${NC} $message"
            ;;
        WARN)
            echo -e "${YELLOW}[WARN]${NC} $message"
            ;;
        ERROR)
            echo -e "${RED}[ERROR]${NC} $message"
            ;;
        SUCCESS)
            echo -e "${GREEN}[SUCCESS]${NC} $message"
            ;;
        *)
            echo -e "[${level}] $message"
            ;;
    esac
}

# Function to display help
function show_help {
    echo -e "${BOLD}Anya Core Script Management Utility${NC}"
    echo -e "Following official Bitcoin Improvement Proposals (BIPs)"
    echo
    echo -e "Usage: $0 [OPTION]"
    echo
    echo -e "Options:"
    echo -e "  --analyze              Analyze script organization and identify issues"
    echo -e "  --clean                Clean up redundant scripts"
    echo -e "  --organize             Organize scripts according to hexagonal architecture"
    echo -e "  --validate             Validate AI labeling in all scripts"
    echo -e "  --help                 Display this help message"
    echo
    echo -e "Examples:"
    echo -e "  $0 --analyze           # Analyze script organization"
    echo -e "  $0 --clean             # Clean up redundant scripts"
    echo -e "  $0 --organize          # Organize scripts according to hexagonal architecture"
    echo -e "  $0 --validate          # Validate AI labeling in all scripts"
}

# Function to analyze script organization
function analyze_scripts {
    log INFO "Analyzing script organization..."
    
    # Define hexagonal architecture categories
    local categories=(
        "core"
        "maintenance"
        "security"
        "dev"
        "install"
        "test"
        "ops"
    )
    
    # Count scripts in each category
    echo -e "\n${BOLD}Script Organization Analysis:${NC}"
    echo -e "------------------------------------"
    
    local total_scripts=0
    local categorized_scripts=0
    
    for category in "${categories[@]}"; do
        local count=$(find "$ROOT_DIR/scripts/$category" -type f -name "*.sh" 2>/dev/null | wc -l)
        echo -e "${BOLD}$category:${NC} $count scripts"
        categorized_scripts=$((categorized_scripts + count))
    done
    
    # Count scripts in root scripts directory
    local root_scripts=$(find "$ROOT_DIR/scripts" -maxdepth 1 -type f -name "*.sh" | wc -l)
    echo -e "${BOLD}scripts (root):${NC} $root_scripts scripts"
    
    # Count scripts in project root
    local project_root_scripts=$(find "$ROOT_DIR" -maxdepth 1 -type f -name "*.sh" | wc -l)
    echo -e "${BOLD}project root:${NC} $project_root_scripts scripts"
    
    # Calculate total scripts
    total_scripts=$((categorized_scripts + root_scripts + project_root_scripts))
    
    echo -e "------------------------------------"
    echo -e "${BOLD}Total scripts:${NC} $total_scripts"
    echo -e "${BOLD}Categorized scripts:${NC} $categorized_scripts"
    echo -e "${BOLD}Uncategorized scripts:${NC} $((root_scripts + project_root_scripts))"
    
    # Identify scripts without proper AI labeling
    echo -e "\n${BOLD}Scripts Without Proper AI Labeling:${NC}"
    echo -e "------------------------------------"
    
    local unlabeled_scripts=0
    
    while IFS= read -r script; do
        if ! grep -q "\[AIR-[0-9]\]\[AIS-[0-9]\]\[BPC-[0-9]\]\[RES-[0-9]\]" "$script" && ! grep -q "\[AIR-[0-9]\]\[AIS-[0-9]\]\[AIT-[0-9]\]\[AIP-[0-9]\]\[RES-[0-9]\]" "$script"; then
            echo "$script"
            unlabeled_scripts=$((unlabeled_scripts + 1))
        fi
    done < <(find "$ROOT_DIR" -type f -name "*.sh")
    
    if [ $unlabeled_scripts -eq 0 ]; then
        echo -e "No unlabeled scripts found."
    else
        echo -e "------------------------------------"
        echo -e "${BOLD}Total unlabeled scripts:${NC} $unlabeled_scripts"
    fi
    
    # Identify redundant scripts
    echo -e "\n${BOLD}Potentially Redundant Scripts:${NC}"
    echo -e "------------------------------------"
    
    # Check for duplicate functionality
    local install_scripts=$(find "$ROOT_DIR" -type f -name "*install*.sh" | grep -v "scripts/install" | wc -l)
    if [ $install_scripts -gt 0 ]; then
        echo -e "${YELLOW}Found $install_scripts install scripts outside the install directory${NC}"
    fi
    
    local test_scripts=$(find "$ROOT_DIR" -type f -name "*test*.sh" | grep -v "scripts/test" | wc -l)
    if [ $test_scripts -gt 0 ]; then
        echo -e "${YELLOW}Found $test_scripts test scripts outside the test directory${NC}"
    fi
    
    # Check for scripts in project root
    if [ $project_root_scripts -gt 0 ]; then
        echo -e "${YELLOW}Found $project_root_scripts scripts in project root that should be organized${NC}"
    fi
    
    log SUCCESS "Script analysis completed"
}

# Function to clean up redundant scripts
function clean_scripts {
    log INFO "Cleaning up redundant scripts..."
    
    # Create backup directory
    local backup_dir="$ROOT_DIR/scripts/maintenance/backup_$(date +%Y%m%d-%H%M%S)"
    mkdir -p "$backup_dir"
    
    # Move scripts from project root to appropriate directories
    while IFS= read -r script; do
        local script_name=$(basename "$script")
        
        # Determine appropriate category based on script name
        local target_dir=""
        
        if [[ "$script_name" == *"install"* ]]; then
            target_dir="$ROOT_DIR/scripts/install"
        elif [[ "$script_name" == *"test"* ]]; then
            target_dir="$ROOT_DIR/scripts/test"
        elif [[ "$script_name" == *"setup"* ]]; then
            target_dir="$ROOT_DIR/scripts/core"
        elif [[ "$script_name" == *"security"* || "$script_name" == *"permission"* ]]; then
            target_dir="$ROOT_DIR/scripts/security"
        elif [[ "$script_name" == *"maintenance"* || "$script_name" == *"clean"* ]]; then
            target_dir="$ROOT_DIR/scripts/maintenance"
        elif [[ "$script_name" == *"build"* || "$script_name" == *"dev"* ]]; then
            target_dir="$ROOT_DIR/scripts/dev"
        else
            target_dir="$ROOT_DIR/scripts"
        fi
        
        # Create target directory if it doesn't exist
        mkdir -p "$target_dir"
        
        # Backup the script
        log INFO "Backing up $script_name to $backup_dir"
        cp "$script" "$backup_dir/"
        
        # Move the script to the appropriate directory
        if [ "$target_dir" != "$ROOT_DIR/scripts" ]; then
            log INFO "Moving $script_name to $target_dir"
            git mv "$script" "$target_dir/" 2>/dev/null || mv "$script" "$target_dir/"
        fi
        
    done < <(find "$ROOT_DIR" -maxdepth 1 -type f -name "*.sh")
    
    log SUCCESS "Script cleanup completed"
}

# Function to organize scripts according to hexagonal architecture
function organize_scripts {
    log INFO "Organizing scripts according to hexagonal architecture..."
    
    # Create necessary directories
    mkdir -p "$ROOT_DIR/scripts/core"
    mkdir -p "$ROOT_DIR/scripts/maintenance"
    mkdir -p "$ROOT_DIR/scripts/security"
    mkdir -p "$ROOT_DIR/scripts/dev"
    mkdir -p "$ROOT_DIR/scripts/install"
    mkdir -p "$ROOT_DIR/scripts/test"
    mkdir -p "$ROOT_DIR/scripts/ops"
    
    # Organize scripts in root scripts directory
    while IFS= read -r script; do
        local script_name=$(basename "$script")
        local dir_name=$(dirname "$script")
        
        # Skip if already in a subdirectory
        if [ "$dir_name" != "$ROOT_DIR/scripts" ]; then
            continue
        fi
        
        # Determine appropriate category based on script name
        local target_dir=""
        
        if [[ "$script_name" == *"install"* ]]; then
            target_dir="$ROOT_DIR/scripts/install"
        elif [[ "$script_name" == *"test"* ]]; then
            target_dir="$ROOT_DIR/scripts/test"
        elif [[ "$script_name" == *"setup"* || "$script_name" == *"core"* ]]; then
            target_dir="$ROOT_DIR/scripts/core"
        elif [[ "$script_name" == *"security"* || "$script_name" == *"permission"* ]]; then
            target_dir="$ROOT_DIR/scripts/security"
        elif [[ "$script_name" == *"maintenance"* || "$script_name" == *"clean"* ]]; then
            target_dir="$ROOT_DIR/scripts/maintenance"
        elif [[ "$script_name" == *"build"* || "$script_name" == *"dev"* ]]; then
            target_dir="$ROOT_DIR/scripts/dev"
        elif [[ "$script_name" == *"monitor"* || "$script_name" == *"health"* ]]; then
            target_dir="$ROOT_DIR/scripts/ops"
        else
            continue
        fi
        
        # Move the script to the appropriate directory
        log INFO "Moving $script_name to $target_dir"
        git mv "$script" "$target_dir/" 2>/dev/null || mv "$script" "$target_dir/"
        
    done < <(find "$ROOT_DIR/scripts" -maxdepth 1 -type f -name "*.sh")
    
    log SUCCESS "Script organization completed"
}

# Function to validate AI labeling in all scripts
function validate_ai_labeling {
    log INFO "Validating AI labeling in all scripts..."
    
    local unlabeled_scripts=0
    local total_scripts=0
    
    while IFS= read -r script; do
        total_scripts=$((total_scripts + 1))
        
        if ! grep -q "\[AIR-[0-9]\]\[AIS-[0-9]\]\[BPC-[0-9]\]\[RES-[0-9]\]" "$script" && ! grep -q "\[AIR-[0-9]\]\[AIS-[0-9]\]\[AIT-[0-9]\]\[AIP-[0-9]\]\[RES-[0-9]\]" "$script"; then
            echo -e "${YELLOW}Adding AI labeling to:${NC} $script"
            
            # Add AI labeling to the script
            sed -i '2i# [AIR-3][AIS-3][BPC-3][RES-3]' "$script"
            
            unlabeled_scripts=$((unlabeled_scripts + 1))
        fi
    done < <(find "$ROOT_DIR" -type f -name "*.sh")
    
    echo -e "------------------------------------"
    echo -e "${BOLD}Total scripts:${NC} $total_scripts"
    echo -e "${BOLD}Scripts updated with AI labeling:${NC} $unlabeled_scripts"
    
    log SUCCESS "AI labeling validation completed"
}

# Main function
function main {
    if [ $# -eq 0 ]; then
        show_help
        exit 0
    fi
    
    case "$1" in
        --analyze)
            analyze_scripts
            ;;
        --clean)
            clean_scripts
            ;;
        --organize)
            organize_scripts
            ;;
        --validate)
            validate_ai_labeling
            ;;
        --help)
            show_help
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            show_help
            exit 1
            ;;
    esac
}

# Run the script
main "$@"
