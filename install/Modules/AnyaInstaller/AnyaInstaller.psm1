#Requires -Version 5.0
#Requires -RunAsAdministrator

using module ./Core/InstallationService.psm1
using module ./Adapters/VenvManager.psm1
using module ./Adapters/RustInstaller.psm1
using module ./Adapters/BitcoinInstaller.psm1
using module ./Adapters/Web5Installer.psm1
using module ./Configuration/InstallationConfig.psm1
using module ./Infrastructure/CheckpointManager.psm1
using module ./Utils/SystemChecks.psm1  # This module provides Test-SystemChecks

# Detect OS
$OSIsWindows = $env:OS -like "*Windows*"
$OSIsMacOS = $false
$OSIsLinux = $false

# Safely detect Unix-based OS without using automatic variables
if (-not $OSIsWindows) {
    try { 
        $unameOutput = uname 2>$null
        $OSIsMacOS = $unameOutput -eq "Darwin"
        $OSIsLinux = $unameOutput -eq "Linux"
    } catch { 
        # Ignore errors from uname command
    }
}

# Use platform-specific paths
$dataPath = if ($OSIsWindows) {
    "$env:ProgramData\AnyaCore"
} elseif ($OSIsMacOS) {
    "/Library/Application Support/AnyaCore"
} else {
    "/etc/anya-core"
}

function Install-AnyaCore {
    param(
        [string]$DeploymentType = 'Standalone',
        [bool]$IsNetworked = $false
    )
    
    Write-Host "=== Starting Comprehensive Anya Core Installation ===" -ForegroundColor Cyan
    try {
        # Initialize core components with detailed logging
        Write-Host "Initializing configuration and components..." -ForegroundColor Cyan
        $config = [InstallationConfig]::new()
        $venvManager = [VenvManager]::new()
        $rustInstaller = [RustInstaller]::new()
        $bitcoinInstaller = [BitcoinInstaller]::new()
        $web5Installer = [Web5Installer]::new()
        
        # Run comprehensive system checks
        Write-Host "Performing comprehensive system checks..." -ForegroundColor Cyan
        if (-not (Test-SystemChecks)) {
            throw "System checks failed. Verify hardware, network, and dependencies."
        }
        
        # Optionally, verify that the dashboard is deployed (interactive dashboard prompt)
        if (-not (Test-Path "$dataPath\dash33")) {
            Write-Host "Warning: Dashboard (dash33) not detected under $dataPath." -ForegroundColor Yellow
        }
        
        # Initialize virtual environment
        Write-Host "Initializing virtual environment..." -ForegroundColor Cyan
        if (-not $venvManager.InitializeVenv("$dataPath\venv", "3.9", "21.3.1")) {
            throw "Failed to initialize virtual environment."
        }
        
        # Install Rust toolchain and packages with logging
        Write-Host "Installing Rust toolchain and packages..." -ForegroundColor Cyan
        if (-not $rustInstaller.InstallRustToolchain("1.65")) {
            throw "Rust toolchain installation failed."
        }
        if (-not $rustInstaller.InstallRustPackages(@("cargo-watch", "wasm-pack", "tch", "rust-bert"))) {
            throw "Rust packages installation failed."
        }
        
        # Install Bitcoin layer with detailed network logging
        Write-Host "Installing Bitcoin layer..." -ForegroundColor Cyan
        if (-not $bitcoinInstaller.InstallBitcoinLayer("mainnet")) {
            throw "Bitcoin layer installation failed."
        }
        
        # Install Web5 layer
        Write-Host "Installing Web5 layer..." -ForegroundColor Cyan
        if (-not $web5Installer.InstallWeb5Layer()) {
            throw "Web5 layer installation failed."
        }
        
        # Finally, start core installation service
        Write-Host "Starting core installation service..." -ForegroundColor Cyan
        if (Start-Installation -DeploymentType $DeploymentType -IsNetworked $IsNetworked) {
            Write-Host "Core installation completed successfully." -ForegroundColor Green
            return $true
        }
        else {
            Write-Host "Core installation service failed to start." -ForegroundColor Red
            return $false
        }
    }
    catch {
        Write-Host "Installation failed: $_" -ForegroundColor Red
        return $false
    }
}

Export-ModuleMember -Function Install-AnyaCore