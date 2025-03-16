# Download Clarinet from a valid source
Write-Host "===================================================================" -ForegroundColor Cyan
Write-Host "--- Downloading Clarinet for Windows                            ---" -ForegroundColor Cyan
Write-Host "===================================================================" -ForegroundColor Cyan

# Create installation directory
$installDir = "$env:USERPROFILE\clarinet"
New-Item -ItemType Directory -Path $installDir -Force | Out-Null

# Try multiple potential download URLs, starting with most recent
$downloadUrls = @(
    "https://github.com/hirosystems/clarinet/releases/latest/download/clarinet-windows-x64.exe",
    "https://github.com/hirosystems/clarinet/releases/download/v2.0.0/clarinet-windows-x64.exe",
    "https://github.com/hirosystems/clarinet/releases/download/v1.7.0/clarinet-windows-x64.exe",
    "https://github.com/hirosystems/clarinet/releases/download/v1.6.0/clarinet-windows-x64.exe"
)

$downloadSuccess = $false
$outputPath = "$installDir\clarinet.exe"

foreach ($url in $downloadUrls) {
    Write-Host "  Attempting to download Clarinet from: $url" -ForegroundColor Gray
    try {
        $webClient = New-Object System.Net.WebClient
        $webClient.DownloadFile($url, $outputPath)
        $downloadSuccess = $true
        Write-Host "  ✅ Clarinet successfully downloaded to $outputPath" -ForegroundColor Green
        break
    }
    catch {
        Write-Host "  ❌ Failed to download from $url" -ForegroundColor Red
    }
}

if ($downloadSuccess) {
    # Add to PATH for current session
    $env:Path += ";$installDir"
    Write-Host "  ✅ Added to PATH for current session" -ForegroundColor Green
    
    # Test the installation
    try {
        $clarinetVersion = & "$installDir\clarinet.exe" --version
        Write-Host "  ✅ Installed Clarinet version: $clarinetVersion" -ForegroundColor Green
        return $true
    } catch {
        Write-Host "  ❌ Clarinet was downloaded but failed to execute: $_" -ForegroundColor Red
        return $false
    }
} else {
    Write-Host "  ❌ All download attempts failed. Will use manual verification instead." -ForegroundColor Red
    return $false
} 