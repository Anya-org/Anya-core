#!/bin/bash
# GitHub CLI Authentication Helper
# Provides standardized GitHub authentication using GitHub CLI (gh)
# Adheres to Bitcoin Core principles of security and transparency

# Check if GitHub CLI is installed
check_github_cli() {
    if ! command -v gh &>/dev/null; then
        echo "GitHub CLI (gh) is not installed. Please install it first." >&2
        return 1
    fi
    return 0
}

# Check if GitHub CLI is authenticated
check_github_auth() {
    local auto_run="$1"
    local yes_all="$2"

    if ! gh auth status &>/dev/null; then
        echo "Not authenticated with GitHub CLI." >&2

        # If auto_run is enabled, attempt automatic login
        if [[ "$auto_run" == "true" ]]; then
            echo "Attempting automatic login with GitHub CLI..." >&2

            local login_args=()
            if [[ "$yes_all" == "true" ]]; then
                # Use Web mode with default options for automation
                login_args+=("--web")
            fi

            if gh auth login "${login_args[@]}" &>/dev/null; then
                echo "Successfully authenticated with GitHub CLI." >&2
                return 0
            else
                echo "Automatic login failed. Please run 'gh auth login' manually." >&2
                return 1
            fi
        else
            echo "Please run 'gh auth login'." >&2
            return 1
        fi
    fi
    return 0
}

# Get GitHub auth info - returns username and token
# Usage: eval $(get_github_auth_info [auto_run] [yes_all])
get_github_auth_info() {
    local auto_run="${1:-false}"
    local yes_all="${2:-false}"

    # Check requirements
    if ! check_github_cli; then
        return 1
    fi

    if ! check_github_auth "$auto_run" "$yes_all"; then
        return 1
    fi

    # Extract username
    local auth_status=$(gh auth status 2>&1)
    local username=$(echo "$auth_status" | grep "Logged in to github.com" | sed -E 's/.*account ([^ ]+).*/\1/')

    # Get token
    local token=$(gh auth token)

    # Get user email through the GitHub API
    local user_api=$(gh api user --jq '.email')
    local email="$user_api"

    # If no email is public, get from git config or email API
    if [ -z "$email" ] || [ "$email" == "null" ]; then
        # Try git config first
        email=$(git config --get user.email || echo "")

        # If still empty, try the email API
        if [ -z "$email" ]; then
            email=$(gh api user/emails --jq '.[0].email // empty')
        fi
    fi

    # Return values as environment variables
    echo "GITHUB_USERNAME=\"$username\""
    echo "GITHUB_TOKEN=\"$token\""
    echo "GITHUB_EMAIL=\"$email\""
}

# Setup MCP environment variables for GitHub
# Usage: setup_mcp_environment ["owner"] ["repo"] [auto_run] [yes_all]
setup_mcp_environment() {
    # Parse arguments
    local default_owner="${1:-anya-org}"
    local default_repo="${2:-anya-core}"
    local auto_run="false"
    local yes_all="false"

    # Check if auto_run and yes_all are provided
    if [[ "$3" == "true" || "$3" == "--auto-run" ]]; then
        auto_run="true"
        shift
    fi

    if [[ "$3" == "true" || "$3" == "--yes-all" ]]; then
        yes_all="true"
    fi

    # Get GitHub auth info with potential auto-run
    eval $(get_github_auth_info "$auto_run" "$yes_all") || return 1

    # Export MCP environment variables
    export MCP_GITHUB_USERNAME="$GITHUB_USERNAME"
    export MCP_GITHUB_EMAIL="$GITHUB_EMAIL"
    export MCP_GITHUB_DEFAULT_OWNER="$default_owner"
    export MCP_GITHUB_DEFAULT_REPO="$default_repo"
    export GITHUB_TOKEN="$GITHUB_TOKEN"

    echo "MCP environment variables set for GitHub user: $GITHUB_USERNAME"
    return 0
}

# Create MCP GitHub configuration file
# Usage: create_mcp_github_config <output_file> ["owner"] ["repo"] [auto_run] [yes_all]
create_mcp_github_config() {
    local output_file="$1"
    local default_owner="${2:-anya-org}"
    local default_repo="${3:-anya-core}"
    local auto_run="false"
    local yes_all="false"

    # Check if auto_run and yes_all are provided
    if [[ "$4" == "true" || "$4" == "--auto-run" ]]; then
        auto_run="true"
        shift
    fi

    if [[ "$4" == "true" || "$4" == "--yes-all" ]]; then
        yes_all="true"
    fi

    # Get GitHub auth info with potential auto-run
    eval $(get_github_auth_info "$auto_run" "$yes_all") || return 1

    # Create directory if it doesn't exist
    mkdir -p "$(dirname "$output_file")"

    # Create config file
    cat >"$output_file" <<EOF
{
  "github": {
    "username": "$GITHUB_USERNAME",
    "email": "$GITHUB_EMAIL",
    "auth_method": "github-cli",
    "default_owner": "$default_owner",
    "default_repo": "$default_repo"
  },
  "user_preferences": {
    "log_level": "INFO",
    "auto_update": true,
    "auto_run": $auto_run,
    "yes_all": $yes_all
  },
  "bitcoin_core": {
    "principles": ["decentralization", "security", "immutability", "transparency"],
    "version": "24.0"
  }
}
EOF

    echo "MCP GitHub configuration saved to: $output_file"
    return 0
}

# Parse common command line arguments
parse_github_cli_args() {
    local auto_run="false"
    local yes_all="false"

    for arg in "$@"; do
        case "$arg" in
        --auto-run)
            auto_run="true"
            ;;
        --yes-all)
            yes_all="true"
            ;;
        esac
    done

    echo "AUTO_RUN=\"$auto_run\""
    echo "YES_ALL=\"$yes_all\""
}

# Export functions if script is sourced
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    export -f check_github_cli
    export -f check_github_auth
    export -f get_github_auth_info
    export -f setup_mcp_environment
    export -f create_mcp_github_config
    export -f parse_github_cli_args
fi
