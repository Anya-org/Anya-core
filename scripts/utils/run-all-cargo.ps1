# Run-All-Cargo Script for Anya Bitcoin Project
param(
    [switch]$DryRun,
    [switch]$SkipFailedPackages
)

Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "--- Anya Bitcoin - Cargo Build & Test Script                  ---" -ForegroundColor Cyan
Write-Host "--- Author: bo_thebig <botshelomokokoka@gmail.com>            ---" -ForegroundColor Cyan
if ($DryRun) {
    Write-Host "--- DRY RUN MODE - No commands will be executed              ---" -ForegroundColor Yellow
}
if ($SkipFailedPackages) {
    Write-Host "--- SKIP FAILED PACKAGES MODE - Will continue on errors      ---" -ForegroundColor Yellow
}
Write-Host "================================================================" -ForegroundColor Cyan

# Set error handling
$ErrorActionPreference = "Stop"

# Configure logging
$logFile = "logs/run-all-cargo-$(Get-Date -Format 'yyyyMMdd-HHmmss').log"
$logDir = Split-Path -Path $logFile -Parent

if (-not (Test-Path $logDir)) {
    New-Item -ItemType Directory -Path $logDir -Force | Out-Null
}

function Write-Log {
    param(
        [string]$Message,
        [string]$Level = "INFO",
        [string]$ForegroundColor = "White"
    )
    
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logEntry = "[$timestamp] [$Level] $Message"
    
    # Log to console
    Write-Host $logEntry -ForegroundColor $ForegroundColor
    
    # Log to file - use Out-File with append instead of Add-Content to avoid locking issues
    try {
        $logEntry | Out-File -FilePath $logFile -Append -Encoding utf8 -ErrorAction SilentlyContinue
    } catch {
        # If we can't write to the log file, just continue
        Write-Host "Warning: Unable to write to log file" -ForegroundColor Yellow
    }
}

# Track execution time
$totalStartTime = Get-Date
Write-Log "Starting Run-All-Cargo script" "INFO" "Cyan"
if ($DryRun) {
    Write-Log "Running in DRY RUN mode - commands will be simulated" "INFO" "Yellow"
}
if ($SkipFailedPackages) {
    Write-Log "Skip failed packages mode enabled - will continue on errors" "INFO" "Yellow"
}

# Define Rust packages to check and test based on workspace
$packages = @(
    "anya-bitcoin",
    "core",
    "mobile-sdk",
    "dependencies"
)

# Function to check if a package exists
function Test-PackageExists {
    param (
        [string]$PackageName
    )
    
    # First check if it's a direct directory with Cargo.toml
    if (Test-Path -Path "$PackageName/Cargo.toml") {
        return $true
    }
    
    # Try running cargo metadata to see if the package is recognized
    try {
        $output = Invoke-Expression "cargo metadata --no-deps --format-version=1 2>&1"
        if ($output -match "`"name`":\s*`"$PackageName`"") {
            return $true
        }
    } catch {
        # If cargo metadata fails, just continue
    }
    
    return $false
}

# Filter out packages that don't exist
$existingPackages = @()
foreach ($package in $packages) {
    if (Test-PackageExists -PackageName $package) {
        $existingPackages += $package
        Write-Log "Found package: $package" "INFO" "Green"
    } else {
        Write-Log "Package not found: $package" "INFO" "Yellow"
    }
}
$packages = $existingPackages

# Define execution stages
$stages = @(
    @{
        Name = "Fix Dependency Issues"
        IsFixDependencies = $true
        Required = $true
        Description = "Fixes dependency issues in Cargo.toml files"
        EstimatedTime = "5s"
    },
    @{
        Name = "Clean Build Artifacts"
        Script = "cargo clean"
        Required = $true
        Description = "Cleans build artifacts and temporary files"
        EstimatedTime = "5s"
    },
    @{
        Name = "Package Checks"
        IsPackageCheck = $true
        Required = $true
        Description = "Checks if each package compiles without errors"
        EstimatedTime = "1m"
    },
    @{
        Name = "Run Clippy"
        Script = "cargo clippy --all-targets -- -D warnings"
        Required = $true
        Description = "Runs the Rust linter to detect potential issues"
        EstimatedTime = "15s"
    },
    @{
        Name = "Run Tests"
        Script = "cargo test --all"
        Required = $true
        Description = "Runs all unit tests in the project"
        EstimatedTime = "30s"
    },
    @{
        Name = "Build Release"
        Script = "cargo build --release"
        Required = $false
        Description = "Builds optimized release binaries"
        EstimatedTime = "2m"
    }
)

