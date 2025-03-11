#Requires -Version 5.0
#Requires -Modules NetworkTests

#region Configuration
$ErrorActionPreference = "Stop"

# Color definitions
$colors = @{
    Red      = [System.ConsoleColor]::Red
    Green    = [System.ConsoleColor]::Green 
    Yellow   = [System.ConsoleColor]::Yellow
    Cyan     = [System.ConsoleColor]::Cyan
    White    = [System.ConsoleColor]::White
    DarkCyan = [System.ConsoleColor]::DarkCyan
    Gray     = [System.ConsoleColor]::Gray
    Blue     = [System.ConsoleColor]::Blue
}

# Box drawing characters
$box = @{
    TopLeft     = '┌'
    TopRight    = '┐'
    BottomLeft  = '└'
    BottomRight = '┘'
    Horizontal  = '─'
    Vertical    = '│'
    TeeRight    = '├'
    TeeLeft     = '┤'
}

# Test categories configuration
$testCategories = @{
    "Minimal Tests" = @{
        RAM = 2
        CPU = 2
        Disk = 10
        Tests = @(
            @{Name="Core Library Tests"; Command="cargo test --lib --no-default-features"}
            @{Name="Basic Protocol Tests"; Command="cargo test --test basic_protocol_tests"}
        )
        Description = "Basic functionality tests requiring minimal resources"
    }
    "Standard Tests" = @{
        RAM = 4
        CPU = 4
        Disk = 20
        Tests = @(
            @{Name="Integration Tests"; Command="cargo test --test integration_tests"}
            @{Name="Web5 Protocol Tests"; Command="cargo test --test web5_protocols"}
            @{Name="RGB Asset Tests"; Command="cargo test --test rgb_asset_test"}
        )
        Description = "Standard test suite for protocol validation"
    }
    "Full Tests" = @{
        RAM = 8
        CPU = 4
        Disk = 50
        Tests = @(
            @{Name="Layer 2 Tests"; Command="cargo test --test layer2_integration_test"}
            @{Name="ML Operations"; Command="cargo test --test ml_operations_test"}
            @{Name="DLC Tests"; Command="cargo test --features bitcoin_integration --test dlc_tests"}
        )
        Description = "Complete test suite including advanced features"
    }
}
#endregion Configuration

#region UI Functions
function Write-ColorOutput {
    param(
        [System.ConsoleColor]$ForegroundColor,
        [string]$Message
    )
    
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    Write-Output $Message
    $host.UI.RawUI.ForegroundColor = $fc
}

function Write-DashboardBlock {
    param(
        [string]$Title,
        [string[]]$Content,
        [int]$Width = 50,
        [System.ConsoleColor]$TitleColor = $colors.Cyan
    )
    
    # Draw top border with title
    Write-ColorOutput $TitleColor "$($box.TopLeft)$($box.Horizontal * 2)[$Title]$($box.Horizontal * ($Width - 5 - $Title.Length))$($box.TopRight)"
    
    # Draw content
    foreach ($line in $Content) {
        $padding = $Width - $line.Length - 2
        Write-ColorOutput $TitleColor "$($box.Vertical) $line$(' ' * $padding)$($box.Vertical)"
    }
    
    # Draw bottom border
    Write-ColorOutput $TitleColor "$($box.BottomLeft)$($box.Horizontal * ($Width))$($box.BottomRight)"
}

function Show-Dashboard {
    param(
        [hashtable]$SystemCaps,
        [hashtable]$NetworkServices
    )
    
    Clear-Host
    Write-ColorOutput $colors.Cyan "`nAnya Core Test Dashboard"
    Write-ColorOutput $colors.Cyan "=========================="
    
    # System Block
    $systemContent = @(
        "RAM: $($SystemCaps.RAM)GB / Required: 8GB",
        "CPU: $($SystemCaps.CPU) cores / Required: 4",
        "Disk: $($SystemCaps.Disk)GB / Required: 50GB"
    )
    Write-DashboardBlock "System Status" $systemContent
    
    # Network Block
    $networkContent = $NetworkServices.GetEnumerator() | ForEach-Object {
        $status = if ($_.Value.Available) { "✓" } else { "✗" }
        $installed = if ($_.Value.Installed) { "Installed" } else { "Not Installed" }
        "$status $($_.Key): $installed"
    }
    Write-DashboardBlock "Network Services" $networkContent -TitleColor $colors.Blue
    
    # Tests Block
    $testContent = $testCategories.GetEnumerator() | ForEach-Object -Begin {$i = 1} {
        $reqs = "RAM:$($_.Value.RAM)GB CPU:$($_.Value.CPU) Disk:$($_.Value.Disk)GB"
        "$i. $($_.Key) - $reqs"
        $i++
    }
    Write-DashboardBlock "Available Tests" $testContent -TitleColor $colors.Green
}
#endregion UI Functions

