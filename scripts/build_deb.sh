#!/bin/bash
# Anya Core Debian Package Builder
# [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
#
# This script builds a Debian package for the Anya Core Unified Installer.

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
VERSION=$(cat "$PROJECT_ROOT/VERSION" 2>/dev/null || echo "2.5.0")
ARCHITECTURE=$(dpkg --print-architecture 2>/dev/null || echo "amd64")
PACKAGE_NAME="anya-core_${VERSION}_${ARCHITECTURE}"
BUILD_DIR="$PROJECT_ROOT/build/$PACKAGE_NAME"

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
}

# Function to display help
show_help() {
    echo "Anya Core Debian Package Builder"
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help                 Show this help message"
    echo "  -v, --version VERSION      Set package version (default: $VERSION)"
    echo "  -a, --arch ARCHITECTURE    Set package architecture (default: $ARCHITECTURE)"
    echo "  --skip-build               Skip building the Rust binary"
    echo "  --skip-tests               Skip running tests"
    echo "  --sign                     Sign the Debian package"
    echo "  --publish                  Publish to repository after building"
    echo ""
    echo "Examples:"
    echo "  $0 --version 2.5.1 --arch amd64"
    echo "  $0 --sign --publish"
    echo ""
}

# Function to ensure dependencies are installed
ensure_dependencies() {
    log "info" "Checking build dependencies..."
    
    # Check for required commands
    local deps=("cargo" "rustc" "dpkg-deb" "fakeroot")
    local missing=()
    
    for cmd in "${deps[@]}"; do
        if ! command -v "$cmd" &> /dev/null; then
            missing+=("$cmd")
        fi
    done
    
    if [ ${#missing[@]} -gt 0 ]; then
        log "warning" "Missing dependencies: ${missing[*]}"
        log "info" "Installing missing dependencies..."
        
        # Update package cache
        sudo apt-get update
        
        # Install required packages
        if [[ " ${missing[*]} " =~ " cargo " ]] || [[ " ${missing[*]} " =~ " rustc " ]]; then
            log "info" "Installing Rust..."
            sudo apt-get install -y curl
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source "$HOME/.cargo/env"
        fi
        
        if [[ " ${missing[*]} " =~ " dpkg-deb " ]]; then
            log "info" "Installing dpkg-dev..."
            sudo apt-get install -y dpkg-dev
        fi
        
        if [[ " ${missing[*]} " =~ " fakeroot " ]]; then
            log "info" "Installing fakeroot..."
            sudo apt-get install -y fakeroot
        fi
    else
        log "success" "All dependencies are installed"
    fi
}

# Function to build the Rust binary
build_binary() {
    log "info" "Building Rust binary..."
    
    # Navigate to project root
    cd "$PROJECT_ROOT"
    
    # Build with Cargo
    cargo build --release --bin unified_installer
    
    # Check if build succeeded
    if [ ! -f "$PROJECT_ROOT/target/release/unified_installer" ]; then
        log "error" "Failed to build Rust binary"
        exit 1
    fi
    
    log "success" "Rust binary built successfully"
}

# Function to run tests
run_tests() {
    log "info" "Running tests..."
    
    # Navigate to project root
    cd "$PROJECT_ROOT"
    
    # Run tests
    cargo test
    
    if [ $? -ne 0 ]; then
        log "error" "Tests failed"
        exit 1
    fi
    
    log "success" "Tests passed"
}

# Function to create Debian package structure
create_package_structure() {
    log "info" "Creating package structure..."
    
    # Create directory structure
    mkdir -p "$BUILD_DIR/DEBIAN"
    mkdir -p "$BUILD_DIR/usr/bin"
    mkdir -p "$BUILD_DIR/usr/share/anya-core"
    mkdir -p "$BUILD_DIR/usr/share/anya-core/config"
    mkdir -p "$BUILD_DIR/usr/share/anya-core/scripts"
    mkdir -p "$BUILD_DIR/usr/share/doc/anya-core"
    mkdir -p "$BUILD_DIR/usr/share/man/man1"
    mkdir -p "$BUILD_DIR/lib/systemd/system"
    
    log "success" "Package structure created"
}

# Function to create Debian control file
create_control_file() {
    log "info" "Creating control file..."
    
    # Create control file
    cat > "$BUILD_DIR/DEBIAN/control" << EOF
Package: anya-core
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCHITECTURE
Depends: libssl-dev, pkg-config, build-essential, curl
Recommends: postgresql, git
Maintainer: Anya Development Team <support@anya.org>
Description: Anya Core Bitcoin Development Framework
 Implements BIP-341, BIP-342, BIP-174, and AIS-3 security standards.
 Provides DAO, Web5, and Bitcoin functionality in a unified interface.
 .
 Full BPC-3 compliance with comprehensive testing and validation.
EOF
    
    log "success" "Control file created"
}

# Function to create postinst script
create_postinst_script() {
    log "info" "Creating post-installation script..."
    
    # Create postinst script
    cat > "$BUILD_DIR/DEBIAN/postinst" << 'EOF'
#!/bin/bash
set -e

# Create symlink
if [ ! -f /usr/bin/anya-core ]; then
    ln -s /usr/share/anya-core/bin/unified_installer /usr/bin/anya-core
fi

# Check if systemd is available
if [ -d /run/systemd/system ]; then
    # Reload systemd to pick up new service
    systemctl daemon-reload >/dev/null 2>&1 || true
    
    # Print message about service
    echo "Anya Core service installed. To enable and start:"
    echo "  sudo systemctl enable anya-core.service"
    echo "  sudo systemctl start anya-core.service"
fi

# Set permissions
chmod 755 /usr/share/anya-core/bin/unified_installer
chmod 755 /usr/share/anya-core/scripts/*.sh

# Create default configuration if not exists
if [ ! -f /etc/anya-core/config.conf ]; then
    mkdir -p /etc/anya-core
    cp /usr/share/anya-core/config/default.conf /etc/anya-core/config.conf
fi

# Create log directory
mkdir -p /var/log/anya-core

exit 0
EOF
    
    # Make postinst executable
    chmod 755 "$BUILD_DIR/DEBIAN/postinst"
    
    log "success" "Post-installation script created"
}

# Function to create prerm script
create_prerm_script() {
    log "info" "Creating pre-removal script..."
    
    # Create prerm script
    cat > "$BUILD_DIR/DEBIAN/prerm" << 'EOF'
#!/bin/bash
set -e

# Check if systemd is available
if [ -d /run/systemd/system ]; then
    # Stop anya-core service if running
    systemctl stop anya-core.service >/dev/null 2>&1 || true
    systemctl disable anya-core.service >/dev/null 2>&1 || true
fi

# Remove symlink
if [ -L /usr/bin/anya-core ]; then
    rm /usr/bin/anya-core
fi

exit 0
EOF
    
    # Make prerm executable
    chmod 755 "$BUILD_DIR/DEBIAN/prerm"
    
    log "success" "Pre-removal script created"
}

# Function to create systemd service file
create_systemd_service() {
    log "info" "Creating systemd service file..."
    
    # Create systemd service file
    cat > "$BUILD_DIR/lib/systemd/system/anya-core.service" << EOF
[Unit]
Description=Anya Core Service
After=network.target
Documentation=https://github.com/anya-org/anya-core

[Service]
Type=simple
ExecStart=/usr/share/anya-core/bin/unified_installer --service
Restart=on-failure
User=root
Group=root
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF
    
    log "success" "Systemd service file created"
}

# Function to create man page
create_man_page() {
    log "info" "Creating man page..."
    
    # Create man page
    cat > "$BUILD_DIR/usr/share/man/man1/anya-core.1" << EOF
.TH ANYA-CORE 1 "$(date +"%B %Y")" "Version $VERSION" "Anya Core Manual"
.SH NAME
anya-core \- Bitcoin Development Framework with BIP-341 compliance
.SH SYNOPSIS
.B anya-core
[\fIOPTIONS\fR]
.SH DESCRIPTION
Anya Core is a comprehensive Bitcoin Development Framework that provides
a unified interface for Bitcoin, DAO, Web5, and ML components with full
BIP compliance.
.SH OPTIONS
.TP
.BR \-h ", " \-\-help
Show help message.
.TP
.BR \-p ", " \-\-path " " \fIDIR\fR
Installation path.
.TP
.BR \-m ", " \-\-mode " " \fIMODE\fR
Installation mode (development or production).
.TP
.BR \-\-profile " " \fIPROFILE\fR
Installation profile (minimal, standard, full, enterprise).
.TP
.BR \-c ", " \-\-components " " \fICOMPONENTS\fR
Comma-separated list of components to install.
.TP
.BR \-v ", " \-\-verify
Only verify system requirements.
.TP
.BR \-r ", " \-\-report
Generate detailed installation report.
.SH FILES
.TP
.I /etc/anya-core/config.conf
Main configuration file.
.TP
.I /var/log/anya-core
Log directory.
.SH EXAMPLES
.PP
Install with standard profile:
.PP
.nf
.RS
anya-core --path /opt/anya-core --profile standard
.RE
.fi
.PP
Verify system requirements:
.PP
.nf
.RS
anya-core --verify
.RE
.fi
.SH AUTHOR
Anya Development Team <support@anya.org>
.SH REPORTING BUGS
Report bugs to: https://github.com/anya-org/anya-core/issues
.SH COPYRIGHT
Copyright \(co $(date +"%Y") Anya Core. License: MIT
EOF
    
    # Compress man page
    gzip -9 -n "$BUILD_DIR/usr/share/man/man1/anya-core.1"
    
    log "success" "Man page created"
}

# Function to copy files to package structure
copy_files() {
    log "info" "Copying files to package structure..."
    
    # Create bin directory
    mkdir -p "$BUILD_DIR/usr/share/anya-core/bin"
    
    # Copy binary
    cp "$PROJECT_ROOT/target/release/unified_installer" "$BUILD_DIR/usr/share/anya-core/bin/"
    
    # Copy scripts
    cp "$PROJECT_ROOT/scripts/install.sh" "$BUILD_DIR/usr/share/anya-core/scripts/"
    
    # Copy default config
    cp -r "$PROJECT_ROOT/config" "$BUILD_DIR/usr/share/anya-core/"
    
    # Copy documentation
    cp "$PROJECT_ROOT/README.md" "$BUILD_DIR/usr/share/doc/anya-core/"
    cp "$PROJECT_ROOT/CHANGELOG.md" "$BUILD_DIR/usr/share/doc/anya-core/" 2>/dev/null || cp "$SCRIPT_DIR/changelog.md" "$BUILD_DIR/usr/share/doc/anya-core/CHANGELOG.md"
    
    log "success" "Files copied to package structure"
}

# Function to build Debian package
build_package() {
    log "info" "Building Debian package..."
    
    # Set permissions
    find "$BUILD_DIR" -type d -exec chmod 755 {} \;
    find "$BUILD_DIR" -type f -exec chmod 644 {} \;
    chmod 755 "$BUILD_DIR/DEBIAN/postinst"
    chmod 755 "$BUILD_DIR/DEBIAN/prerm"
    chmod 755 "$BUILD_DIR/usr/share/anya-core/bin/unified_installer"
    chmod 755 "$BUILD_DIR/usr/share/anya-core/scripts/"*.sh
    
    # Build package
    fakeroot dpkg-deb --build "$BUILD_DIR" "$PROJECT_ROOT/build/${PACKAGE_NAME}.deb"
    
    # Check if build succeeded
    if [ ! -f "$PROJECT_ROOT/build/${PACKAGE_NAME}.deb" ]; then
        log "error" "Failed to build Debian package"
        exit 1
    fi
    
    log "success" "Debian package built successfully: ${PACKAGE_NAME}.deb"
}

# Function to sign package
sign_package() {
    log "info" "Signing Debian package..."
    
    # Check if dpkg-sig is installed
    if ! command -v dpkg-sig &> /dev/null; then
        log "info" "Installing dpkg-sig..."
        sudo apt-get install -y dpkg-sig
    fi
    
    # Sign package
    dpkg-sig --sign builder "$PROJECT_ROOT/build/${PACKAGE_NAME}.deb"
    
    if [ $? -ne 0 ]; then
        log "error" "Failed to sign Debian package"
        exit 1
    fi
    
    log "success" "Debian package signed successfully"
}

# Function to publish package to repository
publish_package() {
    log "info" "Publishing Debian package..."
    
    # This is a placeholder for your specific repository configuration
    # You would typically use something like dput or a custom script
    
    log "warning" "Publishing is not implemented yet"
    log "info" "To manually publish, use: dput your-repo $PROJECT_ROOT/build/${PACKAGE_NAME}.deb"
}

# Main function
main() {
    # Parse command line arguments
    SKIP_BUILD=false
    SKIP_TESTS=false
    SIGN_PACKAGE=false
    PUBLISH_PACKAGE=false
    
    while [[ $# -gt 0 ]]; do
        key="$1"
        case $key in
            -h|--help)
                show_help
                exit 0
                ;;
            -v|--version)
                VERSION="$2"
                shift 2
                ;;
            -a|--arch)
                ARCHITECTURE="$2"
                shift 2
                ;;
            --skip-build)
                SKIP_BUILD=true
                shift
                ;;
            --skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            --sign)
                SIGN_PACKAGE=true
                shift
                ;;
            --publish)
                PUBLISH_PACKAGE=true
                shift
                ;;
            *)
                log "error" "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    log "info" "Building Anya Core Debian package version $VERSION for $ARCHITECTURE"
    
    # Update PACKAGE_NAME with current values
    PACKAGE_NAME="anya-core_${VERSION}_${ARCHITECTURE}"
    BUILD_DIR="$PROJECT_ROOT/build/$PACKAGE_NAME"
    
    # Create build directory
    mkdir -p "$PROJECT_ROOT/build"
    
    # Ensure dependencies are installed
    ensure_dependencies
    
    # Build Rust binary
    if [ "$SKIP_BUILD" = false ]; then
        build_binary
    else
        log "info" "Skipping Rust binary build"
    fi
    
    # Run tests
    if [ "$SKIP_TESTS" = false ]; then
        run_tests
    else
        log "info" "Skipping tests"
    fi
    
    # Create package structure
    create_package_structure
    
    # Create Debian control file
    create_control_file
    
    # Create postinst script
    create_postinst_script
    
    # Create prerm script
    create_prerm_script
    
    # Create systemd service file
    create_systemd_service
    
    # Create man page
    create_man_page
    
    # Copy files to package structure
    copy_files
    
    # Build Debian package
    build_package
    
    # Sign package if requested
    if [ "$SIGN_PACKAGE" = true ]; then
        sign_package
    fi
    
    # Publish package if requested
    if [ "$PUBLISH_PACKAGE" = true ]; then
        publish_package
    fi
    
    log "success" "Debian package for Anya Core v$VERSION ($ARCHITECTURE) created successfully"
    log "info" "Package location: $PROJECT_ROOT/build/${PACKAGE_NAME}.deb"
}

# Run main function
main "$@" 