# Enhanced Unified Test System - Version 2.0
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "--- Anya DAO - Enterprise-Grade Unified Test System v2.0      ---" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan

param(
    [switch]$SkipSlowTests,
    [switch]$QuickMode,
    [string]$FocusModule,
    [string]$TestResultsFormat = "JSON",
    [switch]$GenerateHtmlReport
)

# Track test execution time
$totalStartTime = Get-Date

# Initialize logging
$logFile = "test-results/test-run-$(Get-Date -Format 'yyyyMMdd-HHmmss').log"
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
    
    # Log to file
    Add-Content -Path $logFile -Value $logEntry
}

# Create unified test configuration
$testConfig = @{
    SkipSlowTests = $SkipSlowTests
    QuickMode = $QuickMode
    FocusModule = $FocusModule
    TestResultsFormat = $TestResultsFormat
    ParallelExecution = $true
    MaxThreads = 4
    GenerateHtmlReport = $GenerateHtmlReport
}

# Define test stages with enhanced metadata
$testStages = @(
    @{
        Name = "Environment Setup"
        Script = "scripts/setup-test-environment.ps1"
        Status = "Pending"
        Required = $true
        Description = "Prepares the test environment, directories, and dependencies"
        TimeEstimate = "15s"
        Category = "Setup"
        SupportsParallel = $false
    },
    @{
        Name = "Basic Contract Verification"
        Script = "scripts/verify-contracts-basic.ps1"
        Status = "Pending"
        Required = $true
        Description = "Performs syntax and basic validation of all Clarity contracts"
        TimeEstimate = "30s"
        Category = "Verification"
        SupportsParallel = $false
    },
    @{
        Name = "Module Tests"
        Script = "scripts/run-module-tests.ps1" 
        Status = "Pending"
        Required = $true
        Description = "Runs unit tests for individual modules and contracts"
        TimeEstimate = "2m"
        Category = "Testing"
        SupportsParallel = $true
    },
    @{
        Name = "Integration Tests"
        Script = "scripts/run-integration-tests.ps1"
        Status = "Pending"
        Required = $true
        Description = "Tests interactions between related modules"
        TimeEstimate = "3m"
        Category = "Testing"
        SupportsParallel = $false
    },
    @{
        Name = "System Tests"
        Script = "scripts/run-system-tests.ps1"
        Status = "Pending"
        Required = $true
        Description = "Executes full system workflows and end-to-end scenarios"
        TimeEstimate = "5m"
        Category = "Testing"
        SupportsParallel = $false
        SkipInQuickMode = $true
    },
    @{
        Name = "Compliance Checks"
        Script = "scripts/run-compliance-checks.ps1"
        Status = "Pending"
        Required = $true
        Description = "Validates compliance with Bitcoin Improvement Proposals (BIPs)"
        TimeEstimate = "1m"
        Category = "Compliance"
        SupportsParallel = $false
    },
    @{
        Name = "Performance Benchmarks"
        Script = "scripts/run-benchmarks.ps1"
        Status = "Pending"
        Required = $false
        Description = "Measures performance characteristics of contracts and workflows"
        TimeEstimate = "5m"
        Category = "Performance"
        SupportsParallel = $true
        SkipInQuickMode = $true
    },
    @{
        Name = "Security Analysis"
        Script = "scripts/run-security-analysis.ps1"
        Status = "Pending"
        Required = $false
        Description = "Scans for security vulnerabilities and best practices"
        TimeEstimate = "10m"
        Category = "Security"
        SupportsParallel = $false
        SkipInQuickMode = $true
    },
    @{
        Name = "Report Generation"
        Script = "scripts/generate-reports.ps1"
        Status = "Pending"
        Required = $true
        Description = "Generates comprehensive test reports and visualizations"
        TimeEstimate = "30s"
        Category = "Reporting"
        SupportsParallel = $false
        RunAlways = $true
    }
)

# Create the scripts directory if it doesn't exist
if (-not (Test-Path "scripts")) {
    New-Item -ItemType Directory -Path "scripts" -Force | Out-Null
    Write-Log "Created scripts directory" "INFO" "Green"
}

# Create test results directory
$testResultsDir = "test-results"
if (-not (Test-Path $testResultsDir)) {
    New-Item -ItemType Directory -Path $testResultsDir -Force | Out-Null
    Write-Log "Created test results directory: $testResultsDir" "INFO" "Green"
}

# Enhanced test results summary
$testResults = @{
    StartTime = $totalStartTime.ToString("yyyy-MM-dd HH:mm:ss")
    EndTime = ""
    TotalDuration = ""
    Configuration = $testConfig
    Stages = @{}
    OverallStatus = "PASS"
    ModuleCoverage = @{}
    SystemCoverage = 0.0
    FailedTests = @()
    WarningTests = @()
    Metrics = @{
        TotalContractsVerified = 0
        TotalModulesTested = 0
        TotalIntegrationsTested = 0
        TotalSystemScenariosTested = 0
        AverageCoverage = 0.0
        PerformanceScore = 0.0
        SecurityScore = 0.0
        ComplianceScore = 0.0
    }
}

# Check if Clarinet is installed, and download if necessary
Write-Log "Checking for Clarinet installation" "INFO" "Yellow"
$clarinetAvailable = $null -ne (Get-Command clarinet -ErrorAction SilentlyContinue)

if (-not $clarinetAvailable) {
    Write-Log "Clarinet not found. Attempting to download..." "WARN" "Yellow"
    if (Test-Path "scripts/download-clarinet.ps1") {
        $clarinetAvailable = & "scripts/download-clarinet.ps1"
    } else {
        Write-Log "download-clarinet.ps1 not found" "ERROR" "Red"
    }
}

