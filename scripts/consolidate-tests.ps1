# Remove duplicate test directories and migrate to unified structure
Write-Host "==================================================================" -ForegroundColor Cyan
Write-Host "--- Test Directory Consolidation                               ---" -ForegroundColor Cyan
Write-Host "==================================================================" -ForegroundColor Cyan

# 1. Migrate legacy test files to new structure
$migrationMap = @{
    "$PSScriptRoot/../test/dao" = "$PSScriptRoot/../tests/modules/dao"
    "$PSScriptRoot/../test/compliance" = "$PSScriptRoot/../tests/system/compliance"
    "$PSScriptRoot/../test/performance" = "$PSScriptRoot/../tests/performance"
}

foreach ($entry in $migrationMap.GetEnumerator()) {
    if (Test-Path $entry.Key) {
        Move-Item -Path $entry.Key -Destination $entry.Value -Force
        Write-Host "Migrated: $($entry.Key) -> $($entry.Value)" -ForegroundColor Yellow
    }
}

# 2. Deprecate old test patterns
$deprecatedPatterns = @(
    "legacy-dao-test-*.clar",
    "v1-compliance-check-*.clar",
    "old-performance-metrics.clar"
)

Get-ChildItem -Path "$PSScriptRoot/../tests" -Include $deprecatedPatterns -Recurse | ForEach-Object {
    Write-Host "Deprecating: $($_.FullName)" -ForegroundColor Gray
    $_ | Rename-Item -NewName { "DEPRECATED_$($_.Name)" }
}

# 3. Update test references in Clarinet.toml
$clarinetConfig = Get-Content "$PSScriptRoot/../Clarinet.toml" -Raw
$updatedConfig = $clarinetConfig -replace 'path = "test/', 'path = "tests/'
$updatedConfig | Set-Content "$PSScriptRoot/../Clarinet.toml"

Write-Host "Updated Clarinet.toml test references" -ForegroundColor Green

# 4. Clean empty directories
Get-ChildItem -Path "$PSScriptRoot/../test" -Recurse -Directory | Where-Object {
    $_.GetFiles().Count -eq 0 -and $_.GetDirectories().Count -eq 0
} | Remove-Item -Force

if (Test-Path "$PSScriptRoot/../test") {
    Remove-Item "$PSScriptRoot/../test" -Recurse -Force
    Write-Host "Removed legacy test directory" -ForegroundColor Green
}

# 5. Update test runner configurations
$testRunnerConfig = @{
    testDirectories = @(
        @{ path = "tests/modules"; type = "unit" },
        @{ path = "tests/system"; type = "integration" },
        @{ path = "tests/performance"; type = "benchmark" }
    )
    outputPath = "test-results"
}
$testRunnerConfig | ConvertTo-Json -Depth 3 | Out-File "$PSScriptRoot/../test-config.json"

Write-Host "Test consolidation complete!" -ForegroundColor Cyan 