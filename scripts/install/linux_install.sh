#!/bin/bash
# Anya Core Linux Installation Script
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
LOGS_DIR="${PROJECT_ROOT}/logs"
mkdir -p "$LOGS_DIR"

# Log file
INSTALL_LOG="${LOGS_DIR}/install_$(date +%Y%m%d-%H%M%S).log"

# Script version
VERSION="1.0.0"

# Default values
INTERACTIVE=true
AUTO_START=false
NETWORK="testnet"
SKIP_FIREWALL=false
SKIP_DEPS=false
SKIP_RUST=false
CUSTOM_USER=""
FEATURE_FLAGS="std"  # Default minimal feature set
UPGRADE_MODE=false    # Whether we're upgrading an existing installation
CONFIG_BACKUP=""      # Path to a backup config file to restore after upgrade

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
        log ERROR "Installation failed with error code $error_code. Check log at $INSTALL_LOG"
    fi
    exit $error_code
}

# Set up trap for cleanup on script exit
trap cleanup EXIT INT TERM

# Show help
show_help() {
    echo "Anya Core Linux Installation Script v${VERSION}"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --non-interactive      Run without interactive prompts"
    echo "  --auto-start           Start the service after installation"
    echo "  --network=NETWORK      Specify network (mainnet, testnet, regtest)"
    echo "  --skip-firewall        Skip firewall configuration"
    echo "  --skip-rust            Skip Rust installation (use existing)"
    echo "  --skip-deps            Skip all dependencies installation"
    echo "  --user=USERNAME        Run service as this user (default: current user)"
    echo "  --features=FLAGS       Cargo feature flags (comma-separated, e.g. std,hsm,bitcoin_integration)"
    echo "  --upgrade              Upgrade an existing installation"
    echo "  --config-backup=PATH   Path to configuration backup to restore after upgrade"
    echo "  --help                 Display this help message"
    echo "  --version              Display script version"
    echo ""
    echo "Example:"
    echo "  sudo $0 --non-interactive --network=testnet --auto-start"
}

# Show version
show_version() {
    echo "Anya Core Linux Installation Script v${VERSION}"
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
            --non-interactive)
                INTERACTIVE=false
                shift
                ;;
            --auto-start)
                AUTO_START=true
                shift
                ;;
            --network=*)
                NETWORK="${1#*=}"
                if [[ ! "$NETWORK" =~ ^(mainnet|testnet|regtest)$ ]]; then
                    log ERROR "Invalid network: $NETWORK. Must be mainnet, testnet, or regtest."
                    exit 1
                fi
                shift
                ;;
            --skip-firewall)
                SKIP_FIREWALL=true
                shift
                ;;
            --skip-rust)
                SKIP_RUST=true
                shift
                ;;
            --skip-deps)
                SKIP_DEPS=true
                shift
                ;;
            --features=*)
                FEATURE_FLAGS="${1#*=}"
                shift
                ;;
            --upgrade)
                UPGRADE_MODE=true
                shift
                ;;
            --config-backup=*)
                CONFIG_BACKUP="${1#*=}"
                shift
                ;;
            --user=*)
                CUSTOM_USER="${1#*=}"
                # Validate user exists
                if ! id "$CUSTOM_USER" &>/dev/null; then
                    log ERROR "User $CUSTOM_USER does not exist"
                    exit 1
                fi
                shift
                ;;
            *)
                log ERROR "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    # Set user if custom user is provided
    if [[ -n "$CUSTOM_USER" ]]; then
        SERVICE_USER="$CUSTOM_USER"
    else
        SERVICE_USER="${SUDO_USER:-$USER}"
    fi
    
    log INFO "Installation configured with:"
    log INFO "- Network: $NETWORK"
    log INFO "- AutoStart: $AUTO_START"
    log INFO "- User: $SERVICE_USER"
    log INFO "- Feature Flags: $FEATURE_FLAGS"
    log INFO "- Upgrade Mode: $UPGRADE_MODE"
    if [ -n "$CONFIG_BACKUP" ]; then
        log INFO "- Config Backup: $CONFIG_BACKUP"
    fi
}

