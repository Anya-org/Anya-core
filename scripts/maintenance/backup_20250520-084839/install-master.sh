#!/bin/bash
# Anya Core Comprehensive Installation Script
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"
LOGS_DIR="${PROJECT_ROOT}/logs"
mkdir -p "$LOGS_DIR"

# Log file
INSTALL_LOG="${LOGS_DIR}/install_$(date +%Y%m%d-%H%M%S).log"

# Default values
NETWORK="testnet"
START_SERVICE=true
INSTALL_DEPS=true
CONFIGURE_FIREWALL=true
INSTALL_TYPE="standard"  # standard, minimal, full
HARDENING_LEVEL="standard"  # basic, standard, strict
AUTO_RUN=false
INTERACTIVE=true
FORCE=false
SERVICE_NAME="anya-core"
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

# Check for root privileges
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log ERROR "This script requires root privileges. Please run with sudo."
        exit 1
    fi
}

# Show help
show_help() {
    echo "Anya Core Comprehensive Installation Script v${VERSION}"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --network=NETWORK       Specify network (mainnet, testnet, regtest)"
    echo "  --no-start              Don't start the service after installation"
    echo "  --no-deps               Skip dependency installation"
    echo "  --no-firewall           Skip firewall configuration"
    echo "  --type=TYPE             Installation type (minimal, standard, full)"
    echo "  --hardening=LEVEL       Security hardening level (basic, standard, strict)"
    echo "  --auto-run              Automatically run with no prompts (non-interactive)"
    echo "  --skip-rust             Skip Rust installation (use existing)"
    echo "  --help                  Display this help message"
    echo "  --version               Display script version"
    echo ""
    echo "Example:"
    echo "  sudo $0 --network=testnet --type=standard --auto-run"
}

# Show version
show_version() {
    echo "Anya Core Comprehensive Installation Script v${VERSION}"
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
            --network=*)
                NETWORK="${1#*=}"
                if [[ ! "$NETWORK" =~ ^(mainnet|testnet|regtest)$ ]]; then
                    log ERROR "Invalid network: $NETWORK. Must be mainnet, testnet, or regtest."
                    exit 1
                fi
                shift
                ;;
            --no-start)
                START_SERVICE=false
                shift
                ;;
            --no-deps)
                INSTALL_DEPS=false
                shift
                ;;
            --no-firewall)
                CONFIGURE_FIREWALL=false
                shift
                ;;
            --type=*)
                INSTALL_TYPE="${1#*=}"
                if [[ ! "$INSTALL_TYPE" =~ ^(minimal|standard|full)$ ]]; then
                    log ERROR "Invalid installation type: $INSTALL_TYPE. Must be minimal, standard, or full."
                    exit 1
                fi
                shift
                ;;
            --hardening=*)
                HARDENING_LEVEL="${1#*=}"
                if [[ ! "$HARDENING_LEVEL" =~ ^(basic|standard|strict)$ ]]; then
                    log ERROR "Invalid hardening level: $HARDENING_LEVEL. Must be basic, standard, or strict."
                    exit 1
                fi
                shift
                ;;
            --auto-run)
                AUTO_RUN=true
                INTERACTIVE=false
                shift
                ;;
            --skip-rust)
                SKIP_RUST=true
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
    
    log INFO "Installation configured with:"
    log INFO "- Network: $NETWORK"
    log INFO "- Start Service: $START_SERVICE"
    log INFO "- Install Dependencies: $INSTALL_DEPS"
    log INFO "- Configure Firewall: $CONFIGURE_FIREWALL"
    log INFO "- Installation Type: $INSTALL_TYPE"
    log INFO "- Hardening Level: $HARDENING_LEVEL"
    log INFO "- Auto Run: $AUTO_RUN"
}

