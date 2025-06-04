# MCP Tools Quick Reference Card

**Last updated:** June 2, 2025  
**AI Generated:** BPC-3, AIR-3, AIS-3, AIT-3, PFM-3, SCL-3, RES-3, DID-3

## 🚀 Quick Start

```bash
# Initialize MCP toolbox
./scripts/mcp/init-toolbox.sh

# Set up environment
source mcp/toolbox/setup-env.sh

# Test MCP setup
./scripts/mcp/test-mcp-setup.sh

# Manage MCP tools
./scripts/mcp/manage-tools.sh status
```

## 🛠️ Available MCP Tools

| Tool | Purpose | Status |
|------|---------|--------|
| **mem0-knowledge** | Personal memory and context retention | ✅ Ready |
| **github-tools** | Repository management and automation | ✅ Ready |
| **filesystem-tools** | File operations and workspace management | ✅ Ready |
| **anya-bitcoin-tools** | Bitcoin development with Taproot support | ✅ Ready |
| **postgres-tools** | Database operations and analysis | ✅ Ready |
| **web-scraper** | Web automation and testing | ✅ Ready |
| **brave-search** | Privacy-focused web search | ✅ Ready |
| **sequential-thinking** | Structured problem-solving | ✅ Ready |
| **anya-dev-tools** | Custom Anya development server | ✅ Ready |

## 🔧 Anya Development Tools Commands

```bash
# Project analysis
analyze_project_structure --depth=3 --includeFiles=true

# Dependency management
check_dependencies --securityAudit=true --updateCheck=false

# Documentation validation
validate_documentation --checkLinks=true --checkTimestamps=true

# Test execution
run_tests --testType=unit --coverage=true --parallel=true

# Compliance reporting
generate_compliance_report --includeSecurity=true --includeBitcoin=true

# Build optimization
optimize_build --profile=release --analyze=true

# Resource monitoring
monitor_resources --duration=60 --interval=5

# Repository cleanup
cleanup_repository --cleanBuild=true --optimizeGit=true
```

## 📁 Directory Structure

```
mcp/
├── toolbox/
│   ├── servers/anya-dev-tools.js     # Custom development server
│   ├── mcp-tools-config.json        # Complete tools configuration
│   ├── setup-env.sh                 # Environment setup
│   └── README.md                     # Detailed documentation
├── mcp.json                          # Main MCP configuration
├── logs/                             # Operation logs
├── config/                           # Additional configurations
└── backups/                          # Configuration backups
```

## 🔑 Environment Variables

### Required
```bash
export MEM0_API_KEY="your_mem0_api_key"
export GITHUB_TOKEN="your_github_token"
```

### Optional
```bash
export BRAVE_API_KEY="your_brave_api_key"
export MCP_GITHUB_USERNAME="username"
export MCP_GITHUB_EMAIL="email"
```

## 📋 Management Commands

```bash
# Initialize complete toolbox
./scripts/mcp/manage-tools.sh init

# Start all MCP servers
./scripts/mcp/manage-tools.sh start all

# Check server status
./scripts/mcp/manage-tools.sh status

# Test specific server
./scripts/mcp/manage-tools.sh test github

# Update all servers
./scripts/mcp/manage-tools.sh update

# Clean up temporary files
./scripts/mcp/manage-tools.sh clean

# Perform health check
./scripts/mcp/manage-tools.sh health

# Backup configuration
./scripts/mcp/manage-tools.sh backup
```

## 🎯 IDE Integration

1. **Copy configuration:**
   ```bash
   cp mcp/mcp.json ~/.cursor/mcp_config.json
   # or your IDE's MCP configuration location
   ```

2. **Set environment variables in your shell profile:**
   ```bash
   source mcp/toolbox/setup-env.sh
   ```

3. **Restart your IDE** to load MCP tools

## 🔍 Troubleshooting

| Issue | Solution |
|-------|----------|
| MCP servers not starting | Check `./scripts/mcp/manage-tools.sh status` |
| Authentication errors | Verify API keys in environment variables |
| Permission errors | Run `chmod +x scripts/mcp/*.sh` |
| Missing dependencies | Run `./scripts/mcp/manage-tools.sh health` |

## 📚 Documentation

- **Complete Guide:** `mcp/toolbox/README.md`
- **Configuration:** `mcp/toolbox/mcp-tools-config.json`
- **Management:** `scripts/mcp/manage-tools.sh --help`
- **Setup:** `scripts/mcp/init-toolbox.sh`

---

**🚀 Ready for enhanced development with MCP tools!**
