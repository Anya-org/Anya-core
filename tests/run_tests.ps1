# Anya Core Advanced Test Suite for Windows

# Color and formatting definitions
$colors = @{
    Red = [System.ConsoleColor]::Red
    Green = [System.ConsoleColor]::Green
    Yellow = [System.ConsoleColor]::Yellow
    Cyan = [System.ConsoleColor]::Cyan
    White = [System.ConsoleColor]::White
}

# System requirement tiers
$systemTiers = @{
    Minimal = @{
        RAM = 4GB
        CPU_Cores = 2
        Disk_Space = 20GB
        GPU_Memory = 2GB
        Description = "Basic protocol testing only"
        Tests = @("core", "basic_protocol")
    }
    Standard = @{
        RAM = 8GB
        CPU_Cores = 4
        Disk_Space = 50GB
        GPU_Memory = 4GB
        Description = "Full protocol suite without ML"
        Tests = @("core", "protocols", "bitcoin", "web5")
    }
    Full = @{
        RAM = 16GB
        CPU_Cores = 8
        Disk_Space = 100GB
        GPU_Memory = 8GB
        Description = "Complete test suite including ML"
        Tests = @("core", "protocols", "bitcoin", "web5", "ml", "dlc")
    }
}

function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) { Write-Output $args }
    $host.UI.RawUI.ForegroundColor = $fc
}

function Get-SystemSpecs {
    $systemInfo = Get-CimInstance Win32_ComputerSystem
    $osInfo = Get-CimInstance Win32_OperatingSystem
    $diskInfo = Get-PSDrive C
    $gpuInfo = Get-CimInstance Win32_VideoController | 
               Select-Object -First 1 AdapterRAM, Name, DriverVersion

    return @{
        RAM = [math]::Round($systemInfo.TotalPhysicalMemory/1GB, 2)
        CPU_Cores = $systemInfo.NumberOfLogicalProcessors
        CPU_Model = $systemInfo.ProcessorName
        Disk_Space = [math]::Round($diskInfo.Free/1GB, 2)
        OS_Version = $osInfo.Version
        OS_Build = $osInfo.BuildNumber
        GPU_Memory = if($gpuInfo.AdapterRAM) { [math]::Round($gpuInfo.AdapterRAM/1GB, 2) } else { 0 }
        GPU_Name = $gpuInfo.Name
    }
}

function Show-SystemAnalysis {
    param($specs)
    
    Write-ColorOutput $colors.Cyan "`nSystem Analysis"
    Write-ColorOutput $colors.Cyan "==============="
    Write-ColorOutput $colors.White "CPU: $($specs.CPU_Model) ($($specs.CPU_Cores) cores)"
    Write-ColorOutput $colors.White "RAM: $($specs.RAM)GB"
    Write-ColorOutput $colors.White "Free Disk Space: $($specs.Disk_Space)GB"
    Write-ColorOutput $colors.White "GPU: $($specs.GPU_Name) ($($specs.GPU_Memory)GB)"
    Write-ColorOutput $colors.White "OS Version: Windows $($specs.OS_Version)"

    Write-ColorOutput $colors.Cyan "`nAvailable Test Tiers:"
    foreach ($tier in $systemTiers.GetEnumerator()) {
        $meetsReqs = ($specs.RAM -ge $tier.Value.RAM) -and 
                     ($specs.CPU_Cores -ge $tier.Value.CPU_Cores) -and
                     ($specs.Disk_Space -ge $tier.Value.Disk_Space) -and
                     ($specs.GPU_Memory -ge $tier.Value.GPU_Memory)
        
        $color = if ($meetsReqs) { $colors.Green } else { $colors.Red }
        Write-ColorOutput $color "`n[$($tier.Name)] $($tier.Value.Description)"
        Write-ColorOutput $color "- Required: RAM:$($tier.Value.RAM)GB, CPU:$($tier.Value.CPU_Cores) cores, Disk:$($tier.Value.Disk_Space)GB, GPU:$($tier.Value.GPU_Memory)GB"
    }
}

function Get-TestOptions {
    param($specs)
    
    $options = @()
    foreach ($tier in $systemTiers.GetEnumerator()) {
        if (($specs.RAM -ge $tier.Value.RAM) -and 
            ($specs.CPU_Cores -ge $tier.Value.CPU_Cores) -and
            ($specs.Disk_Space -ge $tier.Value.Disk_Space) -and
            ($specs.GPU_Memory -ge $tier.Value.GPU_Memory)) {
            $options += $tier.Value.Tests
        }
    }
    return $options | Select-Object -Unique
}

function Show-TestMenu {
    param(
        $availableTests,
        $specs
    )
    
    Write-ColorOutput $colors.Cyan "`nAvailable Test Suites:"
    Write-ColorOutput $colors.Yellow "0. Run All Available Tests (Recommended for your system)"
    
    $i = 1
    foreach ($test in $availableTests) {
        $requirements = Get-TestRequirements $test
        $meetsReqs = ($specs.RAM -ge $requirements.RAM) -and 
                    ($specs.CPU_Cores -ge $requirements.CPU) -and
                    ($specs.Disk_Space -ge $requirements.Disk)
        
        $color = if ($meetsReqs) { $colors.Green } else { $colors.Yellow }
        $warning = if (!$meetsReqs) { " (May impact performance)" } else { "" }
        
        Write-ColorOutput $color "$i. $test$warning"
        Write-ColorOutput $colors.White "   Requires: RAM:$($requirements.RAM)GB, CPU:$($requirements.CPU) cores, Disk:$($requirements.Disk)GB"
        $i++
    }
    
    Write-ColorOutput $colors.Yellow "`nEnter test numbers (comma-separated) or 0 for all recommended tests:"
    $selection = Read-Host
    
    if ($selection -eq "0") { 
        return $availableTests | Where-Object { 
            $reqs = Get-TestRequirements $_
            ($specs.RAM -ge $reqs.RAM) -and 
            ($specs.CPU_Cores -ge $reqs.CPU) -and
            ($specs.Disk_Space -ge $reqs.Disk)
        }
    }
    
    $selected = $selection.Split(",") | ForEach-Object { $_.Trim() }
    return $availableTests[$selected]
}

