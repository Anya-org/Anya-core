#!/bin/bash
# Anya Core Systemd Service Configuration
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
INSTALL_LOG="${LOGS_DIR}/systemd_config_$(date +%Y%m%d-%H%M%S).log"

# Function to log messages
log() {
    local level=$1
    shift
    local message=$*
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    case $level in
        ERROR) printf "\033[0;31m[%s] ERROR: %s\033[0m\n" "$timestamp" "$message" | tee -a "$INSTALL_LOG" ;;
        WARN)  printf "\033[1;33m[%s] WARN: %s\033[0m\n" "$timestamp" "$message" | tee -a "$INSTALL_LOG" ;;
        INFO)  printf "\033[0;32m[%s] INFO: %s\033[0m\n" "$timestamp" "$message" | tee -a "$INSTALL_LOG" ;;
        DEBUG) printf "\033[0;34m[%s] DEBUG: %s\033[0m\n" "$timestamp" "$message" | tee -a "$INSTALL_LOG" ;;
    esac
    
    # Also log errors and warnings to stderr
    if [[ "$level" == "ERROR" || "$level" == "WARN" ]]; then
        printf "\033[0;31m[%s] %s: %s\033[0m\n" "$timestamp" "$level" "$message" >&2
    fi
}

# Setup error handling
cleanup() {
    local error_code=$?
    if [ $error_code -ne 0 ]; then
        log ERROR "Systemd configuration failed with error code $error_code. Check log at $INSTALL_LOG"
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
    echo "Anya Core Systemd Service Configuration v${VERSION}"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --user=USERNAME         Set the user to run the service (default: current user)"
    echo "  --service-name=NAME     Set custom service name (default: anya-core)"
    echo "  --no-enable             Don't enable the service to start on boot"
    echo "  --start                 Start the service after configuration"
    echo "  --binary-path=PATH      Custom path to the binary (default: auto-detect)"
    echo "  --config-path=PATH      Custom path to config file (default: auto-detect)"
    echo "  --hardening=LEVEL       Security hardening level (basic, standard, strict)"
    echo "  --help                  Display this help message"
    echo "  --version               Display script version"
    echo ""
    echo "Example:"
    echo "  sudo $0 --user=anya --start --hardening=standard"
}

# Show version
show_version() {
    echo "Anya Core Systemd Service Configuration v${VERSION}"
}

# Parse command line arguments
parse_args() {
    USER_NAME="${SUDO_USER:-$USER}"
    SERVICE_NAME="anya-core"
    ENABLE_SERVICE=true
    START_SERVICE=false
    BINARY_PATH=""
    CONFIG_PATH=""
    HARDENING_LEVEL="standard"
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --user=*)
                USER_NAME="${1#*=}"
                shift
                ;;
            --service-name=*)
                SERVICE_NAME="${1#*=}"
                # Validate service name (alphanumeric, dash, underscore only)
                if ! [[ $SERVICE_NAME =~ ^[a-zA-Z0-9_-]+$ ]]; then
                    log ERROR "Invalid service name: $SERVICE_NAME (use only letters, numbers, dash, underscore)"
                    exit 1
                fi
                shift
                ;;
            --no-enable)
                ENABLE_SERVICE=false
                shift
                ;;
            --start)
                START_SERVICE=true
                shift
                ;;
            --binary-path=*)
                BINARY_PATH="${1#*=}"
                # Validate binary path exists
                if [ ! -f "$BINARY_PATH" ]; then
                    log ERROR "Binary not found at path: $BINARY_PATH"
                    exit 1
                fi
                shift
                ;;
            --config-path=*)
                CONFIG_PATH="${1#*=}"
                # Validate config path exists
                if [ ! -f "$CONFIG_PATH" ]; then
                    log ERROR "Config file not found at path: $CONFIG_PATH"
                    exit 1
                fi
                shift
                ;;
            --hardening=*)
                HARDENING_LEVEL="${1#*=}"
                # Validate hardening level
                if ! [[ "$HARDENING_LEVEL" =~ ^(basic|standard|strict)$ ]]; then
                    log ERROR "Invalid hardening level: $HARDENING_LEVEL (must be basic, standard, or strict)"
                    exit 1
                fi
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            --version)
                show_version
                exit 0
                ;;
            *)
                log ERROR "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    # Check if user exists
    if ! id "$USER_NAME" &>/dev/null; then
        log ERROR "User $USER_NAME does not exist"
        exit 1
    fi
    
    # Auto-detect binary path if not specified
    if [ -z "$BINARY_PATH" ]; then
        if [ -f "${PROJECT_ROOT}/target/release/anya-core" ]; then
            BINARY_PATH="${PROJECT_ROOT}/target/release/anya-core"
        else
            log ERROR "Anya Core binary not found. Specify with --binary-path or build the project first."
            exit 1
        fi
    fi
    
    # Auto-detect config path if not specified
    if [ -z "$CONFIG_PATH" ]; then
        if [ -f "${PROJECT_ROOT}/config/anya.conf" ]; then
            CONFIG_PATH="${PROJECT_ROOT}/config/anya.conf"
        else
            CONFIG_PATH="${PROJECT_ROOT}/config/anya.conf"
            log WARN "Config file not found. Will be created at: $CONFIG_PATH"
        fi
    fi
    
    log INFO "Configuration parameters:"
    log INFO "- User: $USER_NAME"
    log INFO "- Service Name: $SERVICE_NAME"
    log INFO "- Binary Path: $BINARY_PATH"
    log INFO "- Config Path: $CONFIG_PATH"
    log INFO "- Hardening Level: $HARDENING_LEVEL"
    log INFO "- Auto Start: $START_SERVICE"
    log INFO "- Auto Enable: $ENABLE_SERVICE"
}