# Run each stage
$overallStatus = "PASS"
$stageResults = @{}

foreach ($stage in $stages) {
    $stageName = $stage.Name
    $stageStartTime = Get-Date
    Write-Log "Starting stage: $stageName" "STAGE" "Magenta"
    
    if ($DryRun) {
        if ($stage.IsFixDependencies) {
            Write-Log "DRY RUN: Would fix dependency issues in Cargo.toml files" "DRY-RUN" "Yellow"
        }
        elseif ($stage.IsPackageCheck) {
            foreach ($package in $packages) {
                Write-Log "DRY RUN: Would execute 'cargo check --package $package'" "DRY-RUN" "Yellow"
            }
        } else {
            Write-Log "DRY RUN: Would execute '$($stage.Script)'" "DRY-RUN" "Yellow"
        }
        Write-Log "Description: $($stage.Description)" "DRY-RUN" "Gray"
        Write-Log "Estimated time: $($stage.EstimatedTime)" "DRY-RUN" "Gray"
        
        # In dry run mode, we simulate successful execution with a short delay
        Start-Sleep -Seconds 1
        
        $stageResults[$stageName] = @{
            Status = "SIMULATED"
            Duration = 1
            Description = $stage.Description
            EstimatedTime = $stage.EstimatedTime
        }
    }
    else {
        try {
            if ($stage.IsFixDependencies) {
                Write-Log "Fixing dependency issues..." "INFO" "Yellow"
                
                # Update workspace Cargo.toml
                $workspaceCargoPath = "Cargo.toml"
                if (Test-Path $workspaceCargoPath) {
                    $cargoContent = Get-Content $workspaceCargoPath -Raw
                    $modified = $false
                    
                    # Check for secp256k1 dependency with bip340 feature
                    if ($cargoContent -match "secp256k1\s*=\s*\{\s*version\s*=\s*`"([^`"]+)`"\s*,\s*features\s*=\s*\[") {
                        $originalLine = $Matches[0]
                        $versionMatch = $Matches[1]
                        
                        # Check if any features include bip340
                        if ($cargoContent -match "secp256k1.*bip340") {
                            Write-Log "Found secp256k1 with bip340 feature in workspace Cargo.toml" "INFO" "Yellow"
                            
                            # Check for a more recent version of secp256k1
                            $output = Invoke-Expression "cargo search secp256k1 --limit 1 2>&1"
                            if ($output -match "secp256k1 = `"(\d+\.\d+\.\d+)`"") {
                                $latestVersion = $Matches[1]
                                Write-Log "Latest secp256k1 version is $latestVersion" "INFO" "Green"
                                
                                if ([version]$latestVersion -gt [version]$versionMatch) {
                                    # Update to use latest version
                                    $newLine = $originalLine -replace "`"$versionMatch`"", "`"$latestVersion`""
                                    $cargoContent = $cargoContent -replace [regex]::Escape($originalLine), $newLine
                                    $modified = $true
                                    Write-Log "Updated workspace Cargo.toml to use secp256k1 $latestVersion" "SUCCESS" "Green"
                                }
                            }
                            
                            # Check if rand-std feature is missing and add it
                            if (-not ($cargoContent -match "secp256k1.*rand-std")) {
                                $featurePattern = "(secp256k1.*features\s*=\s*\[[^\]]+)"
                                if ($cargoContent -match $featurePattern) {
                                    $featureSection = $Matches[1]
                                    if ($featureSection.EndsWith("]")) {
                                        $newFeatureSection = $featureSection.Substring(0, $featureSection.Length - 1) + ', "rand-std"]'
                                    } else {
                                        $newFeatureSection = $featureSection + ', "rand-std"'
                                    }
                                    $cargoContent = $cargoContent -replace [regex]::Escape($featureSection), $newFeatureSection
                                    $modified = $true
                                    Write-Log "Added rand-std feature to secp256k1 in workspace Cargo.toml" "SUCCESS" "Green"
                                }
                            }
                        }
                    }
                    
                    if ($modified) {
                        Set-Content -Path $workspaceCargoPath -Value $cargoContent
                    }
                }
                
                # Check core/Cargo.toml for mobile feature
                $coreCargoPath = "core/Cargo.toml"
                if (Test-Path $coreCargoPath) {
                    $cargoContent = Get-Content $coreCargoPath -Raw
                    
                    # Fix mobile feature if it exists
                    if ($cargoContent -match "mobile\s*=\s*\[\s*`"([^`"]+)`"\s*,\s*`"([^`"]+)`"\s*\]") {
                        $originalLine = $Matches[0]
                        
                        # Add rand-std feature if not present
                        if (-not $originalLine.Contains("secp256k1/rand-std") -and 
                            ($originalLine.Contains("bitcoin/serde") -or $originalLine.Contains("secp256k1/global-context"))) {
                            $newLine = $originalLine.TrimEnd(']') + ', "secp256k1/rand-std"]'
                            $cargoContent = $cargoContent -replace [regex]::Escape($originalLine), $newLine
                            Set-Content -Path $coreCargoPath -Value $cargoContent
                            Write-Log "Updated mobile feature in core/Cargo.toml to include rand-std" "SUCCESS" "Green"
                        }
                    }
                }
                
                $stageResults[$stageName] = @{
                    Status = "PASS"
                    Duration = ((Get-Date) - $stageStartTime).TotalSeconds
                }
            }
            elseif ($stage.IsPackageCheck) {
                $packageResults = @{}
                $packageSuccesses = 0
                $packageFailures = 0
                
                foreach ($package in $packages) {
                    Write-Log "Checking package: $package" "INFO" "Yellow"
                    $command = "cargo check --package $package"
                    $output = Invoke-Expression $command 2>&1
                    $exitCode = $LASTEXITCODE
                    
                    # Log output (selected lines to avoid verbosity)
                    $output | Where-Object { $_ -match 'error:|warning:' } | ForEach-Object {
                        Write-Log "  $_" "OUTPUT" "DarkGray"
                    }
                    
                    if ($exitCode -eq 0) {
                        $packageResults[$package] = "PASS"
                        $packageSuccesses++
                        Write-Log "✅ Package $package check passed" "SUCCESS" "Green"
                    } else {
                        $packageResults[$package] = "FAIL"
                        $packageFailures++
                        Write-Log "❌ Package $package check failed with exit code $exitCode" "ERROR" "Red"
                        
                        # Exit early if we're not skipping failed packages
                        if (-not $SkipFailedPackages) {
                            throw "Package check failed for $package"
                        }
                    }
                }
                
                $stageResults[$stageName] = @{
                    Status = if ($packageFailures -eq 0) { "PASS" } else { "PARTIAL" }
                    Duration = ((Get-Date) - $stageStartTime).TotalSeconds
                    PackageResults = $packageResults
                    SuccessCount = $packageSuccesses
                    FailureCount = $packageFailures
                }
                
                if ($packageFailures -gt 0 -and $stage.Required -and -not $SkipFailedPackages) {
                    $overallStatus = "FAIL"
                }
            }
            elseif ($stage.Script.StartsWith("cargo ")) {
                # Execute Cargo command directly
                $command = $stage.Script
                Write-Log "Executing: $command" "INFO" "Yellow"
                $output = Invoke-Expression $command 2>&1
                $exitCode = $LASTEXITCODE
                
                # Log output (selected lines to avoid verbosity)
                $output | Where-Object { $_ -match 'error:|warning:' } | ForEach-Object {
                    Write-Log "  $_" "OUTPUT" "DarkGray"
                }
                
                if ($exitCode -eq 0) {
                    Write-Log "✅ $stageName completed successfully" "SUCCESS" "Green"
                    $stageResults[$stageName] = @{
                        Status = "PASS"
                        Duration = ((Get-Date) - $stageStartTime).TotalSeconds
                    }
                } else {
                    Write-Log "❌ $stageName failed with exit code $exitCode" "ERROR" "Red"
                    $stageResults[$stageName] = @{
                        Status = "FAIL"
                        Duration = ((Get-Date) - $stageStartTime).TotalSeconds
                        ExitCode = $exitCode
                    }
                    
                    if ($stage.Required -and -not $SkipFailedPackages) {
                        $overallStatus = "FAIL"
                        break
                    }
                }
            }
            else {
                Write-Log "⚠️ Command not recognized: $($stage.Script)" "WARN" "Yellow"
                $stageResults[$stageName] = @{
                    Status = "SKIPPED"
                    Reason = "Command not recognized"
                }
                
                if ($stage.Required -and -not $SkipFailedPackages) {
                    $overallStatus = "FAIL"
                    break
                }
            }
        } catch {
            $errorMessage = $_.Exception.Message
            Write-Log "❌ Error executing $stageName - $errorMessage" "ERROR" "Red"
            $stageResults[$stageName] = @{
                Status = "ERROR"
                Duration = ((Get-Date) - $stageStartTime).TotalSeconds
                Error = $errorMessage
            }
            
            if ($stage.Required -and -not $SkipFailedPackages) {
                $overallStatus = "FAIL"
                break
            }
        }
    }
}

# Calculate total duration
$totalEndTime = Get-Date
$totalDuration = ($totalEndTime - $totalStartTime).TotalSeconds

# Final summary
Write-Log "================================================================" "SUMMARY" "Cyan"
Write-Log "Run-All-Cargo Execution Summary" "SUMMARY" "Cyan"
if ($DryRun) {
    Write-Log "DRY RUN MODE - Simulated Execution" "SUMMARY" "Yellow"
}
Write-Log "----------------------------------------------------------------" "SUMMARY" "Cyan"
Write-Log "Overall Status: $overallStatus" "SUMMARY" $(if ($overallStatus -eq "PASS") { "Green" } else { "Red" })
Write-Log "Total Duration: $([math]::Round($totalDuration, 2)) seconds" "SUMMARY" "Yellow"
Write-Log "----------------------------------------------------------------" "SUMMARY" "Cyan"

foreach ($stageName in $stageResults.Keys) {
    $result = $stageResults[$stageName]
    $statusColor = switch ($result.Status) {
        "PASS" { "Green" }
        "FAIL" { "Red" }
        "ERROR" { "Red" }
        "SKIPPED" { "Yellow" }
        "SIMULATED" { "Yellow" }
        "PARTIAL" { "Yellow" }
        default { "White" }
    }
    
    Write-Log "  $stageName - $($result.Status)" "SUMMARY" $statusColor
    
    # For package checks, show detailed results
    if ($result.PackageResults) {
        Write-Log "    Package Results: $($result.SuccessCount) passed, $($result.FailureCount) failed" "SUMMARY" "Gray"
        foreach ($package in $result.PackageResults.Keys) {
            $packageStatus = $result.PackageResults[$package]
            $packageColor = if ($packageStatus -eq "PASS") { "Green" } else { "Red" }
            Write-Log "      $package`: $packageStatus" "SUMMARY" $packageColor
        }
    }
    
    if ($result.Duration) {
        Write-Log "    Duration: $([math]::Round($result.Duration, 2)) seconds" "SUMMARY" "Gray"
    }
    if ($result.EstimatedTime) {
        Write-Log "    Estimated Time: $($result.EstimatedTime)" "SUMMARY" "Gray"
    }
    if ($result.Description) {
        Write-Log "    Description: $($result.Description)" "SUMMARY" "Gray"
    }
    if ($result.ExitCode) {
        Write-Log "    Exit Code: $($result.ExitCode)" "SUMMARY" "Gray"
    }
    if ($result.Error) {
        Write-Log "    Error: $($result.Error)" "SUMMARY" "Gray"
    }
    if ($result.Reason) {
        Write-Log "    Reason: $($result.Reason)" "SUMMARY" "Gray"
    }
}

Write-Log "----------------------------------------------------------------" "SUMMARY" "Cyan"
Write-Log "Execution completed at: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" "SUMMARY" "Cyan"
Write-Log "Log file: $logFile" "SUMMARY" "Cyan"
Write-Log "================================================================" "SUMMARY" "Cyan" 