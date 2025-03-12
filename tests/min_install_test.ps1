# Minimal Installation Test Script

# Define minimum requirements
$minRequirements = @{
    RAM = 2GB
    CPU_Cores = 2
    Disk_Space = 1GB
    Rust_Version = "1.70.0"
    Cargo_Version = "1.70.0"
}

# Test functions
function Test-RustEnvironment {
    try {
        $rustVersion = (rustc --version) -replace 'rustc\s+([0-9]+\.[0-9]+\.[0-9]+).*','$1'
        $cargoVersion = (cargo --version) -replace 'cargo\s+([0-9]+\.[0-9]+\.[0-9]+).*','$1'
        
        Write-Host "Rust version: $rustVersion"
        Write-Host "Cargo version: $cargoVersion"
        
        return [version]$rustVersion -ge [version]$minRequirements.Rust_Version
    } catch {
        Write-Host "Rust/Cargo not found or version check failed"
        return $false
    }
}

function Test-MinimumSpecs {
    $systemInfo = Get-CimInstance Win32_ComputerSystem
    $diskInfo = Get-PSDrive C
    
    $specs = @{
        RAM = [math]::Round($systemInfo.TotalPhysicalMemory/1GB, 2)
        CPU_Cores = $systemInfo.NumberOfLogicalProcessors
        Disk_Space = [math]::Round($diskInfo.Free/1GB, 2)
    }
    
    Write-Host "`nSystem Specifications:"
    Write-Host "RAM: $($specs.RAM)GB (Min: $($minRequirements.RAM)GB)"
    Write-Host "CPU Cores: $($specs.CPU_Cores) (Min: $($minRequirements.CPU_Cores))"
    Write-Host "Free Disk Space: $($specs.Disk_Space)GB (Min: $($minRequirements.Disk_Space)GB)"
    
    return ($specs.RAM -ge $minRequirements.RAM) -and 
           ($specs.CPU_Cores -ge $minRequirements.CPU_Cores) -and 
           ($specs.Disk_Space -ge $minRequirements.Disk_Space)
}

function Run-MinimalTests {
    Write-Host "`nRunning minimal test suite..."
    
    # Create temporary test directory
    $testDir = New-Item -ItemType Directory -Path "$env:TEMP\anya-test" -Force
    Push-Location $testDir
    
    try {
        # Run basic library tests without features
        Write-Host "Running core library tests..."
        cargo test --lib --no-default-features

        # Run minimal protocol tests
        Write-Host "`nRunning minimal protocol tests..."
        cargo test --test basic_protocol_tests --no-default-features

        # Test specific components
        $testResults = @(
            @{Name="Core Library"; Result=(cargo test --lib --no-default-features)}
            @{Name="Basic Protocol"; Result=(cargo test --test basic_protocol_tests --no-default-features)}
            @{Name="Web5 Basic"; Result=(cargo test --test web5_protocols --no-default-features)}
        )

        Write-Host "`nTest Results:"
        $testResults | ForEach-Object {
            $status = if ($_.Result) { "Passed" } else { "Failed" }
            Write-Host "$($_.Name): $status"
        }

        return -not ($testResults.Result -contains $false)
    }
    finally {
        Pop-Location
        Remove-Item -Path $testDir -Recurse -Force
    }
}

# Main execution
Write-Host "Anya Core Minimal Installation Test"
Write-Host "================================`n"

# Check Rust environment
Write-Host "Checking Rust environment..."
if (-not (Test-RustEnvironment)) {
    Write-Host "ERROR: Minimum Rust/Cargo version requirements not met"
    exit 1
}

# Check system specifications
Write-Host "`nChecking system specifications..."
if (-not (Test-MinimumSpecs)) {
    Write-Host "ERROR: System does not meet minimum requirements"
    exit 1
}

# Run minimal tests
if (Run-MinimalTests) {
    Write-Host "`nSUCCESS: All minimal installation tests passed"
    exit 0
} else {
    Write-Host "`nERROR: Some tests failed"
    exit 1
}