# Configure systemd service
configure_systemd() {
    log INFO "Configuring systemd service: $SERVICE_NAME..."
    
    # Get user details
    USER_HOME=$(eval echo ~${USER_NAME})
    USER_GROUP=$(id -gn "$USER_NAME")
    
    # Create systemd service file with appropriate hardening
    SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
    
    # Generate hardening parameters based on level
    HARDENING_PARAMS=""
    case "$HARDENING_LEVEL" in
        basic)
            HARDENING_PARAMS=$(cat << EOF
# Basic security hardening
ProtectSystem=full
PrivateTmp=true
NoNewPrivileges=true
EOF
)
            ;;
        standard)
            HARDENING_PARAMS=$(cat << EOF
# Standard security hardening
ProtectSystem=full
PrivateTmp=true
NoNewPrivileges=true
PrivateDevices=true
MemoryDenyWriteExecute=true
ProtectHome=read-only
ProtectKernelTunables=true
ProtectControlGroups=true
RestrictAddressFamilies=AF_UNIX AF_INET AF_INET6
RestrictNamespaces=true
EOF
)
            ;;
        strict)
            HARDENING_PARAMS=$(cat << EOF
# Strict security hardening
ProtectSystem=strict
PrivateTmp=true
NoNewPrivileges=true
PrivateDevices=true
MemoryDenyWriteExecute=true
ProtectHome=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictAddressFamilies=AF_UNIX AF_INET AF_INET6
RestrictNamespaces=true
RestrictRealtime=true
RestrictSUIDSGID=true
LockPersonality=true
SystemCallArchitectures=native
SystemCallFilter=@system-service
SystemCallFilter=~@privileged @resources
CapabilityBoundingSet=~CAP_SYS_ADMIN CAP_SYS_PTRACE CAP_SETUID CAP_SETGID
EOF
)
            ;;
    esac
    
    # Get system resources for limiting
    CPU_CORES="${CPU_CORES:-$(nproc)}"
    TOTAL_MEM="${TOTAL_MEM:-$(free -m | awk '/^Mem:/{print $2}')}"
    
    # Calculate resource limits (75% of available by default)
    CPU_PERCENT=75
    MEM_LIMIT="${MEMORY_LIMIT:-$((TOTAL_MEM * 75 / 100))}"
    
    # Calculate nice value based on installation type
    NICE_VALUE=0
    if [ "${HSM_TYPE:-software}" = "hardware" ]; then
        # Lower nice value (higher priority) for hardware HSM setups
        NICE_VALUE=-5
    fi
    
    # Create service file with system-specific resource limits
    cat > "$SERVICE_FILE" << EOF
[Unit]
Description=Anya Core Bitcoin Service
Documentation=https://docs.anya-core.org
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=${USER_NAME}
Group=${USER_GROUP}
WorkingDirectory=${PROJECT_ROOT}
ExecStart=${BINARY_PATH}
ExecStop=/bin/kill -s SIGTERM \$MAINPID
Restart=on-failure
RestartSec=5s
TimeoutStartSec=30s
TimeoutStopSec=30s
StandardOutput=journal
StandardError=journal
SyslogIdentifier=anya-core

${HARDENING_PARAMS}

