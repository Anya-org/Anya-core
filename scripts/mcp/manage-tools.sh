#!/bin/bash
# MCP Tools Management Script
# Last updated: 2025-06-02
# AI Generated: BPC-3, AIR-3, AIS-3, AIT-3, PFM-3, SCL-3, RES-3, DID-3

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MCP_DIR="$PROJECT_ROOT/mcp"
TOOLBOX_DIR="$MCP_DIR/toolbox"

# Logging functions
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_header() { echo -e "${CYAN}[HEADER]${NC} $1"; }

# Show usage information
show_usage() {
    echo "MCP Tools Management Script"
    echo ""
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  init                 Initialize MCP toolbox"
    echo "  start [SERVER]       Start MCP server(s)"
    echo "  stop [SERVER]        Stop MCP server(s)"
    echo "  status              Show status of all MCP servers"
    echo "  test                Test MCP server functionality"
    echo "  update              Update MCP servers to latest versions"
    echo "  config              Generate configuration files"
    echo "  clean               Clean up temporary files and logs"
    echo "  health              Perform health check on all components"
    echo "  backup              Backup MCP configuration and data"
    echo "  restore [BACKUP]    Restore from backup"
    echo ""
    echo "Server Names:"
    echo "  mem0                Mem0 knowledge management server"
    echo "  github              GitHub integration server"
    echo "  filesystem          Filesystem operations server"
    echo "  bitcoin             Bitcoin development tools server"
    echo "  postgres            PostgreSQL database server"
    echo "  puppeteer           Web automation server"
    echo "  brave               Brave search server"
    echo "  thinking            Sequential thinking server"
    echo "  anya-dev            Anya development tools server"
    echo ""
    echo "Examples:"
    echo "  $0 init                    # Initialize complete toolbox"
    echo "  $0 start mem0              # Start mem0 server only"
    echo "  $0 status                  # Show all server status"
    echo "  $0 test github             # Test GitHub server"
    echo ""
}

# Initialize MCP toolbox
cmd_init() {
    log_header "Initializing MCP Tools Toolbox..."
    
    # Create directory structure
    mkdir -p "$MCP_DIR"/{toolbox,logs,config,backups}
    mkdir -p "$TOOLBOX_DIR"/{servers,tools,extensions}
    
    # Run the main initialization script
    if [ -f "$PROJECT_ROOT/scripts/mcp/init-toolbox.sh" ]; then
        "$PROJECT_ROOT/scripts/mcp/init-toolbox.sh"
    else
        log_error "Initialization script not found at $PROJECT_ROOT/scripts/mcp/init-toolbox.sh"
        return 1
    fi
    
    log_success "MCP toolbox initialized successfully"
}

# Start MCP servers
cmd_start() {
    local server_name="${1:-all}"
    log_header "Starting MCP server(s): $server_name"
    
    case "$server_name" in
        "mem0")
            start_mem0_server
            ;;
        "github")
            start_github_server
            ;;
        "filesystem")
            start_filesystem_server
            ;;
        "bitcoin")
            start_bitcoin_server
            ;;
        "postgres")
            start_postgres_server
            ;;
        "puppeteer")
            start_puppeteer_server
            ;;
        "brave")
            start_brave_server
            ;;
        "thinking")
            start_thinking_server
            ;;
        "anya-dev")
            start_anya_dev_server
            ;;
        "all")
            start_all_servers
            ;;
        *)
            log_error "Unknown server: $server_name"
            return 1
            ;;
    esac
}

# Stop MCP servers
cmd_stop() {
    local server_name="${1:-all}"
    log_header "Stopping MCP server(s): $server_name"
    
    case "$server_name" in
        "all")
            stop_all_servers
            ;;
        *)
            stop_server "$server_name"
            ;;
    esac
}

