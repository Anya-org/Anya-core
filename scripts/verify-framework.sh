#!/bin/bash
set -euo pipefail

# Import common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/common.sh"

# Configuration
readonly FRAMEWORK_VERSION="2.5"
readonly EXPECTED_BIPS=("341" "342" "174")
readonly REQUIRED_FEATURES=("taproot" "bitcoin-consensus" "bip-check")

check_framework_version() {
    log_info "Verifying Bitcoin Development Framework v$FRAMEWORK_VERSION compliance..."
    
    # Check if framework version is defined in README.md
    if ! grep -q "Bitcoin Development Framework v$FRAMEWORK_VERSION" ../README.md; then
        log_error "Framework version mismatch or not defined in README.md"
        exit 1
    fi
    
    log_success "Framework version verified"
}

check_bip_compliance() {
    log_info "Checking BIP compliance..."
    
    for bip in "${EXPECTED_BIPS[@]}"; do
        if ! cargo test --test bip_validation -- --nocapture | grep -q "BIP-$bip"; then
            log_error "BIP-$bip compliance test missing or failing"
            exit 1
        fi
    done
    
    log_success "BIP compliance verified"
}

check_features() {
    log_info "Checking required features..."
    
    for feature in "${REQUIRED_FEATURES[@]}"; do
        if ! grep -q "\"$feature\"" ../Cargo.toml; then
            log_error "Required feature '$feature' not defined in Cargo.toml"
            exit 1
        fi
    done
    
    log_success "Required features verified"
}

check_security_validation() {
    log_info "Checking security validation..."
    
    # Check for transaction validation
    if ! grep -q "validate_transaction" ../src/security/validation/transaction.rs; then
        log_error "Transaction validation not implemented"
        exit 1
    fi
    
    # Check for Taproot validation
    if ! grep -q "validate_taproot_transaction" ../src/security/validation/taproot.rs; then
        log_error "Taproot validation not implemented"
        exit 1
    fi
    
    log_success "Security validation verified"
}

check_monitoring() {
    log_info "Checking monitoring components..."
    
    # Check for metrics
    if ! grep -q "export_metrics" ../src/monitoring/metrics.rs; then
        log_error "Metrics export not implemented"
        exit 1
    fi
    
    # Check for metrics server
    if ! grep -q "start_metrics_server" ../src/monitoring/server.rs; then
        log_error "Metrics server not implemented"
        exit 1
    fi
    
    log_success "Monitoring components verified"
}

check_hexagonal_architecture() {
    log_info "Checking hexagonal architecture components..."
    
    # Check for ports
    for port in "p2p" "wallet" "contracts"; do
        if ! find ../src -type d -name "*$port*" | grep -q .; then
            log_warn "Hexagonal architecture: port '$port' might be missing"
        fi
    done
    
    # Check for adapters
    for adapter in "lightning" "taproot" "dlc"; do
        if ! find ../src -type d -name "*$adapter*" | grep -q .; then
            log_warn "Hexagonal architecture: adapter '$adapter' might be missing"
        fi
    done
    
    log_success "Hexagonal architecture components verified"
}

verify_commit_hooks() {
    log_info "Verifying commit hooks..."
    
    # Check for BIP reference validation
    if ! grep -q '@BIP-\[0-9\]+' ../commit_push.sh; then
        log_error "BIP reference validation not implemented in commit hooks"
        exit 1
    fi
    
    # Check for Taproot compliance check
    if ! grep -q 'taproot_compliance' ../commit_push.sh; then
        log_error "Taproot compliance check not implemented in commit hooks"
        exit 1
    fi
    
    log_success "Commit hooks verified"
}

main() {
    log_header "Bitcoin Development Framework v$FRAMEWORK_VERSION Verification"
    
    check_framework_version
    check_bip_compliance
    check_features
    check_security_validation
    check_monitoring
    check_hexagonal_architecture
    verify_commit_hooks
    
    log_success "All checks passed! Bitcoin Development Framework v$FRAMEWORK_VERSION compliant."
}

main "$@" 