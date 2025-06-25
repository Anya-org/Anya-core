#!/bin/bash
# DAO GitHub Contribution Tracker Wrapper
# This script runs the contribution tracker and integrates with the DAO system

# Source the GitHub auth helper
source "$(dirname "$0")/../../scripts/common/github-auth.sh"

# Default tracking period
TRACKING_PERIOD="30days"
FULL_HISTORY=""
COMMAND_ARGS=""

# Process arguments
for arg in "$@"; do
    case "$arg" in
    --period=*)
        TRACKING_PERIOD="${arg#*=}"
        ;;
    --full-history)
        FULL_HISTORY="--full-history"
        ;;
    --auto-run)
        COMMAND_ARGS="$COMMAND_ARGS --auto-run"
        ;;
    --yes-all)
        COMMAND_ARGS="$COMMAND_ARGS --yes-all"
        ;;
    esac
done

# Parse automation flags
eval $(parse_github_cli_args "$@")

echo "DAO GitHub Contribution Tracker"
echo "=============================="

# Check if GitHub CLI is available
if ! check_github_cli; then
    echo "Error: GitHub CLI (gh) is not installed. Please install it first."
    exit 1
fi

# Check if GitHub CLI is authenticated
if ! check_github_auth "$AUTO_RUN" "$YES_ALL"; then
    echo "Error: Not authenticated with GitHub CLI. Please run 'gh auth login'."
    exit 1
fi

# Get GitHub auth info
eval $(get_github_auth_info "$AUTO_RUN" "$YES_ALL")
echo "Authenticated as: $GITHUB_USERNAME"

# Setup MCP environment
setup_mcp_environment anya-org anya-core "$AUTO_RUN" "$YES_ALL"

# Create necessary directories to ensure they exist
mkdir -p "$(dirname "$0")/../data" "$(dirname "$0")/../config"

# Check for existing tracking metadata and force full history scan if first run
METADATA_FILE="$(dirname "$0")/../data/tracking_metadata.json"
if [ ! -f "$METADATA_FILE" ]; then
    echo "First run detected - will perform a full history scan."
    FULL_HISTORY="--full-history"
fi

# Always ensure we run with full history scan during first run
if [ ! -f "$(dirname "$0")/../data/contribution_history.json" ]; then
    echo "No history data found - will perform a full history scan."
    FULL_HISTORY="--full-history"
fi

# Run the Node.js contribution tracker with proper arguments
echo "Running contribution tracker..."
echo "Tracking period: $TRACKING_PERIOD"
echo "$([ -n "$FULL_HISTORY" ] && echo "Full history tracking: enabled" || echo "Full history tracking: disabled")"
echo "Auto-run: $([ "$AUTO_RUN" == "true" ] && echo "enabled" || echo "disabled")"
echo "Non-interactive mode: $([ "$YES_ALL" == "true" ] && echo "enabled" || echo "disabled")"

# Combine all arguments
ALL_ARGS="--period=$TRACKING_PERIOD $FULL_HISTORY $COMMAND_ARGS"
echo "Using arguments: $ALL_ARGS"

node "$(dirname "$0")/contribution-tracker.js" $ALL_ARGS

# Check exit status
if [ $? -eq 0 ]; then
    echo "Contribution tracking completed successfully!"
else
    echo "Error: Contribution tracking failed."
    exit 1
fi

# Inform about DAO integration
echo
echo "Next Steps for DAO Integration:"
echo "1. Review your contribution points in the DAO dashboard"
echo "2. Submit a proposal for contribution rewards"
echo "3. Connect your Web5 DID for decentralized identity"
echo "4. Set up Lightning wallet for automatic payouts"

# Print path to the contribution data
echo
echo "Contribution data stored at:"
echo "- Current period: dao/data/contribution_tracking.json"
echo "- Full history: dao/data/contribution_history.json"
echo
echo "Available tracking periods:"
echo "- all-time    (Full project history)"
echo "- year        (Last 365 days)"
echo "- quarter     (Last 91 days)"
echo "- month       (Last 30 days)"
echo "- week        (Last 7 days)"
echo
echo "Usage examples:"
echo "  ./track-contributions.sh --period=quarter"
echo "  ./track-contributions.sh --full-history"
echo "  ./track-contributions.sh --auto-run --yes-all"