# Check for root privileges
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log ERROR "This script requires root privileges. Please run with sudo."
        exit 1
    fi
}

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
        VERSION_ID=$VERSION_ID
        log INFO "Detected Linux distribution: $DISTRO $VERSION_ID"
    else
        log ERROR "Could not detect Linux distribution"
        exit 1
    fi
}

# Install dependencies based on distribution
install_dependencies() {
    log INFO "Installing dependencies..."
    
    case $DISTRO in
        ubuntu|debian)
            apt-get update
            apt-get install -y \
                build-essential \
                pkg-config \
                libssl-dev \
                curl \
                git \
                cmake \
                clang \
                llvm \
                libsqlite3-dev \
                || { log ERROR "Failed to install dependencies"; exit 1; }
            ;;
        fedora)
            dnf install -y \
                gcc \
                gcc-c++ \
                make \
                pkgconfig \
                openssl-devel \
                curl \
                git \
                cmake \
                clang \
                llvm \
                sqlite-devel \
                || { log ERROR "Failed to install dependencies"; exit 1; }
            ;;
        centos|rhel)
            yum install -y \
                gcc \
                gcc-c++ \
                make \
                pkgconfig \
                openssl-devel \
                curl \
                git \
                cmake \
                clang \
                llvm \
                sqlite-devel \
                || { log ERROR "Failed to install dependencies"; exit 1; }
            ;;
        arch|manjaro)
            pacman -Syu --noconfirm \
                base-devel \
                openssl \
                curl \
                git \
                cmake \
                clang \
                llvm \
                sqlite \
                || { log ERROR "Failed to install dependencies"; exit 1; }
            ;;
        *)
            log ERROR "Unsupported distribution: $DISTRO"
            exit 1
            ;;
    esac
    
    log INFO "Dependencies installed successfully"
}

# Install Rust and Cargo
install_rust() {
    if [ "$SKIP_RUST" = true ]; then
        if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
            RUST_VERSION=$(rustc --version | awk '{print $2}')
            log INFO "Using existing Rust installation (version $RUST_VERSION)"
            return 0
        else
            log ERROR "Rust installation skipped but rustc/cargo not found in PATH"
            exit 1
        fi
    fi

    log INFO "Installing Rust..."
    
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
        RUST_VERSION=$(rustc --version | awk '{print $2}')
        log INFO "Rust is already installed (version $RUST_VERSION)"
        
        # Update Rust if requested
        if [ "$INTERACTIVE" = true ]; then
            read -p "Update Rust to latest stable? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                rustup update stable
                log INFO "Rust updated to $(rustc --version | awk '{print $2}')"
            fi
        fi
    else
        log INFO "Installing Rust from rustup.rs..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y || {
            log ERROR "Failed to install Rust"
            exit 1
        }
        source "$HOME/.cargo/env"
        log INFO "Rust installed successfully: $(rustc --version)"
    fi
    
    # Install additional Rust components
    log INFO "Installing additional Rust components..."
    rustup component add rustfmt clippy || log WARN "Failed to install some Rust components"
    
    # Install cross-compilation tools if needed
    if ! command -v cross &> /dev/null; then
        log INFO "Installing 'cross' tool for cross-compilation..."
        cargo install cross || log WARN "Failed to install 'cross' tool"
    fi
}

# Clone repository if needed
clone_repository() {
    if [ ! -d "${PROJECT_ROOT}/.git" ]; then
        log INFO "Cloning Anya Core repository..."
        PARENT_DIR=$(dirname "$PROJECT_ROOT")
        cd "$PARENT_DIR"
        git clone https://github.com/anya-org/anya-core.git || {
            log ERROR "Failed to clone repository"
            exit 1
        }
        cd "$PROJECT_ROOT"
    else
        log INFO "Repository already exists, updating to latest version..."
        cd "$PROJECT_ROOT"
        git fetch || log WARN "Failed to fetch latest changes"
        
        # Only pull if not in interactive mode or user confirms
        if [ "$INTERACTIVE" = false ]; then
            git pull || log WARN "Failed to pull latest changes"
        else
            read -p "Pull latest changes? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                git pull || log WARN "Failed to pull latest changes"
            fi
        fi
    fi
    
    # Get current version
    GIT_VERSION=$(git describe --tags --always 2>/dev/null || echo "unknown")
    log INFO "Using Anya Core version: $GIT_VERSION"
}