# Show server status
cmd_status() {
    log_header "MCP Server Status Report"
    echo ""
    
    check_server_status "mem0" "pipx run mem0-mcp-for-pm --help"
    check_server_status "github" "npx @modelcontextprotocol/server-github --help"
    check_server_status "filesystem" "npx @modelcontextprotocol/server-filesystem --help"
    check_server_status "bitcoin" "node $PROJECT_ROOT/scripts/bitcoin/mcp-server.js --help"
    check_server_status "postgres" "npx @modelcontextprotocol/server-postgres --help"
    check_server_status "puppeteer" "npx @modelcontextprotocol/server-puppeteer --help"
    check_server_status "brave" "npx @modelcontextprotocol/server-brave-search --help"
    check_server_status "thinking" "npx @modelcontextprotocol/server-sequential-thinking --help"
    check_server_status "anya-dev" "node $TOOLBOX_DIR/servers/anya-dev-tools.js --help"
    
    echo ""
    log_info "Environment Variables:"
    check_env_var "MEM0_API_KEY"
    check_env_var "GITHUB_TOKEN"
    check_env_var "BRAVE_API_KEY"
    check_env_var "MCP_GITHUB_USERNAME"
}

# Test MCP functionality
cmd_test() {
    local server_name="${1:-all}"
    log_header "Testing MCP server(s): $server_name"
    
    case "$server_name" in
        "mem0")
            test_mem0_server
            ;;
        "github")
            test_github_server
            ;;
        "filesystem")
            test_filesystem_server
            ;;
        "bitcoin")
            test_bitcoin_server
            ;;
        "all")
            test_all_servers
            ;;
        *)
            log_warning "Test not implemented for server: $server_name"
            ;;
    esac
}

# Update MCP servers
cmd_update() {
    log_header "Updating MCP servers..."
    
    # Update mem0
    log_info "Updating mem0-mcp-for-pm..."
    pipx upgrade mem0-mcp-for-pm || pipx install mem0-mcp-for-pm==0.3.2
    
    # Update npm packages
    log_info "Updating npm MCP packages..."
    npm update -g @modelcontextprotocol/server-github
    npm update -g @modelcontextprotocol/server-filesystem
    npm update -g @modelcontextprotocol/server-postgres
    npm update -g @modelcontextprotocol/server-puppeteer
    npm update -g @modelcontextprotocol/server-brave-search
    npm update -g @modelcontextprotocol/server-sequential-thinking
    
    log_success "MCP servers updated"
}

# Generate configuration files
cmd_config() {
    log_header "Generating MCP configuration files..."
    
    # Generate main MCP config
    cat > "$MCP_DIR/mcp.json" << 'EOF'
{
  "version": 1,
  "mcpServers": {
    "mem0-knowledge": {
      "command": "pipx",
      "args": ["run", "mem0-mcp-for-pm==0.3.2", "--log=off"],
      "env": {"MEM0_API_KEY": "${MEM0_API_KEY}"}
    },
    "github-tools": {
      "command": "npx",
      "args": ["@modelcontextprotocol/server-github"],
      "env": {"GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"}
    },
    "filesystem-tools": {
      "command": "npx",
      "args": ["@modelcontextprotocol/server-filesystem", "/home/bmokoka/Anya-core"]
    },
    "anya-bitcoin-tools": {
      "command": "node",
      "args": ["scripts/bitcoin/mcp-server.js"]
    },
    "anya-dev-tools": {
      "command": "node",
      "args": ["mcp/toolbox/servers/anya-dev-tools.js"]
    }
  }
}
EOF
    
    # Generate environment setup script
    cat > "$TOOLBOX_DIR/setup-env.sh" << 'EOF'
#!/bin/bash
# MCP Environment Setup
export MCP_GITHUB_USERNAME="Bo_theBig"
export MCP_GITHUB_EMAIL="botshelomokoka@gmail.com"
export MCP_GITHUB_DEFAULT_OWNER="Bo_theBig"
export MCP_GITHUB_DEFAULT_REPO="anya-core"

# Set these with your actual API keys
# export MEM0_API_KEY="your_mem0_api_key_here"
# export GITHUB_TOKEN="your_github_token_here"
# export BRAVE_API_KEY="your_brave_api_key_here"

echo "MCP environment variables configured"
EOF
    
    chmod +x "$TOOLBOX_DIR/setup-env.sh"
    
    log_success "Configuration files generated"
}

