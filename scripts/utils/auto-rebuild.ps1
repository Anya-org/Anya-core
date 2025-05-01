# Auto-rebuild script for anya-bitcoin
# This script periodically rebuilds the project

Write-Host "Starting auto-rebuild for anya-bitcoin..." -ForegroundColor Green
Write-Host "Press Ctrl+C at any time to stop" -ForegroundColor Yellow

# Clean up first
Write-Host "Cleaning up build artifacts..." -ForegroundColor Cyan
cargo clean -p anya-bitcoin

# Initial build
Write-Host "Running initial build..." -ForegroundColor Cyan
cargo check -p anya-bitcoin --lib --no-default-features

# Auto-rebuild loop
$interval = 10 # seconds between checks
while ($true) {
    Write-Host "Waiting $interval seconds before next rebuild..." -ForegroundColor Gray
    Start-Sleep -Seconds $interval
    
    Write-Host "$(Get-Date) - Running build..." -ForegroundColor Cyan
    cargo check -p anya-bitcoin --lib --no-default-features
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Build successful!" -ForegroundColor Green
    } else {
        Write-Host "Build failed." -ForegroundColor Red
    }
} 