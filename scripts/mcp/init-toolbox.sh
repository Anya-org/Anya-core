#!/bin/bash
# MCP Tools Toolbox Initialization Script
# Last updated: 2025-06-02
# AI Generated: BPC-3, AIR-3, AIS-3, AIT-3, PFM-3, SCL-3, RES-3, DID-3

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MCP_CONFIG_DIR="$PROJECT_ROOT/mcp"
TOOLBOX_DIR="$MCP_CONFIG_DIR/toolbox"
LOGS_DIR="$MCP_CONFIG_DIR/logs"

# Logging
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create directory structure
create_directories() {
    log_info "Creating MCP toolbox directory structure..."
    
    mkdir -p "$MCP_CONFIG_DIR"
    mkdir -p "$TOOLBOX_DIR"
    mkdir -p "$LOGS_DIR"
    mkdir -p "$TOOLBOX_DIR/servers"
    mkdir -p "$TOOLBOX_DIR/tools"
    mkdir -p "$TOOLBOX_DIR/extensions"
    
    log_success "Directory structure created"
}

# Check for required tools
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    local missing=()
    
    # Check for Node.js
    if ! command -v node &> /dev/null; then
        missing+=("node")
    fi
    
    # Check for npm
    if ! command -v npm &> /dev/null; then
        missing+=("npm")
    fi
    
    # Check for pipx
    if ! command -v pipx &> /dev/null; then
        missing+=("pipx")
    fi
    
    # Check for Python
    if ! command -v python3 &> /dev/null; then
        missing+=("python3")
    fi
    
    if [ ${#missing[@]} -gt 0 ]; then
        log_error "Missing required tools: ${missing[*]}"
        log_info "Please install the missing tools and run this script again"
        exit 1
    fi
    
    log_success "All prerequisites are available"
}

# Install MCP servers
install_mcp_servers() {
    log_info "Installing MCP servers..."
    
    # Install mem0 MCP server
    if ! pipx list | grep -q "mem0-mcp-for-pm"; then
        log_info "Installing mem0-mcp-for-pm..."
        pipx install mem0-mcp-for-pm==0.3.2
        log_success "mem0-mcp-for-pm installed"
    else
        log_info "mem0-mcp-for-pm already installed"
    fi
    
    # Install GitHub MCP server
    log_info "Installing GitHub MCP server..."
    npm install -g @modelcontextprotocol/server-github
    
    # Install filesystem MCP server
    log_info "Installing filesystem MCP server..."
    npm install -g @modelcontextprotocol/server-filesystem
    
    # Install PostgreSQL MCP server
    log_info "Installing PostgreSQL MCP server..."
    npm install -g @modelcontextprotocol/server-postgres
    
    # Install Puppeteer MCP server
    log_info "Installing Puppeteer MCP server..."
    npm install -g @modelcontextprotocol/server-puppeteer
    
    # Install Brave Search MCP server
    log_info "Installing Brave Search MCP server..."
    npm install -g @modelcontextprotocol/server-brave-search
    
    # Install Sequential Thinking MCP server
    log_info "Installing Sequential Thinking MCP server..."
    npm install -g @modelcontextprotocol/server-sequential-thinking
    
    log_success "All MCP servers installed"
}

# Validate environment variables
validate_environment() {
    log_info "Validating environment variables..."
    
    local warnings=()
    
    # Check required variables
    if [ -z "${MEM0_API_KEY:-}" ]; then
        warnings+=("MEM0_API_KEY not set")
    fi
    
    if [ -z "${GITHUB_TOKEN:-}" ]; then
        warnings+=("GITHUB_TOKEN not set")
    fi
    
    # Check optional variables
    if [ -z "${BRAVE_API_KEY:-}" ]; then
        log_warning "BRAVE_API_KEY not set (optional)"
    fi
    
    if [ ${#warnings[@]} -gt 0 ]; then
        log_warning "Missing environment variables:"
        for warning in "${warnings[@]}"; do
            log_warning "  - $warning"
        done
        log_info "Some features may not work without these variables"
    else
        log_success "All required environment variables are set"
    fi
}

# Test MCP servers
test_mcp_servers() {
    log_info "Testing MCP server installations..."
    
    # Test mem0
    if pipx run mem0-mcp-for-pm --help &> /dev/null; then
        log_success "mem0-mcp-for-pm: OK"
    else
        log_error "mem0-mcp-for-pm: FAILED"
    fi
    
    # Test GitHub server
    if npx @modelcontextprotocol/server-github --help &> /dev/null; then
        log_success "GitHub MCP server: OK"
    else
        log_error "GitHub MCP server: FAILED"
    fi
    
    # Test filesystem server
    if npx @modelcontextprotocol/server-filesystem --help &> /dev/null; then
        log_success "Filesystem MCP server: OK"
    else
        log_error "Filesystem MCP server: FAILED"
    fi
    
    log_success "MCP server testing completed"
}

# Create configuration files
create_config_files() {
    log_info "Creating configuration files..."
    
    # Create main MCP configuration
    cat > "$MCP_CONFIG_DIR/mcp.json" << 'EOF'
{
  "version": 1,
  "mcpServers": {
    "mem0-knowledge": {
      "command": "pipx",
      "args": [
        "run",
        "mem0-mcp-for-pm==0.3.2",
        "--log=off"
      ],
      "env": {
        "MEM0_API_KEY": "${MEM0_API_KEY}"
      }
    },
    "github-tools": {
      "command": "npx",
      "args": [
        "@modelcontextprotocol/server-github"
      ],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"
      }
    },
    "filesystem-tools": {
      "command": "npx",
      "args": [
        "@modelcontextprotocol/server-filesystem",
        "/home/bmokoka/Anya-core"
      ]
    },
    "anya-bitcoin-tools": {
      "command": "node",
      "args": [
        "scripts/bitcoin/mcp-server.js"
      ]
    }
  }
}
EOF
    
    # Create environment setup script
    cat > "$TOOLBOX_DIR/setup-env.sh" << 'EOF'
#!/bin/bash
# MCP Environment Setup
# Source this file to set up MCP environment variables

export MCP_GITHUB_USERNAME="Bo_theBig"
export MCP_GITHUB_EMAIL="botshelomokoka@gmail.com"
export MCP_GITHUB_DEFAULT_OWNER="Bo_theBig"
export MCP_GITHUB_DEFAULT_REPO="anya-core"

# Uncomment and set these if you have the API keys
# export MEM0_API_KEY="your_mem0_api_key_here"
# export GITHUB_TOKEN="your_github_token_here"
# export BRAVE_API_KEY="your_brave_api_key_here"

echo "MCP environment variables configured"
EOF
    
    chmod +x "$TOOLBOX_DIR/setup-env.sh"
    
    log_success "Configuration files created"
}

# Create usage documentation
create_documentation() {
    log_info "Creating documentation..."
    
    cat > "$TOOLBOX_DIR/README.md" << 'EOF'
# MCP Tools Toolbox

This directory contains the Model Context Protocol (MCP) tools configuration and utilities for Anya Core development.

## Quick Start

1. **Initialize the toolbox:**
   ```bash
   ./scripts/mcp/init-toolbox.sh
   ```

2. **Set up environment variables:**
   ```bash
   source mcp/toolbox/setup-env.sh
   ```

3. **Configure your IDE to use the MCP configuration:**
   - Copy `mcp/mcp.json` to your IDE's MCP configuration directory
   - Restart your IDE

## Available Tools

### Core Tools
- **mem0-knowledge**: Personal memory and knowledge management
- **github-tools**: GitHub repository management and automation
- **filesystem-tools**: File system operations
- **anya-bitcoin-tools**: Bitcoin development tools with Taproot support

### Optional Tools
- **postgres-tools**: PostgreSQL database operations
- **web-scraper**: Web scraping and browser automation
- **brave-search**: Web search capabilities
- **sequential-thinking**: Structured problem-solving framework

## Environment Variables

### Required
- `MEM0_API_KEY`: API key for mem0 service
- `GITHUB_TOKEN`: Personal access token for GitHub

### Optional
- `BRAVE_API_KEY`: API key for Brave Search
- `MCP_GITHUB_USERNAME`: GitHub username (default: Bo_theBig)
- `MCP_GITHUB_EMAIL`: GitHub email
- `MCP_GITHUB_DEFAULT_OWNER`: Default repository owner
- `MCP_GITHUB_DEFAULT_REPO`: Default repository name

## Configuration

The main configuration is in `mcp/mcp.json`. This file should be copied to your IDE's MCP configuration directory.

## Troubleshooting

1. **MCP servers not starting**: Check that all required tools are installed
2. **Authentication errors**: Verify environment variables are set correctly
3. **Permission errors**: Ensure scripts have execute permissions

## Maintenance

- Run `./scripts/mcp/init-toolbox.sh` to update server installations
- Check logs in `mcp/logs/` for debugging
- Update API keys as needed
EOF
    
    log_success "Documentation created"
}

# Main execution
main() {
    log_info "Initializing MCP Tools Toolbox for Anya Core..."
    
    create_directories
    check_prerequisites
    install_mcp_servers
    validate_environment
    test_mcp_servers
    create_config_files
    create_documentation
    
    log_success "MCP Tools Toolbox initialization completed!"
    log_info ""
    log_info "Next steps:"
    log_info "1. Set your API keys in environment variables or in setup-env.sh"
    log_info "2. Source the environment: source $TOOLBOX_DIR/setup-env.sh"
    log_info "3. Copy $MCP_CONFIG_DIR/mcp.json to your IDE's MCP configuration"
    log_info "4. Restart your IDE to load the new MCP tools"
    log_info ""
    log_info "Documentation available at: $TOOLBOX_DIR/README.md"
}

# Run main function
main "$@"
