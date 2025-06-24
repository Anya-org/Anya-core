#!/bin/bash
# GitHub CLI Authentication CI/CD Integration Example
# This script demonstrates how to use the GitHub CLI authentication in CI/CD pipelines

# Source the github-auth.sh script
source "$(dirname "$0")/../scripts/common/github-auth.sh"

# Parse command line arguments with defaults for CI environment
AUTO_RUN=true
YES_ALL=true

# Parse any passed arguments to override defaults
eval $(parse_github_cli_args "$@")

# Print header
echo "GitHub CLI Authentication CI/CD Integration"
echo "=========================================="
echo "Auto Run: $AUTO_RUN"
echo "Yes All: $YES_ALL"

# Check GitHub CLI availability
if ! check_github_cli; then
    echo "Error: GitHub CLI is not installed. Installing now..."
    # Example of installing GitHub CLI in CI environment
    # This depends on the CI system you're using
    if command -v apt-get &>/dev/null; then
        curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list >/dev/null
        sudo apt update
        sudo apt install -y gh
    elif command -v brew &>/dev/null; then
        brew install gh
    else
        echo "Error: Could not install GitHub CLI automatically."
        exit 1
    fi
fi

# Check GitHub authentication and auto-login if needed
if ! check_github_auth "$AUTO_RUN" "$YES_ALL"; then
    echo "Error: Not authenticated with GitHub CLI and automatic login failed."
    echo "In a CI environment, you should configure authentication tokens."
    exit 1
fi

# Get GitHub auth info
eval $(get_github_auth_info "$AUTO_RUN" "$YES_ALL")
echo "Authenticated as: $GITHUB_USERNAME"

# Setup MCP environment
setup_mcp_environment "anya-org" "anya-core" "$AUTO_RUN" "$YES_ALL"

# Run additional CI tasks that require authentication
echo "Running CI tasks that require GitHub authentication..."

# Example: Run contribution tracker
if [ -f "$(dirname "$0")/../dao/tools/track-contributions.sh" ]; then
    echo "Running contribution tracker..."
    "$(dirname "$0")/../dao/tools/track-contributions.sh" --auto-run --yes-all
fi

echo "CI tasks completed successfully!"
