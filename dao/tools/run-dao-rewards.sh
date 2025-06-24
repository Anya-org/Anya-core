#!/bin/bash
# DAO Reward Engine Runner
# This script runs the DAO reward engine and handles token distribution

# Default parameters
SIMULATION=true
AUDIT_ONLY=false
FORCE=false
BLOCK=1

# Process arguments
for arg in "$@"; do
    case "$arg" in
    --mainnet)
        SIMULATION=false
        ;;
    --audit-only)
        AUDIT_ONLY=true
        ;;
    --force)
        FORCE=true
        ;;
    --block=*)
        BLOCK="${arg#*=}"
        ;;
    esac
done

# Build command arguments
COMMAND_ARGS=""

if [ "$SIMULATION" = true ]; then
    COMMAND_ARGS="$COMMAND_ARGS --simulate"
fi

if [ "$AUDIT_ONLY" = true ]; then
    COMMAND_ARGS="$COMMAND_ARGS --audit"
fi

if [ "$FORCE" = true ]; then
    COMMAND_ARGS="$COMMAND_ARGS --force"
fi

COMMAND_ARGS="$COMMAND_ARGS --block=$BLOCK"

# Print header
echo "Anya DAO Reward Engine"
echo "======================="
echo "Mode: $([ "$SIMULATION" = true ] && echo "SIMULATION" || echo "MAINNET")"
echo "Audit only: $([ "$AUDIT_ONLY" = true ] && echo "Yes" || echo "No")"
echo "Force execution: $([ "$FORCE" = true ] && echo "Yes" || echo "No")"
echo "Current block: $BLOCK"
echo

# Ensure data directories exist
mkdir -p "$(dirname "$0")/../data"

# Check that contribution history exists
if [ ! -f "$(dirname "$0")/../data/contribution_history.json" ]; then
    echo "Error: Contribution history not found. Run track-contributions.sh first."
    exit 1
fi

# Run the reward engine
echo "Running DAO reward engine with: $COMMAND_ARGS"
node "$(dirname "$0")/dao-reward-engine.js" $COMMAND_ARGS

# Check exit status
if [ $? -eq 0 ]; then
    echo
    echo "DAO reward processing completed successfully!"

    if [ "$SIMULATION" = true ]; then
        echo
        echo "This was a SIMULATION run. To execute on mainnet, use --mainnet flag."
    fi

    echo
    echo "Reward distribution data saved to: dao/data/reward_distribution.json"
else
    echo
    echo "Error: DAO reward processing failed."
    exit 1
fi

# Provide help information
echo
echo "Available options:"
echo "  --mainnet        Execute actual token transfers (default: simulation)"
echo "  --audit-only     Calculate rewards without distributing tokens"
echo "  --force          Force execution even if checks fail"
echo "  --block=NUMBER   Set the current block number (for reward calculations)"
echo
echo "Examples:"
echo "  ./run-dao-rewards.sh --audit-only"
echo "  ./run-dao-rewards.sh --block=105000"
echo "  ./run-dao-rewards.sh --mainnet"
