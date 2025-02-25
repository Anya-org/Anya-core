#!/bin/bash
# Packaging script for anya-core
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
VERSION=$(cat VERSION 2>/dev/null || echo "0.1.0")
OUTPUT_DIR="./dist"
PACKAGE_NAME="anya-core-${VERSION}"
BUILD_TYPE="release"
INCLUDE_DOCS=true
INCLUDE_EXAMPLES=true
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCHITECTURE=$(uname -m)

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --name)
            PACKAGE_NAME="$2"
            shift 2
            ;;
        --debug)
            BUILD_TYPE="debug"
            shift
            ;;
        --no-docs)
            INCLUDE_DOCS=false
            shift
            ;;
        --no-examples)
            INCLUDE_EXAMPLES=false
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --version VERSION     Set package version (default: from VERSION file or 0.1.0)"
            echo "  --output-dir DIR      Set output directory (default: ./dist)"
            echo "  --name NAME           Set package name (default: anya-core-VERSION)"
            echo "  --debug               Build debug version instead of release"
            echo "  --no-docs             Don't include documentation"
            echo "  --no-examples         Don't include examples"
            echo "  --help                Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}Packaging anya-core v${VERSION}...${NC}"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed.${NC}"
    echo "Please install Rust and Cargo: https://rustup.rs/"
    exit 1
fi

# Check current directory
if [[ ! -f "Cargo.toml" ]]; then
    echo -e "${RED}Error: This script must be run from the anya-core directory.${NC}"
    exit 1
fi

# Create output directory
mkdir -p "${OUTPUT_DIR}"

# Run tests to ensure everything is working
echo -e "${YELLOW}Running tests...${NC}"
cargo test

# Build the project
echo -e "${YELLOW}Building ${BUILD_TYPE} version...${NC}"
if [[ "${BUILD_TYPE}" == "release" ]]; then
    cargo build --release
    BINARY_DIR="target/release"
else
    cargo build
    BINARY_DIR="target/debug"
fi

# Create package directory
PACKAGE_DIR="${OUTPUT_DIR}/${PACKAGE_NAME}"
mkdir -p "${PACKAGE_DIR}/bin"
mkdir -p "${PACKAGE_DIR}/lib"
mkdir -p "${PACKAGE_DIR}/include"
mkdir -p "${PACKAGE_DIR}/scripts"

# Copy binaries and libraries
echo -e "${YELLOW}Copying binaries and libraries...${NC}"
cp "${BINARY_DIR}"/*.{a,so,dll,dylib,rlib,lib} "${PACKAGE_DIR}/lib/" 2>/dev/null || true
cp "${BINARY_DIR}"/anya* "${PACKAGE_DIR}/bin/" 2>/dev/null || true

# Copy header files
echo -e "${YELLOW}Copying header files...${NC}"
cp -r include/* "${PACKAGE_DIR}/include/" 2>/dev/null || true

# Copy scripts
echo -e "${YELLOW}Copying scripts...${NC}"
cp scripts/*.{sh,py,js} "${PACKAGE_DIR}/scripts/" 2>/dev/null || true

# Copy documentation
if [[ "${INCLUDE_DOCS}" == true ]]; then
    echo -e "${YELLOW}Copying documentation...${NC}"
    mkdir -p "${PACKAGE_DIR}/docs"
    cp -r docs/* "${PACKAGE_DIR}/docs/" 2>/dev/null || true
    cp README.md CHANGELOG.md LICENSE.md "${PACKAGE_DIR}/" 2>/dev/null || true
fi

# Copy examples
if [[ "${INCLUDE_EXAMPLES}" == true ]]; then
    echo -e "${YELLOW}Copying examples...${NC}"
    mkdir -p "${PACKAGE_DIR}/examples"
    cp -r examples/* "${PACKAGE_DIR}/examples/" 2>/dev/null || true
fi

# Create metadata file
echo -e "${YELLOW}Creating metadata...${NC}"
cat > "${PACKAGE_DIR}/metadata.json" << EOL
{
    "name": "anya-core",
    "version": "${VERSION}",
    "platform": "${PLATFORM}",
    "architecture": "${ARCHITECTURE}",
    "build_type": "${BUILD_TYPE}",
    "build_date": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "dependencies": {
        "rust": "$(rustc --version | cut -d ' ' -f 2)",
        "cargo": "$(cargo --version | cut -d ' ' -f 2)"
    }
}
EOL

# Create archive
echo -e "${YELLOW}Creating archive...${NC}"
cd "${OUTPUT_DIR}"
if [[ "${PLATFORM}" == "windows" ]]; then
    # Create zip archive on Windows
    if command -v powershell &> /dev/null; then
        powershell -Command "Compress-Archive -Path '${PACKAGE_NAME}' -DestinationPath '${PACKAGE_NAME}.zip'"
        echo -e "${GREEN}Package created: ${OUTPUT_DIR}/${PACKAGE_NAME}.zip${NC}"
    else
        echo -e "${RED}Error: PowerShell is not available to create zip archive.${NC}"
        echo "Please manually zip the package directory."
    fi
else
    # Create tarball on Unix-like systems
    tar -czf "${PACKAGE_NAME}.tar.gz" "${PACKAGE_NAME}"
    echo -e "${GREEN}Package created: ${OUTPUT_DIR}/${PACKAGE_NAME}.tar.gz${NC}"
fi

echo -e "${GREEN}Packaging completed successfully!${NC}"
exit 0 