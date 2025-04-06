# Test configuration loading from PowerShell
# [AIS-3][BPC-3]

Write-Host "Testing platform configuration loading from PowerShell..."

# Check if the platform configuration files exist
$windowsConfig = "config\platform\windows.yaml"
$unixConfig = "config\platform\unix.yaml"

if (-not (Test-Path $windowsConfig)) {
    Write-Host "Error: Windows configuration file not found at $windowsConfig" -ForegroundColor Red
    exit 1
}

if (-not (Test-Path $unixConfig)) {
    Write-Host "Error: Unix configuration file not found at $unixConfig" -ForegroundColor Red
    exit 1
}

Write-Host "Configuration files found:" -ForegroundColor Green
Write-Host "  - $windowsConfig"
Write-Host "  - $unixConfig"

# Attempt to load the Windows configuration
try {
    $configContent = Get-Content $windowsConfig -Raw
    Write-Host "`nWindows configuration content:" -ForegroundColor Cyan
    Write-Host $configContent
    
    # Try to expand environment variables
    $basePath = "%USERPROFILE%\.anya"
    $expandedPath = $basePath.Replace("%USERPROFILE%", $env:USERPROFILE)
    
    Write-Host "`nTesting environment variable expansion:" -ForegroundColor Cyan
    Write-Host "  Original: $basePath"
    Write-Host "  Expanded: $expandedPath"
    
    # Check if the directory exists or can be created
    if (-not (Test-Path $expandedPath)) {
        Write-Host "Directory doesn't exist, trying to create it..." -ForegroundColor Yellow
        New-Item -Path $expandedPath -ItemType Directory -Force | Out-Null
        Write-Host "Directory created successfully!" -ForegroundColor Green
    } else {
        Write-Host "Directory already exists!" -ForegroundColor Green
    }
} catch {
    Write-Host "Error reading configuration: $_" -ForegroundColor Red
    exit 1
}

Write-Host "`nConfiguration test completed successfully!" -ForegroundColor Green