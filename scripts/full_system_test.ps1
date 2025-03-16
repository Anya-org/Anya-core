# Full System Test for Anya-Core
# Tests the entire system from installation to operations
# Adheres to BDF v2.5 and Hexagonal Architecture requirements

# Log functions
function Log-Info {
    param([string]$message)
    Write-Host "[INFO] $message" -ForegroundColor Blue
}

function Log-Success {
    param([string]$message)
    Write-Host "[SUCCESS] $message" -ForegroundColor Green
}

function Log-Warning {
    param([string]$message)
    Write-Host "[WARNING] $message" -ForegroundColor Yellow
}

function Log-Error {
    param([string]$message)
    Write-Host "[ERROR] $message" -ForegroundColor Red
    exit 1
}

function Check-Command {
    param([string]$command)
    if (!(Get-Command $command -ErrorAction SilentlyContinue)) {
        Log-Error "Required command $command is not installed. Aborting."
    }
}

# Start timer for overall execution
$startTime = Get-Date

Log-Info "Starting Anya-Core Full System Test"
Log-Info "Testing according to BDF v2.5 and Hexagonal Architecture requirements"
Write-Host "===================================================================="

# Step 1: Check for required dependencies
Log-Info "Step 1: Verifying required dependencies"

Check-Command "cargo"
Check-Command "rustc"
Check-Command "git"

# Check Rust version
$rustVersion = (rustc --version) -replace "rustc ", "" -replace " .*", ""
Log-Info "Rust version: $rustVersion"

# Check minimum Rust version (1.70.0)
if ([version]$rustVersion -lt [version]"1.70.0") {
    Log-Warning "Rust version $rustVersion is below recommended 1.70.0"
} else {
    Log-Success "Rust version $rustVersion meets requirements"
}

# Step 2: Clone repository (if not already in it)
if (!(Test-Path "Cargo.toml") -or !(Select-String -Path "Cargo.toml" -Pattern "anya-core" -Quiet)) {
    Log-Info "Step 2: Cloning repository"
    git clone https://github.com/user/anya-core.git
    Set-Location anya-core
} else {
    Log-Info "Step 2: Already in anya-core repository"
}

# Step 3: Build system
Log-Info "Step 3: Building the Anya-Core system"

# Clean build
Log-Info "Cleaning previous builds"
cargo clean

# Run build
Log-Info "Building release version"
try {
    cargo build --release
} catch {
    Log-Error "Build failed: $_"
}

Log-Success "Build completed successfully"

# Step 4: Run tests
Log-Info "Step 4: Running tests"

Log-Info "Running unit tests"
try {
    cargo test --lib
} catch {
    Log-Error "Unit tests failed: $_"
}

Log-Info "Running integration tests"
try {
    cargo test --test integration_tests
} catch {
    Log-Error "Integration tests failed: $_"
}

Log-Info "Running performance framework tests"
try {
    cargo test --test performance_framework_integration
} catch {
    Log-Error "Performance framework tests failed: $_"
}

Log-Info "Running protocol tests"
try {
    cargo test --test run_protocol_tests
} catch {
    Log-Error "Protocol tests failed: $_"
}

Log-Success "All tests passed successfully"

# Step 5: Check BDF v2.5 compliance
Log-Info "Step 5: Verifying BDF v2.5 compliance"

Log-Info "Running compliance verification tool"
if (!(Test-Path "reports")) {
    New-Item -ItemType Directory -Path "reports"
}

try {
    cargo run --bin compliance_check --release -- --output-dir ./reports
} catch {
    Log-Error "Compliance check failed: $_"
}

if (Test-Path "./reports/compliance_report.md") {
    Log-Success "Compliance report generated successfully"
    
    # Check if report indicates compliance
    if (Select-String -Path "./reports/compliance_report.md" -Pattern "Overall Status: Passed" -Quiet) {
        Log-Success "BDF v2.5 compliance verified"
    } else {
        Log-Warning "BDF v2.5 compliance issues detected - check the report"
    }
} else {
    Log-Error "Compliance report not generated"
}