# Resource limits (auto-configured)
CPUQuota=${CPU_PERCENT}%
MemoryLimit=${MEM_LIMIT}M
TasksMax=4096
LimitNOFILE=65535
Nice=${NICE_VALUE}

# Environment variables
Environment="RUST_LOG=info"
Environment="ANYA_HOME=${PROJECT_ROOT}"
Environment="ANYA_CONFIG=${CONFIG_PATH}"
# System-specific environment variables
Environment="ANYA_CPU_CORES=${CPU_CORES}"
Environment="ANYA_MEMORY_LIMIT=${MEM_LIMIT}"
Environment="ANYA_HSM_TYPE=${HSM_TYPE:-software}"

[Install]
WantedBy=multi-user.target
EOF
    
    log INFO "Created systemd service at $SERVICE_FILE with resource limits (CPU: ${CPU_PERCENT}%, Memory: ${MEM_LIMIT}MB)"
    chmod 644 "$SERVICE_FILE"
    
    # Verify service file syntax
    if ! systemd-analyze verify "$SERVICE_FILE" &>/dev/null; then
        log WARN "Systemd service file verification failed, checking for errors..."
        systemd-analyze verify "$SERVICE_FILE" || {
            log ERROR "Systemd service file has errors. Manual review needed."
            return 1
        }
    else
        log INFO "Systemd service file verified successfully"
    fi
    
    # Reload systemd
    systemctl daemon-reload || {
        log ERROR "Failed to reload systemd configuration"
        return 1
    }
    log INFO "Reloaded systemd configuration"
    
    # Enable service if requested
    if [ "$ENABLE_SERVICE" = true ]; then
        systemctl enable "$SERVICE_NAME" || {
            log ERROR "Failed to enable $SERVICE_NAME service"
            return 1
        }
        log INFO "Enabled $SERVICE_NAME service to start on boot"
    fi
    
    # Start service if requested
    if [ "$START_SERVICE" = true ]; then
        systemctl start "$SERVICE_NAME" || {
            log ERROR "Failed to start $SERVICE_NAME service"
            systemctl status "$SERVICE_NAME"
            return 1
        }
        
        # Wait for service to stabilize
        sleep 2
        
        # Check service status
        if systemctl is-active "$SERVICE_NAME" &>/dev/null; then
            log INFO "Service $SERVICE_NAME is running"
        else
            log ERROR "Service $SERVICE_NAME failed to start"
            systemctl status "$SERVICE_NAME"
            return 1
        fi
    fi
    
    return 0
}

# Create environment file
create_environment_file() {
    log INFO "Creating environment file..."
    
    USER_HOME=$(eval echo ~${USER_NAME})
    ENV_FILE="${USER_HOME}/.anya-env"
    
    # Create the file
    cat > "$ENV_FILE" << EOF
# Anya Core Environment
# Generated by systemd_config.sh on $(date)
# Script version: ${VERSION}

export ANYA_HOME="${PROJECT_ROOT}"
export ANYA_CONFIG="${CONFIG_PATH}"
export RUST_LOG="info"
export PATH="\$PATH:$(dirname "$BINARY_PATH")"

# Add this to your shell config or run source ~/.anya-env
EOF
    
    # Set ownership
    chown "${USER_NAME}:$(id -gn "$USER_NAME")" "$ENV_FILE"
    chmod 644 "$ENV_FILE"
    
    # Add to user's profile if not already there
    BASHRC="${USER_HOME}/.bashrc"
    
    if [ -f "$BASHRC" ] && ! grep -q "source ~/.anya-env" "$BASHRC"; then
        # Ask before modifying bashrc
        if [ -t 0 ]; then  # Check if script is running in an interactive terminal
            echo
            read -p "Add environment to ${USER_NAME}'s .bashrc? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                echo "# Anya Core environment" >> "$BASHRC"
                echo "source ~/.anya-env" >> "$BASHRC"
                log INFO "Added environment to user's .bashrc"
            else
                log INFO "Skipped adding to .bashrc"
            fi
        else
            # Non-interactive mode, just add it
            echo "# Anya Core environment" >> "$BASHRC"
            echo "source ~/.anya-env" >> "$BASHRC"
            log INFO "Added environment to user's .bashrc (non-interactive mode)"
        fi
    elif [ -f "$BASHRC" ]; then
        log INFO "Environment already added to .bashrc"
    else
        log WARN "User's .bashrc not found. Environment file created but not automatically loaded."
    fi
    
    log INFO "Created environment file at $ENV_FILE"
}

