# Run everything script for Anya DAO
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "--- Anya DAO - Complete Verification Suite                    ---" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan

# Create scripts directory if it doesn't exist
if (-not (Test-Path "scripts")) {
    New-Item -ItemType Directory -Path "scripts" -Force | Out-Null
    Write-Host "Created scripts directory" -ForegroundColor Green
}

# Ensure all required scripts exist before continuing
$requiredScripts = @(
    @{Path="scripts/setup-directories.ps1"; Create=$true},
    @{Path="scripts/manual-verify.ps1"; Create=$true},
    @{Path="scripts/create-contract-templates.ps1"; Create=$true},
    @{Path="scripts/verify-contracts-basic.ps1"; Create=$true},
    @{Path="scripts/download-clarinet.ps1"; Create=$true},
    @{Path="scripts/generate-compliance-report.ts"; Create=$true}
)

$missingScripts = 0
foreach ($script in $requiredScripts) {
    if (-not (Test-Path $script.Path)) {
        $missingScripts++
        if ($script.Create) {
            Write-Host "Missing script: $($script.Path). This will be created." -ForegroundColor Yellow
        } else {
            Write-Host "Missing script: $($script.Path)" -ForegroundColor Red
        }
    }
}

# Step 1: Check if Clarinet is available
$clarinetAvailable = $null -ne (Get-Command clarinet -ErrorAction SilentlyContinue)

if (-not $clarinetAvailable) {
    Write-Host "`n--- Clarinet not found, attempting to download ---" -ForegroundColor Yellow
    
    # Download and install Clarinet if needed
    if (Test-Path "scripts/download-clarinet.ps1") {
        $clarinetAvailable = & "scripts/download-clarinet.ps1"
    } else {
        Write-Host "  ❌ download-clarinet.ps1 script not found" -ForegroundColor Red
    }
}

# Step 2: Setup directory structure and create missing files
Write-Host "`n--- Setting up project structure ---" -ForegroundColor Yellow
if (Test-Path "scripts/setup-directories.ps1") {
    & "scripts/setup-directories.ps1"
} else {
    Write-Host "  ❌ setup-directories.ps1 script not found" -ForegroundColor Red
}

# Step 3: Run verification
if ($clarinetAvailable) {
    Write-Host "`n--- Running Clarinet verification ---" -ForegroundColor Yellow
    try {
        & clarinet check
        Write-Host "  ✅ Clarinet syntax check passed" -ForegroundColor Green
    } catch {
        Write-Host "  ❌ Clarinet syntax check failed: $_" -ForegroundColor Red
    }
    
    # Run tests if clarinet is available
    if (Test-Path "scripts/run-simple-tests.ps1") {
        & "scripts/run-simple-tests.ps1"
    } else {
        Write-Host "  ❌ run-simple-tests.ps1 script not found" -ForegroundColor Red
    }
} else {
    Write-Host "`n--- Running manual verification ---" -ForegroundColor Yellow
    if (Test-Path "scripts/manual-verify.ps1") {
        & "scripts/manual-verify.ps1"
    } else {
        Write-Host "  ❌ manual-verify.ps1 script not found" -ForegroundColor Red
    }
}

Write-Host "`n✅ Verification process completed!" -ForegroundColor Green
Write-Host "================================================================" -ForegroundColor Cyan 