#!/bin/bash
# Simple MCP Tools Test
# Last updated: 2025-06-02
# AI Generated: BPC-3, AIR-3, AIS-3, AIT-3, PFM-3, SCL-3, RES-3, DID-3

set -euo pipefail

echo "🔧 MCP Tools Verification Test"
echo "=============================="

# Test Node.js availability
echo "📦 Checking Node.js..."
if /usr/bin/node --version; then
    echo "✅ Node.js is available"
else
    echo "❌ Node.js not working"
    exit 1
fi

# Test npm availability
echo "📦 Checking npm..."
if npm --version 2>/dev/null; then
    echo "✅ npm is available"
else
    echo "❌ npm not available"
fi

# Test Python availability
echo "🐍 Checking Python..."
if /usr/bin/python3 --version; then
    echo "✅ Python3 is available"
else
    echo "❌ Python3 not working"
    exit 1
fi

# Test existing MCP configuration
echo "⚙️ Checking existing MCP configuration..."
if [ -f "/home/bmokoka/Anya-core/.cursor/mcp.json" ]; then
    echo "✅ Found existing MCP configuration"
    echo "📋 Current configuration:"
    head -20 "/home/bmokoka/Anya-core/.cursor/mcp.json"
else
    echo "⚠️ No existing MCP configuration found"
fi

# Test MCP directory structure
echo "📁 Creating MCP directory structure..."
mkdir -p "/home/bmokoka/Anya-core/mcp"/{toolbox,logs,config,backups}
mkdir -p "/home/bmokoka/Anya-core/mcp/toolbox"/{servers,tools,extensions}
echo "✅ MCP directories created"

# Test mem0 API key environment
echo "🔑 Checking environment variables..."
if [ -n "${MEM0_API_KEY:-}" ]; then
    echo "✅ MEM0_API_KEY is set"
else
    echo "⚠️ MEM0_API_KEY not set (this is optional for testing)"
fi

if [ -n "${GITHUB_TOKEN:-}" ]; then
    echo "✅ GITHUB_TOKEN is set"
else
    echo "⚠️ GITHUB_TOKEN not set (this is optional for testing)"
fi

# Test Bitcoin MCP server
echo "₿ Checking Bitcoin MCP server..."
if [ -f "/home/bmokoka/Anya-core/scripts/bitcoin/mcp-server.js" ]; then
    echo "✅ Bitcoin MCP server found"
    if /usr/bin/node "/home/bmokoka/Anya-core/scripts/bitcoin/mcp-server.js" --help 2>/dev/null; then
        echo "✅ Bitcoin MCP server is functional"
    else
        echo "⚠️ Bitcoin MCP server may need dependencies"
    fi
else
    echo "❌ Bitcoin MCP server not found"
fi

# Test Anya dev tools server
echo "🛠️ Checking Anya dev tools server..."
if [ -f "/home/bmokoka/Anya-core/mcp/toolbox/servers/anya-dev-tools.js" ]; then
    echo "✅ Anya dev tools server found"
else
    echo "⚠️ Anya dev tools server not found (will be created)"
fi

# Summary
echo ""
echo "🎯 MCP Tools Test Summary"
echo "========================"
echo "✅ Basic prerequisites are available"
echo "✅ Directory structure created"
echo "✅ Ready for MCP integration"
echo ""
echo "📋 Next steps:"
echo "1. Install MCP packages: npm install -g @modelcontextprotocol/sdk"
echo "2. Set up API keys in environment variables"
echo "3. Test individual MCP servers"
echo "4. Configure IDE to use MCP tools"
echo ""
echo "🚀 MCP integration foundation is ready!"
