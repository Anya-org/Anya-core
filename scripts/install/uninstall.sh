#!/bin/bash
# Anya Core Uninstall Script
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
LOGS_DIR="${PROJECT_ROOT}/logs"
mkdir -p "$LOGS_DIR"

# Log file
UNINSTALL_LOG="${LOGS_DIR}/uninstall_$(date +%Y%m%d-%H%M%S).log"

# Default settings
REMOVE_CONFIG=false
REMOVE_DATA=false
REMOVE_LOGS=false
REMOVE_SYSTEMD=true
REMOVE_ENV=true
REMOVE_BINARY=true
REMOVE_ALL=false
SERVICE_NAME="anya-core"
INTERACTIVE=true
FORCE=false

# Function to log messages
log() {
    local level=$1
    shift
    local message=$*
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    case $level in
        ERROR) printf "\033[0;31m[%s] ERROR: %s\033[0m\n" "$timestamp" "$message" | tee -a "$UNINSTALL_LOG" ;;
        WARN)  printf "\033[1;33m[%s] WARN: %s\033[0m\n" "$timestamp" "$message" | tee -a "$UNINSTALL_LOG" ;;
        INFO)  printf "\033[0;32m[%s] INFO: %s\033[0m\n" "$timestamp" "$message" | tee -a "$UNINSTALL_LOG" ;;
        DEBUG) printf "\033[0;34m[%s] DEBUG: %s\033[0m\n" "$timestamp" "$message" | tee -a "$UNINSTALL_LOG" ;;
    esac
    
    # Also log errors and warnings to stderr
    if [[ "$level" == "ERROR" || "$level" == "WARN" ]]; then
        printf "\033[0;31m[%s] %s: %s\033[0m\n" "$timestamp" "$level" "$message" >&2
    fi
}

# Error handling
cleanup() {
    local error_code=$?
    if [ $error_code -ne 0 ]; then
        log ERROR "Uninstall failed with error code $error_code. Check log at $UNINSTALL_LOG"
    fi
    exit $error_code
}

# Set up trap for cleanup on script exit
trap cleanup EXIT INT TERM

# Check for root privileges
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log ERROR "This script requires root privileges. Please run with sudo."
        exit 1
    fi
}

# Show help
show_help() {
    echo "Anya Core Uninstall Script v${VERSION}"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --remove-config          Remove configuration files"
    echo "  --remove-data            Remove data files (blockchain data)"
    echo "  --remove-logs            Remove log files"
    echo "  --no-remove-systemd      Do not remove systemd service"
    echo "  --no-remove-env          Do not remove environment files"
    echo "  --no-remove-binary       Do not remove binary files"
    echo "  --remove-all             Remove everything (overrides other options)"
    echo "  --service-name=NAME      Specify service name (default: anya-core)"
    echo "  --non-interactive        Do not prompt for confirmation"
    echo "  --auto-run               Same as --non-interactive --force"
    echo "  --force                  Force removal even if service is running"
    echo "  --help                   Display this help message"
    echo "  --version                Display script version"
    echo ""
    echo "Example:"
    echo "  sudo $0 --remove-all --auto-run"
}

# Show version
show_version() {
    echo "Anya Core Uninstall Script v${VERSION}"
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help)
                show_help
                exit 0
                ;;
            --version)
                show_version
                exit 0
                ;;
            --remove-config)
                REMOVE_CONFIG=true
                shift
                ;;
            --remove-data)
                REMOVE_DATA=true
                shift
                ;;
            --remove-logs)
                REMOVE_LOGS=true
                shift
                ;;
            --no-remove-systemd)
                REMOVE_SYSTEMD=false
                shift
                ;;
            --no-remove-env)
                REMOVE_ENV=false
                shift
                ;;
            --no-remove-binary)
                REMOVE_BINARY=false
                shift
                ;;
            --remove-all)
                REMOVE_ALL=true
                REMOVE_CONFIG=true
                REMOVE_DATA=true
                REMOVE_LOGS=true
                REMOVE_SYSTEMD=true
                REMOVE_ENV=true
                REMOVE_BINARY=true
                shift
                ;;
            --service-name=*)
                SERVICE_NAME="${1#*=}"
                shift
                ;;
            --non-interactive)
                INTERACTIVE=false
                shift
                ;;
            --auto-run)
                INTERACTIVE=false
                FORCE=true
                shift
                ;;
            --force)
                FORCE=true
                shift
                ;;
            *)
                log ERROR "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    log INFO "Uninstall configured with the following options:"
    log INFO "- Remove Configuration: $REMOVE_CONFIG"
    log INFO "- Remove Data: $REMOVE_DATA"
    log INFO "- Remove Logs: $REMOVE_LOGS"
    log INFO "- Remove Systemd Service: $REMOVE_SYSTEMD"
    log INFO "- Remove Environment Files: $REMOVE_ENV"
    log INFO "- Remove Binary Files: $REMOVE_BINARY"
    log INFO "- Service Name: $SERVICE_NAME"
    log INFO "- Interactive: $INTERACTIVE"
    log INFO "- Force: $FORCE"
}

