param(
    [hashtable]$Configuration,
    [string]$ResultsDirectory
)

Write-Host "Setting up test environment..." -ForegroundColor Cyan

# Ensure the results directory exists
if (-not (Test-Path $ResultsDirectory)) {
    New-Item -ItemType Directory -Path $ResultsDirectory -Force | Out-Null
    Write-Host "  Created results directory: $ResultsDirectory" -ForegroundColor Green
}

# Create test configuration file
$configFile = "$ResultsDirectory/config.json"
$configJson = $Configuration | ConvertTo-Json -Depth 5
Set-Content -Path $configFile -Value $configJson
Write-Host "  Created configuration file: $configFile" -ForegroundColor Green

# Create necessary test directories
$testDirs = @(
    "$ResultsDirectory/bitcoin",
    "$ResultsDirectory/dao",
    "$ResultsDirectory/layer2",
    "$ResultsDirectory/compliance"
)

foreach ($dir in $testDirs) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Host "  Created directory: $dir" -ForegroundColor Green
    }
}

# Create a marker file to indicate setup is complete
$marker = "$ResultsDirectory/.setup-complete"
Set-Content -Path $marker -Value (Get-Date).ToString()
Write-Host "Test environment setup complete!" -ForegroundColor Green

# Return success
exit 0 