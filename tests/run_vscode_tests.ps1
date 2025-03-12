# VS Code Minimal Test Suite

# Color definitions
$colors = @{
    Red = [System.ConsoleColor]::Red
    Green = [System.ConsoleColor]::Green
    Yellow = [System.ConsoleColor]::Yellow
    Cyan = [System.ConsoleColor]::Cyan
}

# VS Code minimum requirements
$vscodeMins = @{
    RAM = 2GB
    CPU_Cores = 2
    Disk_Space = 1GB
    OS_Build = "10.0" # Windows 10
}

# Cargo minimum requirements 
$cargoMins = @{
    RAM = 4GB
    CPU_Cores = 2
    Disk_Space = 10GB
    Description = "Minimum for Rust/Cargo builds"
}

function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) { Write-Output $args }
    $host.UI.RawUI.ForegroundColor = $fc
}

function Test-VSCodeRequirements {
    $systemInfo = Get-CimInstance Win32_ComputerSystem
    $osInfo = Get-CimInstance Win32_OperatingSystem
    $diskInfo = Get-PSDrive C

    return @{
        VSCode = @{
            RAM_OK = ($systemInfo.TotalPhysicalMemory/1GB) -ge $vscodeMins.RAM
            CPU_OK = $systemInfo.NumberOfLogicalProcessors -ge $vscodeMins.CPU_Cores
            Disk_OK = ($diskInfo.Free/1GB) -ge $vscodeMins.Disk_Space
            OS_OK = [version]$osInfo.Version -ge [version]$vscodeMins.OS_Build
        }
        Cargo = @{
            RAM_OK = ($systemInfo.TotalPhysicalMemory/1GB) -ge $cargoMins.RAM
            CPU_OK = $systemInfo.NumberOfLogicalProcessors -ge $cargoMins.CPU_Cores
            Disk_OK = ($diskInfo.Free/1GB) -ge $cargoMins.Disk_Space
        }
        Specs = @{
            RAM = [math]::Round($systemInfo.TotalPhysicalMemory/1GB, 2)
            CPU_Cores = $systemInfo.NumberOfLogicalProcessors
            Disk_Space = [math]::Round($diskInfo.Free/1GB, 2)
            OS_Version = $osInfo.Version
        }
    }
}

function Test-CargoInstallation {
    try {
        $cargoVersion = (cargo --version)
        $rustcVersion = (rustc --version)
        return $true, "$cargoVersion`n$rustcVersion"
    }
    catch {
        return $false, "Cargo/Rust not found"
    }
}

function Test-VSCodeInstallation {
    try {
        $codePath = Get-Command code -ErrorAction Stop
        $codeVersion = (& code --version)[0]
        return $true, "VS Code $codeVersion"
    }
    catch {
        return $false, "VS Code not found"
    }
}

function Run-MinimalTests {
    Write-ColorOutput $colors.Yellow "Running minimal test suite..."
    
    try {
        # Run core lib tests only
        cargo test --lib --no-default-features
        return $true
    }
    catch {
        Write-ColorOutput $colors.Red "Error running tests: $_"
        return $false
    }
}

# Main execution
Clear-Host
Write-ColorOutput $colors.Cyan "VS Code & Cargo Minimal Test Suite"
Write-ColorOutput $colors.Cyan "================================"

# Check system requirements
$requirements = Test-VSCodeRequirements
$specs = $requirements.Specs

# Display system specs
Write-ColorOutput $colors.Yellow "`nSystem Specifications:"
Write-ColorOutput $colors.White "RAM: $($specs.RAM)GB"
Write-ColorOutput $colors.White "CPU Cores: $($specs.CPU_Cores)"
Write-ColorOutput $colors.White "Free Disk Space: $($specs.Disk_Space)GB"
Write-ColorOutput $colors.White "OS Version: Windows $($specs.OS_Version)"

# Check VS Code installation
$vsCodeInstalled, $vsCodeVersion = Test-VSCodeInstallation
if ($vsCodeInstalled) {
    Write-ColorOutput $colors.Green "`nVS Code Installation: OK"
    Write-ColorOutput $colors.White $vsCodeVersion
} else {
    Write-ColorOutput $colors.Red "`nVS Code Installation: Not Found"
}

# Check Cargo installation