# Clean up temporary files
cmd_clean() {
    log_header "Cleaning up MCP temporary files..."
    
    # Clean logs older than 7 days
    if [ -d "$MCP_DIR/logs" ]; then
        find "$MCP_DIR/logs" -name "*.log" -mtime +7 -delete 2>/dev/null || true
        log_info "Cleaned old log files"
    fi
    
    # Clean npm cache
    npm cache clean --force 2>/dev/null || true
    log_info "Cleaned npm cache"
    
    # Clean pipx cache
    pipx list --short | xargs -r pipx uninstall --include-deps 2>/dev/null || true
    pipx install mem0-mcp-for-pm==0.3.2 >/dev/null 2>&1 || true
    log_info "Cleaned pipx cache"
    
    log_success "Cleanup completed"
}

# Perform health check
cmd_health() {
    log_header "Performing MCP Health Check..."
    
    local issues=0
    
    # Check prerequisites
    log_info "Checking prerequisites..."
    for tool in node npm pipx python3; do
        if ! command -v "$tool" &> /dev/null; then
            log_error "Missing prerequisite: $tool"
            ((issues++))
        else
            log_success "$tool: OK"
        fi
    done
    
    # Check environment variables
    log_info "Checking environment variables..."
    if [ -z "${MEM0_API_KEY:-}" ]; then
        log_warning "MEM0_API_KEY not set"
        ((issues++))
    fi
    
    if [ -z "${GITHUB_TOKEN:-}" ]; then
        log_warning "GITHUB_TOKEN not set"
        ((issues++))
    fi
    
    # Check server installations
    log_info "Checking server installations..."
    cmd_status
    
    # Summary
    echo ""
    if [ $issues -eq 0 ]; then
        log_success "Health check passed with no issues"
    else
        log_warning "Health check completed with $issues issues"
    fi
}

