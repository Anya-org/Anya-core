# Simple test script for Anya DAO
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "--- Anya DAO - Simple Test Runner                            ---" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan

# Check if Clarinet is available
$clarinetAvailable = $null -ne (Get-Command clarinet -ErrorAction SilentlyContinue)

if (-not $clarinetAvailable) {
    Write-Host "`n⚠️ Clarinet is not installed or not in your PATH!" -ForegroundColor Red
    Write-Host "Please install Clarinet from: https://github.com/hirosystems/clarinet/releases" -ForegroundColor Yellow
    exit 1
}

Write-Host "`nClarinet found. Proceeding with tests..." -ForegroundColor Green

# Step 1: Verify contract syntax
Write-Host "`n--- Checking contract syntax ---" -ForegroundColor Yellow
clarinet check

# Step 2: Run a simple test for each contract
$testFiles = Get-ChildItem -Path "tests" -Filter "*.test.clar" -ErrorAction SilentlyContinue

Write-Host "`n--- Running individual tests ---" -ForegroundColor Yellow
foreach ($test in $testFiles) {
    Write-Host "Running test: $($test.Name)" -ForegroundColor Gray
    clarinet test $test.FullName
}

Write-Host "`n✅ Tests completed!" -ForegroundColor Green 