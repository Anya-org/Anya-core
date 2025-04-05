#!/bin/bash
# [AIR-3][AIS-3][BPC-3] Anya Core startup script
set -euo pipefail

source "$(dirname "$0")/common/utils.sh"

# System startup sequence with proper error handling
startup() {
    log_info "Starting Anya Core system..."

    # Start services with timeout handling
    TIMEOUT=30

    # 1. Core services
    systemctl is-active bitcoind || {
        systemctl start bitcoind
        wait_for_service bitcoind $TIMEOUT || {
            log_error "Failed to start bitcoind"
            exit 1
        }
    }

    # 2. Layer 2 services with proper checks
    for service in lightning rgb-node; do
        systemctl is-active $service || {
            systemctl start $service
            wait_for_service $service $TIMEOUT || {
                log_error "Failed to start $service"
                exit 1
            }
        }
    }

    # 3. Auxiliary services with port validation
    declare -A services=(
        ["web5-dwn"]=8080
        ["prometheus"]=9090
    )

    for service in "${!services[@]}"; do
        systemctl is-active $service || {
            systemctl start $service
            wait_for_port "${services[$service]}" $TIMEOUT || {
                log_error "Failed to start $service on port ${services[$service]}"
                exit 1
            }
        }
    }

    # 4. Initialize ML system with validation
    initialize_ml_system || {
        log_error "Failed to initialize ML system"
        exit 1
    }

    # 5. Run health checks with detailed output
    check_system_health || {
        log_error "System health check failed"
        exit 1
    }

    log_success "Anya Core system started successfully"
}

# Only run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    startup
fi
