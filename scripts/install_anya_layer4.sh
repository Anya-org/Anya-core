#!/bin/bash
# Anya Core Layer 4 Bitcoin Application Installer
# [AIR-3][AIS-3][BPC-3][RES-3]
#
# This script installs and starts Anya Core as a Layer 4 Bitcoin application
# with all required components across layers 1-4.

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
DEFAULT_INSTALL_PATH="/opt/anya-core"
NETWORK=${NETWORK:-"testnet"}
INSTALL_PATH=${1:-$DEFAULT_INSTALL_PATH}
LOG_FILE="$PROJECT_ROOT/anya_install.log"

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
    
    echo "[${timestamp}] [${level^^}] $message" >> "$LOG_FILE"
}

# Function to check prerequisites
check_prerequisites() {
    log "info" "Checking installation prerequisites..."
    
    # Check if running as root for system-wide installation
    if [[ "$INSTALL_PATH" == "/opt/"* || "$INSTALL_PATH" == "/usr/"* ]] && [ "$EUID" -ne 0 ]; then
        log "error" "Installing to $INSTALL_PATH requires root privileges. Please run with sudo."
        exit 1
    fi
    
    # Check for required commands
    local missing_deps=()
    for cmd in curl tar git; do
        if ! command -v $cmd &> /dev/null; then
            missing_deps+=($cmd)
        fi
    done
    
    if [ ${#missing_deps[@]} -gt 0 ]; then
        log "warning" "Missing required dependencies: ${missing_deps[*]}"
        log "info" "Installing missing dependencies..."
        
        if [ "$EUID" -ne 0 ]; then
            # Not root, use sudo
            sudo apt-get update
            sudo apt-get install -y ${missing_deps[@]}
        else
            # Already root
            apt-get update
            apt-get install -y ${missing_deps[@]}
        fi
    fi
    
    log "success" "All prerequisites satisfied"
}

# Function to install Bitcoin Core (Layer 1)
install_bitcoin_core() {
    log "info" "Installing Bitcoin Core (Layer 1)..."
    
    local bitcoin_dir="$INSTALL_PATH/layer1/bitcoin"
    local version="25.0"
    local tarball="bitcoin-$version-x86_64-linux-gnu.tar.gz"
    local download_url="https://bitcoincore.org/bin/bitcoin-core-$version/$tarball"
    
    # Create directories
    mkdir -p "$bitcoin_dir/bin"
    
    # Download Bitcoin Core
    log "info" "Downloading Bitcoin Core $version..."
    curl -L -o "/tmp/$tarball" "$download_url"
    
    # Extract binaries
    log "info" "Extracting Bitcoin Core binaries..."
    tar -xzf "/tmp/$tarball" -C "/tmp"
    cp "/tmp/bitcoin-$version/bin/bitcoind" "$bitcoin_dir/bin/"
    cp "/tmp/bitcoin-$version/bin/bitcoin-cli" "$bitcoin_dir/bin/"
    
    # Clean up
    rm -rf "/tmp/bitcoin-$version" "/tmp/$tarball"
    
    # Create symbolic links
    mkdir -p "$INSTALL_PATH/bin"
    ln -sf "$bitcoin_dir/bin/bitcoind" "$INSTALL_PATH/bin/bitcoind"
    ln -sf "$bitcoin_dir/bin/bitcoin-cli" "$INSTALL_PATH/bin/bitcoin-cli"
    
    # Set permissions
    chmod +x "$bitcoin_dir/bin/bitcoind"
    chmod +x "$bitcoin_dir/bin/bitcoin-cli"
    
    log "success" "Bitcoin Core (Layer 1) installed successfully"
}

# Function to install LND (Layer 2)
install_lnd() {
    log "info" "Installing LND (Layer 2)..."
    
    local lnd_dir="$INSTALL_PATH/layer2/lightning/lnd"
    local version="0.17.0-beta"
    local tarball="lnd-linux-amd64-v$version.tar.gz"
    local download_url="https://github.com/lightningnetwork/lnd/releases/download/v$version/$tarball"
    
    # Create directories
    mkdir -p "$lnd_dir/bin"
    
    # Download LND
    log "info" "Downloading LND $version..."
    curl -L -o "/tmp/$tarball" "$download_url"
    
    # Extract binaries
    log "info" "Extracting LND binaries..."
    tar -xzf "/tmp/$tarball" -C "/tmp"
    cp "/tmp/lnd-linux-amd64-v$version/lnd" "$lnd_dir/bin/"
    cp "/tmp/lnd-linux-amd64-v$version/lncli" "$lnd_dir/bin/"
    
    # Clean up
    rm -rf "/tmp/lnd-linux-amd64-v$version" "/tmp/$tarball"
    
    # Create symbolic links
    mkdir -p "$INSTALL_PATH/bin"
    ln -sf "$lnd_dir/bin/lnd" "$INSTALL_PATH/bin/lnd"
    ln -sf "$lnd_dir/bin/lncli" "$INSTALL_PATH/bin/lncli"
    
    # Set permissions
    chmod +x "$lnd_dir/bin/lnd"
    chmod +x "$lnd_dir/bin/lncli"
    
    log "success" "LND (Layer 2) installed successfully"
}

# Function to install RGB (Layer 3)
install_rgb() {
    log "info" "Installing RGB Protocol (Layer 3)..."
    
    local rgb_dir="$INSTALL_PATH/layer3/rgb"
    local version="0.10.8"
    local download_url="https://github.com/RGB-WG/rgb-node/releases/download/v$version/rgb-v$version-x86_64-unknown-linux-gnu.tar.gz"
    
    # Create directories
    mkdir -p "$rgb_dir/bin"
    
    # Download RGB
    log "info" "Downloading RGB $version..."
    curl -L -o "/tmp/rgb.tar.gz" "$download_url"
    
    # Extract binaries
    log "info" "Extracting RGB binaries..."
    tar -xzf "/tmp/rgb.tar.gz" -C "/tmp"
    cp "/tmp/rgb" "$rgb_dir/bin/"
    
    # Clean up
    rm -rf "/tmp/rgb" "/tmp/rgb.tar.gz"
    
    # Create symbolic links
    mkdir -p "$INSTALL_PATH/bin"
    ln -sf "$rgb_dir/bin/rgb" "$INSTALL_PATH/bin/rgb"
    
    # Set permissions
    chmod +x "$rgb_dir/bin/rgb"
    
    log "success" "RGB Protocol (Layer 3) installed successfully"
}

# Function to install Anya Core (Layer 4)
install_anya_core() {
    log "info" "Installing Anya Core (Layer 4)..."
    
    # Apply Layer 4 architecture
    "$SCRIPT_DIR/setup_layer4.sh" "$INSTALL_PATH"
    
    # Build Anya Core from source if needed
    if [ ! -f "$PROJECT_ROOT/target/release/anya-core" ]; then
        log "info" "Building Anya Core from source..."
        cd "$PROJECT_ROOT"
        cargo build --release
    fi
    
    # Copy binaries
    mkdir -p "$INSTALL_PATH/bin"
    cp "$PROJECT_ROOT/target/release/anya-core" "$INSTALL_PATH/bin/"
    cp "$PROJECT_ROOT/target/release/unified_installer" "$INSTALL_PATH/bin/"
    
    # Set permissions
    chmod +x "$INSTALL_PATH/bin/anya-core"
    chmod +x "$INSTALL_PATH/bin/unified_installer"
    
    log "success" "Anya Core (Layer 4) installed successfully"
}

# Function to create systemd services
create_systemd_services() {
    log "info" "Creating systemd services..."
    
    # Bitcoin service
    cat > "/tmp/bitcoind.service" << EOF
[Unit]
Description=Bitcoin Core Daemon (Layer 1)
After=network.target

[Service]
User=$USER
Group=$USER
Type=forking
ExecStart=$INSTALL_PATH/bin/bitcoind -conf=$INSTALL_PATH/layer1/bitcoin/bitcoin.conf -datadir=$INSTALL_PATH/data/bitcoin -daemon
ExecStop=$INSTALL_PATH/bin/bitcoin-cli -conf=$INSTALL_PATH/layer1/bitcoin/bitcoin.conf -datadir=$INSTALL_PATH/data/bitcoin stop
PIDFile=$INSTALL_PATH/data/bitcoin/bitcoind.pid
Restart=on-failure
TimeoutStartSec=120
TimeoutStopSec=120

[Install]
WantedBy=multi-user.target
EOF

    # LND service
    cat > "/tmp/lnd.service" << EOF
[Unit]
Description=Lightning Network Daemon (Layer 2)
After=network.target bitcoind.service
Requires=bitcoind.service

[Service]
User=$USER
Group=$USER
Type=simple
ExecStart=$INSTALL_PATH/bin/lnd --configfile=$INSTALL_PATH/layer2/lightning/lnd/lnd.conf --datadir=$INSTALL_PATH/data/lightning
Restart=on-failure
TimeoutStartSec=60
TimeoutStopSec=60
RestartSec=60

[Install]
WantedBy=multi-user.target
EOF

    # RGB service
    cat > "/tmp/rgb.service" << EOF
[Unit]
Description=RGB Protocol Daemon (Layer 3)
After=network.target bitcoind.service
Requires=bitcoind.service

[Service]
User=$USER
Group=$USER
Type=simple
ExecStart=$INSTALL_PATH/bin/rgb -c $INSTALL_PATH/layer3/rgb/rgb.yaml daemon
Restart=on-failure
TimeoutStartSec=60
TimeoutStopSec=60
RestartSec=60

[Install]
WantedBy=multi-user.target
EOF

    # Anya Core service
    cat > "/tmp/anya-core.service" << EOF
[Unit]
Description=Anya Core Layer 4 Bitcoin Network Service
After=network.target bitcoind.service lnd.service rgb.service
Wants=bitcoind.service lnd.service rgb.service

[Service]
User=$USER
Group=$USER
Type=simple
WorkingDirectory=$INSTALL_PATH
ExecStart=$INSTALL_PATH/bin/anya-core
Restart=on-failure
RestartSec=10
TimeoutStartSec=120

# Layer 4 service environment
Environment=ANYA_LAYER4_SERVICE=1
Environment=ANYA_LOG_LEVEL=info

[Install]
WantedBy=multi-user.target
EOF

    # Install services if running as root
    if [ "$EUID" -eq 0 ]; then
        cp "/tmp/bitcoind.service" "/etc/systemd/system/"
        cp "/tmp/lnd.service" "/etc/systemd/system/"
        cp "/tmp/rgb.service" "/etc/systemd/system/"
        cp "/tmp/anya-core.service" "/etc/systemd/system/"
        
        systemctl daemon-reload
        log "success" "Systemd services installed"
    else
        log "info" "To install systemd services, run:"
        log "info" "sudo cp /tmp/bitcoind.service /etc/systemd/system/"
        log "info" "sudo cp /tmp/lnd.service /etc/systemd/system/"
        log "info" "sudo cp /tmp/rgb.service /etc/systemd/system/"
        log "info" "sudo cp /tmp/anya-core.service /etc/systemd/system/"
        log "info" "sudo systemctl daemon-reload"
    fi
    
    log "success" "Systemd service files created"
}

# Function to start services
start_services() {
    log "info" "Starting Anya Core Layer 4 stack..."
    
    if [ "$EUID" -eq 0 ]; then
        # Running as root, use systemctl
        systemctl enable bitcoind.service
        systemctl enable lnd.service
        systemctl enable rgb.service
        systemctl enable anya-core.service
        
        systemctl start bitcoind.service
        sleep 5
        systemctl start lnd.service
        sleep 5
        systemctl start rgb.service
        sleep 5
        systemctl start anya-core.service
        
        log "success" "All services started successfully"
    else
        log "info" "To start all services, run:"
        log "info" "sudo systemctl enable bitcoind.service lnd.service rgb.service anya-core.service"
        log "info" "sudo systemctl start bitcoind.service"
        log "info" "sudo systemctl start lnd.service"
        log "info" "sudo systemctl start rgb.service"
        log "info" "sudo systemctl start anya-core.service"
    fi
}

# Function to create data directories
create_data_dirs() {
    log "info" "Creating data directories..."
    
    mkdir -p "$INSTALL_PATH/data/bitcoin"
    mkdir -p "$INSTALL_PATH/data/lightning"
    mkdir -p "$INSTALL_PATH/data/rgb"
    mkdir -p "$INSTALL_PATH/data/taproot-assets"
    mkdir -p "$INSTALL_PATH/data/web5"
    mkdir -p "$INSTALL_PATH/data/dao"
    mkdir -p "$INSTALL_PATH/logs"
    
    log "success" "Data directories created"
}

# Main function
main() {
    log "info" "Starting Anya Core Layer 4 Bitcoin Application installation..."
    log "info" "Installation path: $INSTALL_PATH"
    log "info" "Network: $NETWORK"
    
    # Check prerequisites
    check_prerequisites
    
    # Create data directories
    create_data_dirs
    
    # Install each layer
    install_bitcoin_core
    install_lnd
    install_rgb
    install_anya_core
    
    # Create systemd services
    create_systemd_services
    
    # Start services
    start_services
    
    log "success" "Anya Core Layer 4 Bitcoin Application installed successfully"
    log "info" "To access Bitcoin CLI: $INSTALL_PATH/bin/bitcoin-cli -conf=$INSTALL_PATH/layer1/bitcoin/bitcoin.conf"
    log "info" "To access Lightning CLI: $INSTALL_PATH/bin/lncli"
    log "info" "To access Anya Core: $INSTALL_PATH/bin/anya-core --help"
    log "info" "Layer 4 documentation: $INSTALL_PATH/docs/LAYER_ARCHITECTURE.md"
}

# Run main function
main "$@" 