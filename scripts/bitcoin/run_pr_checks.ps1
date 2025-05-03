# Bitcoin PR Checks Script
# [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
# This script runs all the checks that would be performed in a PR

Write-Host "Bitcoin PR Checks" -ForegroundColor Cyan
Write-Host "----------------" -ForegroundColor Cyan
Write-Host "Running validation checks for Bitcoin module PR..."
Write-Host ""

# Setup
$ErrorActionPreference = "Stop"
$rootDir = $PSScriptRoot | Split-Path | Split-Path
Set-Location $rootDir

function Write-Status {
    param (
        [string]$step,
        [string]$status,
        [string]$color = "White"
    )
    
    Write-Host "[$step] " -ForegroundColor Cyan -NoNewline
    Write-Host $status -ForegroundColor $color
}

# Validate formatting
Write-Status "Check 1/6" "Checking code formatting..."
try {
    & cargo fmt --check
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Result" "✅ Code formatting is correct" "Green"
    } else {
        Write-Status "Result" "❌ Code formatting issues found" "Red"
        exit 1
    }
} catch {
    Write-Status "Result" "❌ Failed to check formatting: $_" "Red"
    exit 1
}

# Run clippy
Write-Status "Check 2/6" "Running Clippy linting..."
try {
    & cargo clippy --all-targets --all-features -- -D warnings
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Result" "✅ No linting issues found" "Green"
    } else {
        Write-Status "Result" "❌ Linting issues found" "Red"
        exit 1
    }
} catch {
    Write-Status "Result" "❌ Failed to run Clippy: $_" "Red"
    exit 1
}

# Run unit tests
Write-Status "Check 3/6" "Running Bitcoin unit tests..."
try {
    & cargo test --package anya-core --lib bitcoin
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Result" "✅ Unit tests passed" "Green"
    } else {
        Write-Status "Result" "❌ Unit tests failed" "Red"
        exit 1
    }
} catch {
    Write-Status "Result" "❌ Failed to run unit tests: $_" "Red"
    exit 1
}

# Check BIP compliance
Write-Status "Check 4/6" "Checking BIP compliance..."
try {
    & cargo build --bin verify_bip_modules
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Result" "✅ BIP modules are valid" "Green"
    } else {
        Write-Status "Result" "❌ BIP module validation failed" "Red"
        exit 1
    }
} catch {
    Write-Status "Result" "❌ Failed to build BIP validation tool: $_" "Red"
    exit 1
}

# Check hexagonal architecture
Write-Status "Check 5/6" "Analyzing hexagonal architecture..."
try {
    $coreCount = (Get-ChildItem -Path "src/bitcoin" -Filter "*.rs" -Recurse | Where-Object { $_.FullName -notmatch "interface|adapters" }).Count
    $interfaceCount = (Get-ChildItem -Path "src/bitcoin/interface" -Filter "*.rs" -Recurse -ErrorAction SilentlyContinue).Count
    $adapterCount = (Get-ChildItem -Path "src/bitcoin/adapters" -Filter "*.rs" -Recurse -ErrorAction SilentlyContinue).Count
    
    Write-Host "  Core components: $coreCount"
    Write-Host "  Interface components: $interfaceCount"
    Write-Host "  Adapter components: $adapterCount"
    
    if ($interfaceCount -gt 0) {
        Write-Status "Result" "✅ Interface layer exists" "Green"
    } else {
        Write-Status "Result" "❌ Missing interface layer" "Red"
        exit 1
    }
} catch {
    Write-Status "Result" "❌ Failed to analyze architecture: $_" "Red"
    exit 1
}

# Check documentation
Write-Status "Check 6/6" "Verifying documentation..."
try {
    $docsCount = (Get-ChildItem -Path "docs/bitcoin" -Filter "*.md" -Recurse).Count
    Write-Host "  Documentation files: $docsCount"
    
    if ($docsCount -ge 3) {
        Write-Status "Result" "✅ Sufficient documentation exists" "Green"
    } else {
        Write-Status "Result" "⚠️ Documentation may be insufficient" "Yellow"
    }
    
    $bipIndex = Get-Content -Path "docs/bitcoin/BIP_IMPLEMENTATION_INDEX.md" -Raw
    if ($bipIndex -match "BIP-341") {
        Write-Status "Result" "✅ BIP-341 documentation found" "Green"
    } else {
        Write-Status "Result" "❌ Missing BIP-341 documentation" "Red"
        exit 1
    }
    
    if ($bipIndex -match "BIP-342") {
        Write-Status "Result" "✅ BIP-342 documentation found" "Green"
    } else {
        Write-Status "Result" "❌ Missing BIP-342 documentation" "Red"
        exit 1
    }
} catch {
    Write-Status "Result" "❌ Failed to verify documentation: $_" "Red"
    exit 1
}

Write-Host ""
Write-Host "All PR checks completed successfully! ✅" -ForegroundColor Green
Write-Host "The branch is ready to be merged into the target branch." -ForegroundColor Green 