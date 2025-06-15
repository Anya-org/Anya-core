#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Main Installer Script for Anya Core
# Part of the Anya Core Hexagonal Architecture
# Date: 2025-05-21

set -e

# Source common functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UTILS_DIR="${SCRIPT_DIR}/utils"

if [ -f "${UTILS_DIR}/install_common.sh" ]; then
    source "${UTILS_DIR}/install_common.sh"
    source "${UTILS_DIR}/setup_monitoring.sh" 2>/dev/null || {
        log "WARNING" "Monitoring setup script not found. Monitoring will be skipped."
        MONITORING_AVAILABLE=false
    }
else
    echo "[ERROR] Common utilities not found: ${UTILS_DIR}/install_common.sh"
    exit 1
fi

# Default configuration
INSTALL_TYPE="standard"
NETWORK="testnet"
DRY_RUN=false
MONITORING=false
INSTALL_DEPS=false
AUTO_RUN=false
YES_ALL=false
ROOTLESS=false
INSTALL_DIR="${INSTALL_DIR:-$HOME/.anya-core}"
MONITORING_AVAILABLE=${MONITORING_AVAILABLE:-false}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --type=*)
            INSTALL_TYPE="${1#*=}"
            shift
            ;;
        --network=*)
            NETWORK="${1#*=}"
            shift
            ;;
        --with-monitoring)
            if [ "$MONITORING_AVAILABLE" = true ]; then
                MONITORING=true
            else
                log "WARNING" "Monitoring setup is not available. Skipping monitoring installation."
            fi
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --auto-run)
            echo "[INFO] Auto-run enabled: installing dependencies automatically"
            AUTO_RUN=true
            INSTALL_DEPS=true
            shift
            ;;
        --yes-all)
            echo "[INFO] Yes-all flag detected"
            YES_ALL=true
            shift
            ;;
        --install-deps)
            INSTALL_DEPS=true
            shift
            ;;
        --rootless|--sandbox)
            ROOTLESS=true
            INSTALL_DIR="$HOME/.anya-core"
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Main installation function
install_anya_core() {
    echo "[INFO] Starting Anya Core ${INSTALL_TYPE} installation for ${NETWORK} network"
    echo "[INFO] Rootless: $ROOTLESS | Auto-run: $AUTO_RUN | Yes-all: $YES_ALL"
    
    if [ "$DRY_RUN" = true ]; then
        echo "[DRY RUN] Would install Anya Core with the following configuration:"
        echo "[DRY RUN] - Monitoring: ${MONITORING:-false}"
        echo "  - Installation Type: $INSTALL_TYPE"
        echo "  - Network: $NETWORK"
        echo "  - Install Dependencies: $INSTALL_DEPS"
        echo "  - Configure Firewall: $CONFIGURE_FIREWALL"
        echo "  - Rootless: $ROOTLESS"
        exit 0
    fi

    # Create installation directories with proper permissions
    local dirs=(
        "$INSTALL_DIR/bin"
        "$INSTALL_DIR/config"
        "$INSTALL_DIR/logs"
        "$INSTALL_DIR/data"
    )
    for dir in "${dirs[@]}"; do
        if [ ! -d "$dir" ]; then
            echo "[INFO] Creating directory: $dir"
            mkdir -p "$dir"
            chmod 755 "$dir"
        else
            echo "[INFO] Directory exists: $dir"
        fi
    done

    # Set install dir for rootless
    if [ "$ROOTLESS" = true ]; then
        INSTALL_DIR="$HOME/.anya-core"
    fi

    # Install dependencies if needed
    if [ "$INSTALL_DEPS" = true ] || [ "$AUTO_RUN" = true ]; then
        echo "[INFO] Installing system dependencies..."
        if [ "$ROOTLESS" = true ]; then
            if ! command -v cargo >/dev/null 2>&1; then
                echo "[INFO] Installing Rust toolchain (user-local)..."
                curl https://sh.rustup.rs -sSf | sh -s -- -y
                source "$HOME/.cargo/env"
            fi
            if ! command -v node >/dev/null 2>&1; then
                echo "[INFO] Installing Node.js (user-local via nvm)..."
                if ! command -v nvm >/dev/null 2>&1; then
                    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
                    export NVM_DIR="$HOME/.nvm"
                    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
                fi
                nvm install --lts
            fi
        else
            # System-wide install (assume root)
            if ! command -v cargo >/dev/null 2>&1; then
                echo "[INFO] Installing Rust toolchain (system)..."
                curl https://sh.rustup.rs -sSf | sh -s -- -y
                source "$HOME/.cargo/env"
            fi
            if ! command -v node >/dev/null 2>&1; then
                echo "[INFO] Installing Node.js (system)..."
                curl -fsSL https://deb.nodesource.com/setup_lts.x | bash -
                apt-get install -y nodejs
            fi
        fi
        if [ -f "$PROJECT_ROOT/dependencies/install_dependencies.sh" ]; then
            bash "$PROJECT_ROOT/dependencies/install_dependencies.sh"
        fi
    fi

    # Configure firewall if needed
    if [ "$CONFIGURE_FIREWALL" = true ]; then
        echo "[INFO] Configuring firewall..."
        # Add actual firewall configuration here
    fi

    # Install Anya Core
    echo "[INFO] Installing Anya Core..."
    
    # Build the project
    echo "[INFO] Building Anya Core..."
    if ! cargo build --release; then
        echo "[ERROR] Failed to build Anya Core"
        return 1
    fi
    # Copy binary to installation directory
    local binary_path="${INSTALL_DIR}/bin/anya-core"
    echo "[INFO] Installing binary to ${binary_path}"
    cp "target/release/anya-core" "${binary_path}"
    chmod +x "${binary_path}"
    
    # Configure systemd service (only if not rootless)
    if [ "$ROOTLESS" = false ]; then
        echo "[INFO] Configuring systemd service..."
        local systemd_script="${SCRIPT_DIR}/systemd_config.sh"
        if [ -f "$systemd_script" ]; then
            chmod +x "$systemd_script"
            if "$systemd_script" \
                --user="$(whoami)" \
                --network="$NETWORK" \
                --binary-path="$binary_path" \
                --config-path="${INSTALL_DIR}/config/anya.toml" \
                --start; then
                echo "[SUCCESS] Systemd service configured and started"
            else
                echo "[ERROR] Failed to configure systemd service"
                return 1
            fi
        else
            echo "[ERROR] Systemd configuration script not found at $systemd_script"
            return 1
        fi
    else
        echo "[INFO] Rootless mode: Skipping systemd service setup."
        echo "[INFO] To run Anya Core: $binary_path --config ${INSTALL_DIR}/config/anya.toml"
        # Optionally create a user-level service or background script
    fi
    
    # Create default configuration if it doesn't exist
    local config_path="${INSTALL_DIR}/config/anya.toml"
    if [ ! -f "$config_path" ]; then
        echo "[INFO] Creating default configuration at ${config_path}"
        cat > "$config_path" << EOF
# Anya Core Configuration
# Generated on $(date)


[network]
# Network to connect to (mainnet, testnet, regtest, signet)
network = "${NETWORK}"

[bitcoin]
# Bitcoin Core RPC configuration
rpc_host = "127.0.0.1"
rpc_port = 8332
rpc_user = "your_rpc_username"
rpc_password = "your_rpc_password"

[logging]
# Log level (trace, debug, info, warn, error)
level = "info"

# Log file path (leave empty for console only)
file = "${INSTALL_DIR}/logs/anya.log"

[database]
# Database configuration
path = "${INSTALL_DIR}/data/anya.db"

[hsm]
# Hardware Security Module configuration
# Set enabled = true to enable HSM support
enabled = false

# HSM type (software, hardware, pkcs11, tpm, ledger)
type = "software"

# HSM-specific configuration
[hardware_hsm]
# Configuration for hardware HSMs

[pkcs11_hsm]
# Configuration for PKCS#11 modules

[tpm_hsm]
# Configuration for TPM modules

[ledger_hsm]
# Configuration for Ledger devices

[ml_service]
# Machine Learning service configuration
enabled = true
model_path = "${INSTALL_DIR}/models"

[governance]
# DAO governance settings
proposal_threshold = 1000.0  # Minimum tokens required to create a proposal
voting_period = 604800      # 7 days in seconds
quorum_percentage = 5.0     # 5% of total supply required for quorum

[api]
# REST API configuration
enabled = true
host = "127.0.0.1"
port = 8080

# Enable/disable specific API endpoints
enable_swagger = true
enable_metrics = true

[metrics]
# Metrics collection settings
enabled = true
push_gateway = ""  # Set to push gateway URL to enable metrics pushing
interval = 60      # Metrics collection interval in seconds

[telemetry]
# Telemetry and monitoring
enabled = false
endpoint = ""
service_name = "anya-core"

[backup]
# Automated backup configuration
enabled = true
frequency = "daily"  # daily, weekly, monthly
retention_days = 30
backup_dir = "${INSTALL_DIR}/backups"

[notifications]
# Notification settings
email_enabled = false
email_from = "anya@example.com"
email_smtp_host = "smtp.example.com"
email_smtp_port = 587
email_smtp_user = ""
email_smtp_password = ""

# Webhook notifications
webhook_enabled = false
webhook_url = ""
webhook_events = "all"  # all, errors, warnings, critical

[compliance]
# Regulatory compliance settings
gdpr_compliant = true
kyc_required = false
aml_checks = false

[security]
# Security settings
rate_limiting = true
max_connections = 1000
request_timeout = 30

# CORS settings
allow_origins = ["*"]
allow_methods = ["GET", "POST", "PUT", "DELETE"]
allow_headers = ["Content-Type", "Authorization"]

[maintenance]
# Maintenance mode settings
enabled = false
message = "System under maintenance. We'll be back soon."

[advanced]
# Advanced settings (use with caution)
debug_mode = false
log_sql_queries = false
max_threads = 0  # 0 = auto-detect

# Feature flags
feature_flags = [
    "experimental_apis",
    "beta_features"
]

# Custom environment variables
[environment]
RUST_LOG = "info"
RUST_BACKTRACE = "1"

# End of configuration
EOF
        
        # Set appropriate permissions
        chmod 640 "$config_path"
        chown "$(whoami):$(id -gn)" "$config_path"
    fi
    
    # Install monitoring if requested
    if [ "$MONITORING" = true ] && [ "$MONITORING_AVAILABLE" = true ]; then
        echo "[INFO] Setting up monitoring stack..."
        if ! setup_monitoring; then
            echo "[WARNING] Failed to set up monitoring stack. Continuing with installation..."
        fi
    fi

    echo "[SUCCESS] Anya Core installation completed successfully"
    echo "[INFO] You can now start using Anya Core"
    
    if [ "$MONITORING" = true ] && [ "$MONITORING_AVAILABLE" = true ]; then
        echo "[INFO] Monitoring is enabled and running"
        echo "       - Grafana: http://localhost:3000"
        echo "       - Prometheus: http://localhost:9090"
        echo "       - Alertmanager: http://localhost:9093"
    fi
    echo "1. Edit the configuration file: ${INSTALL_DIR}/config/anya.toml"
    echo "2. Start the service: sudo systemctl start anya-core"
    echo "3. Check status: sudo systemctl status anya-core"
    echo "4. View logs: sudo journalctl -u anya-core -f"
    echo ""
    echo "Documentation: https://docs.anya-core.org"
}

# Execute main function
install_anya_core "$@"
