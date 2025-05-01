# Clean Rust/Cargo locks and build artifacts
Write-Host "Starting Rust/Cargo cleanup..." -ForegroundColor Green

# Try to stop any cargo processes
Write-Host "Attempting to stop any cargo processes..." -ForegroundColor Yellow
$cargoProcesses = Get-Process | Where-Object { $_.ProcessName -like "*cargo*" -or $_.ProcessName -like "*rustc*" } -ErrorAction SilentlyContinue
if ($cargoProcesses) {
    $cargoProcesses | ForEach-Object { 
        Write-Host "Stopping process: $($_.ProcessName) (PID: $($_.Id))" -ForegroundColor Cyan
        Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue 
    }
    # Give processes time to shut down
    Start-Sleep -Seconds 2
}

# Clean cargo build artifacts
Write-Host "Cleaning cargo build artifacts..." -ForegroundColor Yellow
cargo clean

# Remove lock files
Write-Host "Removing lock files..." -ForegroundColor Yellow
$lockPaths = @(
    "./.cargo/.package-cache",
    "./target/.rustc_info.json",
    "./target/debug/.fingerprint",
    "./target/CACHEDIR.TAG"
)

foreach ($path in $lockPaths) {
    if (Test-Path $path) {
        try {
            if ((Get-Item $path) -is [System.IO.DirectoryInfo]) {
                Remove-Item $path -Recurse -Force -ErrorAction SilentlyContinue
            } else {
                Remove-Item $path -Force -ErrorAction SilentlyContinue
            }
            Write-Host "Removed: $path" -ForegroundColor Cyan
        } catch {
            Write-Host "Could not remove $path : $_" -ForegroundColor Red
        }
    }
}

# Reset Cargo.lock if it exists
if (Test-Path "./Cargo.lock") {
    Write-Host "Resetting Cargo.lock..." -ForegroundColor Yellow
    Move-Item -Path "./Cargo.lock" -Destination "./Cargo.lock.bak" -Force
    Write-Host "Backed up Cargo.lock to Cargo.lock.bak" -ForegroundColor Cyan
}

Write-Host "Cleanup complete!" -ForegroundColor Green
Write-Host "You should now be able to run cargo commands without lock issues." -ForegroundColor Green 