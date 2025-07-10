#!/bin/bash
# Test Status Checker
# This script checks the status of background tests

LOG_DIR="$HOME/anya-core/test_logs"
STATUS_FILE="$LOG_DIR/last_test_status"
LOG_PATH_FILE="$LOG_DIR/last_test_log_path"

if [ ! -f "$STATUS_FILE" ]; then
    echo "No test has been run yet."
    exit 0
fi

STATUS=$(cat "$STATUS_FILE")
LOG_PATH=$(cat "$LOG_PATH_FILE" 2>/dev/null)

echo "Last test status: $STATUS"
echo "$LOG_PATH"

if [ "$STATUS" == "SUCCESS" ]; then
    echo "✅ All tests passed successfully"
else
    echo "❌ Tests failed - see log for details"
    if [ -n "$LOG_PATH" ]; then
        echo "Last 10 lines of error log:"
        LOG_FILE=$(echo "$LOG_PATH" | awk '{print $NF}')
        if [ -f "$LOG_FILE" ]; then
            grep -A 10 "FAILED" "$LOG_FILE" | head -n 10
        fi
    fi
fi

# Check if tests are currently running
if pgrep -f "cargo test --all-features" > /dev/null; then
    echo "⚠️ Tests are currently running..."
    ps -eo pid,ppid,%cpu,%mem,cmd | grep "cargo test" | grep -v grep
fi