# Confirm uninstallation
confirm_uninstall() {
    if [ "$INTERACTIVE" = true ]; then
        echo
        echo "WARNING: This will uninstall Anya Core with the following settings:"
        echo "- Remove Configuration: $REMOVE_CONFIG"
        echo "- Remove Data: $REMOVE_DATA"
        echo "- Remove Logs: $REMOVE_LOGS"
        echo "- Remove Systemd Service: $REMOVE_SYSTEMD"
        echo "- Remove Environment Files: $REMOVE_ENV"
        echo "- Remove Binary Files: $REMOVE_BINARY"
        echo
        echo "Service to be removed: $SERVICE_NAME"
        echo
        
        # Display extra warning if removing data
        if [ "$REMOVE_DATA" = true ]; then
            echo "WARNING: You have chosen to remove data files, which may include blockchain data."
            echo "This action CANNOT be undone and may result in loss of important information."
            echo
        fi
        
        read -p "Are you sure you want to uninstall Anya Core? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log INFO "Uninstall cancelled by user"
            exit 0
        fi
        
        # Double-confirm if removing data
        if [ "$REMOVE_DATA" = true ]; then
            echo
            read -p "Are you ABSOLUTELY SURE you want to delete ALL data? (Type 'yes' to confirm): " confirm
            if [ "$confirm" != "yes" ]; then
                log INFO "Uninstall cancelled by user"
                exit 0
            fi
        fi
    fi
}

# Stop and remove systemd service
remove_systemd_service() {
    if [ "$REMOVE_SYSTEMD" = false ]; then
        log INFO "Skipping systemd service removal as requested"
        return 0
    fi
    
    log INFO "Removing systemd service: $SERVICE_NAME"
    
    # Check if service exists
    if ! systemctl list-unit-files | grep -q "$SERVICE_NAME"; then
        log INFO "Service $SERVICE_NAME does not exist. Skipping."
        return 0
    fi
    
    # Check if service is running
    if systemctl is-active "$SERVICE_NAME" &>/dev/null; then
        log INFO "Service $SERVICE_NAME is currently running"
        
        if [ "$FORCE" = false ] && [ "$INTERACTIVE" = true ]; then
            read -p "Stop the service? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                log ERROR "Cannot uninstall while service is running"
                exit 1
            fi
        fi
        
        log INFO "Stopping service..."
        systemctl stop "$SERVICE_NAME" || {
            log ERROR "Failed to stop service. Use --force to override."
            if [ "$FORCE" = false ]; then
                exit 1
            fi
            log WARN "Forcing continued removal despite service stop failure"
        }
    fi
    
    # Disable service
    if systemctl is-enabled "$SERVICE_NAME" &>/dev/null; then
        log INFO "Disabling service..."
        systemctl disable "$SERVICE_NAME" || log WARN "Failed to disable service"
    fi
    
    # Remove service file
    SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
    if [ -f "$SERVICE_FILE" ]; then
        log INFO "Removing service file: $SERVICE_FILE"
        rm -f "$SERVICE_FILE" || log WARN "Failed to remove service file"
    fi
    
    # Reload systemd
    log INFO "Reloading systemd configuration..."
    systemctl daemon-reload || log WARN "Failed to reload systemd configuration"
    
    log INFO "Systemd service removed successfully"
}

# Remove binary files
remove_binary_files() {
    if [ "$REMOVE_BINARY" = false ]; then
        log INFO "Skipping binary files removal as requested"
        return 0
    fi
    
    log INFO "Removing binary files..."
    
    # Check if binary exists
    if [ -f "${PROJECT_ROOT}/target/release/anya-core" ]; then
        log INFO "Removing binary: ${PROJECT_ROOT}/target/release/anya-core"
        rm -f "${PROJECT_ROOT}/target/release/anya-core" || log WARN "Failed to remove binary"
    else
        log INFO "Binary not found at ${PROJECT_ROOT}/target/release/anya-core. Skipping."
    fi
    
    log INFO "Binary files removed successfully"
}

# Remove configuration files
remove_config_files() {
    if [ "$REMOVE_CONFIG" = false ]; then
        log INFO "Skipping configuration files removal as requested"
        return 0
    fi
    
    log INFO "Removing configuration files..."
    
    # Check if config directory exists
    if [ -d "${PROJECT_ROOT}/config" ]; then
        log INFO "Removing config directory: ${PROJECT_ROOT}/config"
        rm -rf "${PROJECT_ROOT}/config" || log WARN "Failed to remove config directory"
    else
        log INFO "Config directory not found at ${PROJECT_ROOT}/config. Skipping."
    fi
    
    log INFO "Configuration files removed successfully"
}