# Build the project
build_project() {
    log INFO "Building Anya Core..."
    cd "$PROJECT_ROOT"
    
    # Create necessary directories
    mkdir -p "${PROJECT_ROOT}/config"
    
    # Check for Cargo.toml
    if [ ! -f "${PROJECT_ROOT}/Cargo.toml" ]; then
        log ERROR "Cargo.toml not found. Are you in the correct directory?"
        exit 1
    fi
    
    # Build the project with system-specific optimization
    log INFO "Running cargo build --release..."
    
    # Prepare build command
    BUILD_CMD="RUST_BACKTRACE=1 cargo build --release"
    
    # Add feature flags
    if [ -n "$FEATURE_FLAGS" ]; then
        # Clean up feature flags (remove spaces, handle std vs default)
        CLEAN_FLAGS=$(echo "$FEATURE_FLAGS" | tr -d ' ')
        if [[ "$CLEAN_FLAGS" == "std" ]]; then
            # For just std, use default features
            BUILD_CMD="$BUILD_CMD"
            log INFO "Using default features for build"
        else
            # For custom features, add them explicitly
            BUILD_CMD="$BUILD_CMD --no-default-features --features=$CLEAN_FLAGS"
            log INFO "Using custom features for build: $CLEAN_FLAGS"
        fi
    fi
    
    # Add parallel jobs if set from system analysis
    if [ -n "$PARALLEL_JOBS" ]; then
        BUILD_CMD="$BUILD_CMD -j $PARALLEL_JOBS"
        log INFO "Using $PARALLEL_JOBS parallel jobs for build"
    fi
    
    # Add any Rust flags from system analysis
    if [ -n "$RUST_FLAGS" ]; then
        BUILD_CMD="$BUILD_CMD $RUST_FLAGS"
        log INFO "Using additional Rust flags: $RUST_FLAGS"
    fi
    
    # Log and execute the build command
    log INFO "Executing: $BUILD_CMD"
    eval $BUILD_CMD || {
        log ERROR "Build failed. Check the logs for details."
        exit 1
    }
    
    # Verify binary exists
    if [ ! -f "${PROJECT_ROOT}/target/release/anya-core" ]; then
        log ERROR "Binary not found after build. Build may have failed."
        exit 1
    fi
    
    # Get binary version
    BINARY_VERSION=$(${PROJECT_ROOT}/target/release/anya-core --version 2>/dev/null || echo "Version information unavailable")
    log INFO "Build completed successfully: $BINARY_VERSION"
}

# Set up systemd service
setup_systemd() {
    log INFO "Setting up systemd service..."
    
    # Create systemd service file
    cat > /etc/systemd/system/anya-core.service << EOF
[Unit]
Description=Anya Core Bitcoin Service
Documentation=https://docs.anya-core.org
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=$SERVICE_USER
Group=$(id -gn "$SERVICE_USER")
WorkingDirectory=${PROJECT_ROOT}
ExecStart=${PROJECT_ROOT}/target/release/anya-core
ExecStop=/bin/kill -s SIGTERM \$MAINPID
Restart=on-failure
RestartSec=5s
TimeoutStartSec=30s
TimeoutStopSec=30s
StandardOutput=journal
StandardError=journal
SyslogIdentifier=anya-core

# Security hardening
ProtectSystem=full
PrivateTmp=true
NoNewPrivileges=true
PrivateDevices=true
MemoryDenyWriteExecute=true

# Environment variables
Environment="RUST_LOG=info"
Environment="ANYA_HOME=${PROJECT_ROOT}"
Environment="ANYA_CONFIG=${PROJECT_ROOT}/config/anya.conf"

[Install]
WantedBy=multi-user.target
EOF
    
    # Reload systemd and enable service
    systemctl daemon-reload || {
        log ERROR "Failed to reload systemd configuration"
        exit 1
    }
    
    systemctl enable anya-core.service || {
        log ERROR "Failed to enable anya-core service"
        exit 1
    }
    
    # Start service if requested
    if [ "$AUTO_START" = true ]; then
        log INFO "Starting anya-core service..."
        systemctl start anya-core.service || {
            log ERROR "Failed to start anya-core service"
            systemctl status anya-core.service
            exit 1
        }
        log INFO "Service started successfully"
    fi
    
    log INFO "Systemd service configured successfully"
}

