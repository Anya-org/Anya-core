#!/bin/bash
# Background Test Runner
# This script runs tests in the background with resource optimization

LOG_DIR="$HOME/anya-core/test_logs"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/test_run_$TIMESTAMP.log"

# Create log directory if it doesn't exist
mkdir -p "$LOG_DIR"

echo "Starting test run at $(date)" | tee "$LOG_FILE"
echo "Hardware info:" | tee -a "$LOG_FILE"
lscpu | grep -E 'CPU\(s\)|Core|Thread|Model name' | tee -a "$LOG_FILE"
free -h | tee -a "$LOG_FILE"

# Run with optimized parallelism based on available cores
# For this system with 2 CPUs, use j=1
echo "Running cargo test with optimized parallelism..." | tee -a "$LOG_FILE"
RUSTFLAGS="-C opt-level=1" cargo test --all-features -- --test-threads=1 2>&1 | tee -a "$LOG_FILE"

TEST_STATUS=$?

echo "Test run completed at $(date) with status: $TEST_STATUS" | tee -a "$LOG_FILE"

# Create status file for easy checking
if [ $TEST_STATUS -eq 0 ]; then
    echo "SUCCESS" > "$LOG_DIR/last_test_status"
else
    echo "FAILURE" > "$LOG_DIR/last_test_status"
fi

echo "Log file: $LOG_FILE" | tee -a "$LOG_DIR/last_test_log_path"
