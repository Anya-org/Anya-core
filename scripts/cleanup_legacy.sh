#!/bin/bash
# Anya Core Legacy System Cleanup Script
# [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
#
# This script safely removes legacy installation components after migrating
# to the unified installer system.

set -e

# Terminal colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script variables
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKUP_DIR="$PROJECT_ROOT/legacy_backup_$(date +%Y%m%d%H%M%S)"
LOG_FILE="$PROJECT_ROOT/logs/cleanup.log"

# Function to print status messages
log() {
    local level=$1
    shift
    local message="$@"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    case $level in
        "info")
            echo -e "${BLUE}[INFO]${NC} $message"
            ;;
        "success")
            echo -e "${GREEN}[SUCCESS]${NC} $message"
            ;;
        "warning")
            echo -e "${YELLOW}[WARNING]${NC} $message"
            ;;
        "error")
            echo -e "${RED}[ERROR]${NC} $message"
            ;;
        *)
            echo -e "$message"
            ;;
    esac
    
    # Create log directory if it doesn't exist
    mkdir -p "$(dirname "$LOG_FILE")"
    
    # Write to log file
    echo "[${timestamp}] [${level^^}] $message" >> "$LOG_FILE"
}

# Function to display help
show_help() {
    echo "Anya Core Legacy System Cleanup Script"
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help                 Show this help message"
    echo "  -b, --backup               Create a backup of legacy files before removing"
    echo "  -f, --force                Skip confirmation prompts"
    echo "  --skip-powershell          Skip cleanup of PowerShell installer"
    echo "  --skip-python              Skip cleanup of Python installer"
    echo "  --skip-rust                Skip cleanup of old Rust installer"
    echo ""
    echo "Examples:"
    echo "  $0 --backup"
    echo "  $0 --force --skip-powershell"
    echo ""
}

