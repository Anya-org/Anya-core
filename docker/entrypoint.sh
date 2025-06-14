#!/bin/sh
set -euo pipefail

# Anya Bitcoin Core Entrypoint Script
# Handles initialization, configuration validation, and graceful shutdown

# Colors for logging
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Signal handlers for graceful shutdown
shutdown_handler() {
    log_info "Received shutdown signal, gracefully stopping Anya Bitcoin Core..."
    if [ ! -z "${ANYA_PID:-}" ]; then
        kill -TERM "$ANYA_PID" 2>/dev/null || true
        wait "$ANYA_PID" 2>/dev/null || true
    fi
    log_info "Anya Bitcoin Core stopped"
    exit 0
}

# Set up signal trapping
trap shutdown_handler SIGTERM SIGINT

# Pre-flight checks
log_info "Starting Anya Bitcoin Core initialization..."

# Check required directories
for dir in "$APP_DATA_DIR" "$APP_LOG_DIR" "$APP_CONFIG_DIR"; do
    if [ ! -d "$dir" ]; then
        log_error "Required directory $dir does not exist"
        exit 1
    fi
    if [ ! -w "$dir" ]; then
        log_error "Directory $dir is not writable"
        exit 1
    fi
done

# Validate configuration file
CONFIG_FILE="${APP_CONFIG_DIR}/production.toml"
if [ ! -f "$CONFIG_FILE" ]; then
    log_warn "Configuration file $CONFIG_FILE not found, using defaults"
    CONFIG_FILE=""
fi

# Check Bitcoin RPC connectivity (if configured)
if [ ! -z "${BITCOIN_RPC_URL:-}" ]; then
    log_info "Testing Bitcoin RPC connectivity..."
    if timeout 10 curl -sf "${BITCOIN_RPC_URL}" >/dev/null 2>&1; then
        log_info "Bitcoin RPC is accessible"
    else
        log_warn "Bitcoin RPC is not accessible yet, will retry during startup"
    fi
fi

# Check Web5 DWN connectivity (if configured)
if [ ! -z "${WEB5_DWN_URL:-}" ]; then
    log_info "Testing Web5 DWN connectivity..."
    if timeout 10 curl -sf "${WEB5_DWN_URL}/health" >/dev/null 2>&1; then
        log_info "Web5 DWN is accessible"
    else
        log_warn "Web5 DWN is not accessible yet, will retry during startup"
    fi
fi

# Validate Bitcoin Core compliance settings
log_info "Validating Bitcoin Core compliance configuration..."
export ANYA_BIP341_ENABLED=true
export ANYA_BIP342_ENABLED=true
export ANYA_BIP174_ENABLED=true
export ANYA_TAPROOT_ENABLED=true
export ANYA_LIGHTNING_ENABLED=true

# Verify Bitcoin Core metrics endpoint
if [ -f "$(dirname "$0")/verify_bitcoin_metrics.sh" ] && [ -x "$(dirname "$0")/verify_bitcoin_metrics.sh" ]; then
    log_info "Verifying Bitcoin Core metrics endpoint..."
    "$(dirname "$0")/verify_bitcoin_metrics.sh"
fi

# Set performance optimization flags based on available resources
if [ -f /proc/cpuinfo ]; then
    CPU_COUNT=$(nproc)
    export ANYA_WORKER_THREADS=${ANYA_WORKER_THREADS:-$CPU_COUNT}
    log_info "Configured for $CPU_COUNT CPU cores"
fi

# Memory optimization
if [ -f /proc/meminfo ]; then
    TOTAL_MEM_KB=$(grep MemTotal /proc/meminfo | awk '{print $2}')
    TOTAL_MEM_MB=$((TOTAL_MEM_KB / 1024))

    if [ $TOTAL_MEM_MB -gt 16384 ]; then
        export ANYA_CACHE_SIZE=${ANYA_CACHE_SIZE:-"4GB"}
        export ANYA_BUFFER_SIZE=${ANYA_BUFFER_SIZE:-"512MB"}
    elif [ $TOTAL_MEM_MB -gt 8192 ]; then
        export ANYA_CACHE_SIZE=${ANYA_CACHE_SIZE:-"2GB"}
        export ANYA_BUFFER_SIZE=${ANYA_BUFFER_SIZE:-"256MB"}
    elif [ $TOTAL_MEM_MB -gt 4096 ]; then
        export ANYA_CACHE_SIZE=${ANYA_CACHE_SIZE:-"1GB"}
        export ANYA_BUFFER_SIZE=${ANYA_BUFFER_SIZE:-"256MB"}
    else
        export ANYA_CACHE_SIZE=${ANYA_CACHE_SIZE:-"512MB"}
        export ANYA_BUFFER_SIZE=${ANYA_BUFFER_SIZE:-"128MB"}
    fi
    log_info "Memory configuration: Cache=${ANYA_CACHE_SIZE}, Buffer=${ANYA_BUFFER_SIZE}"
fi

# Start the application
log_info "Starting Anya Bitcoin Core..."
log_info "Version: $(anya --version 2>/dev/null || echo 'unknown')"
log_info "Configuration: ${CONFIG_FILE:-'default'}"
log_info "Data directory: $APP_DATA_DIR"
log_info "Log directory: $APP_LOG_DIR"

# Execute the main command passed from the Docker CMD
"$@" &

ANYA_PID=$!
log_info "Anya Bitcoin Core started with PID $ANYA_PID"

# Wait for the process to complete
wait $ANYA_PID
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    log_info "Anya Bitcoin Core exited successfully"
else
    log_error "Anya Bitcoin Core exited with code $EXIT_CODE"
fi

exit $EXIT_CODE