# Configure firewall if needed
configure_firewall() {
    if [ "$SKIP_FIREWALL" = true ]; then
        log INFO "Skipping firewall configuration as requested"
        return 0
    fi

    log INFO "Configuring firewall..."
    
    if command -v ufw &> /dev/null; then
        # Allow Bitcoin ports
        ufw allow 8333/tcp  # Bitcoin mainnet
        ufw allow 18333/tcp # Bitcoin testnet
        ufw allow 3000/tcp  # Anya API
        log INFO "Firewall configured with UFW"
    elif command -v firewall-cmd &> /dev/null; then
        # For firewalld (RHEL/CentOS/Fedora)
        firewall-cmd --permanent --add-port=8333/tcp
        firewall-cmd --permanent --add-port=18333/tcp
        firewall-cmd --permanent --add-port=3000/tcp
        firewall-cmd --reload
        log INFO "Firewall configured with firewalld"
    else
        log WARN "No supported firewall detected. Please configure manually."
    fi
}

# Create configuration file
create_config() {
    log INFO "Creating configuration file..."
    
    CONFIG_DIR="${PROJECT_ROOT}/config"
    mkdir -p "$CONFIG_DIR"
    
    # Select network port based on network type
    case "$NETWORK" in
        mainnet) RPC_PORT=8332 ;;
        testnet) RPC_PORT=18332 ;;
        regtest) RPC_PORT=18443 ;;
    esac
    
    # Generate a secure password
    SECURE_PASSWORD=$(openssl rand -hex 16)
    
    # Determine HSM provider type based on hardware detection
    HSM_PROVIDER_TYPE="${HSM_TYPE:-software}"
    if [ "$HSM_PROVIDER_TYPE" = "hardware" ]; then
        HSM_CONFIG="provider_type = \"hardware\"  # Hardware HSM detected
hardware.token_dir = \"${PROJECT_ROOT}/.tokens\"
hardware.max_sessions = 5
hardware.tpm_enabled = true"
    else
        HSM_CONFIG="provider_type = \"software\"  # Software HSM fallback
software.token_dir = \"${PROJECT_ROOT}/.tokens\"
software.max_sessions = 10"
    fi
    
    # Set memory limit from system analysis or default
    MEMORY_LIMIT="${MEMORY_LIMIT:-1024}"
    
    # Set CPU count from system analysis or default
    CPU_COUNT="${CPU_CORES:-4}"
    
    # Create default configuration file with system-optimized values
    cat > "${CONFIG_DIR}/anya.conf" << EOF
# Anya Core Configuration
# Generated on $(date)
# Installation script version: ${VERSION}
# System-optimized configuration

[general]
log_level = "info"
network = "${NETWORK}"  # Options: mainnet, testnet, regtest
cpu_cores = ${CPU_COUNT}
memory_limit = "${MEMORY_LIMIT}"

[bitcoin]
rpc_user = "anya"
rpc_password = "${SECURE_PASSWORD}"
rpc_port = ${RPC_PORT}
rpc_host = "127.0.0.1"

[hsm]
${HSM_CONFIG}
audit_enabled = true
# Secure HSM configuration
software.token_dir = "${PROJECT_ROOT}/.tokens"
software.max_sessions = 10

[api]
enabled = true
host = "0.0.0.0"
port = 3000
rate_limit = 100

[resources]
# System-optimized resource settings
max_memory_mb = ${MEMORY_LIMIT}
max_cpu_percent = 75
thread_pool_size = ${CPU_COUNT}
concurrent_operations = $((CPU_COUNT * 4))
EOF
    
    # Set appropriate permissions
    chown -R "$SERVICE_USER:$(id -gn "$SERVICE_USER")" "$CONFIG_DIR"
    chmod 700 "$CONFIG_DIR"
    chmod 600 "${CONFIG_DIR}/anya.conf"
    
    log INFO "System-optimized configuration file created at ${CONFIG_DIR}/anya.conf"
}

