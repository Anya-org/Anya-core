#!/bin/bash
# Simple script to start MCP GitHub server
# This ensures we have the proper environment

# Exit on error
set -e

echo "Starting MCP GitHub server..."
echo "============================="

# Check for required tools
if ! command -v npx &>/dev/null; then
    echo "Error: npx is not installed. Please install Node.js and npm."
    exit 1
fi

# Set required environment variables if not already set
if [ -z "${GITHUB_TOKEN:-}" ]; then
    echo "Warning: GITHUB_TOKEN is not set. Some features may not work."
    # For testing, we'll use a placeholder
    export GITHUB_TOKEN="placeholder"
fi

# Set other GitHub environment variables
export MCP_GITHUB_USERNAME="Bo_theBig"
export MCP_GITHUB_EMAIL="botshelomokoka@gmail.com"
export MCP_GITHUB_DEFAULT_OWNER="anya-org"
export MCP_GITHUB_DEFAULT_REPO="anya-core"

# Change to toolbox directory
cd "$(dirname "$0")/toolbox"

echo "Starting MCP GitHub server..."
npx @modelcontextprotocol/server-github
