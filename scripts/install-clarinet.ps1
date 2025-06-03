# Install Clarinet for Windows
# This script downloads and installs Clarinet for Windows

# Configuration
$clarinetVersion = "2.3.0"
$downloadUrl = "https://github.com/hirosystems/clarinet/releases/download/v$clarinetVersion/clarinet-windows-x64.msi"
$installerPath = "$env:TEMP\clarinet-windows-x64.msi"

Write-Host "Installing Clarinet v$clarinetVersion..." -ForegroundColor Cyan

# Download the installer
Write-Host "Downloading Clarinet installer..."
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $installerPath
    Write-Host "Download completed successfully." -ForegroundColor Green
}
catch {
    Write-Host "Failed to download Clarinet: $_" -ForegroundColor Red
    exit 1
}

# Install Clarinet
Write-Host "Installing Clarinet..."
try {
    Start-Process -FilePath "msiexec.exe" -ArgumentList "/i", $installerPath, "/quiet", "/norestart" -Wait
    Write-Host "Installation completed successfully." -ForegroundColor Green
}
catch {
    Write-Host "Failed to install Clarinet: $_" -ForegroundColor Red
    exit 1
}

# Verify installation
Write-Host "Verifying installation..."
try {
    $clarinetPath = (Get-Command clarinet -ErrorAction Stop).Source
    $installedVersion = (& clarinet --version)
    Write-Host "Clarinet $installedVersion installed at: $clarinetPath" -ForegroundColor Green
}
catch {
    Write-Host "Clarinet was installed but is not in the PATH. You may need to restart your terminal or add it to your PATH manually." -ForegroundColor Yellow
    Write-Host "Expected location: C:\Program Files\Clarinet" -ForegroundColor Yellow
}

# Clean up
Remove-Item $installerPath -Force
Write-Host "Temporary files cleaned up." -ForegroundColor Green

Write-Host "Installation process completed." -ForegroundColor Cyan
Write-Host "You can now use Clarinet to develop and test Clarity smart contracts." -ForegroundColor Cyan

# Then run with elevated permissions
Start-Process powershell -Verb RunAs -ArgumentList "-File `"$PSScriptRoot\install-clarinet.ps1`"" 