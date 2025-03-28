# Anya Core Unified Installer PowerShell Wrapper
# [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
#
# This script acts as a wrapper for the Rust-based unified installer,
# allowing for easy installation on Windows systems.

[CmdletBinding()]
param (
    [Parameter()]
    [string]$Path = "$env:ProgramFiles\AnyaCore",
    
    [Parameter()]
    [ValidateSet("development", "production")]
    [string]$Mode = "development",
    
    [Parameter()]
    [ValidateSet("minimal", "standard", "full", "enterprise", "custom")]
    [string]$Profile = "standard",
    
    [Parameter()]
    [string]$Components = "core,bitcoin,dao,web5",
    
    [Parameter()]
    [string]$RpcEndpoint,
    
    [Parameter()]
    [switch]$VerifyOnly,
    
    [Parameter()]
    [switch]$Report,
    
    [Parameter()]
    [switch]$SkipDependencies,
    
    [Parameter()]
    [switch]$Verbose,
    
    [Parameter()]
    [switch]$Help
)

# Script variables
$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
$InstallerBin = Join-Path -Path $ProjectRoot -ChildPath "target\release\unified_installer.exe"
$InstallerSrc = Join-Path -Path $ProjectRoot -ChildPath "src\bin\unified_installer.rs"
$LogsDir = Join-Path -Path $ProjectRoot -ChildPath "logs"
$LogFile = Join-Path -Path $LogsDir -ChildPath "installer.log"

# Create logs directory if it doesn't exist
if (-not (Test-Path $LogsDir)) {
    New-Item -ItemType Directory -Path $LogsDir -Force | Out-Null
}

# Function to write log messages
function Write-Log {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [string]$Message,
        
        [Parameter()]
        [ValidateSet("INFO", "SUCCESS", "WARNING", "ERROR")]
        [string]$Level = "INFO"
    )
    
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [$Level] $Message"
    
    # Console output with color
    switch ($Level) {
        "INFO" { Write-Host $Message -ForegroundColor Cyan }
        "SUCCESS" { Write-Host $Message -ForegroundColor Green }
        "WARNING" { Write-Host $Message -ForegroundColor Yellow }
        "ERROR" { Write-Host $Message -ForegroundColor Red }
    }
    
    # Write to log file
    Add-Content -Path $LogFile -Value $logMessage
}

# Function to show help
function Show-Help {
    Write-Host "Anya Core Unified Installer" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\Install-AnyaCore.ps1 [options]" -ForegroundColor White
    Write-Host ""
    Write-Host "Options:" -ForegroundColor White
    Write-Host "  -Path <path>             Installation path (default: $env:ProgramFiles\AnyaCore)"
    Write-Host "  -Mode <mode>             Installation mode (development or production)"
    Write-Host "  -Profile <profile>       Installation profile (minimal, standard, full, enterprise)"
    Write-Host "  -Components <list>       Comma-separated list of components to install"
    Write-Host "  -RpcEndpoint <url>       Custom Bitcoin RPC endpoint"
    Write-Host "  -VerifyOnly              Only verify system requirements"
    Write-Host "  -Report                  Generate detailed installation report"
    Write-Host "  -SkipDependencies        Skip dependency installation"
    Write-Host "  -Verbose                 Verbose output"
    Write-Host "  -Help                    Show this help message"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor White
    Write-Host '  .\Install-AnyaCore.ps1 -Path "$env:USERPROFILE\AnyaCore" -Mode development'
    Write-Host '  .\Install-AnyaCore.ps1 -Profile enterprise -Components "core,bitcoin,dao,web5,ml"'
    Write-Host '  .\Install-AnyaCore.ps1 -VerifyOnly -Verbose'
    Write-Host ""
}

# Show help if requested
if ($Help) {
    Show-Help
    exit 0
}

# Function to ensure Rust is installed
function Ensure-Rust {
    try {
        $rustc = Get-Command rustc -ErrorAction SilentlyContinue
        if ($null -eq $rustc) {
            Write-Log "Rust not detected, installing..." -Level INFO
            
            # Download rustup-init.exe
            $rustupInitUrl = "https://win.rustup.rs/x86_64"
            $rustupInitPath = Join-Path -Path $env:TEMP -ChildPath "rustup-init.exe"
            
            Invoke-WebRequest -Uri $rustupInitUrl -OutFile $rustupInitPath
            
            # Install Rust
            Start-Process -FilePath $rustupInitPath -ArgumentList "-y" -Wait
            
            # Clean up
            Remove-Item -Path $rustupInitPath -Force
            
            # Update PATH environment variable for the current session
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")
            
            Write-Log "Rust installed successfully" -Level SUCCESS
        } else {
            Write-Log "Rust already installed: $(& rustc --version)" -Level INFO
        }
    } catch {
        Write-Log "Failed to install Rust: $_" -Level ERROR
        exit 1
    }
}

