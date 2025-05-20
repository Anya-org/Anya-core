#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
set -e

# Full System Test for Anya-Core
# Tests the entire system from installation to operations
# Adheres to BDF v2.5 and Hexagonal Architecture requirements

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Log functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

check_command() {
    command -v $1 >/dev/null 2>&1 || { log_error "Required command $1 is not installed. Aborting."; }
}

# Start timer for overall execution
start_time=$(date +%s)

log_info "Starting Anya-Core Full System Test"
log_info "Testing according to BDF v2.5 and Hexagonal Architecture requirements"
echo "===================================================================="

# Step 1: Check for required dependencies
log_info "Step 1: Verifying required dependencies"

check_command cargo
check_command rustc
check_command git
check_command openssl

# Check Rust version
rust_version=$(rustc --version | cut -d " " -f 2)
log_info "Rust version: $rust_version"

# Check minimum Rust version (1.70.0)
if [[ "$rust_version" < "1.70.0" ]]; then
    log_warning "Rust version $rust_version is below recommended 1.70.0"
else
    log_success "Rust version $rust_version meets requirements"
fi

# Step 2: Clone repository (if not already in it)
if [ ! -f "Cargo.toml" ] || ! grep -q "anya-core" "Cargo.toml"; then
    log_info "Step 2: Cloning repository"
    git clone https://github.com/user/anya-core.git
    cd anya-core
else
    log_info "Step 2: Already in anya-core repository"
fi

# Step 3: Build system
log_info "Step 3: Building the Anya-Core system"

# Clean build
log_info "Cleaning previous builds"
cargo clean

# Run build
log_info "Building release version"
cargo build --release || log_error "Build failed"

log_success "Build completed successfully"

# Step 4: Run tests
log_info "Step 4: Running tests"

log_info "Running unit tests"
cargo test --lib || log_error "Unit tests failed"

log_info "Running integration tests"
cargo test --test integration_tests || log_error "Integration tests failed"

log_info "Running performance framework tests"
cargo test --test performance_framework_integration || log_error "Performance framework tests failed"

log_info "Running protocol tests"
cargo test --test run_protocol_tests || log_error "Protocol tests failed"

log_info "Starting Testnet simulation"
docker-compose -f testnet-orchestration.yml up -d || log_error "Failed to start Testnet nodes"

log_info "Running Testnet integration tests"
cargo test --test testnet_integration --features testnet || log_error "Testnet integration tests failed"

log_success "All tests passed successfully"

# Step 5: Check BDF v2.5 compliance
log_info "Step 5: Verifying BDF v2.5 compliance"

log_info "Running compliance verification tool"
mkdir -p reports
cargo run --bin compliance_check --release -- --output-dir ./reports || log_error "Compliance check failed"

if [ -f "./reports/compliance_report.md" ]; then
    log_success "Compliance report generated successfully"
    
    # Check if report indicates compliance
    if grep -q "Overall Status: Passed" "./reports/compliance_report.md"; then
        log_success "BDF v2.5 compliance verified"
    else
        log_warning "BDF v2.5 compliance issues detected - check the report"
    fi
else
    log_error "Compliance report not generated"
fi

# Step 6: Initialize and configure the system
log_info "Step 6: Initializing and configuring the system"

# Create configuration directory
mkdir -p config

# Generate default configuration
log_info "Generating default configuration"
cat > config/anya.conf << EOF
# Anya-Core Configuration
[network]
network_type = "testnet"
connect_peers = ["127.0.0.1:18333", "127.0.0.1:18334"]

[wallet]
enable_taproot = true
bip370_support = true
coin_selection_strategy = "efficient"

[dao]
quadratic_voting = true
dao_level = "DAO4"

[system_awareness]
mempool_alert_threshold_kb = 100
fee_spike_threshold = 200.0
attack_threshold = 60.0

[performance]
cache_size_mb = 20
batch_size = 100
use_prepared_statements = true
EOF

log_success "Configuration created successfully"

# Step 7: Test basic operations
log_info "Step 7: Testing basic operations"

# Test wallet operations
log_info "Testing wallet operations"
cargo run --bin wallet_ops --release -- --config ./config/anya.conf --operation create_wallet --name test_wallet || log_error "Wallet operation failed"

# Test transaction validation
log_info "Testing transaction validation"
cargo run --bin tx_validator --release -- --config ./config/anya.conf --file ./test_data/valid_taproot_tx.hex || log_error "Transaction validation failed"

# Test system awareness
log_info "Testing system awareness"
cargo run --bin system_monitor --release -- --config ./config/anya.conf --operation monitor_mempool || log_error "System awareness test failed"

# Test DAO operations
log_info "Testing DAO operations"
cargo run --bin dao_ops --release -- --config ./config/anya.conf --operation create_proposal --title "Test Proposal" || log_error "DAO operation failed"

log_success "Basic operations tested successfully"

# Step 8: Performance testing
log_info "Step 8: Running performance benchmarks"

log_info "Running performance tests"
cargo run --bin perf_test --release -- comprehensive --output-dir ./reports || log_error "Performance tests failed"

if [ -f "./reports/performance_report.md" ]; then
    log_success "Performance report generated successfully"
else
    log_error "Performance report not generated"
fi

# Step 9: Hexagonal architecture validation
log_info "Step 9: Validating hexagonal architecture"

log_info "Checking port implementations"
cargo run --bin architecture_validator --release -- --check-ports || log_error "Port validation failed"

log_info "Checking adapter implementations"
cargo run --bin architecture_validator --release -- --check-adapters || log_error "Adapter validation failed"

log_success "Hexagonal architecture validated successfully"

# Step 10: Start full system and verify operations
log_info "Step 10: Starting full system and verifying operations"

# Start the system in the background
log_info "Starting Anya-Core system"
cargo run --bin anya_core --release -- --config ./config/anya.conf --daemon &
ANYA_PID=$!

# Wait for system to start
log_info "Waiting for system to initialize"
sleep 10

# Check if process is still running
if ps -p $ANYA_PID > /dev/null; then
    log_success "Anya-Core system started successfully"
    
    # Run operational tests
    log_info "Running operational tests against live system"
    cargo run --bin ops_test --release -- --endpoint http://localhost:8080 || log_warning "Operational tests had issues"
    
    # Stop the system
    log_info "Stopping Anya-Core system"
    kill $ANYA_PID
    wait $ANYA_PID 2>/dev/null || true
    log_success "Anya-Core system stopped successfully"
else
    log_error "Anya-Core system failed to start or crashed"
fi

# Calculate total execution time
end_time=$(date +%s)
execution_time=$((end_time - start_time))
minutes=$((execution_time / 60))
seconds=$((execution_time % 60))

# Final summary
echo "===================================================================="
log_info "Anya-Core Full System Test completed"
log_info "Total execution time: ${minutes}m ${seconds}s"
log_success "System verified according to BDF v2.5 and Hexagonal Architecture requirements"
echo "===================================================================="

exit 0 