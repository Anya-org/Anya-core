# Unified Test Runner v2.5
# Implements BIP-341/342/174 compliance
# Hexagonal Architecture Validation

param(
    [ValidateSet("modules","system","performance","security","all")]
    [string]$TestType = "all",
    [ValidateSet("basic","compliance","stress")]
    [string]$TestLevel = "compliance"
)

# Load BIP Configuration
$bipConfig = Get-Content "$PSScriptRoot/../configs/bip-config.json" | ConvertFrom-Json

# Initialize Test Environment
. "$PSScriptRoot/setup-test-environment.ps1" -Mode "validation"

# Hexagonal Test Execution
$testResults = @{
    modules = @{}
    system = @{}
    performance = @{}
    security = @{}
}

# Module Tests (Hexagonal Port 1)
if ($TestType -in @("modules","all")) {
    $testResults.modules = . "$PSScriptRoot/run-module-tests.ps1" `
        -TestLevel $TestLevel `
        -BIPVersion $bipConfig.requiredBIPs `
        -PSBTVersion 2
}

# System Tests (Hexagonal Port 2)
if ($TestType -in @("system","all")) {
    $testResults.system = . "$PSScriptRoot/run-system-tests.ps1" `
        -ComplianceLevel "strict" `
        -BIP341Required $true
}

# Performance Tests (Hexagonal Port 3)
if ($TestType -in @("performance","all")) {
    $testResults.performance = . "$PSScriptRoot/run-performance-tests.ps1" `
        -Iterations 1000 `
        -PSBTVersion 2
}

# Security Validation (Hexagonal Port 4)
if ($TestType -in @("security","all")) {
    $testResults.security = . "$PSScriptRoot/run-security-analysis.ps1" `
        -BIPPatterns $bipConfig.securityPatterns
}

# Generate Unified Report
$reportData = @{
    metadata = @{
        timestamp = (Get-Date).ToUniversalTime().ToString("o")
        bipCompliance = $bipConfig.requiredBIPs
        testFrameworkVersion = "2.5"
    }
    results = $testResults
}

$reportData | ConvertTo-Json -Depth 5 | Out-File "$PSScriptRoot/../test-results/unified-report.json"

Write-Host "Unified testing completed. Report generated: test-results/unified-report.json" -ForegroundColor Green 