# Validate binary file
validate_binary() {
    log INFO "Validating binary file: $BINARY_PATH"
    
    # Check file existence
    if [ ! -f "$BINARY_PATH" ]; then
        log ERROR "Binary file does not exist: $BINARY_PATH"
        return 1
    fi
    
    # Check if file is executable
    if [ ! -x "$BINARY_PATH" ]; then
        log WARN "Binary file is not executable. Setting executable permission..."
        chmod +x "$BINARY_PATH" || {
            log ERROR "Failed to set executable permission on binary"
            return 1
        }
    fi
    
    # Try to get version or help info
    if ! "$BINARY_PATH" --version &>/dev/null && ! "$BINARY_PATH" --help &>/dev/null; then
        log WARN "Binary does not respond to --version or --help flags. This may not be a valid executable."
    else
        log INFO "Binary validation successful"
    fi
    
    return 0
}

# Check for existing service
check_existing_service() {
    if systemctl list-unit-files | grep -q "$SERVICE_NAME"; then
        log WARN "Service $SERVICE_NAME already exists"
        
        if [ -t 0 ]; then  # Check if script is running in an interactive terminal
            echo
            read -p "Replace existing service? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                log INFO "Operation cancelled by user"
                exit 0
            fi
        fi
        
        # Stop and disable existing service
        if systemctl is-active "$SERVICE_NAME" &>/dev/null; then
            log INFO "Stopping existing service..."
            systemctl stop "$SERVICE_NAME" || log WARN "Failed to stop existing service"
        fi
        
        if systemctl is-enabled "$SERVICE_NAME" &>/dev/null; then
            log INFO "Disabling existing service..."
            systemctl disable "$SERVICE_NAME" || log WARN "Failed to disable existing service"
        fi
    fi
}

# Create directories needed by service
create_service_directories() {
    log INFO "Creating service directories..."
    
    # Create config directory if it doesn't exist
    CONFIG_DIR=$(dirname "$CONFIG_PATH")
    if [ ! -d "$CONFIG_DIR" ]; then
        mkdir -p "$CONFIG_DIR"
        log INFO "Created config directory: $CONFIG_DIR"
    fi
    
    # Create data directory
    DATA_DIR="${PROJECT_ROOT}/data"
    mkdir -p "$DATA_DIR"
    
    # Create logs directory
    mkdir -p "${PROJECT_ROOT}/logs"
    
    # Set appropriate permissions
    chown -R "${USER_NAME}:$(id -gn "$USER_NAME")" "$CONFIG_DIR" "$DATA_DIR" "${PROJECT_ROOT}/logs"
    chmod 750 "$CONFIG_DIR" "$DATA_DIR" "${PROJECT_ROOT}/logs"
    
    log INFO "Service directories created and secured"
}

# Show service management instructions
show_instructions() {
    echo
    echo "================================================================"
    echo "        Anya Core Service Configuration Complete"
    echo "================================================================"
    echo
    echo "Service: $SERVICE_NAME"
    echo "User: $USER_NAME"
    echo "Binary: $BINARY_PATH"
    echo "Config: $CONFIG_PATH"
    echo
    echo "Start the service:"
    echo "  sudo systemctl start $SERVICE_NAME"
    echo
    echo "Stop the service:"
    echo "  sudo systemctl stop $SERVICE_NAME"
    echo
    echo "Restart the service:"
    echo "  sudo systemctl restart $SERVICE_NAME"
    echo
    echo "Check service status:"
    echo "  sudo systemctl status $SERVICE_NAME"
    echo
    echo "View logs:"
    echo "  sudo journalctl -u $SERVICE_NAME -f"
    echo
    echo "Environment file created at:"
    echo "  ${USER_HOME}/.anya-env"
    echo
    echo "Configuration log:"
    echo "  $INSTALL_LOG"
    echo
    echo "================================================================"
}

# Main function
main() {
    log INFO "Starting systemd configuration for Anya Core (version $VERSION)..."
    
    # Check for root privileges
    check_root
    
    # Parse command line arguments
    parse_args "$@"
    
    # Check for existing service
    check_existing_service
    
    # Validate binary
    validate_binary
    
    # Create necessary directories
    create_service_directories
    
    # Configure systemd service
    configure_systemd
    
    # Create environment file
    create_environment_file
    
    # Show instructions
    show_instructions
    
    log INFO "Systemd configuration completed successfully"
}

# Run the script
main "$@" 