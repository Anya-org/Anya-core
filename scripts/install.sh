#!/bin/bash
# Cross-platform installer for Anya Core

# Default values
INSTALL=false
UNINSTALL=false
CONFIG=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  key="$1"
  case $key in
    --install)
      INSTALL=true
      shift
      ;;
    --uninstall)
      UNINSTALL=true
      shift
      ;;
    --config)
      CONFIG="$2"
      shift
      shift
      ;;
    *)
      echo "Unknown option: $key"
      exit 1
      ;;
  esac
done

# Determine OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS_TYPE="macOS"
    DATA_DIR="/Library/Application Support/AnyaCore"
    BIN_DIR="/Applications/AnyaCore.app/Contents/MacOS"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS_TYPE="Linux"
    DATA_DIR="/etc/anya-core"
    BIN_DIR="/usr/local/bin"
else
    OS_TYPE="Unknown"
    DATA_DIR="./data"
    BIN_DIR="./bin"
fi

# Check for root
if [ "$EUID" -ne 0 ]; then
  echo "This installer requires root privileges. Please run with sudo."
  exit 1
fi

# Ensure Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust is required but not installed. Would you like to install it now? (y/n)"
    read -r install_rust
    if [[ "$install_rust" =~ ^[Yy]$ ]]; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo "Rust is required to continue. Exiting."
        exit 1
    fi
fi

# Create necessary directories
create_directories() {
    echo "Creating data directory: $DATA_DIR"
    mkdir -p "$DATA_DIR"
    
    echo "Creating bin directory: $BIN_DIR"
    mkdir -p "$BIN_DIR"
}

# Install Anya Core
install_anya_core() {
    # Build the project
    echo "Building Anya Core..."
    SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
    cd "$SCRIPT_DIR/.." || exit 1
    
    cargo build --release
    
    # Copy binaries
    echo "Copying binaries to $BIN_DIR"
    cp "target/release/anya-installer" "$BIN_DIR/anya-installer"
    chmod +x "$BIN_DIR/anya-installer"
    
    # Run the installer
    echo "Running Anya Core installer..."
    INSTALL_ARGS="install"
    if [ -n "$CONFIG" ]; then
        INSTALL_ARGS="$INSTALL_ARGS --config \"$CONFIG\""
    fi
    
    "$BIN_DIR/anya-installer" $INSTALL_ARGS
    
    echo "Installation complete!"
}

# Uninstall Anya Core
uninstall_anya_core() {
    echo "Uninstalling Anya Core..."
    SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
    cd "$SCRIPT_DIR/.." || exit 1
    
    cargo build --release
    
    # Run the uninstaller
    "target/release/anya-installer" uninstall
    
    # Remove data directory
    if [ -d "$DATA_DIR" ]; then
        echo "Removing data directory: $DATA_DIR"
        rm -rf "$DATA_DIR"
    fi
    
    echo "Uninstallation complete!"
}

# Main execution
if [ "$INSTALL" = true ]; then
    create_directories
    install_anya_core
elif [ "$UNINSTALL" = true ]; then
    uninstall_anya_core
else
    echo "No action specified. Use --install or --uninstall"
    echo "Usage: ./install.sh --install [--config path/to/config.json] | --uninstall"
fi