# Create default configuration if not upgrading or creating a new one
create_default_config()
{
    log INFO "Creating default configuration..."
    mkdir -p "${PROJECT_ROOT}/config"
    cat > "${PROJECT_ROOT}/config/anya.conf" << EOF
# Anya Core Configuration File
# Generated on $(date)

[network]
network=$NETWORK

[security]
enable_hsm=true
hsm_provider=auto

[api]
listening_port=3300
bind_address=127.0.0.1
max_connections=100

[bitcoin]
rpc_url=http://localhost:8332
rpc_user=bitcoinrpc
rpc_password=password
testnet=$([ "$NETWORK" == "testnet" ] && echo "true" || echo "false")

[storage]
data_dir=${PROJECT_ROOT}/data
log_dir=${PROJECT_ROOT}/logs

[dao]
governance_enabled=true

[logging]
level=info
enabled=true

[system]
auto_save=true
auto_save_interval=300
metrics_enabled=true
performance_mode=balanced
feature_flags=$FEATURE_FLAGS
EOF
}

# This section was misplaced - removed standalone call to create_default_config

# Set up environment
setup_environment() {
    log INFO "Setting up environment..."
    
    # Get user home
    if [[ "$SERVICE_USER" == "$SUDO_USER" || "$SERVICE_USER" == "$USER" ]]; then
        USER_HOME=$(eval echo ~${SERVICE_USER})
    else
        USER_HOME=$(eval echo ~${SERVICE_USER})
    fi
    
    # Create environment file for user
    ENV_FILE="${USER_HOME}/.anya-env"
    
    cat > "$ENV_FILE" << EOF
# Anya Core Environment
# Generated by installation script on $(date)
# Installation script version: ${VERSION}

export ANYA_HOME="${PROJECT_ROOT}"
export ANYA_CONFIG="${PROJECT_ROOT}/config/anya.conf"
export ANYA_NETWORK="${NETWORK}"
export RUST_LOG="info"
export PATH="\$PATH:${PROJECT_ROOT}/target/release"
EOF
    
    # Add to user's profile
    PROFILE_FILE="${USER_HOME}/.bashrc"
    
    if [ -f "$PROFILE_FILE" ] && ! grep -q "source ~/.anya-env" "$PROFILE_FILE"; then
        echo "# Anya Core environment" >> "$PROFILE_FILE"
        echo "source ~/.anya-env" >> "$PROFILE_FILE"
    fi
    
    # Apply ownership
    chown "$SERVICE_USER:$(id -gn "$SERVICE_USER")" "$ENV_FILE"
    chmod 644 "$ENV_FILE"
    
    log INFO "Environment setup completed"
}

# Create data directories with proper permissions
setup_data_directories() {
    log INFO "Setting up data directories..."
    
    # Create data directories
    DATA_DIR="${PROJECT_ROOT}/data"
    mkdir -p "${DATA_DIR}"
    mkdir -p "${DATA_DIR}/bitcoin"
    mkdir -p "${DATA_DIR}/hsm"
    
    # Set permissions
    chown -R "$SERVICE_USER:$(id -gn "$SERVICE_USER")" "${DATA_DIR}"
    chmod 700 "${DATA_DIR}"
    chmod 700 "${DATA_DIR}/hsm"
    
    log INFO "Data directories created and secured"
}

# Verify installation
verify_installation() {
    log INFO "Verifying installation..."
    local errors=0
    
    # Check binary
    if [ -f "${PROJECT_ROOT}/target/release/anya-core" ]; then
        log INFO "Anya Core binary exists"
        
        # Check if it runs
        if "${PROJECT_ROOT}/target/release/anya-core" --version &> /dev/null; then
            log INFO "Anya Core binary is executable"
        else
            log WARN "Anya Core binary exists but may not be executable"
            ((errors++))
        fi
    else
        log ERROR "Anya Core binary is missing"
        ((errors++))
    fi
    
    # Check configuration
    if [ -f "${PROJECT_ROOT}/config/anya.conf" ]; then
        log INFO "Configuration file exists"
    else
        log ERROR "Configuration file is missing"
        ((errors++))
    fi
    
    # Check systemd service
    if systemctl is-enabled anya-core.service &> /dev/null; then
        log INFO "Systemd service is enabled"
    else
        log ERROR "Systemd service is not enabled"
        ((errors++))
    fi
    
    # Check service status if auto-started
    if [ "$AUTO_START" = true ]; then
        if systemctl is-active anya-core.service &> /dev/null; then
            log INFO "Systemd service is running"
        else
            log ERROR "Systemd service failed to start"
            ((errors++))
        fi
    fi
    
    if [ $errors -eq 0 ]; then
        log INFO "Installation verified successfully"
    else
        log WARN "Installation verification completed with $errors issues"
    fi
}

