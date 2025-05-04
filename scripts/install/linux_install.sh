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
SKIP_RUST=false
CUSTOM_USER=""

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
    echo "  --user=USERNAME        Run service as this user (default: current user)"
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
    
    log INFO "Installation configured with: Network=$NETWORK, AutoStart=$AUTO_START, User=$SERVICE_USER"
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

# Main installation function
main() {
    log INFO "Starting Anya Core installation (version $VERSION)..."
    
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
    
    log INFO "Anya Core installation completed successfully!"
}

# Run the installer
main "$@" 