function Run-Test {
    param($testName)
    
    Write-ColorOutput $colors.Yellow "`nRunning $testName tests..."
    
    try {
        switch ($testName) {
            "core" { 
                cargo test --lib
            }
            "basic_protocol" {
                cargo test --test basic_protocol_tests
            }
            "bitcoin" {
                cargo test --features bitcoin_integration --test bitcoin_tests
            }
            "web5" {
                cargo test --test web5_protocols
            }
            "ml" {
                cargo test --features ml --test ml_tests
            }
            "dlc" {
                cargo test --features bitcoin_integration --test dlc_tests
            }
            default {
                Write-ColorOutput $colors.Red "Unknown test suite: $testName"
                return $false
            }
        }
        return $true
    }
    catch {
        Write-ColorOutput $colors.Red "Error in $testName tests: $_"
        return $false
    }
}

function Export-TestReport {
    param($results, $specs, $selectedTests)
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportPath = ".\test_reports\test_report_${timestamp}.json"
    
    $report = @{
        timestamp = (Get-Date).ToString("o")
        system_specs = $specs
        selected_tests = $selectedTests
        test_results = $results
        environment = @{
            os_version = $specs.OS_Version
            powershell_version = $PSVersionTable.PSVersion.ToString()
            rust_version = (rustc --version)
        }
    }
    
    $report | ConvertTo-Json -Depth 10 | Out-File $reportPath
    return $reportPath
}

# Main execution
Clear-Host
Write-ColorOutput $colors.Cyan "Anya Core Test Suite"
Write-ColorOutput $colors.Cyan "=================="

# Get and display system specs
$specs = Get-SystemSpecs
Show-SystemAnalysis $specs

# Get available tests based on system specs
$availableTests = Get-TestOptions $specs

if ($availableTests.Count -eq 0) {
    Write-ColorOutput $colors.Red "`nSystem does not meet minimum requirements for any test suite"
    exit 1
}

# Let user select tests
$selectedTests = Show-TestMenu $availableTests $specs

# Initialize test environment
$testReportsPath = ".\test_reports"
if (-not (Test-Path $testReportsPath)) {
    New-Item -ItemType Directory -Path $testReportsPath | Out-Null
}

# Run selected tests
$results = @{}
foreach ($test in $selectedTests) {
    $results[$test] = @{
        success = Run-Test $test
        completion_time = [datetime]::Now
    }
    param($results, $specs, $selectedTests)
}

# Generate report
$reportPath = Export-TestReport -results $results -specs $specs -selectedTests $selectedTests

# Show summary
Write-ColorOutput $colors.Cyan "`nTest Summary"
Write-ColorOutput $colors.Cyan "============"
foreach ($result in $results.GetEnumerator()) {
    $color = if ($result.Value.success) { $colors.Green } else { $colors.Red }
    Write-ColorOutput $color "$($result.Key): $(if($result.Value.success){'Passed'}else{'Failed'})"
}

Write-ColorOutput $colors.Yellow "`nTest report generated: $reportPath"

# Exit with appropriate code
$success = $results.Values.success -notcontains $false
exit ([int](!$success))

    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportPath = ".\test_reports\test_report_${timestamp}.json"
    
    $report = @{
        timestamp = (Get-Date).ToString("o")
        system_specs = $specs
        selected_tests = $selectedTests
        test_results = $results
        environment = @{
            os_version = $specs.OS_Version
            powershell_version = $PSVersionTable.PSVersion.ToString()
            rust_version = (rustc --version)
        }
    }
    
    $report | ConvertTo-Json -Depth 10 | Out-File $reportPath
    return $reportPath
}

# Main execution
Clear-Host
Write-ColorOutput $colors.Cyan "Anya Core Test Suite"
Write-ColorOutput $colors.Cyan "=================="

# Get and display system specs
$specs = Get-SystemSpecs
Show-SystemAnalysis $specs

# Show test recommendations
Show-TestRecommendations $specs

# Get available tests based on system specs
$availableTests = Get-TestOptions $specs

if ($availableTests.Count -eq 0) {
    Write-ColorOutput $colors.Red "`nSystem does not meet minimum requirements for any test suite"
    exit 1
}

# Let user select tests
$selectedTests = Show-TestMenu $availableTests

# Initialize test environment
$testReportsPath = ".\test_reports"
        completion_time = [datetime]::Now
    }
}
    Write-ColorOutput $color "$($result.Key): $(if($result.Value.success){'Passed'}else{'Failed'})"
}

Write-ColorOutput $colors.Yellow "`nTest report generated: $reportPath"

# Exit with appropriate code
$success = $results.Values.success -notcontains $false
exit ([int](!$success))


# Generate report
    $color = if ($result.Value.success) { $colors.Green } else { $colors.Red }
$reportPath = Export-TestReport -results $results -specs $specs -selectedTests $selectedTests

# Show summary
foreach ($result in $results.GetEnumerator()) {
Write-ColorOutput $colors.Cyan "`nTest Summary"
Write-ColorOutput $colors.Cyan "============"
if (-not (Test-Path $testReportsPath)) {
    New-Item -ItemType Directory -Path $testReportsPath | Out-Null
}

foreach ($test in $selectedTests) {
    $results[$test] = @{
        success = Run-Test $test
# Run selected tests
$results = @{}