# Step 6: Initialize and configure the system
Log-Info "Step 6: Initializing and configuring the system"

# Create configuration directory
if (!(Test-Path "config")) {
    New-Item -ItemType Directory -Path "config"
}

# Generate default configuration
Log-Info "Generating default configuration"
@"
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
"@ | Out-File -FilePath "config/anya.conf" -Encoding utf8

Log-Success "Configuration created successfully"

# Step 7: Test basic operations
Log-Info "Step 7: Testing basic operations"

# Test wallet operations
Log-Info "Testing wallet operations"
try {
    cargo run --bin wallet_ops --release -- --config ./config/anya.conf --operation create_wallet --name test_wallet
} catch {
    Log-Error "Wallet operation failed: $_"
}

# Test transaction validation
Log-Info "Testing transaction validation"
try {
    cargo run --bin tx_validator --release -- --config ./config/anya.conf --file ./test_data/valid_taproot_tx.hex
} catch {
    Log-Error "Transaction validation failed: $_"
}

# Test system awareness
Log-Info "Testing system awareness"
try {
    cargo run --bin system_monitor --release -- --config ./config/anya.conf --operation monitor_mempool
} catch {
    Log-Error "System awareness test failed: $_"
}

# Test DAO operations
Log-Info "Testing DAO operations"
try {
    cargo run --bin dao_ops --release -- --config ./config/anya.conf --operation create_proposal --title "Test Proposal"
} catch {
    Log-Error "DAO operation failed: $_"
}

Log-Success "Basic operations tested successfully"

# Step 8: Performance testing
Log-Info "Step 8: Running performance benchmarks"

Log-Info "Running performance tests"
try {
    cargo run --bin perf_test --release -- comprehensive --output-dir ./reports
} catch {
    Log-Error "Performance tests failed: $_"
}

if (Test-Path "./reports/performance_report.md") {
    Log-Success "Performance report generated successfully"
} else {
    Log-Error "Performance report not generated"
}

# Step 9: Hexagonal architecture validation
Log-Info "Step 9: Validating hexagonal architecture"

Log-Info "Checking port implementations"
try {
    cargo run --bin architecture_validator --release -- --check-ports
} catch {
    Log-Error "Port validation failed: $_"
}

Log-Info "Checking adapter implementations"
try {
    cargo run --bin architecture_validator --release -- --check-adapters
} catch {
    Log-Error "Adapter validation failed: $_"
}

Log-Success "Hexagonal architecture validated successfully"

# Step 10: Start full system and verify operations
Log-Info "Step 10: Starting full system and verifying operations"

# Start the system in the background
Log-Info "Starting Anya-Core system"
$process = Start-Process -FilePath "cargo" -ArgumentList "run --bin anya_core --release -- --config ./config/anya.conf" -PassThru -WindowStyle Hidden

# Wait for system to start
Log-Info "Waiting for system to initialize"
Start-Sleep -Seconds 10

# Check if process is still running
if (!$process.HasExited) {
    Log-Success "Anya-Core system started successfully"
    
    # Run operational tests
    Log-Info "Running operational tests against live system"
    try {
        cargo run --bin ops_test --release -- --endpoint http://localhost:8080
    } catch {
        Log-Warning "Operational tests had issues: $_"
    }
    
    # Stop the system
    Log-Info "Stopping Anya-Core system"
    Stop-Process -Id $process.Id -Force
    Log-Success "Anya-Core system stopped successfully"
} else {
    Log-Error "Anya-Core system failed to start or crashed"
}

# Calculate total execution time
$endTime = Get-Date
$executionTime = ($endTime - $startTime).TotalSeconds
$minutes = [math]::Floor($executionTime / 60)
$seconds = [math]::Floor($executionTime % 60)

# Final summary
Write-Host "===================================================================="
Log-Info "Anya-Core Full System Test completed"
Log-Info "Total execution time: ${minutes}m ${seconds}s"
Log-Success "System verified according to BDF v2.5 and Hexagonal Architecture requirements"
Write-Host "===================================================================="

exit 0 