# Analyze system capabilities
analyze_system() {
    log INFO "Analyzing system capabilities..."
    
    # CPU analysis
    CPU_CORES=$(nproc)
    CPU_MODEL=$(grep "model name" /proc/cpuinfo | head -1 | cut -d':' -f2 | sed 's/^[ \t]*//' || echo "Unknown")
    CPU_ARCH=$(uname -m)
    
    # Memory analysis
    TOTAL_MEM=$(free -m | awk '/^Mem:/{print $2}')
    AVAIL_MEM=$(free -m | awk '/^Mem:/{print $7}')
    MEM_PERCENTAGE=$((AVAIL_MEM * 100 / TOTAL_MEM))
    
    # Disk analysis
    ROOT_DISK_AVAIL=$(df -h "${PROJECT_ROOT}" | awk 'NR==2 {print $4}')
    ROOT_DISK_AVAIL_BYTES=$(df -B1 "${PROJECT_ROOT}" | awk 'NR==2 {print $4}')
    
    # Network analysis
    INTERNET_SPEED=$(which speedtest-cli > /dev/null && speedtest-cli --simple 2>/dev/null | grep Download | awk '{print $2}' || echo "Unknown")
    
    # HSM detection
    HAS_TPM=$(test -e /dev/tpm0 && echo "true" || echo "false")
    HAS_YUBIKEY=$(lsusb 2>/dev/null | grep -i "yubico" > /dev/null && echo "true" || echo "false")
    
    # Environment detection
    IS_CONTAINER=$(grep -q "container=" /proc/1/environ 2>/dev/null && echo "true" || echo "false")
    IS_VIRTUAL=$(dmesg 2>/dev/null | grep -i "hypervisor" > /dev/null && echo "true" || echo "false")
    
    # Log findings
    log INFO "System capabilities:"
    log INFO "- CPU: $CPU_CORES cores, $CPU_MODEL ($CPU_ARCH)"
    log INFO "- Memory: ${AVAIL_MEM}MB available out of ${TOTAL_MEM}MB (${MEM_PERCENTAGE}%)"
    log INFO "- Disk: ${ROOT_DISK_AVAIL} available for installation"
    log INFO "- Internet: ${INTERNET_SPEED} Mbps download (if available)"
    log INFO "- HSM: TPM=${HAS_TPM}, YubiKey=${HAS_YUBIKEY}"
    log INFO "- Environment: Container=${IS_CONTAINER}, VM=${IS_VIRTUAL}"
    
    # Check for minimum requirements
    if [ "$TOTAL_MEM" -lt 2048 ]; then
        log WARN "Less than 2GB RAM available. Installation may be slow."
        if [ "$AUTO_RUN" = false ]; then
            read -p "Continue with low memory? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                log ERROR "Installation cancelled due to low memory"
                exit 1
            fi
        fi
    fi
    
    if [ "$ROOT_DISK_AVAIL_BYTES" -lt 5368709120 ]; then # 5GB
        log WARN "Less than 5GB disk space available. Installation may fail."
        if [ "$AUTO_RUN" = false ]; then
            read -p "Continue with low disk space? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                log ERROR "Installation cancelled due to low disk space"
                exit 1
            fi
        fi
    fi
    
    # Export system variables
    export CPU_CORES
    export TOTAL_MEM
    export AVAIL_MEM
    export HAS_TPM
    export HAS_YUBIKEY
    export IS_CONTAINER
    export IS_VIRTUAL
}

# Configure installation based on type and system analysis
configure_installation_type() {
    log INFO "Configuring installation type: $INSTALL_TYPE"
    
    case "$INSTALL_TYPE" in
        minimal)
            # Minimal installation: basic components only
            START_SERVICE=false
            INSTALL_DEPS=true
            CONFIGURE_FIREWALL=false
            HARDENING_LEVEL="basic"
            # Low memory configuration
            export RUST_FLAGS="--cfg minimal"
            export MEMORY_LIMIT=$((TOTAL_MEM / 2))
            log INFO "Configuring for minimal installation with ${MEMORY_LIMIT}MB memory limit"
            ;;
        standard)
            # Standard installation: default settings
            START_SERVICE=true
            INSTALL_DEPS=true
            CONFIGURE_FIREWALL=true
            HARDENING_LEVEL="standard"
            
            # Memory-based optimization
            if [ "$TOTAL_MEM" -lt 4096 ]; then
                export RUST_FLAGS="--cfg memory_optimized"
                export MEMORY_LIMIT=$((TOTAL_MEM * 2 / 3))
                log INFO "Configuring for memory-optimized build with ${MEMORY_LIMIT}MB limit"
            else
                export MEMORY_LIMIT=$((TOTAL_MEM * 3 / 4))
                log INFO "Configuring with standard memory settings: ${MEMORY_LIMIT}MB limit"
            fi
            ;;
        full)
            # Full installation: all components with highest security
            START_SERVICE=true
            INSTALL_DEPS=true
            CONFIGURE_FIREWALL=true
            
            # Set hardening based on detected hardware security
            if [ "$HAS_TPM" = "true" ] || [ "$HAS_YUBIKEY" = "true" ]; then
                HARDENING_LEVEL="strict"
                export HSM_TYPE="hardware"
                log INFO "Hardware security module detected, using strict security profile"
            else
                HARDENING_LEVEL="standard"
                export HSM_TYPE="software"
                log INFO "No hardware security module detected, using software HSM"
            fi
            
            # CPU-based optimization
            if [ "$CPU_CORES" -gt 4 ]; then
                export PARALLEL_JOBS=$((CPU_CORES - 1))
                log INFO "Setting build to use $PARALLEL_JOBS parallel jobs"
            fi
            
            # Memory configuration for full installation
            export MEMORY_LIMIT=$((TOTAL_MEM * 3 / 4))
            log INFO "Configuring with full memory settings: ${MEMORY_LIMIT}MB limit"
            ;;
    esac
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
    if [ "$INSTALL_DEPS" = false ]; then
        log INFO "Skipping dependency installation as requested"
        return 0
    fi

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
    if [ -n "${PARALLEL_JOBS:-}" ]; then
        BUILD_CMD="$BUILD_CMD -j $PARALLEL_JOBS"
        log INFO "Using $PARALLEL_JOBS parallel jobs for build"
    fi
    
    # Add any Rust flags from system analysis
    if [ -n "${RUST_FLAGS:-}" ]; then
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
    chown -R "${SERVICE_USER:-$USER}:$(id -gn "${SERVICE_USER:-$USER}")" "$CONFIG_DIR"
    chmod 700 "$CONFIG_DIR"
    chmod 600 "${CONFIG_DIR}/anya.conf"
    
    log INFO "System-optimized configuration file created at ${CONFIG_DIR}/anya.conf"
}

