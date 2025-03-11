# Launcher script for InstallManager that handles elevation

# Check if running in virtual environment
if (-not $env:VIRTUAL_ENV) {
    Write-Host "Please run this installer in the virtual environment" -ForegroundColor Yellow
    Write-Host "Run Setup-VirtualEnv.ps1 first" -ForegroundColor Yellow
    exit 1
}

function Test-AdminPrivileges {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (-not (Test-AdminPrivileges)) {
    Write-Host "Launching installer with administrative privileges..." -ForegroundColor Yellow
    try {
        $venvPath = $env:VIRTUAL_ENV
        Start-Process powershell -Verb RunAs -ArgumentList "-NoProfile -ExecutionPolicy Bypass -Command `"cd '$PWD'; . '$venvPath\Scripts\Activate.ps1'; .\InstallManager.ps1`""
    }
    catch {
        Write-Host "Failed to launch installer with admin privileges: $_" -ForegroundColor Red
        Write-Host "Please run PowerShell as Administrator and try again" -ForegroundColor Yellow
        exit 1
    }
}
else {
    # Already running as admin, execute directly
    & "$PSScriptRoot\InstallManager.ps1"
}
