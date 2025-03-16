# Remove deprecated files and fix symlinks
Write-Host "==================================================================" -ForegroundColor Cyan
Write-Host "--- Legacy File Cleanup & Symlink Fixes                        ---" -ForegroundColor Cyan
Write-Host "==================================================================" -ForegroundColor Cyan

# 1. Remove deprecated test patterns (from consolidate-tests.ps1)
$deprecatedFiles = Get-ChildItem -Path "$PSScriptRoot/../tests" -Include @(
    "DEPRECATED_*", 
    "legacy-dao-test-*.clar",
    "v1-compliance-check-*.clar"
) -Recurse

$deprecatedFiles | ForEach-Object {
    Write-Host "Permanently removing: $($_.FullName)" -ForegroundColor Red
    Remove-Item $_ -Force
}

# 2. Clean up old symlinks (from verify-clarinet-config.ps1 patterns)
$symlinkTargets = @(
    "$PSScriptRoot/../test-results/old-reports",
    "$PSScriptRoot/../test/performance" 
)

Get-ChildItem -Path $symlinkTargets -ErrorAction SilentlyContinue | Where-Object {
    $_.LinkType -eq "SymbolicLink"
} | ForEach-Object {
    Write-Host "Removing legacy symlink: $($_.FullName)" -ForegroundColor Yellow
    $_ | Remove-Item -Force
}

# 3. Create new symlinks for consolidated structure (hexagonal architecture)
$newSymlinks = @{
    "$PSScriptRoot/../test-results/compliance" = "$PSScriptRoot/../tests/system/compliance"
    "$PSScriptRoot/../test-results/performance" = "$PSScriptRoot/../tests/performance"
}

foreach ($link in $newSymlinks.GetEnumerator()) {
    if (-not (Test-Path $link.Key)) {
        New-Item -ItemType SymbolicLink -Path $link.Key -Target $link.Value | Out-Null
        Write-Host "Created symlink: $($link.Key) -> $($link.Value)" -ForegroundColor Green
    }
}

# 4. Final cleanup of empty directories (enhanced from consolidate-tests.ps1)
Get-ChildItem -Path "$PSScriptRoot/.." -Directory -Recurse | Where-Object {
    $_.FullName -match "test[\\/]|DEPRECATED" -and 
    $_.GetFiles().Count -eq 0 -and 
    $_.GetDirectories().Count -eq 0
} | Remove-Item -Force -Recurse

Write-Host "Cleanup complete!" -ForegroundColor Cyan 