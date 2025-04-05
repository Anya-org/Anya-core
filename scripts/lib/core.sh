#!/bin/bash
# Core utility functions for Anya scripts
# [AIR-3][AIS-3][BPC-3]

set -euo pipefail

# Import core standards
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/bitcoin_standards.sh"

# Project configuration
readonly MIN_DISK_SPACE=50  # GB
readonly REQUIRED_PACKAGES=(
    "build-essential"
    "curl"
    "libssl-dev"
    "pkg-config"
    "cmake"
    "llvm"
)

# Platform-specific setup functions
setup_linux_specific() {
    # Set system limits
    if [ -w "/etc/security/limits.conf" ]; then
        cat >> /etc/security/limits.conf << EOF
* soft nofile 65535
* hard nofile 65535
* soft nproc 32768
* hard nproc 32768
EOF
    fi

    # Configure performance settings
    [ -w "/proc/sys/vm/swappiness" ] && echo 10 > /proc/sys/vm/swappiness
    for governor in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
        [ -w "$governor" ] && echo "performance" > "$governor"
    done

    # Setup systemd service
    setup_systemd_service
}

setup_macos_specific() {
    # Configure sysctl settings
    cat > /tmp/sysctl.conf << EOF
kern.maxfiles=65536
kern.maxfilesperproc=65536
kern.maxvnodes=262144
EOF
    sudo mv /tmp/sysctl.conf /etc/sysctl.conf

    # Setup Homebrew
    command -v brew >/dev/null || {
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    }
    brew update

    # Setup launchd service
    setup_launchd_service
}

# Core setup function
setup_environment() {
    local platform=$1
    verify_system
    
    case "$platform" in
        "linux")  setup_linux_specific ;;
        "darwin") setup_macos_specific ;;
        *)        log_error "Unsupported platform: $platform" ;;
    esac

    install_required_packages
    setup_rust
    setup_web5
    configure_git
}

# Utility functions
verify_system() {
    check_dependencies "${REQUIRED_PACKAGES[@]}"
    check_disk_space "$MIN_DISK_SPACE"
    check_services
}

setup_rust() {
    command -v rustc &>/dev/null || {
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    }
}

setup_web5() {
    cargo install web5-cli
    web5 db init
    web5 db migrate
    setup_web5_protocols
}

# Export functions
export -f setup_environment
export -f verify_system
export -f setup_rust
export -f setup_web5
