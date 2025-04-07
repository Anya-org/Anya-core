#Requires -Version 5.0

<#
.SYNOPSIS
    Cross-platform installer for Anya Core
.DESCRIPTION
    Installs Anya Core on Windows, macOS, or Linux
.PARAMETER Install
    Run installation
.PARAMETER Uninstall
    Run uninstallation
.PARAMETER Config
    Path to configuration file
.EXAMPLE
    ./install.ps1 -Install
.EXAMPLE
    ./install.ps1 -Uninstall
.EXAMPLE
    ./install.ps1 -Install -Config custom-config.json
#>
[CmdletBinding()]
param(
    [switch]$Install = $false,
    [switch]$Uninstall = $false,
    [string]$Config = ""
)

# Detect platform
$OsWindows = $false
$OsMacOS = $false
$OsLinux = $false

# Use built-in variables if available (PowerShell 6+)
if ($null -ne $global:IsWindows) {
    $OsWindows = $global:IsWindows
    $OsMacOS = $global:IsMacOS
    $OsLinux = $global:IsLinux
} else {
    # For older PowerShell versions without automatic variables
    $osInfo = [System.Environment]::OSVersion
    $OsWindows = $osInfo.Platform -eq "Win32NT"
    
    if (-not $OsWindows) {
        try {
            $uname = if (Get-Command uname -ErrorAction SilentlyContinue) { uname }
            if ($uname -eq "Darwin") {
                $OsMacOS = $true
            } elseif ($uname -eq "Linux") {
                $OsLinux = $true
            }
        } catch {
            # Default to Windows if detection fails
            $OsWindows = $true
        }
    }
}

# Determine data path based on platform
$dataDirPath = if ($OsWindows) {
    "$env:ProgramData\AnyaCore"
} elseif ($OsMacOS) {
    "/Library/Application Support/AnyaCore"
} else {
    # Linux
    "/etc/anya-core"
}

# Check for elevated permissions
function Test-AdminPrivileges {
    if ($OsWindows) {
        $identity = [System.Security.Principal.WindowsIdentity]::GetCurrent()
        $principal = New-Object System.Security.Principal.WindowsPrincipal($identity)
        return $principal.IsInRole([System.Security.Principal.WindowsBuiltInRole]::Administrator)
    } else {
        # On Unix, check for root (UID 0)
        try {
            $uid = id -u
            return $uid -eq 0
        } catch {
            # If id command not available, try another approach
            return $(whoami) -eq "root"
        }
    }
}

# Check if Rust is installed
function Test-RustInstalled {
    try {
        $rustcVersion = rustc --version
        $cargoVersion = cargo --version
        Write-Host "Found Rust: $rustcVersion" -ForegroundColor Green
        Write-Host "Found Cargo: $cargoVersion" -ForegroundColor Green
        return $true
    } catch {
        Write-Host "Rust is not installed or not in PATH" -ForegroundColor Yellow
        return $false
    }
}

# Install Rust if needed
function Install-Rust {
    if ($OsWindows) {
        Write-Host "Installing Rust using rustup-init.exe..."
        Invoke-WebRequest -Uri "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe" -OutFile "rustup-init.exe"
        Start-Process -FilePath ".\rustup-init.exe" -ArgumentList "-y" -Wait
        Remove-Item "rustup-init.exe" -Force
    } else {
        Write-Host "Installing Rust using rustup script..."
        Invoke-Expression -Command "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
        # Source cargo env
        $env:PATH = "$HOME/.cargo/bin:$env:PATH"
    }
    
    Write-Host "Rust installation complete" -ForegroundColor Green
}

# Main installation function
function Install-AnyaCore {
    # Create data directory if it doesn't exist
    if (-not (Test-Path $dataDirPath)) {
        Write-Host "Creating data directory: $dataDirPath" -ForegroundColor Cyan
        New-Item -ItemType Directory -Path $dataDirPath -Force | Out-Null
    }
    
    # Build and install Anya Core
    Write-Host "Building Anya Core..." -ForegroundColor Cyan
    Push-Location $(Join-Path $PSScriptRoot "..")
    
    try {
        cargo build --release
        
        # Copy binaries to appropriate location
        $binDir = if ($OsWindows) {
            "$env:ProgramFiles\AnyaCore\bin"
        } elseif ($OsMacOS) {
            "/Applications/AnyaCore.app/Contents/MacOS"
        } else {
            # Linux
            "/usr/local/bin"
        }
        
        if (-not (Test-Path $binDir)) {
            New-Item -ItemType Directory -Path $binDir -Force | Out-Null
        }
        
        Write-Host "Copying binaries to $binDir" -ForegroundColor Cyan
        $sourceFile = Join-Path (Get-Location) "target/release/anya-installer$(if ($OsWindows) { '.exe' } else { '' })"
        $destFile = Join-Path $binDir "anya-installer$(if ($OsWindows) { '.exe' } else { '' })"
        Copy-Item -Path $sourceFile -Destination $destFile -Force
        
        # Run the installer
        Write-Host "Running Anya Core installer..." -ForegroundColor Cyan
        $installArgs = "install"
        if ($Config) {
            $installArgs += " --config `"$Config`""
        }
        
        if ($OsWindows) {
            Start-Process -FilePath $destFile -ArgumentList $installArgs -Wait -NoNewWindow
        } else {
            & $destFile $installArgs.Split(" ")
        }
        
        Write-Host "Installation complete!" -ForegroundColor Green
    } finally {
        Pop-Location
    }
}

# Uninstallation function
function Uninstall-AnyaCore {
    Write-Host "Uninstalling Anya Core..." -ForegroundColor Cyan
    
    # Build and run uninstaller
    Push-Location $(Join-Path $PSScriptRoot "..")
    
    try {
        cargo build --release
        
        $installerPath = Join-Path (Get-Location) "target/release/anya-installer$(if ($OsWindows) { '.exe' } else { '' })"
        
        # Run the uninstaller
        if ($OsWindows) {
            Start-Process -FilePath $installerPath -ArgumentList "uninstall" -Wait -NoNewWindow
        } else {
            & $installerPath uninstall
        }
        
        # Remove data directory
        if (Test-Path $dataDirPath) {
            Write-Host "Removing data directory: $dataDirPath" -ForegroundColor Cyan
            Remove-Item -Path $dataDirPath -Recurse -Force
        }
        
        Write-Host "Uninstallation complete!" -ForegroundColor Green
    } finally {
        Pop-Location
    }
}

# Main execution
try {
    # Check for admin privileges
    if (-not (Test-AdminPrivileges)) {
        Write-Host "This script requires administrator/root privileges. Please run as administrator/root." -ForegroundColor Red
        exit 1
    }
    
    # Check for Rust
    if (-not (Test-RustInstalled)) {
        $installRust = Read-Host "Rust is required but not installed. Would you like to install it? (y/n)"
        if ($installRust -match "^[Yy]") {
            Install-Rust
        } else {
            Write-Host "Rust is required to continue. Exiting." -ForegroundColor Red
            exit 1
        }
    }
    
    # Perform requested action
    if ($Install) {
        Install-AnyaCore
    } elseif ($Uninstall) {
        Uninstall-AnyaCore
    } else {
        Write-Host "No action specified. Use -Install or -Uninstall" -ForegroundColor Yellow
        Write-Host "Usage: ./install.ps1 -Install [-Config path/to/config.json] | -Uninstall"
    }
} catch {
    Write-Host "An error occurred: $_" -ForegroundColor Red
    exit 1
} 