# Display post-installation instructions
show_instructions() {
    echo 
    echo "================================================================"
    echo "            Anya Core Installation Complete"
    echo "================================================================"
    echo
    echo "Configuration file:"
    echo "  ${PROJECT_ROOT}/config/anya.conf"
    echo
    echo "Service management:"
    echo "  sudo systemctl start anya-core.service     # Start the service"
    echo "  sudo systemctl stop anya-core.service      # Stop the service"
    echo "  sudo systemctl restart anya-core.service   # Restart the service"
    echo "  sudo systemctl status anya-core.service    # Check service status"
    echo
    echo "Logs:"
    echo "  sudo journalctl -u anya-core.service -f    # Follow service logs"
    echo "  ${INSTALL_LOG}                             # Installation log"
    echo
    echo "Environment:"
    echo "  source ~/.anya-env                         # Load environment variables"
    echo
    echo "For more information, visit: https://docs.anya-core.org"
    echo "================================================================"
}

# Save version information for upgrade tracking
save_version_info() {
    log INFO "Saving version information for future upgrades..."
    
    # Create version info directory if it doesn't exist
    mkdir -p "${PROJECT_ROOT}/var/versions"
    
    # Get git information if available
    local GIT_COMMIT="unknown"
    local GIT_BRANCH="unknown"
    if command -v git >/dev/null && [ -d "${PROJECT_ROOT}/.git" ]; then
        GIT_COMMIT=$(git -C "${PROJECT_ROOT}" rev-parse HEAD 2>/dev/null || echo "unknown")
        GIT_BRANCH=$(git -C "${PROJECT_ROOT}" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
    fi
    
    # Get binary version if available
    local BINARY_VERSION="unknown"
    if [ -x "${PROJECT_ROOT}/target/release/anya-core" ]; then
        BINARY_VERSION=$("${PROJECT_ROOT}/target/release/anya-core" --version 2>/dev/null | head -1 || echo "unknown")
    fi
    
    # Create version file
    local VERSION_FILE="${PROJECT_ROOT}/var/versions/version_$(date +%Y%m%d-%H%M%S).json"
    cat > "$VERSION_FILE" << EOF
{
    "installation_date": "$(date -Iseconds)",
    "git_commit": "$GIT_COMMIT",
    "git_branch": "$GIT_BRANCH",
    "binary_version": "$BINARY_VERSION",
    "feature_flags": "$FEATURE_FLAGS",
    "network": "$NETWORK"
}
EOF
    
    log INFO "Version information saved to $VERSION_FILE"
    
    # Create a symlink to the latest version file
    ln -sf "$VERSION_FILE" "${PROJECT_ROOT}/var/versions/latest.json"
}

# Perform pre-upgrade backup and preparation
prepare_for_upgrade() {
    log INFO "Preparing for upgrade..."
    
    # Create backup directory
    local BACKUP_DIR="${PROJECT_ROOT}/var/backups/$(date +%Y%m%d-%H%M%S)"
    mkdir -p "$BACKUP_DIR"
    
    # Backup existing binary if it exists
    if [ -f "${PROJECT_ROOT}/target/release/anya-core" ]; then
        log INFO "Backing up existing binary to ${BACKUP_DIR}/anya-core.backup"
        cp "${PROJECT_ROOT}/target/release/anya-core" "${BACKUP_DIR}/anya-core.backup"
    fi
    
    # Backup configuration if it exists and no backup was specified
    if [ -f "${PROJECT_ROOT}/config/anya.conf" ] && [ -z "$CONFIG_BACKUP" ]; then
        log INFO "Backing up existing configuration to ${BACKUP_DIR}/anya.conf.backup"
        cp "${PROJECT_ROOT}/config/anya.conf" "${BACKUP_DIR}/anya.conf.backup"
        CONFIG_BACKUP="${BACKUP_DIR}/anya.conf.backup"
    fi
    
    # Backup database files if they exist
    if [ -d "${PROJECT_ROOT}/data/db" ]; then
        log INFO "Backing up database files to ${BACKUP_DIR}/db.backup"
        cp -r "${PROJECT_ROOT}/data/db" "${BACKUP_DIR}/db.backup"
    fi
    
    # Backup wallet files if they exist
    if [ -d "${PROJECT_ROOT}/data/wallet" ]; then
        log INFO "Backing up wallet files to ${BACKUP_DIR}/wallet.backup"
        cp -r "${PROJECT_ROOT}/data/wallet" "${BACKUP_DIR}/wallet.backup"
    fi
    
    # Save metadata about the backup
    cat > "${BACKUP_DIR}/backup_info.json" << EOF
{
    "backup_date": "$(date -Iseconds)",
    "pre_upgrade": true,
    "network": "$NETWORK",
    "files": [
        $([ -f "${BACKUP_DIR}/anya-core.backup" ] && echo '"anya-core.backup",'),
        $([ -f "${BACKUP_DIR}/anya.conf.backup" ] && echo '"anya.conf.backup",'),
        $([ -d "${BACKUP_DIR}/db.backup" ] && echo '"db.backup",'),
        $([ -d "${BACKUP_DIR}/wallet.backup" ] && echo '"wallet.backup"')
    ]
}
EOF
    
    log INFO "Pre-upgrade backup completed to $BACKUP_DIR"
    export UPGRADE_BACKUP_DIR="$BACKUP_DIR"
    
    # Stop the service if it's running
    if systemctl is-active anya-core.service >/dev/null 2>&1; then
        log INFO "Stopping anya-core service for upgrade"
        systemctl stop anya-core.service
        sleep 2
    fi
}

# Restore configuration after upgrade
restore_configuration() {
    log INFO "Restoring configuration after upgrade..."
    
    if [ -n "$CONFIG_BACKUP" ] && [ -f "$CONFIG_BACKUP" ]; then
        # Create config directory if it doesn't exist
        mkdir -p "${PROJECT_ROOT}/config"
        
        log INFO "Restoring configuration from $CONFIG_BACKUP"
        cp "$CONFIG_BACKUP" "${PROJECT_ROOT}/config/anya.conf"
        
        # Update network setting in config if needed
        if grep -q "^network=" "${PROJECT_ROOT}/config/anya.conf"; then
            log INFO "Updating network setting in restored configuration to $NETWORK"
            sed -i "s/^network=.*/network=$NETWORK/" "${PROJECT_ROOT}/config/anya.conf"
        else
            log INFO "Adding network setting to restored configuration"
            echo "network=$NETWORK" >> "${PROJECT_ROOT}/config/anya.conf"
        fi
    else
        log WARN "No configuration backup found to restore"
        # Create a new config file
        create_default_config
    fi
}

# Main function to install Anya Core
install_anya_core() {
    if [ "$UPGRADE_MODE" = true ]; then
        log INFO "Starting Anya Core upgrade..."
        prepare_for_upgrade
    else
        log INFO "Starting fresh Anya Core installation..."
    fi
    
    # Parse command line arguments
    parse_args "$@"
    
    # Check for root privileges
    check_root
    
    # Detect Linux distribution
    detect_distro
    
    # Install dependencies
    install_dependencies
    
    # Install Rust
    install_rust
    
    # Clone repository
    clone_repository
    
    # Build the project
    build_project
    
    # Create configuration
    create_config
    
    # Setup data directories
    setup_data_directories
    
    # Setup systemd service
    setup_systemd
    
    # Configure firewall
    configure_firewall
    
    # Setup environment
    setup_environment
    
    # Verify installation
    verify_installation
    
    # Show instructions
    show_instructions
    
    # Restore configuration for upgrades
    if [ "$UPGRADE_MODE" = true ]; then
        restore_configuration
    fi
    
    # Save version information for future reference
    save_version_info
    
    if [ "$UPGRADE_MODE" = true ]; then
        log INFO "Anya Core upgrade completed successfully"
    else
        log INFO "Anya Core installation completed successfully"
    fi
}

# Run the installer
install_anya_core "$@"