#region System Functions
function Get-SystemCapabilities {
    $systemInfo = Get-CimInstance Win32_ComputerSystem
    $diskInfo = Get-PSDrive C

    return @{
        RAM = [math]::Round($systemInfo.TotalPhysicalMemory/1GB, 2)
        CPU = $systemInfo.NumberOfLogicalProcessors
        Disk = [math]::Round($diskInfo.Free/1GB, 2)
        Description = "Current system capabilities"
    }
}

function Get-NetworkServices {
    $services = @{
        "Bitcoin Core" = @{
            Port = 8333
            InstallScript = "$PSScriptRoot\..\install\components\install_bitcoin.ps1"
            ConfigPath = "$env:APPDATA\Bitcoin\bitcoin.conf"
        }
        "Lightning Network" = @{
            Port = 9735
            InstallScript = "$PSScriptRoot\..\install\components\install_lnd.ps1"
            ConfigPath = "$env:APPDATA\Lnd\lnd.conf"
        }
        "RGB Node" = @{
            Port = 3000
            InstallScript = "$PSScriptRoot\..\install\components\install_rgb.ps1"
            ConfigPath = "$env:APPDATA\RGB\config.json"
        }
    }
    
    $results = @{ }
    foreach ($svc in $services.GetEnumerator()) {
        $results[$svc.Key] = @{
            Available = Test-NetConnection -ComputerName localhost -Port $svc.Value.Port -WarningAction SilentlyContinue -InformationLevel Quiet
            Installed = Test-Path $svc.Value.ConfigPath
            InstallScript = $svc.Value.InstallScript
        }
    }
    return $results
}

function Test-NetworkDependencies {
    param($systemCaps)
    
    Write-ColorOutput $colors.Cyan "`nTesting Network Dependencies..."
    
    $networkDevices = @("Bitcoin Node", "Lightning Node", "RGB Node")
    $results = @{ }
    
    foreach ($device in $networkDevices) {
        Write-ColorOutput $colors.Yellow "`nChecking $device..."
        $results[$device] = Test-NetworkDevice -deviceType $device
        
        if (-not $results[$device].Available) {
            Write-ColorOutput $colors.Yellow "Would you like to initialize $device? (Y/N)"
            $response = Read-Host
            if ($response -eq 'Y') {
                Write-ColorOutput $colors.Yellow "Initializing $device..."
                Initialize-NetworkDevice -deviceType $device
                $results[$device] = Test-NetworkDevice -deviceType $device
            }
        }
        
        # Display results
        $status = if ($results[$device].Available) { "Available" } else { "Not Available" }
        $color = if ($results[$device].Available) { $colors.Green } else { $colors.Red }
        Write-ColorOutput $color "$device Status: $status"
        
        if ($results[$device].Available) {
            Write-ColorOutput $colors.White "  Services: $($results[$device].Services.Count) found"
            Write-ColorOutput $colors.White "  Config: $($results[$device].ConfigStatus)"
        }
    }
    
    return $results
}

function Install-NetworkService {
    param(
        [string]$ServiceName,
        [hashtable]$ServiceInfo
    )
    
    if (-not $ServiceInfo.Installed) {
        Write-ColorOutput $colors.Yellow "Installing $ServiceName..."
        if (Test-Path $ServiceInfo.InstallScript) {
            & $ServiceInfo.InstallScript
            return $true
        } else {
            Write-ColorOutput $colors.Red "Installation script not found: $($ServiceInfo.InstallScript)"
            return $false
        }
    }
    return $true
}
#endregion System Functions

#region Test Functions
function Show-TestAvailability {
    param($systemCaps)
    
    Write-ColorOutput $colors.Cyan "`nSystem Capabilities:"
    Write-ColorOutput $colors.White ("RAM: {0:N2}GB" -f $systemCaps.RAM)
    Write-ColorOutput $colors.White "CPU Cores: $($systemCaps.CPU)"
    Write-ColorOutput $colors.White ("Free Disk Space: {0:N2}GB" -f $systemCaps.Disk)

    Write-ColorOutput $colors.Cyan "`nAvailable Test Categories:"
    $i = 1
    foreach ($category in $testCategories.GetEnumerator()) {
        $canRun = ($systemCaps.RAM -ge $category.Value.RAM) -and 
                 ($systemCaps.CPU -ge $category.Value.CPU) -and 
                 ($systemCaps.Disk -ge $category.Value.Disk)
        
        $color = if ($canRun) { $colors.Green } else { $colors.Yellow }
        $status = if ($canRun) { "AVAILABLE" } else { "INSUFFICIENT RESOURCES" }
        
        Write-ColorOutput $color "`n$i. $($category.Key) [$status]"
        Write-ColorOutput $colors.White "   Description: $($category.Value.Description)"
        Write-ColorOutput $colors.White ("   Requirements: RAM:{0}GB, CPU:{1} cores, Disk:{2}GB" -f $category.Value.RAM, $category.Value.CPU, $category.Value.Disk)
        
        if ($canRun) {
            Write-ColorOutput $colors.White "   Available Tests:"
            foreach ($test in $category.Value.Tests) {
                Write-ColorOutput $colors.White "     - $($test.Name)"
            }
        }
        $i++
    }
}