# Apply filters based on configuration
if ($QuickMode) {
    Write-Log "Running in QUICK MODE - skipping slow tests" "INFO" "Yellow"
    $testStages = $testStages | Where-Object { -not $_.SkipInQuickMode -or $_.RunAlways }
}

if ($FocusModule) {
    Write-Log "Focus testing on module: $FocusModule" "INFO" "Yellow"
    # Add focus module to test config
    $testConfig.FocusModule = $FocusModule
}

# Execute each test stage
Write-Log "Executing Test Stages ($($testStages.Count) stages)" "INFO" "Yellow"
foreach ($stage in $testStages) {
    $stageStartTime = Get-Date
    $stageName = $stage.Name
    
    Write-Log "Stage: $stageName - $($stage.Description)" "STAGE" "Magenta"
    
    if (-not (Test-Path $stage.Script)) {
        Write-Log "Script not found: $($stage.Script)" "ERROR" "Red"
        $stage.Status = "SKIPPED"
        if ($stage.Required) {
            $testResults.OverallStatus = "FAIL"
            $testResults.FailedTests += "$stageName (Script not found)"
        } else {
            $testResults.WarningTests += "$stageName (Script not found - Optional)"
        }
    } else {
        try {
            # Pass relevant configuration to the stage script
            $scriptParams = @{
                Configuration = $testConfig
                ResultsDirectory = "$testResultsDir/$($stage.Category.ToLower())"
            }
            
            # Create category-specific results directory
            if (-not (Test-Path $scriptParams.ResultsDirectory)) {
                New-Item -ItemType Directory -Path $scriptParams.ResultsDirectory -Force | Out-Null
            }
            
            # Execute the stage script with parameters
            & $stage.Script @scriptParams
            $exitCode = $LASTEXITCODE
            
            if ($exitCode -eq 0) {
                $stage.Status = "PASS"
                Write-Log "$stageName completed successfully" "SUCCESS" "Green"
            } else {
                $stage.Status = "FAIL"
                Write-Log "$stageName failed with exit code $exitCode" "ERROR" "Red"
                if ($stage.Required) {
                    $testResults.OverallStatus = "FAIL"
                    $testResults.FailedTests += "$stageName (Exit Code: $exitCode)"
                } else {
                    $testResults.WarningTests += "$stageName (Exit Code: $exitCode - Optional)"
                }
            }
        } catch {
            $stage.Status = "ERROR"
            Write-Log "Error during $stageName: $_" "ERROR" "Red"
            if ($stage.Required) {
                $testResults.OverallStatus = "FAIL"
                $testResults.FailedTests += "$stageName (Error: $_)"
            } else {
                $testResults.WarningTests += "$stageName (Error: $_ - Optional)"
            }
        }
    }
    
    $stageDuration = (Get-Date) - $stageStartTime
    $testResults.Stages[$stageName] = @{
        Status = $stage.Status
        Duration = $stageDuration.TotalSeconds
        Required = $stage.Required
        Category = $stage.Category
        Description = $stage.Description
    }
    
    Write-Log "$stageName completed in $([math]::Round($stageDuration.TotalSeconds, 2)) seconds with status: $($stage.Status)" "INFO" "Yellow"
}

# Calculate total duration
$totalEndTime = Get-Date
$totalDuration = $totalEndTime - $totalStartTime
$testResults.EndTime = $totalEndTime.ToString("yyyy-MM-dd HH:mm:ss")
$testResults.TotalDuration = $totalDuration.TotalSeconds

# Generate final report
Write-Log "Test Summary" "SUMMARY" "Yellow"
Write-Log "Overall Status: $($testResults.OverallStatus)" "SUMMARY" $(if ($testResults.OverallStatus -eq "PASS") { "Green" } else { "Red" })
Write-Log "Total Duration: $([math]::Round($totalDuration.TotalSeconds, 2)) seconds" "SUMMARY" "Cyan"

foreach ($stageName in $testResults.Stages.Keys) {
    $stageResult = $testResults.Stages[$stageName]
    $statusColor = switch ($stageResult.Status) {
        "PASS" { "Green" }
        "FAIL" { "Red" }
        "ERROR" { "Red" }
        "SKIPPED" { "Yellow" }
        default { "White" }
    }
    
    $requiredLabel = if ($stageResult.Required) { "[Required]" } else { "[Optional]" }
    Write-Log "  $stageName $requiredLabel`: $($stageResult.Status)" "SUMMARY" $statusColor
    Write-Log "    Category: $($stageResult.Category)" "SUMMARY" "Gray"
    Write-Log "    Duration: $([math]::Round($stageResult.Duration, 2)) seconds" "SUMMARY" "Gray"
}

# Save test results in requested format
if ($TestResultsFormat -eq "JSON") {
    $testResultsJson = $testResults | ConvertTo-Json -Depth 5
    Set-Content -Path "$testResultsDir/test-results-summary.json" -Value $testResultsJson
    Write-Log "Test results saved to: $testResultsDir/test-results-summary.json" "INFO" "Cyan"
}

# Optionally generate HTML report
if ($GenerateHtmlReport) {
    if (Test-Path "scripts/generate-html-report.ps1") {
        Write-Log "Generating HTML report..." "INFO" "Yellow"
        & "scripts/generate-html-report.ps1" -ResultsFile "$testResultsDir/test-results-summary.json"
    } else {
        Write-Log "HTML report generator script not found" "WARN" "Yellow"
    }
}

Write-Log "Test execution complete!" "INFO" "Cyan"
Write-Log "================================================================" "INFO" "Cyan" 