#!/bin/bash
set -euo pipefail

source "$(dirname "$0")/common/utils.sh"

# System startup sequence
startup() {
    log_info "Starting Anya Core system..."

    # 1. Start core services
    systemctl start bitcoind
    wait_for_service bitcoind 30

    # 2. Start Layer 2 services
    systemctl start lightning
    systemctl start rgb-node
    wait_for_port 9735 30  # LN port
    wait_for_port 3000 30  # RGB port

    # 3. Start auxiliary services
    systemctl start web5-dwn
    systemctl start prometheus
    wait_for_port 8080 30  # Web5 port
    wait_for_port 9090 30  # Prometheus port

    # 4. Initialize ML components
    initialize_ml_system

    # 5. Run health checks
    check_system_health || {
        log_error "System health check failed"
        exit 1
    }

    log_success "Anya Core system started successfully"
}

startup