# Function to build the installer if needed
function Build-Installer {
    try {
        $needsBuild = -not (Test-Path $InstallerBin) -or ((Get-Item $InstallerSrc).LastWriteTime -gt (Get-Item $InstallerBin).LastWriteTime)
        
        if ($needsBuild) {
            Write-Log "Building installer..." -Level INFO
            
            # Create target directory if it doesn't exist
            $targetDir = Split-Path -Parent $InstallerBin
            if (-not (Test-Path $targetDir)) {
                New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
            }
            
            # Check if we have all required dependencies
            Ensure-Rust
            
            # Build the installer
            Push-Location -Path $ProjectRoot
            & cargo build --release --bin unified_installer
            Pop-Location
            
            if (-not (Test-Path $InstallerBin)) {
                Write-Log "Failed to build the installer" -Level ERROR
                exit 1
            }
            
            Write-Log "Installer built successfully" -Level SUCCESS
        } else {
            Write-Log "Using existing installer binary" -Level INFO
        }
    } catch {
        Write-Log "Failed to build installer: $_" -Level ERROR
        exit 1
    }
}

# Function to check if running with admin privileges
function Test-Admin {
    $currentUser = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    $isAdmin = $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    return $isAdmin
}

# Check if we need admin privileges
if ($Path.StartsWith($env:ProgramFiles) -or $Path.StartsWith($env:ProgramW6432)) {
    if (-not (Test-Admin)) {
        Write-Log "Installing to $Path requires administrator privileges" -Level WARNING
        Write-Log "Please run this script as Administrator or specify a different installation path" -Level INFO
        exit 1
    }
}

# Log start of installation
Write-Log "Starting Anya Core installation" -Level INFO
Write-Log "Installation path: $Path" -Level INFO

# Build the installer if needed
Build-Installer

# Prepare arguments for the installer
$installerArgs = @()

$installerArgs += "--path", $Path
$installerArgs += "--mode", $Mode
$installerArgs += "--profile", $Profile
$installerArgs += "--components", $Components

if ($RpcEndpoint) {
    $installerArgs += "--rpc-endpoint", $RpcEndpoint
}

if ($VerifyOnly) {
    $installerArgs += "--verify-only"
}

if ($Report) {
    $installerArgs += "--report"
}

if ($SkipDependencies) {
    $installerArgs += "--skip-dependencies"
}

if ($Verbose) {
    $installerArgs += "--verbose"
}

# Register Windows service if not VerifyOnly
function Register-WindowsService {
    $serviceName = "AnyaCore"
    $servicePath = Join-Path -Path $Path -ChildPath "bin\anya-core.exe"
    $serviceDisplayName = "Anya Core Service"
    $serviceStartupType = "Automatic"
    $serviceDescription = "Anya Core Bitcoin Development Framework"
    
    # Check if the service already exists
    $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
    
    if ($null -ne $service) {
        Write-Log "Service $serviceName already exists" -Level INFO
    } else {
        Write-Log "Registering Windows service: $serviceName" -Level INFO
        
        # Create the service using sc.exe
        $result = & sc.exe create $serviceName binPath= "`"$servicePath`"" DisplayName= "$serviceDisplayName" start= $serviceStartupType
        
        if ($LASTEXITCODE -ne 0) {
            Write-Log "Failed to create service: $result" -Level ERROR
            return
        }
        
        # Set the service description
        & sc.exe description $serviceName "$serviceDescription"
        
        Write-Log "Service registered successfully" -Level SUCCESS
    }
}

# Run the installer
try {
    Write-Log "Running installer with args: $($installerArgs -join ' ')" -Level INFO
    
    $process = Start-Process -FilePath $InstallerBin -ArgumentList $installerArgs -Wait -PassThru -NoNewWindow
    
    if ($process.ExitCode -eq 0) {
        Write-Log "Installation completed successfully" -Level SUCCESS
        
        # Register Windows service if not VerifyOnly
        if (-not $VerifyOnly) {
            Register-WindowsService
        }
        
        exit 0
    } else {
        Write-Log "Installation failed with exit code $($process.ExitCode)" -Level ERROR
        exit $process.ExitCode
    }
} catch {
    Write-Log "Error running installer: $_" -Level ERROR
    exit 1
} 