# Set up systemd service
setup_systemd() {
    log INFO "Setting up systemd service..."
    
    # Create systemd service file
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
User=${SERVICE_USER:-$USER}
Group=$(id -gn "${SERVICE_USER:-$USER}")
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
Environment="ANYA_CONFIG=${PROJECT_ROOT}/config/anya.conf"
# System-specific environment variables
Environment="ANYA_CPU_CORES=${CPU_CORES}"
Environment="ANYA_MEMORY_LIMIT=${MEM_LIMIT}"
Environment="ANYA_HSM_TYPE=${HSM_TYPE:-software}"

[Install]
WantedBy=multi-user.target
EOF
    
    log INFO "Created systemd service at $SERVICE_FILE with resource limits (CPU: ${CPU_PERCENT}%, Memory: ${MEM_LIMIT}MB)"
    chmod 644 "$SERVICE_FILE"
    
    # Reload systemd and enable service
    systemctl daemon-reload || {
        log ERROR "Failed to reload systemd configuration"
        exit 1
    }
    
    systemctl enable "$SERVICE_NAME.service" || {
        log ERROR "Failed to enable $SERVICE_NAME service"
        exit 1
    }
    
    # Start service if requested
    if [ "$START_SERVICE" = true ]; then
        log INFO "Starting $SERVICE_NAME service..."
        systemctl start "$SERVICE_NAME.service" || {
            log ERROR "Failed to start $SERVICE_NAME service"
            systemctl status "$SERVICE_NAME.service"
            exit 1
        }
        log INFO "Service started successfully"
    fi
    
    log INFO "Systemd service configured successfully"
}

# Configure firewall if needed
configure_firewall() {
    if [ "$CONFIGURE_FIREWALL" = false ]; then
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
    if systemctl is-enabled "$SERVICE_NAME.service" &> /dev/null; then
        log INFO "Systemd service is enabled"
    else
        log ERROR "Systemd service is not enabled"
        ((errors++))
    fi
    
    # Check service status if auto-started
    if [ "$START_SERVICE" = true ]; then
        if systemctl is-active "$SERVICE_NAME.service" &> /dev/null; then
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
    echo "        Anya Core Installation Complete"
    echo "================================================================"
    echo
    echo "System Configuration:"
    echo "- CPU: $CPU_CORES cores"
    echo "- Memory: ${MEMORY_LIMIT}MB allocated"
    echo "- HSM type: ${HSM_TYPE:-software}"
    echo "- Security level: $HARDENING_LEVEL"
    echo
    echo "Configuration file:"
    echo "  ${PROJECT_ROOT}/config/anya.conf"
    echo
    echo "Service management:"
    echo "  sudo systemctl start $SERVICE_NAME.service     # Start the service"
    echo "  sudo systemctl stop $SERVICE_NAME.service      # Stop the service"
    echo "  sudo systemctl restart $SERVICE_NAME.service   # Restart the service"
    echo "  sudo systemctl status $SERVICE_NAME.service    # Check service status"
    echo
    echo "Logs:"
    echo "  sudo journalctl -u $SERVICE_NAME.service -f    # Follow service logs"
    echo "  ${INSTALL_LOG}                                 # Installation log"
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
    
    # Analyze system
    analyze_system
    
    # Configure installation based on type and system analysis
    configure_installation_type
    
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
    
    # Setup systemd service
    setup_systemd
    
    # Configure firewall
    configure_firewall
    
    # Verify installation
    verify_installation
    
    # Show instructions
    show_instructions
    
    log INFO "Anya Core installation completed successfully!"
}

# Run the installer
main "$@" 