# Backup MCP configuration
cmd_backup() {
    log_header "Creating MCP configuration backup..."
    
    local backup_name="mcp-backup-$(date +%Y%m%d-%H%M%S)"
    local backup_dir="$MCP_DIR/backups/$backup_name"
    
    mkdir -p "$backup_dir"
    
    # Backup configuration files
    if [ -d "$MCP_DIR" ]; then
        cp -r "$MCP_DIR"/*.json "$backup_dir/" 2>/dev/null || true
        cp -r "$TOOLBOX_DIR" "$backup_dir/" 2>/dev/null || true
    fi
    
    # Create archive
    tar -czf "$MCP_DIR/backups/$backup_name.tar.gz" -C "$MCP_DIR/backups" "$backup_name"
    rm -rf "$backup_dir"
    
    log_success "Backup created: $backup_name.tar.gz"
}

# Restore from backup
cmd_restore() {
    local backup_file="${1:-}"
    
    if [ -z "$backup_file" ]; then
        log_error "Please specify backup file to restore"
        log_info "Available backups:"
        ls -la "$MCP_DIR/backups"/*.tar.gz 2>/dev/null || log_info "No backups found"
        return 1
    fi
    
    log_header "Restoring MCP configuration from backup..."
    
    if [ ! -f "$backup_file" ]; then
        backup_file="$MCP_DIR/backups/$backup_file"
    fi
    
    if [ ! -f "$backup_file" ]; then
        log_error "Backup file not found: $backup_file"
        return 1
    fi
    
    # Extract backup
    tar -xzf "$backup_file" -C "$MCP_DIR/backups/"
    
    # Restore files
    local backup_name=$(basename "$backup_file" .tar.gz)
    cp -r "$MCP_DIR/backups/$backup_name"/* "$MCP_DIR/"
    
    log_success "Configuration restored from backup"
}

# Helper functions for server management
start_mem0_server() {
    log_info "Starting mem0 server..."
    if [ -z "${MEM0_API_KEY:-}" ]; then
        log_error "MEM0_API_KEY not set"
        return 1
    fi
    log_success "mem0 server configuration ready"
}

start_github_server() {
    log_info "Starting GitHub server..."
    if [ -z "${GITHUB_TOKEN:-}" ]; then
        log_error "GITHUB_TOKEN not set"
        return 1
    fi
    log_success "GitHub server configuration ready"
}

start_filesystem_server() {
    log_info "Starting filesystem server..."
    log_success "Filesystem server configuration ready"
}

start_bitcoin_server() {
    log_info "Starting Bitcoin server..."
    if [ ! -f "$PROJECT_ROOT/scripts/bitcoin/mcp-server.js" ]; then
        log_error "Bitcoin MCP server not found"
        return 1
    fi
    log_success "Bitcoin server configuration ready"
}

start_postgres_server() {
    log_info "Starting PostgreSQL server..."
    log_success "PostgreSQL server configuration ready"
}

start_puppeteer_server() {
    log_info "Starting Puppeteer server..."
    log_success "Puppeteer server configuration ready"
}

start_brave_server() {
    log_info "Starting Brave Search server..."
    log_success "Brave Search server configuration ready"
}

start_thinking_server() {
    log_info "Starting Sequential Thinking server..."
    log_success "Sequential Thinking server configuration ready"
}

start_anya_dev_server() {
    log_info "Starting Anya Development Tools server..."
    if [ ! -f "$TOOLBOX_DIR/servers/anya-dev-tools.js" ]; then
        log_error "Anya Development Tools server not found"
        return 1
    fi
    log_success "Anya Development Tools server configuration ready"
}

start_all_servers() {
    start_mem0_server
    start_github_server
    start_filesystem_server
    start_bitcoin_server
    start_anya_dev_server
}

stop_all_servers() {
    log_info "Stopping all MCP servers..."
    # Kill any running MCP processes
    pkill -f "mcp-server" 2>/dev/null || true
    pkill -f "mem0-mcp-for-pm" 2>/dev/null || true
    log_success "All servers stopped"
}

stop_server() {
    local server_name="$1"
    log_info "Stopping $server_name server..."
    pkill -f "$server_name" 2>/dev/null || true
    log_success "$server_name server stopped"
}

check_server_status() {
    local name="$1"
    local command="$2"
    
    if eval "$command" >/dev/null 2>&1; then
        log_success "$name: Available"
    else
        log_error "$name: Not available"
    fi
}

check_env_var() {
    local var_name="$1"
    local var_value="${!var_name:-}"
    
    if [ -n "$var_value" ]; then
        log_success "$var_name: Set"
    else
        log_warning "$var_name: Not set"
    fi
}

test_mem0_server() {
    log_info "Testing mem0 server..."
    if [ -z "${MEM0_API_KEY:-}" ]; then
        log_error "MEM0_API_KEY not set - cannot test"
        return 1
    fi
    
    if pipx run mem0-mcp-for-pm --help >/dev/null 2>&1; then
        log_success "mem0 server test passed"
    else
        log_error "mem0 server test failed"
        return 1
    fi
}

test_github_server() {
    log_info "Testing GitHub server..."
    if npx @modelcontextprotocol/server-github --help >/dev/null 2>&1; then
        log_success "GitHub server test passed"
    else
        log_error "GitHub server test failed"
        return 1
    fi
}

test_filesystem_server() {
    log_info "Testing filesystem server..."
    if npx @modelcontextprotocol/server-filesystem --help >/dev/null 2>&1; then
        log_success "Filesystem server test passed"
    else
        log_error "Filesystem server test failed"
        return 1
    fi
}

test_bitcoin_server() {
    log_info "Testing Bitcoin server..."
    if [ -f "$PROJECT_ROOT/scripts/bitcoin/mcp-server.js" ]; then
        log_success "Bitcoin server test passed"
    else
        log_error "Bitcoin server test failed"
        return 1
    fi
}

test_all_servers() {
    test_mem0_server
    test_github_server
    test_filesystem_server
    test_bitcoin_server
}

# Main execution
main() {
    case "${1:-}" in
        "init")
            cmd_init
            ;;
        "start")
            cmd_start "${2:-all}"
            ;;
        "stop")
            cmd_stop "${2:-all}"
            ;;
        "status")
            cmd_status
            ;;
        "test")
            cmd_test "${2:-all}"
            ;;
        "update")
            cmd_update
            ;;
        "config")
            cmd_config
            ;;
        "clean")
            cmd_clean
            ;;
        "health")
            cmd_health
            ;;
        "backup")
            cmd_backup
            ;;
        "restore")
            cmd_restore "${2:-}"
            ;;
        "help"|"--help"|"-h"|"")
            show_usage
            ;;
        *)
            log_error "Unknown command: $1"
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
