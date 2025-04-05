#!/bin/bash
# Environment configuration utilities
# [AIR-3][AIS-3][BPC-3]

set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/core.sh"

# Environment configuration
readonly CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/anya-core"
readonly CONFIG_FILE="$CONFIG_DIR/config"

# Initialize environment
init_environment() {
    mkdir -p "$CONFIG_DIR"
    
    # Configure user role if not set
    if [ -z "${USER_ROLE:-}" ]; then
        select_user_role
    fi

    # Configure environment if not set
    if [ -z "${ENVIRONMENT:-}" ]; then
        select_environment
    fi

    save_config
}

select_user_role() {
    echo "Select user role:"
    select USER_ROLE in "developer" "user" "owner"; do
        case $USER_ROLE in
            developer|user|owner) break ;;
            *) echo "Invalid selection" ;;
        esac
    done
}

select_environment() {
    case "${USER_ROLE:-}" in
        "user")     ENVIRONMENT="live" ;;
        "owner")    ENVIRONMENT="all" ;;
        *)
            select ENVIRONMENT in "testnet" "live"; do
                case $ENVIRONMENT in
                    testnet|live) break ;;
                    *) echo "Invalid selection" ;;
                esac
            done
            ;;
    esac
}

save_config() {
    cat > "$CONFIG_FILE" << EOF
USER_ROLE=${USER_ROLE:-developer}
ENVIRONMENT=${ENVIRONMENT:-testnet}
EOF
}

# Export functions
export -f init_environment
export -f select_user_role
export -f select_environment
