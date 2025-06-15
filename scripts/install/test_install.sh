#!/bin/bash
# Anya Core Install Test Script
# Tests all install modes: root, rootless, dry-run, auto-run, yes-all

set -euo pipefail

LOGFILE="install_test_$(date +%Y%m%d-%H%M%S).log"

run_test() {
    local desc="$1"
    local cmd="$2"
    echo "[TEST] $desc"
    echo "[CMD] $cmd" | tee -a "$LOGFILE"
    if eval "$cmd" >> "$LOGFILE" 2>&1; then
        echo "[PASS] $desc" | tee -a "$LOGFILE"
    else
        echo "[FAIL] $desc" | tee -a "$LOGFILE"
    fi
    echo "" | tee -a "$LOGFILE"
}

# Dry run rootless
run_test "Dry run, rootless, auto-run, yes-all" "bash install.sh --rootless --auto-run --yes-all --dry-run"

# Real install rootless
run_test "Real install, rootless, auto-run, yes-all" "bash install.sh --rootless --auto-run --yes-all"

# Dry run system (if root)
if [ "$(id -u)" -eq 0 ]; then
    run_test "Dry run, system, auto-run, yes-all" "bash install.sh --auto-run --yes-all --dry-run"
    run_test "Real install, system, auto-run, yes-all" "bash install.sh --auto-run --yes-all"
fi

echo "[INFO] Install test complete. See $LOGFILE for details."
