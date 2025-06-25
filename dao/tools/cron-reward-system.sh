#!/bin/bash
# DAO Reward System Cronjob
# This script is designed to be run by a cron service to automate the reward system

# Navigate to the project directory
cd "$(dirname "$0")/../.."

# Set environment variables
export NODE_ENV=${NODE_ENV:-production}
export HOME=${HOME:-/root}
export PATH=$HOME/.local/bin:$PATH

# Log file
LOG_FILE="./dao/logs/cron_reward_system.log"

# Ensure log directory exists
mkdir -p $(dirname "$LOG_FILE")

# Function to log messages
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$LOG_FILE"
}

# Start log entry
log "Starting DAO reward system cronjob"

# Check which operation to perform based on the day of month
DAY_OF_MONTH=$(date +%-d)

# Run different operations based on the day
if [ "$DAY_OF_MONTH" -eq 1 ]; then
    # Run the contribution tracker on the 1st of the month
    log "Running contribution tracker for the new month"
    node ./dao/tools/contribution-tracker.js --auto >>"$LOG_FILE" 2>&1

    # Submit to blockchain (in simulation mode by default for safety)
    log "Submitting contributions to blockchain"
    node ./dao/tools/on-chain-reward-bridge.js --auto >>"$LOG_FILE" 2>&1

elif [ "$DAY_OF_MONTH" -eq 5 ]; then
    # Process rewards on the 5th of the month
    log "Running reward system manager"
    node ./dao/tools/reward-system-manager.js --auto >>"$LOG_FILE" 2>&1

else
    # On other days, just verify the system
    log "Verifying reward system status"
    node ./dao/tools/reward-system-manager.js --check-status >>"$LOG_FILE" 2>&1
fi

# End log entry
log "DAO reward system cronjob completed"

# Exit successfully
exit 0
