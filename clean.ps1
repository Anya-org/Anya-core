# Improved unified cleanup script with error handling
$ErrorActionPreference = "Continue"  # Don't stop on errors

Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "--- Anya DAO - Enhanced Cleanup Script                        ---" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan

# Initialize error tracking
$errorCount = 0
$warningCount = 0

# Function to log and track errors
function Write-Status {
    param(
        [string]$Message,
        [string]$Type = "INFO"  # INFO, SUCCESS, WARNING, ERROR
    )
    
    $color = switch($Type) {
        "SUCCESS" { "Green" }
        "WARNING" { "Yellow" }
        "ERROR" { "Red" }
        default { "White" }
    }
    
    Write-Host $Message -ForegroundColor $color
    
    if ($Type -eq "ERROR") {
        $script:errorCount++
    }
    if ($Type -eq "WARNING") {
        $script:warningCount++
    }
}

# Clean build artifacts
Write-Status "Cleaning build artifacts..." "INFO"
try {
    if (Test-Path "target") {
        Remove-Item -Recurse -Force target -ErrorAction Stop
        Write-Status "  ✓ Removed target directory" "SUCCESS"
    } else {
        Write-Status "  ✓ No target directory found" "INFO"
    }
} catch {
    $errorMessage = $_.Exception.Message
    Write-Status "  ✗ Failed to remove target directory - $errorMessage" "WARNING"
}

# Clean lock files
Write-Status "Cleaning lock files..." "INFO"
$lockFiles = @(
    "./Cargo.lock",
    "./.cargo/.package-cache",
    "./target/.rustc_info.json"
)

foreach ($file in $lockFiles) {
    try {
        if (Test-Path $file) {
            Remove-Item -Force $file -ErrorAction Stop
            Write-Status "  ✓ Removed ${file}" "SUCCESS"
        }
    } catch {
        $errorMessage = $_.Exception.Message
        Write-Status "  ✗ Failed to remove ${file} - $errorMessage" "WARNING"
    }
}

# Stop any running cargo processes
Write-Status "Checking for and stopping cargo processes..." "INFO"
try {
    $cargoProcesses = Get-Process | Where-Object { $_.ProcessName -like "*cargo*" -or $_.ProcessName -like "*rustc*" } -ErrorAction SilentlyContinue
    if ($cargoProcesses) {
        $cargoProcesses | ForEach-Object {
            try {
                Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
                Write-Status "  ✓ Stopped process: $($_.ProcessName) (PID: $($_.Id))" "SUCCESS"
            } catch {
                $errorMessage = $_.Exception.Message
                Write-Status "  ✗ Failed to stop process: $($_.ProcessName) (PID: $($_.Id))" "WARNING"
            }
        }
        # Give processes time to shut down
        Start-Sleep -Seconds 2
    } else {
        Write-Status "  ✓ No cargo processes running" "INFO"
    }
} catch {
    $errorMessage = $_.Exception.Message
    Write-Status "  ✗ Error checking for cargo processes - $errorMessage" "WARNING"
}

# Verify dependencies are correct
Write-Status "Checking workspace dependencies..." "INFO"
try {
    # Check if Cargo.toml exists
    if (Test-Path "./Cargo.toml") {
        $cargoToml = Get-Content -Path "./Cargo.toml" -Raw
        
        # Check bitcoin dependency
        if ($cargoToml -notmatch 'bitcoin.*features.*serde') {
            Write-Status "  ✗ Bitcoin dependency missing 'serde' feature" "WARNING"
        } else {
            Write-Status "  ✓ Bitcoin dependency has correct 'serde' feature" "SUCCESS"
        }
        
        # Check secp256k1 dependency
        if ($cargoToml -notmatch 'secp256k1.*global-context') {
            Write-Status "  ✗ secp256k1 dependency missing 'global-context' feature" "WARNING"
        } else {
            Write-Status "  ✓ secp256k1 dependency has correct 'global-context' feature" "SUCCESS"
        }
    } else {
        Write-Status "  ✗ Cargo.toml not found" "WARNING"
    }
} catch {
    $errorMessage = $_.Exception.Message
    Write-Status "  ✗ Error checking dependencies - $errorMessage" "WARNING"
}

# Update dependencies (skipping if errors might occur)
Write-Status "Updating dependencies..." "INFO"
try {
    cargo update
    if ($LASTEXITCODE -eq 0) {
        Write-Status "  ✓ Updated dependencies successfully" "SUCCESS"
    } else {
        Write-Status "  ✗ Dependency update failed with code $LASTEXITCODE" "WARNING"
    }
} catch {
    $errorMessage = $_.Exception.Message
    Write-Status "  ✗ Failed to update dependencies - $errorMessage" "WARNING"
}

# Verify workspace
Write-Status "Verifying workspace..." "INFO"
try {
    cargo metadata --no-deps --format-version=1
    if ($LASTEXITCODE -eq 0) {
        Write-Status "  ✓ Workspace verification successful" "SUCCESS"
    } else {
        Write-Status "  ✗ Workspace verification failed with code $LASTEXITCODE" "WARNING"
    }
} catch {
    $errorMessage = $_.Exception.Message
    Write-Status "  ✗ Failed to verify workspace - $errorMessage" "WARNING"
}

# Summary
Write-Host "`n================================================================" -ForegroundColor Cyan
Write-Host "Cleanup Summary:" -ForegroundColor Cyan
Write-Host "----------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "Errors: $errorCount" -ForegroundColor $(if ($errorCount -gt 0) { "Red" } else { "Green" })
Write-Host "Warnings: $warningCount" -ForegroundColor $(if ($warningCount -gt 0) { "Yellow" } else { "Green" })
Write-Host "----------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "Cleanup process completed. The system should be ready for rebuilding." -ForegroundColor Green
Write-Host "================================================================" -ForegroundColor Cyan

# Return success even if there were warnings
exit 0 