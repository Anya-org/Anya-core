#Requires -Version 5.0
#Requires -RunAsAdministrator

# Import utility functions
. "$PSScriptRoot\tests\install\install_utils.ps1"

# Installer configuration
$installerConfig = @{
    Name = "Anya Core Platform"
    Version = "3.1.0"
    LogPath = "$env:TEMP\anya_install.log"
    Components = @(
        @{
            Name = "Bitcoin Core"
            Required = $true
            Script = "tests\install\install_bitcoin.ps1"
            Version = "24.0.1"
            Dependencies = @()
        },
        @{
            Name = "Lightning Network Daemon"
            Required = $false
            Script = "tests\install\install_lnd.ps1"
            Version = "0.17.0-beta"
            Dependencies = @("Bitcoin Core")
        },
        @{
            Name = "RGB Node"
            Required = $false
            Script = "tests\install\install_rgb.ps1"
            Version = "0.9.0"
            Dependencies = @("Bitcoin Core")
        }
    )
}

# Progress bar characters
$progress = @{
    Bar = '█'
    Empty = '░'
    Width = 50
}

function Write-InstallHeader {
    Clear-Host
    Write-Host "`n$($installerConfig.Name) Installer v$($installerConfig.Version)" -ForegroundColor Cyan
    Write-Host "===============================================" -ForegroundColor Cyan
}

function Show-InstallProgress {
    param(
        [string]$Status,
        [int]$PercentComplete
    )
    
    $filled = [math]::Round($progress.Width * ($PercentComplete / 100))
    $empty = $progress.Width - $filled
    
    Write-Host "`r$Status " -NoNewline
    Write-Host "$($progress.Bar * $filled)$($progress.Empty * $empty) " -NoNewline
    Write-Host "$PercentComplete% " -NoNewline
}

function Test-SystemRequirements {
    $requirements = @{
        RAM = 8
        CPU = 4
        Disk = 50
        PowerShell = 5.0
    }
    
    $systemInfo = Get-CimInstance Win32_ComputerSystem
    $diskInfo = Get-PSDrive C
    
    $results = @{
        RAM = [math]::Round($systemInfo.TotalPhysicalMemory/1GB, 2) -ge $requirements.RAM
        CPU = $systemInfo.NumberOfLogicalProcessors -ge $requirements.CPU
        Disk = [math]::Round($diskInfo.Free/1GB, 2) -ge $requirements.Disk
        PowerShell = $PSVersionTable.PSVersion.Major -ge $requirements.PowerShell
    }
    
    Write-Host "`nSystem Requirements:" -ForegroundColor Cyan
    Write-Host "RAM: $([math]::Round($systemInfo.TotalPhysicalMemory/1GB, 2))GB / $($requirements.RAM)GB Required - " -NoNewline
    Write-Host $(if ($results.RAM) { "OK" } else { "INSUFFICIENT" }) -ForegroundColor $(if ($results.RAM) { "Green" } else { "Red" })
    
    Write-Host "CPU: $($systemInfo.NumberOfLogicalProcessors) cores / $($requirements.CPU) Required - " -NoNewline
    Write-Host $(if ($results.CPU) { "OK" } else { "INSUFFICIENT" }) -ForegroundColor $(if ($results.CPU) { "Green" } else { "Red" })
    
    Write-Host "Disk: $([math]::Round($diskInfo.Free/1GB, 2))GB / $($requirements.Disk)GB Required - " -NoNewline
    Write-Host $(if ($results.Disk) { "OK" } else { "INSUFFICIENT" }) -ForegroundColor $(if ($results.Disk) { "Green" } else { "Red" })
    
    Write-Host "PowerShell: $($PSVersionTable.PSVersion) / $($requirements.PowerShell) Required - " -NoNewline
    Write-Host $(if ($results.PowerShell) { "OK" } else { "INSUFFICIENT" }) -ForegroundColor $(if ($results.PowerShell) { "Green" } else { "Red" })
    
    return $results.Values -notcontains $false
}

function Install-Component {
    param(
        [string]$Name,
        [string]$ScriptPath,
        [array]$Dependencies
    )
    
    # Check dependencies
    foreach ($dep in $Dependencies) {
        $depComponent = $installerConfig.Components | Where-Object { $_.Name -eq $dep }
        if (-not (Test-Path "$env:PROGRAMFILES\$($dep)")) {
            Write-Host "`nInstalling dependency: $dep" -ForegroundColor Yellow
            Install-Component @depComponent
        }
    }
    
    # Install component
    Write-Host "`nInstalling $Name..." -ForegroundColor Cyan
    $scriptFullPath = Join-Path $PSScriptRoot $ScriptPath
    
    if (Test-Path $scriptFullPath) {
        try {
            & $scriptFullPath
            Write-Host "Successfully installed $Name" -ForegroundColor Green
            return $true
        } catch {
            Write-Host "Failed to install $Name: $_" -ForegroundColor Red
            Write-InstallLog "Failed to install $Name: $_"
            return $false
        }
    } else {
        Write-Host "Installation script not found: $scriptFullPath" -ForegroundColor Red
        return $false
    }
}

# Main installation
try {
    Write-InstallHeader
    
    # Check system requirements
    if (-not (Test-SystemRequirements)) {
        throw "System does not meet minimum requirements"
    }
    
    # Initialize
    Install-Prerequisites
    
    # Show components
    Write-Host "`nComponents to install:" -ForegroundColor Cyan
    $installerConfig.Components | ForEach-Object {
        Write-Host "- $($_.Name) v$($_.Version)" -ForegroundColor White
        if ($_.Dependencies.Count -gt 0) {
            Write-Host "  Dependencies: $($_.Dependencies -join ', ')" -ForegroundColor Gray
        }
    }
    
    # Confirm installation
    $confirm = Read-Host "`nProceed with installation? (Y/N)"
    if ($confirm -ne 'Y') {
        Write-Host "Installation cancelled" -ForegroundColor Yellow
        exit 0
    }
    
    # Install components
    $total = $installerConfig.Components.Count
    $current = 0
    
    foreach ($component in $installerConfig.Components) {
        $current++
        $percent = [math]::Round(($current / $total) * 100)
        Show-InstallProgress "Installing $($component.Name)..." $percent
        
        if (-not (Install-Component @component)) {
            if ($component.Required) {
                throw "Failed to install required component: $($component.Name)"
            }
        }
    }
    
    # Cleanup
    Remove-TempFiles
    
    Write-Host "`n`nInstallation completed successfully!" -ForegroundColor Green
    
} catch {
    Write-Host "`nInstallation failed: $_" -ForegroundColor Red
    Write-InstallLog "Installation failed: $_"
    exit 1
}