function Get-UserTestSelection {
    param($systemCaps)
    
    $availableCategories = $testCategories.GetEnumerator() | Where-Object {
        $_.Value.RAM -le $systemCaps.RAM -and 
        $_.Value.CPU -le $systemCaps.CPU -and 
        $_.Value.Disk -le $systemCaps.Disk
    }

    Write-ColorOutput $colors.Yellow "`nSelect test categories to run (comma-separated numbers):"
    Write-ColorOutput $colors.White "Enter '?' for help or 'all' for all available tests"
    
    while ($true) {
        $selection = Read-Host "Selection"
        
        if ($selection -eq '?') {
            Write-ColorOutput $colors.Cyan "`nHelp:"
            Write-ColorOutput $colors.White "- Enter single number (e.g. '1') for one category"
            Write-ColorOutput $colors.White "- Enter multiple numbers separated by commas (e.g. '1,2')"
            Write-ColorOutput $colors.White "- Enter 'all' to run all available tests"
            Write-ColorOutput $colors.White "- Enter 'q' to quit"
            continue
        }
        
        if ($selection -eq 'q') {
            exit 0
        }
        
        if ($selection -eq 'all') {
            return $availableCategories
        }
        
        try {
            $selected = $selection.Split(',') | ForEach-Object { $_.Trim() }
            $categoryMap = @{ }
            $i = 1
            foreach ($category in $testCategories.GetEnumerator()) {
                $categoryMap[$i] = $category.Value
                $i++
            }
            
            return $selected | ForEach-Object { 
                $num = [int]$_
                if ($num -lt 1 -or $num -gt $categoryMap.Count) {
                    throw "Invalid selection: $num. Please enter numbers between 1 and $($categoryMap.Count)"
                }
                $categoryMap[$num]
            }
        }
        catch {
            Write-ColorOutput $colors.Red "Error: $_"
            Write-ColorOutput $colors.Yellow "Please try again or enter '?' for help"
            continue
        }
    }
}

function Run-TestCategory {
    param(
        $category,
        $logFile
    )
    
    $results = @{
        Passed = 0
        Failed = 0
        Skipped = 0
    }

    foreach ($test in $category.Tests) {
        Write-ColorOutput $colors.Yellow "`nRunning: $($test.Name)"
        try {
            Invoke-Expression $test.Command
            if ($LASTEXITCODE -eq 0) {
                $results.Passed++
                Write-ColorOutput $colors.Green "✓ $($test.Name) passed"
            } else {
                $results.Failed++
                Write-ColorOutput $colors.Red "✗ $($test.Name) failed"
            }
        } catch {
            $results.Failed++
            Write-ColorOutput $colors.Red "✗ $($test.Name) error: $_"
            $_ | Out-File -Append $logFile
        }
    }
    
    return $results
}
#endregion Test Functions

#region Main Execution
try {
    # Initialize
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $logFile = "test_run_${timestamp}.log"
    $systemCaps = Get-SystemCapabilities
    $networkServices = Get-NetworkServices

    # Main Loop
    while ($true) {
        Show-Dashboard $systemCaps $networkServices
        
        Write-ColorOutput $colors.Yellow "`nOptions:"
        @(
            "1. Run Tests",
            "2. Install Missing Services",
            "3. Refresh Status",
            "4. Exit"
        ) | ForEach-Object { Write-ColorOutput $colors.White $_ }
        
        switch (Read-Host "`nSelect option") {
            "1" { 
                # Test Execution
                $selectedCategories = Get-UserTestSelection $systemCaps
                $totalResults = @{Passed = 0; Failed = 0; Skipped = 0}
                
                foreach ($category in $selectedCategories) {
                    $results = Run-TestCategory $category $logFile
                    $totalResults.Passed += $results.Passed
                    $totalResults.Failed += $results.Failed
                    $totalResults.Skipped += $results.Skipped
                }
                
                # Show Results
                Write-DashboardBlock "Test Results" @(
                    "Passed: $($totalResults.Passed)",
                    "Failed: $($totalResults.Failed)",
                    "Skipped: $($totalResults.Skipped)"
                ) -TitleColor $(if ($totalResults.Failed -eq 0) { $colors.Green } else { $colors.Red })
            }
            "2" { 
                # Service Installation
                $networkServices.GetEnumerator() | Where-Object {
                    -not $_.Value.Available -or -not $_.Value.Installed
                } | ForEach-Object {
                    if ((Read-Host "Install $($_.Key)? (Y/N)") -eq 'Y') {
                        Install-NetworkService $_.Key $_.Value
                    }
                }
                $networkServices = Get-NetworkServices
            }
            "3" { 
                # Refresh Status
                $systemCaps = Get-SystemCapabilities
                $networkServices = Get-NetworkServices
            }
            "4" { exit 0 }
        }
    }
} catch {
    Write-ColorOutput $colors.Red "`nError: $_"
    Write-ColorOutput $colors.Red "Check the log file for details: $logFile"
    $_ | Out-File -Append $logFile
    exit 1
}
#endregion Main Execution
