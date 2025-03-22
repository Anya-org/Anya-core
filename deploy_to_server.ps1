# deploy_to_server.ps1
# Script to deploy Anya Core to the remote server in a single SSH session

param (
    [string]$ServerHost = "192.168.0.212",
    [string]$ServerUser = "anya"
)

Write-Host "==== Anya Core Deployment Script ====" -ForegroundColor Cyan
Write-Host "This script deploys Anya Core to the remote server" -ForegroundColor Cyan
Write-Host

# Check if scripts exist
$serverSetupPath = ".\scripts\server_setup.sh"
$verifyInstallPath = ".\scripts\verify_installation.sh"

if (-not (Test-Path $serverSetupPath)) {
    Write-Host "Error: server_setup.sh not found" -ForegroundColor Red
    exit 1
}

if (-not (Test-Path $verifyInstallPath)) {
    Write-Host "Error: verify_installation.sh not found" -ForegroundColor Red
    exit 1
}

# Fix line endings
Write-Host "Fixing line endings in shell scripts..." -ForegroundColor Yellow
$files = @($serverSetupPath, $verifyInstallPath)
foreach ($file in $files) {
    $content = Get-Content -Path $file -Raw
    if ($content -match "\r\n") {
        Write-Host "Converting $file to Unix line endings (LF)..." -ForegroundColor Yellow
        $content = $content -replace "\r\n", "`n"
        [System.IO.File]::WriteAllText($file, $content)
    }
}

# Create temporary files for deployment
$tempDir = [System.IO.Path]::GetTempPath()
$tempServerSetup = Join-Path $tempDir "server_setup.sh"
$tempVerifyInstall = Join-Path $tempDir "verify_installation.sh"
$tempDeployScript = Join-Path $tempDir "deploy.sh"

# Copy the scripts with Unix line endings
$serverSetupContent = Get-Content $serverSetupPath -Raw
$serverSetupContent = $serverSetupContent -replace "\r\n", "`n"
[System.IO.File]::WriteAllText($tempServerSetup, $serverSetupContent)

$verifyInstallContent = Get-Content $verifyInstallPath -Raw
$verifyInstallContent = $verifyInstallContent -replace "\r\n", "`n"
[System.IO.File]::WriteAllText($tempVerifyInstall, $verifyInstallContent)

# Create the deployment script
$deployScriptContent = @"
#!/bin/bash
echo "===== Starting Anya Core deployment ====="
echo

# Create directory structure
echo "Creating project directory structure..."
mkdir -p ~/projectanya/scripts
mkdir -p ~/projectanya/config
mkdir -p ~/projectanya/logs

# Make scripts executable
echo "Setting execute permissions..."
chmod +x ~/projectanya/scripts/*.sh

# Run server setup
echo "Running server setup script..."
cd ~/projectanya
sudo ./scripts/server_setup.sh

# Verify installation
echo "Verifying installation..."
./scripts/verify_installation.sh

echo
echo "===== Anya Core deployment completed ====="
"@

$deployScriptContent = $deployScriptContent -replace "\r\n", "`n"
[System.IO.File]::WriteAllText($tempDeployScript, $deployScriptContent)

# Now upload the files
Write-Host "Creating directory structure on remote server..." -ForegroundColor Yellow
ssh $ServerUser@$ServerHost "mkdir -p ~/projectanya/scripts"

Write-Host "Copying setup scripts to server..." -ForegroundColor Yellow
scp $tempServerSetup "$ServerUser@$ServerHost`:~/projectanya/scripts/server_setup.sh"
scp $tempVerifyInstall "$ServerUser@$ServerHost`:~/projectanya/scripts/verify_installation.sh"
scp $tempDeployScript "$ServerUser@$ServerHost`:~/projectanya/deploy.sh"

# Execute the deployment script
Write-Host "Running deployment script on server..." -ForegroundColor Yellow
ssh $ServerUser@$ServerHost "chmod +x ~/projectanya/deploy.sh && ~/projectanya/deploy.sh"

# Clean up
Remove-Item -Path $tempServerSetup -Force
Remove-Item -Path $tempVerifyInstall -Force
Remove-Item -Path $tempDeployScript -Force

Write-Host
Write-Host "Deployment process completed!" -ForegroundColor Green
Write-Host 