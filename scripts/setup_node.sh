#!/usr/bin/env bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print error messages and create directories
error() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

create_dir() {
    local dir="$1"
    if [ ! -d "$dir" ]; then
        mkdir -p "$dir" || error "Failed to create directory: $dir"
        echo -e "${GREEN}Created directory: $dir${NC}"
    fi
}

# Function to verify command availability
verify_command() {
    if ! command -v "$1" >/dev/null 2>&1; then
        error "$1 command not found. Installation may have failed."
    fi
}

# Function to load NVM with proper environment setup
load_nvm() {
    # Export NVM directory
    export NVM_DIR="$HOME/.nvm"
    
    # Source NVM without completion first
    if [ -s "$NVM_DIR/nvm.sh" ]; then
        \. "$NVM_DIR/nvm.sh" --no-use || return 1
        
        # Manually set up path
        export PATH="$NVM_DIR/versions/node/$(nvm current)/bin:$PATH"
        
        # Only try bash completion if shell is bash
        if [ -n "$BASH_VERSION" ] && [ -s "$NVM_DIR/bash_completion" ]; then
            \. "$NVM_DIR/bash_completion" || true
        fi
        
        return 0
    fi
    return 1
}

# Function to verify NVM installation
verify_nvm() {
    if ! load_nvm; then
        error "Failed to load NVM"
    fi
    
    # Verify NVM is working
    if ! nvm --version >/dev/null 2>&1; then
        error "NVM installation failed"
    fi
    
    echo -e "${GREEN}NVM $(nvm --version) installed successfully${NC}"
}

# Function to ensure Node.js is installed and available
ensure_node() {
    echo -e "${GREEN}Checking Node.js installation...${NC}"
    
    # First try loading NVM 
    load_nvm
    
    # Check if node command exists after loading NVM
    if command -v node >/dev/null 2>&1; then
        echo -e "${GREEN}Found existing Node.js installation${NC}"
        node --version
        return 0
    fi
    
    echo -e "${GREEN}Installing Node.js LTS version...${NC}"
    
    # Ensure NVM is loaded
    export NVM_DIR="$HOME/.nvm"
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
    
    # Install LTS version
    nvm install --lts --latest-npm || error "Failed to install Node.js LTS"
    nvm use --lts || error "Failed to use Node.js LTS version"
    
    # Verify installation worked
    if ! command -v node >/dev/null 2>&1; then
        error "Node.js installation failed. Please install Node.js manually."
    fi
    
    echo -e "${GREEN}Successfully installed Node.js $(node --version)${NC}"
}

# Setup directories
BASE_DIR="/workspaces/Anya-core"
SCRIPTS_DIR="$BASE_DIR/scripts"
BITCOIN_DIR="$SCRIPTS_DIR/bitcoin"

create_dir "$SCRIPTS_DIR"
create_dir "$BITCOIN_DIR"

# Ensure we're in the right directory
cd "$BASE_DIR" || error "Failed to change to $BASE_DIR"

# Install and load NVM
echo -e "${GREEN}Installing NVM (Node Version Manager)...${NC}"
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash || error "Failed to install NVM"

echo -e "${GREEN}Verifying NVM installation...${NC}"
verify_nvm

echo -e "${GREEN}Setting up Node.js...${NC}"
ensure_node || error "Failed to set up Node.js"

# Initialize package.json if needed
if [ ! -f "package.json" ]; then
    echo -e "${GREEN}Initializing package.json...${NC}"
    # Use explicit paths to npm
    "$NVM_DIR/versions/node/$(node --version)/bin/npm" init -y || error "Failed to initialize package.json"
fi

echo -e "${GREEN}Installing dependencies...${NC}"
# Use explicit paths to npm
"$NVM_DIR/versions/node/$(node --version)/bin/npm" install node-fetch crypto readline || error "Failed to install dependencies"

# Add network-specific dependencies
echo -e "${GREEN}Installing network dependencies...${NC}"
"$NVM_DIR/versions/node/$(node --version)/bin/npm" install \
  @bitcoinerlab/secp256k1 \
  @lightning/lnurl \
  rgb-node \
  @rsksmart/rsk3 \
  @stacks/blockchain-api-client \
  dlc-lib || error "Failed to install network dependencies"

# Add network testing tools
echo -e "${GREEN}Installing testing frameworks...${NC}"
"$NVM_DIR/versions/node/$(node --version)/bin/npm" install --save-dev \
  jest-blockchain \
  hardhat \
  @nomiclabs/hardhat-ethers \
  @stacks/stacking \
  tapscript \
  regtest-client || error "Failed to install testing frameworks"

echo -e "${GREEN}Setting permissions...${NC}"
chmod +x "$BITCOIN_DIR/mcp-server.js" 2>/dev/null || echo -e "${YELLOW}Warning: Could not set permissions on mcp-server.js (file may not exist yet)${NC}"

# Instructions for shell reloading
echo -e "\n${GREEN}Setup complete! To use Node.js:${NC}"
echo "1. First reload NVM in current shell:"
echo "   source ~/.nvm/nvm.sh"
echo
echo "2. Then verify installation:"
echo "   node --version"
echo
echo "3. For new terminals, run:"
echo "   source ~/.bashrc"
