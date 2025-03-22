# Test-AnyaCore.ps1
# Script to test the Anya Core setup

Write-Host "==== Anya Core Test Script ====" -ForegroundColor Cyan
Write-Host "This script validates the setup scripts before deployment" -ForegroundColor Cyan
Write-Host

# Check if scripts exist
$serverSetupPath = ".\scripts\server_setup.sh"
$verifyInstallPath = ".\scripts\verify_installation.sh"
$deployWorkflowPath = ".\.github\workflows\deploy.yml"
$tunnelWorkflowPath = ".\.github\workflows\tunnel.yml"

function Test-ScriptExists {
    param (
        [string]$Path,
        [string]$Description
    )
    
    Write-Host "Checking for $Description... " -NoNewline
    if (Test-Path $Path) {
        Write-Host "Found" -ForegroundColor Green
        return $true
    } else {
        Write-Host "Not found" -ForegroundColor Red
        return $false
    }
}

# Validate script content
function Test-ScriptContent {
    param (
        [string]$Path,
        [string]$SearchString,
        [string]$Description
    )
    
    Write-Host "Checking if $Description contains '$SearchString'... " -NoNewline
    if (Select-String -Path $Path -Pattern $SearchString -Quiet) {
        Write-Host "Yes" -ForegroundColor Green
        return $true
    } else {
        Write-Host "No" -ForegroundColor Red
        return $false
    }
}

# Main validation
$allChecksPass = $true

# Check server_setup.sh
if (Test-ScriptExists -Path $serverSetupPath -Description "server setup script") {
    Test-ScriptContent -Path $serverSetupPath -SearchString "Bitcoin Development Framework" -Description "server setup script"
    Test-ScriptContent -Path $serverSetupPath -SearchString "bitcoind" -Description "server setup script"
    Test-ScriptContent -Path $serverSetupPath -SearchString "Hexagonal Architecture" -Description "server setup script"
} else {
    $allChecksPass = $false
}

# Check verify_installation.sh
if (Test-ScriptExists -Path $verifyInstallPath -Description "verification script") {
    Test-ScriptContent -Path $verifyInstallPath -SearchString "BIP 341/342" -Description "verification script"
    Test-ScriptContent -Path $verifyInstallPath -SearchString "check_service" -Description "verification script"
} else {
    $allChecksPass = $false
}

# Check GitHub workflow files
if (Test-ScriptExists -Path $deployWorkflowPath -Description "deployment workflow") {
    Test-ScriptContent -Path $deployWorkflowPath -SearchString "Deploy Anya Core" -Description "deployment workflow"
    Test-ScriptContent -Path $deployWorkflowPath -SearchString "appleboy/ssh-action" -Description "deployment workflow"
} else {
    $allChecksPass = $false
}

if (Test-ScriptExists -Path $tunnelWorkflowPath -Description "tunnel workflow") {
    Test-ScriptContent -Path $tunnelWorkflowPath -SearchString "Server Tunnel" -Description "tunnel workflow"
    Test-ScriptContent -Path $tunnelWorkflowPath -SearchString "ssh -N -R" -Description "tunnel workflow"
} else {
    $allChecksPass = $false
}

# Check for Linux line endings in shell scripts
function Test-LineEndings {
    param (
        [string]$Path
    )
    
    Write-Host "Checking line endings in $Path... " -NoNewline
    $content = Get-Content -Path $Path -Raw
    if ($content -match "\r\n") {
        Write-Host "Windows line endings detected (CRLF)" -ForegroundColor Yellow
        Write-Host "Converting to Unix line endings (LF)... " -NoNewline
        $content = $content -replace "\r\n", "`n"
        [System.IO.File]::WriteAllText($Path, $content)
        Write-Host "Done" -ForegroundColor Green
    } else {
        Write-Host "Unix line endings (LF) - Good" -ForegroundColor Green
    }
}

if (Test-Path $serverSetupPath) {
    Test-LineEndings -Path $serverSetupPath
}

if (Test-Path $verifyInstallPath) {
    Test-LineEndings -Path $verifyInstallPath
}

# Final report
Write-Host
if ($allChecksPass) {
    Write-Host "All basic checks passed! The scripts are ready to deploy." -ForegroundColor Green
} else {
    Write-Host "Some checks failed. Please fix the issues before deploying." -ForegroundColor Red
}

Write-Host
Write-Host "To test with the remote server, you need to set up these GitHub secrets:"
Write-Host "- SERVER_HOST (e.g., 192.168.0.212)" -ForegroundColor Yellow
Write-Host "- SERVER_USER (e.g., anya)" -ForegroundColor Yellow
Write-Host "- SSH_PRIVATE_KEY (from your SSH private key file)" -ForegroundColor Yellow
Write-Host 