# push_to_server.ps1
# Script to push Anya Core setup files to the remote server

param (
    [string]$ServerHost = "192.168.0.212",
    [string]$ServerUser = "anya",
    [switch]$RunSetup = $false,
    [switch]$UseMasterConnection = $true
)

Write-Host "==== Anya Core Server Push Script ====" -ForegroundColor Cyan
Write-Host "This script pushes the setup files to the remote server" -ForegroundColor Cyan
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

# Check if SSH key is set up
$sshConfigPath = "$env:USERPROFILE\.ssh\config"
$sshKeySetup = $false

if (Test-Path $sshConfigPath) {
    $configContent = Get-Content $sshConfigPath -Raw
    if ($configContent -match "Host $ServerHost") {
        $sshKeySetup = $true
    }
}

if (-not $sshKeySetup) {
    Write-Host "SSH key not set up for $ServerHost. For passwordless login, run:" -ForegroundColor Yellow
    Write-Host "  .\setup_ssh_key.ps1 -ServerHost $ServerHost -ServerUser $ServerUser" -ForegroundColor Yellow
    Write-Host "Continuing with password authentication..." -ForegroundColor Yellow
}

# Set up options for SSH commands 
$sshOptions = ""
if ($UseMasterConnection -and $sshKeySetup) {
    # This creates a master connection that can be reused
    Write-Host "Setting up SSH master connection..." -ForegroundColor Yellow
    $controlPath = "$env:USERPROFILE\.ssh\control-$ServerUser-$ServerHost-22"
    
    # Check if control socket exists already
    if (-not (Test-Path $controlPath)) {
        ssh -M -f -N -o ControlPersist=10m $ServerUser@$ServerHost
        Start-Sleep -Seconds 1
    }
    
    $sshOptions = "-o ControlPath=`"$controlPath`""
    $scpOptions = "-o ControlPath=`"$controlPath`""
}

# Create directory structure on remote server
Write-Host "Creating directory structure on remote server..." -ForegroundColor Yellow
ssh $sshOptions $ServerUser@$ServerHost "mkdir -p ~/projectanya/scripts"

# Copy files to remote server
Write-Host "Copying files to remote server..." -ForegroundColor Yellow
scp $scpOptions $serverSetupPath $ServerUser@$ServerHost`:~/projectanya/scripts/
scp $scpOptions $verifyInstallPath $ServerUser@$ServerHost`:~/projectanya/scripts/

# Set execute permissions
Write-Host "Setting execute permissions..." -ForegroundColor Yellow
ssh $sshOptions $ServerUser@$ServerHost "chmod +x ~/projectanya/scripts/*.sh"

# Run setup if requested
if ($RunSetup) {
    Write-Host "Running server setup script..." -ForegroundColor Yellow
    ssh $sshOptions $ServerUser@$ServerHost "cd ~/projectanya && sudo ./scripts/server_setup.sh"
    
    Write-Host "Running verification script..." -ForegroundColor Yellow
    ssh $sshOptions $ServerUser@$ServerHost "cd ~/projectanya && ./scripts/verify_installation.sh"
} else {
    Write-Host "Setup not requested. To run setup, use -RunSetup switch" -ForegroundColor Yellow
}

Write-Host
Write-Host "Files pushed to server successfully!" -ForegroundColor Green
Write-Host
Write-Host "To run setup manually, SSH to the server and run:" -ForegroundColor Yellow
Write-Host "  cd ~/projectanya && sudo ./scripts/server_setup.sh" -ForegroundColor Yellow
Write-Host

# If we created a master connection, let the user know how to close it
if ($UseMasterConnection -and $sshKeySetup) {
    Write-Host "Note: An SSH master connection was created and will stay active for 10 minutes." -ForegroundColor Yellow
    Write-Host "To close it immediately, run:" -ForegroundColor Yellow
    Write-Host "  ssh -O exit $ServerUser@$ServerHost" -ForegroundColor Yellow
    Write-Host
} 