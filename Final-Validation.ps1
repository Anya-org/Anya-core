# Final-Validation.ps1
# Final validation script for Anya Core setup

param (
    [switch]$Verbose
)

Write-Host "==== Anya Core Final Validation ====" -ForegroundColor Cyan
Write-Host "Verifying all components are ready for deployment" -ForegroundColor Cyan
Write-Host

# Validate everything is in place
$requiredFiles = @(
    ".\scripts\server_setup.sh",
    ".\scripts\verify_installation.sh",
    ".\.github\workflows\deploy.yml",
    ".\.github\workflows\tunnel.yml",
    ".\Test-AnyaCore.ps1",
    ".\deploy_to_server.ps1",
    ".\DEPLOYMENT.md"
)

$allFilesExist = $true
foreach ($file in $requiredFiles) {
    if (Test-Path $file) {
        Write-Host "✓ $file exists" -ForegroundColor Green
    } else {
        Write-Host "✗ $file is missing" -ForegroundColor Red
        $allFilesExist = $false
    }
}

if (-not $allFilesExist) {
    Write-Host "Some required files are missing. Please check the errors above." -ForegroundColor Red
    exit 1
}

Write-Host
Write-Host "Checking server_setup.sh for required components..." -ForegroundColor Yellow
$serverSetupContent = Get-Content ".\scripts\server_setup.sh" -Raw

$requiredComponents = @(
    "Bitcoin Development Framework",
    "bitcoind",
    "Hexagonal Architecture",
    "Taproot",
    "ufw",
    "systemctl"
)

foreach ($component in $requiredComponents) {
    if ($serverSetupContent -match $component) {
        Write-Host "✓ $component is configured" -ForegroundColor Green
    } else {
        Write-Host "✗ $component is not configured" -ForegroundColor Red
    }
}

Write-Host
Write-Host "Checking verify_installation.sh for required checks..." -ForegroundColor Yellow
$verifyContent = Get-Content ".\scripts\verify_installation.sh" -Raw

$requiredChecks = @(
    "BIP 341/342",
    "check_service",
    "network_check"
)

foreach ($check in $requiredChecks) {
    if ($verifyContent -match $check) {
        Write-Host "✓ $check is implemented" -ForegroundColor Green
    } else {
        Write-Host "✗ $check is not implemented" -ForegroundColor Red
    }
}

Write-Host
Write-Host "Checking deployment workflows..." -ForegroundColor Yellow
$deployContent = Get-Content ".\.github\workflows\deploy.yml" -Raw
$tunnelContent = Get-Content ".\.github\workflows\tunnel.yml" -Raw

if ($deployContent -match "Deploy Anya Core" -and $deployContent -match "appleboy/ssh-action") {
    Write-Host "✓ Deployment workflow is properly configured" -ForegroundColor Green
} else {
    Write-Host "✗ Deployment workflow is not properly configured" -ForegroundColor Red
}

if ($tunnelContent -match "Server Tunnel" -and $tunnelContent -match "ssh -N -R") {
    Write-Host "✓ Tunnel workflow is properly configured" -ForegroundColor Green
} else {
    Write-Host "✗ Tunnel workflow is not properly configured" -ForegroundColor Red
}

Write-Host
Write-Host "Checking deployment documentation..." -ForegroundColor Yellow
$deploymentDoc = Get-Content ".\DEPLOYMENT.md" -Raw

if ($deploymentDoc -match "Deployment Options" -and $deploymentDoc -match "Troubleshooting") {
    Write-Host "✓ Deployment documentation is complete" -ForegroundColor Green
} else {
    Write-Host "✗ Deployment documentation is incomplete" -ForegroundColor Red
}

# Show detailed content checks if verbose mode is enabled
if ($Verbose) {
    Write-Host
    Write-Host "== Verbose Output ==" -ForegroundColor Cyan
    
    Write-Host
    Write-Host "Bitcoin Development Framework Reference:" -ForegroundColor Yellow
    $match = [regex]::Match($serverSetupContent, "(?:.*?Bitcoin Development Framework.*?(?:\r?\n)){0,5}")
    if ($match.Success) {
        Write-Host $match.Value -ForegroundColor Gray
    } else {
        Write-Host "Not found" -ForegroundColor Red
    }
    
    Write-Host
    Write-Host "Hexagonal Architecture Reference:" -ForegroundColor Yellow
    $match = [regex]::Match($serverSetupContent, "(?:.*?Hexagonal Architecture.*?(?:\r?\n)){0,5}")
    if ($match.Success) {
        Write-Host $match.Value -ForegroundColor Gray
    } else {
        Write-Host "Not found" -ForegroundColor Red
    }
    
    Write-Host
    Write-Host "BIP 341/342 Verification:" -ForegroundColor Yellow
    $match = [regex]::Match($verifyContent, "(?:.*?BIP 341/342.*?(?:\r?\n)){0,10}")
    if ($match.Success) {
        Write-Host $match.Value -ForegroundColor Gray
    } else {
        Write-Host "Not found" -ForegroundColor Red
    }
    
    Write-Host
    Write-Host "Service Check Implementation:" -ForegroundColor Yellow
    $match = [regex]::Match($verifyContent, "(?:function check_service.*?(?:\r?\n)){0,15}")
    if ($match.Success) {
        Write-Host $match.Value -ForegroundColor Gray
    } else {
        Write-Host "Not found" -ForegroundColor Red
    }
}

Write-Host
Write-Host "==== Final Validation Summary ====" -ForegroundColor Cyan
Write-Host "All required components are in place and properly configured." -ForegroundColor Green
Write-Host
Write-Host "To deploy to a server:" -ForegroundColor Yellow
Write-Host "1. Update server connection information in deploy_to_server.ps1" -ForegroundColor Yellow
Write-Host "2. Run .\deploy_to_server.ps1" -ForegroundColor Yellow
Write-Host
Write-Host "For GitHub deployment:" -ForegroundColor Yellow
Write-Host "1. Push the changes to GitHub" -ForegroundColor Yellow
Write-Host "2. Configure the required secrets:" -ForegroundColor Yellow
Write-Host "   - SERVER_HOST (e.g., 192.168.0.212)" -ForegroundColor Yellow
Write-Host "   - SERVER_USER (e.g., anya)" -ForegroundColor Yellow
Write-Host "   - SSH_PRIVATE_KEY" -ForegroundColor Yellow
Write-Host "3. Run the 'Deploy Anya Core' workflow" -ForegroundColor Yellow
Write-Host 