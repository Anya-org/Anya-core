#!/bin/bash
# Test script for GitHub CLI authentication module
# Tests the functionality of the github-auth.sh script

# Source the github-auth.sh script
source "$(dirname "$0")/../scripts/common/github-auth.sh"

# Parse command line arguments
eval $(parse_github_cli_args "$@")

# Print header
echo "Testing GitHub CLI Authentication Module (Bash)"
echo "============================================="
echo "Auto Run: $AUTO_RUN"
echo "Yes All: $YES_ALL"

# Test GitHub CLI availability
echo -n "Testing GitHub CLI availability... "
if check_github_cli; then
    echo "PASS"
else
    echo "FAIL"
    echo "GitHub CLI is not installed. Please install it first."
    exit 1
fi

# Test GitHub authentication (with potential auto-run)
echo -n "Testing GitHub authentication... "
if check_github_auth "$AUTO_RUN" "$YES_ALL"; then
    echo "PASS"
else
    echo "FAIL"
    echo "Not authenticated with GitHub CLI. Please run 'gh auth login'."
    exit 1
fi

# Test getting GitHub auth info
echo "Getting GitHub auth info..."
eval $(get_github_auth_info "$AUTO_RUN" "$YES_ALL")
echo "- Username: $GITHUB_USERNAME"
echo "- Email: ${GITHUB_EMAIL:-'(not available)'}"
echo "- Token: ${GITHUB_TOKEN:0:4}***${GITHUB_TOKEN:(-4)}"

# Test setting up MCP environment
echo "Setting up MCP environment..."
setup_mcp_environment anya-org anya-core "$AUTO_RUN" "$YES_ALL"
echo "- MCP_GITHUB_USERNAME: $MCP_GITHUB_USERNAME"
echo "- MCP_GITHUB_EMAIL: ${MCP_GITHUB_EMAIL:-'(not available)'}"
echo "- MCP_GITHUB_DEFAULT_OWNER: $MCP_GITHUB_DEFAULT_OWNER"
echo "- MCP_GITHUB_DEFAULT_REPO: $MCP_GITHUB_DEFAULT_REPO"

# Test creating MCP GitHub config
CONFIG_PATH="/tmp/mcp-github-config-test-sh.json"
echo "Creating MCP GitHub config at $CONFIG_PATH..."
create_mcp_github_config "$CONFIG_PATH" "anya-org" "anya-core" "$AUTO_RUN" "$YES_ALL"
echo "Config file created:"
cat "$CONFIG_PATH"

echo
echo "All tests completed successfully!"