# Remove data files
remove_data_files() {
    if [ "$REMOVE_DATA" = false ]; then
        log INFO "Skipping data files removal as requested"
        return 0
    fi
    
    log INFO "Removing data files..."
    
    # Check if data directory exists
    if [ -d "${PROJECT_ROOT}/data" ]; then
        log INFO "Removing data directory: ${PROJECT_ROOT}/data"
        rm -rf "${PROJECT_ROOT}/data" || log WARN "Failed to remove data directory"
    else
        log INFO "Data directory not found at ${PROJECT_ROOT}/data. Skipping."
    fi
    
    log INFO "Data files removed successfully"
}

# Remove log files
remove_log_files() {
    if [ "$REMOVE_LOGS" = false ]; then
        log INFO "Skipping log files removal as requested"
        return 0
    fi
    
    log INFO "Removing log files..."
    
    # Check if logs directory exists
    if [ -d "${PROJECT_ROOT}/logs" ]; then
        log INFO "Removing logs directory: ${PROJECT_ROOT}/logs"
        rm -rf "${PROJECT_ROOT}/logs" || log WARN "Failed to remove logs directory"
    else
        log INFO "Logs directory not found at ${PROJECT_ROOT}/logs. Skipping."
    fi
    
    log INFO "Log files removed successfully"
}

# Remove environment files
remove_env_files() {
    if [ "$REMOVE_ENV" = false ]; then
        log INFO "Skipping environment files removal as requested"
        return 0
    fi
    
    log INFO "Removing environment files..."
    
    # Find all possible users
    for USER_HOME in /home/*; do
        if [ -d "$USER_HOME" ]; then
            USER_NAME=$(basename "$USER_HOME")
            
            # Check if environment file exists
            ENV_FILE="${USER_HOME}/.anya-env"
            if [ -f "$ENV_FILE" ]; then
                log INFO "Found environment file for user $USER_NAME: $ENV_FILE"
                
                # Remove environment file
                rm -f "$ENV_FILE" || log WARN "Failed to remove environment file for user $USER_NAME"
                
                # Check if it's referenced in bashrc
                BASHRC="${USER_HOME}/.bashrc"
                if [ -f "$BASHRC" ] && grep -q "source ~/.anya-env" "$BASHRC"; then
                    log INFO "Removing reference from ${USER_NAME}'s .bashrc"
                    
                    # Create temporary file
                    TEMP_FILE=$(mktemp)
                    
                    # Remove lines containing anya-env references
                    grep -v "source ~/.anya-env\|# Anya Core environment" "$BASHRC" > "$TEMP_FILE"
                    
                    # Replace original file
                    mv "$TEMP_FILE" "$BASHRC"
                    
                    # Fix ownership
                    chown "$USER_NAME:$(id -gn "$USER_NAME" 2>/dev/null || echo "$USER_NAME")" "$BASHRC"
                    chmod 644 "$BASHRC"
                fi
            fi
        fi
    done
    
    log INFO "Environment files removed successfully"
}

# Show completion message
show_completion() {
    echo
    echo "================================================================"
    echo "            Anya Core Uninstall Complete"
    echo "================================================================"
    echo
    echo "The following components were removed:"
    [ "$REMOVE_SYSTEMD" = true ] && echo "- Systemd service ($SERVICE_NAME)"
    [ "$REMOVE_BINARY" = true ] && echo "- Binary files"
    [ "$REMOVE_CONFIG" = true ] && echo "- Configuration files"
    [ "$REMOVE_DATA" = true ] && echo "- Data files"
    [ "$REMOVE_LOGS" = true ] && echo "- Log files"
    [ "$REMOVE_ENV" = true ] && echo "- Environment files"
    echo
    echo "Uninstall log saved to: ${UNINSTALL_LOG}"
    echo
    
    if [ "$REMOVE_ALL" = true ]; then
        echo "Anya Core has been completely removed from your system."
    else
        echo "Some components were not removed. Use --remove-all to remove everything."
    fi
    echo "================================================================"
}

# Main function
main() {
    log INFO "Starting Anya Core uninstall process (version $VERSION)..."
    
    # Parse command line arguments
    parse_args "$@"
    
    # Check for root privileges
    check_root
    
    # Confirm uninstallation
    confirm_uninstall
    
    # Stop and remove systemd service
    remove_systemd_service
    
    # Remove binary files
    remove_binary_files
    
    # Remove configuration files
    remove_config_files
    
    # Remove data files
    remove_data_files
    
    # Remove log files
    remove_log_files
    
    # Remove environment files
    remove_env_files
    
    # Show completion message
    show_completion
    
    log INFO "Anya Core uninstall completed successfully"
}

# Run the script
main "$@" 