# Function to backup a file or directory
backup_item() {
    local source=$1
    
    if [ -e "$source" ]; then
        local rel_path=${source#$PROJECT_ROOT/}
        local backup_path="$BACKUP_DIR/$rel_path"
        
        # Create the directory structure
        mkdir -p "$(dirname "$backup_path")"
        
        # Copy the file or directory
        cp -r "$source" "$backup_path"
        log "info" "Backed up: $rel_path"
    fi
}

# Function to safely remove a file or directory
safe_remove() {
    local target=$1
    local backup=$2
    
    if [ -e "$target" ]; then
        local rel_path=${target#$PROJECT_ROOT/}
        
        # Backup the item if requested
        if [ "$backup" = true ]; then
            backup_item "$target"
        fi
        
        # Remove the item
        if [ -d "$target" ]; then
            rm -rf "$target"
        else
            rm -f "$target"
        fi
        
        log "success" "Removed: $rel_path"
    fi
}

# Function to cleanup PowerShell installer
cleanup_powershell() {
    log "info" "Cleaning up PowerShell installer components..."
    
    # List of PowerShell files to remove
    local ps_files=(
        "Install-AnyaCore.ps1"
        "Install-Core.ps1"
        "Install-Dependencies.ps1"
        "Modules/BitcoinModule.ps1"
        "Modules/Web5Module.ps1"
        "Modules/DeploymentModule.ps1"
        "Modules/LoggingModule.ps1"
    )
    
    for file in "${ps_files[@]}"; do
        safe_remove "$PROJECT_ROOT/$file" "$BACKUP"
    done
    
    # Remove Modules directory if empty
    if [ -d "$PROJECT_ROOT/Modules" ] && [ -z "$(ls -A "$PROJECT_ROOT/Modules")" ]; then
        rmdir "$PROJECT_ROOT/Modules"
        log "info" "Removed empty directory: Modules"
    fi
    
    log "success" "PowerShell cleanup completed"
}

# Function to cleanup Python installer
cleanup_python() {
    log "info" "Cleaning up Python installer components..."
    
    # List of Python files to remove
    local py_files=(
        "install.py"
        "setup.py"
        "requirements.txt"
        "python/installer.py"
        "python/validator.py"
        "python/config.py"
    )
    
    for file in "${py_files[@]}"; do
        safe_remove "$PROJECT_ROOT/$file" "$BACKUP"
    done
    
    # Remove Python directory if empty
    if [ -d "$PROJECT_ROOT/python" ] && [ -z "$(ls -A "$PROJECT_ROOT/python")" ]; then
        rmdir "$PROJECT_ROOT/python"
        log "info" "Removed empty directory: python"
    fi
    
    log "success" "Python cleanup completed"
}

# Function to cleanup old Rust installer
cleanup_rust() {
    log "info" "Cleaning up old Rust installer components..."
    
    # List of old Rust files to remove
    local rust_files=(
        "src/bin/anya_installer.rs"
        "src/install/main.rs"
        "src/install/components.rs"
        "src/install/config.rs"
        "src/install/validation.rs"
        "src/install/bitcoin_compliance.rs"
        "src/install/telemetry.rs"
        "src/install/modes.rs"
    )
    
    for file in "${rust_files[@]}"; do
        safe_remove "$PROJECT_ROOT/$file" "$BACKUP"
    done
    
    # Remove install directory if empty
    if [ -d "$PROJECT_ROOT/src/install" ] && [ -z "$(ls -A "$PROJECT_ROOT/src/install")" ]; then
        rmdir "$PROJECT_ROOT/src/install"
        log "info" "Removed empty directory: src/install"
    fi
    
    log "success" "Old Rust installer cleanup completed"
}

# Function to cleanup legacy configuration files
cleanup_config() {
    log "info" "Cleaning up legacy configuration files..."
    
    # List of legacy config files to remove
    local config_files=(
        "config/legacy_config.yaml"
        "config/installer.json"
        "config/deploy_settings.yaml"
    )
    
    for file in "${config_files[@]}"; do
        safe_remove "$PROJECT_ROOT/$file" "$BACKUP"
    done
    
    log "success" "Legacy configuration cleanup completed"
}

# Function to check for mixed installation
check_mixed_installation() {
    log "info" "Checking for mixed installation components..."
    
    local has_new=false
    local has_old=false
    
    # Check for new unified installer
    if [ -f "$PROJECT_ROOT/src/bin/unified_installer.rs" ]; then
        has_new=true
    fi
    
    # Check for old installers
    if [ -f "$PROJECT_ROOT/src/bin/anya_installer.rs" ] || 
       [ -f "$PROJECT_ROOT/Install-AnyaCore.ps1" ] || 
       [ -f "$PROJECT_ROOT/install.py" ]; then
        has_old=true
    fi
    
    # If we have both new and old components, warn about potential conflicts
    if [ "$has_new" = true ] && [ "$has_old" = true ]; then
        log "warning" "Detected both unified installer and legacy components."
        log "warning" "This might cause conflicts. Please backup your configuration."
    fi
}

# Main function
main() {
    # Parse command line arguments
    BACKUP=false
    FORCE=false
    SKIP_POWERSHELL=false
    SKIP_PYTHON=false
    SKIP_RUST=false
    
    while [[ $# -gt 0 ]]; do
        key="$1"
        case $key in
            -h|--help)
                show_help
                exit 0
                ;;
            -b|--backup)
                BACKUP=true
                shift
                ;;
            -f|--force)
                FORCE=true
                shift
                ;;
            --skip-powershell)
                SKIP_POWERSHELL=true
                shift
                ;;
            --skip-python)
                SKIP_PYTHON=true
                shift
                ;;
            --skip-rust)
                SKIP_RUST=true
                shift
                ;;
            *)
                log "error" "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    log "info" "Starting legacy system cleanup"
    
    # Check for mixed installation
    check_mixed_installation
    
    # Create backup directory if backup is enabled
    if [ "$BACKUP" = true ]; then
        mkdir -p "$BACKUP_DIR"
        log "info" "Created backup directory: $BACKUP_DIR"
    fi
    
    # Prompt for confirmation if not forced
    if [ "$FORCE" = false ]; then
        read -p "This will remove legacy installation files. Continue? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log "info" "Cleanup cancelled by user"
            exit 0
        fi
    fi
    
    # Perform component cleanup
    if [ "$SKIP_POWERSHELL" = false ]; then
        cleanup_powershell
    fi
    
    if [ "$SKIP_PYTHON" = false ]; then
        cleanup_python
    fi
    
    if [ "$SKIP_RUST" = false ]; then
        cleanup_rust
    fi
    
    # Always cleanup legacy configs
    cleanup_config
    
    # Provide a summary of actions
    log "success" "Legacy system cleanup completed successfully"
    
    if [ "$BACKUP" = true ]; then
        log "info" "Backup created at: $BACKUP_DIR"
    fi
}

# Run main function
main "$@" 