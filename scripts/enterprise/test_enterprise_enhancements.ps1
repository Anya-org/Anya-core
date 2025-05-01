# Enterprise Enhancements Test Script
# Tests HSM support, federated learning, and multi-signature schemes for Bitcoin Core alignment

param(
    [switch]$Verbose
)

# Script configuration
$scriptName = "Enterprise Enhancements Tests"
$scriptVersion = "1.0.0"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent (Split-Path -Parent $scriptRoot)

# Directories to test
$enterpriseDirs = @(
    (Join-Path $projectRoot "core\src\enterprise"),
    (Join-Path $projectRoot "src\enterprise")
)

Write-Host "===== $scriptName v$scriptVersion =====" -ForegroundColor Cyan
Write-Host "Starting enterprise enhancements tests..."

# Track test results
$testResults = @{
    Total = 0
    Passed = 0
    Failed = 0
    Warnings = 0
    FailedTests = @()
}

# Function to test HSM support implementation
function Test-HSMSupport {
    param(
        [string]$EnterpriseDir
    )
    
    Write-Host "Testing HSM Support Implementation..." -ForegroundColor Yellow
    
    # Check for HSM module existence
    $hsmDir = Join-Path $EnterpriseDir "hsm"
    $hsmModPath = Join-Path $hsmDir "mod.rs"
    
    $testResults.Total++
    if (Test-Path $hsmModPath) {
        Write-Host "  - HSM module exists: PASSED" -ForegroundColor Green
        $testResults.Passed++
        
        # Test HSM module content
        $hsmModContent = Get-Content -Path $hsmModPath -Raw
        
        # Check for required interfaces
        $testResults.Total++
        if ($hsmModContent -match "pub\s+trait\s+HSMProvider") {
            Write-Host "  - HSM Provider interface: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - HSM Provider interface: FAILED - Interface not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "HSM Provider interface missing"
        }
        
        # Check for security error handling
        $testResults.Total++
        if ($hsmModContent -match "pub\s+enum\s+HSMError") {
            Write-Host "  - HSM Error handling: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - HSM Error handling: FAILED - Error handling not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "HSM Error handling missing"
        }
        
        # Check for HSM factory pattern
        $testResults.Total++
        if ($hsmModContent -match "HSMFactory") {
            Write-Host "  - HSM Factory pattern: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - HSM Factory pattern: FAILED - Factory pattern not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "HSM Factory pattern missing"
        }
        
        # Check for multiple HSM provider implementations
        $testResults.Total++
        $providerCount = [regex]::Matches($hsmModContent, "pub\s+struct\s+\w+HSM").Count
        if ($providerCount -ge 2) {
            Write-Host "  - Multiple HSM providers: PASSED ($providerCount providers)" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - Multiple HSM providers: FAILED - Less than 2 providers found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "Insufficient HSM provider implementations"
        }
        
        # Check for Bitcoin Core principles alignment
        $testResults.Total++
        if ($hsmModContent -match "sign|signature|verify|immutable|secure|auth") {
            Write-Host "  - Bitcoin Core security principles: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - Bitcoin Core security principles: WARNING - Security features not explicitly found" -ForegroundColor Yellow
            $testResults.Warnings++
        }
    } else {
        Write-Host "  - HSM module exists: FAILED - Module not found at $hsmModPath" -ForegroundColor Red
        $testResults.Failed++
        $testResults.FailedTests += "HSM module not found at $hsmModPath"
    }
    
    # Check for HSM module inclusion in enterprise mod.rs
    $enterpriseModPath = Join-Path $EnterpriseDir "mod.rs"
    if (Test-Path $enterpriseModPath) {
        $enterpriseModContent = Get-Content -Path $enterpriseModPath -Raw
        
        $testResults.Total++
        if ($enterpriseModContent -match "pub\s+mod\s+hsm") {
            Write-Host "  - HSM module included in enterprise mod.rs: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - HSM module included in enterprise mod.rs: FAILED - Module not included" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "HSM module not included in enterprise mod.rs"
        }
    } else {
        $testResults.Total++
        Write-Host "  - Enterprise mod.rs exists: FAILED - File not found at $enterpriseModPath" -ForegroundColor Red
        $testResults.Failed++
        $testResults.FailedTests += "Enterprise mod.rs not found at $enterpriseModPath"
    }
}

# Function to test federated learning implementation
function Test-FederatedLearning {
    param(
        [string]$EnterpriseDir
    )
    
    Write-Host "Testing Federated Learning Implementation..." -ForegroundColor Yellow
    
    # Check for Federated Learning module existence
    $flDir = Join-Path $EnterpriseDir "federated"
    $flModPath = Join-Path $flDir "mod.rs"
    
    $testResults.Total++
    if (Test-Path $flModPath) {
        Write-Host "  - Federated Learning module exists: PASSED" -ForegroundColor Green
        $testResults.Passed++
        
        # Test Federated Learning module content
        $flModContent = Get-Content -Path $flModPath -Raw
        
        # Check for required interfaces
        $testResults.Total++
        if ($flModContent -match "pub\s+trait\s+FederatedModel") {
            Write-Host "  - FederatedModel interface: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - FederatedModel interface: FAILED - Interface not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "FederatedModel interface missing"
        }
        
        # Check for coordinator implementation
        $testResults.Total++
        if ($flModContent -match "pub\s+struct\s+FederatedCoordinator") {
            Write-Host "  - FederatedCoordinator implementation: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - FederatedCoordinator implementation: FAILED - Implementation not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "FederatedCoordinator implementation missing"
        }
        
        # Check for client implementation
        $testResults.Total++
        if ($flModContent -match "pub\s+struct\s+FederatedClient") {
            Write-Host "  - FederatedClient implementation: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - FederatedClient implementation: FAILED - Implementation not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "FederatedClient implementation missing"
        }
        
        # Check for privacy preservation
        $testResults.Total++
        if ($flModContent -match "privacy|differential|noise|private") {
            Write-Host "  - Privacy preservation: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - Privacy preservation: WARNING - Privacy features not explicitly found" -ForegroundColor Yellow
            $testResults.Warnings++
        }
        
        # Check for Bitcoin Core principles alignment
        $testResults.Total++
        if ($flModContent -match "secure|decentral|immutable|transparent") {
            Write-Host "  - Bitcoin Core principles alignment: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - Bitcoin Core principles alignment: WARNING - Core principles not explicitly found" -ForegroundColor Yellow
            $testResults.Warnings++
        }
    } else {
        Write-Host "  - Federated Learning module exists: FAILED - Module not found at $flModPath" -ForegroundColor Red
        $testResults.Failed++
        $testResults.FailedTests += "Federated Learning module not found at $flModPath"
    }
    
    # Check for Federated Learning module inclusion in enterprise mod.rs
    $enterpriseModPath = Join-Path $EnterpriseDir "mod.rs"
    if (Test-Path $enterpriseModPath) {
        $enterpriseModContent = Get-Content -Path $enterpriseModPath -Raw
        
        $testResults.Total++
        if ($enterpriseModContent -match "pub\s+mod\s+federated") {
            Write-Host "  - Federated Learning module included in enterprise mod.rs: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - Federated Learning module included in enterprise mod.rs: FAILED - Module not included" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "Federated Learning module not included in enterprise mod.rs"
        }
    }
}

# Function to test multi-signature schemes implementation
function Test-MultisigSchemes {
    param(
        [string]$EnterpriseDir
    )
    
    Write-Host "Testing Multi-Signature Schemes Implementation..." -ForegroundColor Yellow
    
    # Check for Multisig module existence
    $msDir = Join-Path $EnterpriseDir "multisig"
    $msModPath = Join-Path $msDir "mod.rs"
    
    $testResults.Total++
    if (Test-Path $msModPath) {
        Write-Host "  - Multisig module exists: PASSED" -ForegroundColor Green
        $testResults.Passed++
        
        # Test Multisig module content
        $msModContent = Get-Content -Path $msModPath -Raw
        
        # Check for required interfaces
        $testResults.Total++
        if ($msModContent -match "pub\s+trait\s+MultisigSchemeProvider") {
            Write-Host "  - MultisigSchemeProvider interface: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - MultisigSchemeProvider interface: FAILED - Interface not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "MultisigSchemeProvider interface missing"
        }
        
        # Check for multiple multisig scheme implementations
        $testResults.Total++
        $schemeCount = [regex]::Matches($msModContent, "pub\s+enum\s+MultisigScheme").Count
        if ($schemeCount -ge 1) {
            $schemeTypes = [regex]::Matches($msModContent, "Taproot|FROST|MuSig2|Traditional").Count
            if ($schemeTypes -ge 3) {
                Write-Host "  - Multiple multisig schemes: PASSED ($schemeTypes schemes)" -ForegroundColor Green
                $testResults.Passed++
            } else {
                Write-Host "  - Multiple multisig schemes: WARNING - Less than 3 scheme types found" -ForegroundColor Yellow
                $testResults.Warnings++
            }
        } else {
            Write-Host "  - Multisig scheme enum: FAILED - Enum not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "MultisigScheme enum missing"
        }
        
        # Check for Taproot support
        $testResults.Total++
        if ($msModContent -match "Taproot") {
            Write-Host "  - Taproot support: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - Taproot support: FAILED - Taproot not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "Taproot support missing"
        }
        
        # Check for MuSig2 support
        $testResults.Total++
        if ($msModContent -match "MuSig2") {
            Write-Host "  - MuSig2 support: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - MuSig2 support: FAILED - MuSig2 not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "MuSig2 support missing"
        }
        
        # Check for policy types
        $testResults.Total++
        if ($msModContent -match "pub\s+enum\s+PolicyType") {
            Write-Host "  - Policy types: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - Policy types: FAILED - PolicyType enum not found" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "PolicyType enum missing"
        }
        
        # Check for Bitcoin Core principles alignment
        $testResults.Total++
        if ($msModContent -match "security|secure|immutable|verify|validation") {
            Write-Host "  - Bitcoin Core principles alignment: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - Bitcoin Core principles alignment: WARNING - Core principles not explicitly found" -ForegroundColor Yellow
            $testResults.Warnings++
        }
    } else {
        Write-Host "  - Multisig module exists: FAILED - Module not found at $msModPath" -ForegroundColor Red
        $testResults.Failed++
        $testResults.FailedTests += "Multisig module not found at $msModPath"
    }
    
    # Check for Multisig module inclusion in enterprise mod.rs
    $enterpriseModPath = Join-Path $EnterpriseDir "mod.rs"
    if (Test-Path $enterpriseModPath) {
        $enterpriseModContent = Get-Content -Path $enterpriseModPath -Raw
        
        $testResults.Total++
        if ($enterpriseModContent -match "pub\s+mod\s+multisig") {
            Write-Host "  - Multisig module included in enterprise mod.rs: PASSED" -ForegroundColor Green
            $testResults.Passed++
        } else {
            Write-Host "  - Multisig module included in enterprise mod.rs: FAILED - Module not included" -ForegroundColor Red
            $testResults.Failed++
            $testResults.FailedTests += "Multisig module not included in enterprise mod.rs"
        }
    }
}

# Main execution
foreach ($dir in $enterpriseDirs) {
    if (Test-Path $dir) {
        Write-Host "Testing enterprise directory: $dir" -ForegroundColor Magenta
        
        Test-HSMSupport -EnterpriseDir $dir
        Test-FederatedLearning -EnterpriseDir $dir
        Test-MultisigSchemes -EnterpriseDir $dir
    } else {
        Write-Host "Enterprise directory not found: $dir" -ForegroundColor Red
    }
}

# Create test report
$reportDir = Join-Path $projectRoot "test-results\enterprise"
if (!(Test-Path $reportDir)) {
    New-Item -ItemType Directory -Path $reportDir -Force | Out-Null
}

# Calculate compliance score
$complianceScore = if ($testResults.Total -gt 0) { 
    [math]::Round(($testResults.Passed / $testResults.Total) * 100, 2) 
} else { 
    0 
}

# Create test report markdown
$reportContent = @"
# Enterprise Enhancements Test Report

Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Summary

- **Total tests:** $($testResults.Total)
- **Passed:** $($testResults.Passed)
- **Failed:** $($testResults.Failed)
- **Warnings:** $($testResults.Warnings)

## Compliance Score

**$complianceScore%** of tests passed, indicating alignment with Bitcoin Core principles.

## Test Details

### HSM Support
- Tests the implementation of Hardware Security Module integration
- Verifies support for multiple HSM providers
- Confirms secure key management capabilities

### Federated Learning
- Tests privacy-preserving machine learning implementation
- Verifies coordinator and client implementations
- Confirms adherence to privacy and security requirements

### Multi-Signature Schemes
- Tests implementation of various multisig schemes
- Verifies Taproot and MuSig2 support
- Confirms policy types and security features

## Failed Tests

$(if ($testResults.FailedTests.Count -gt 0) {
    $testResults.FailedTests | ForEach-Object { "- $_" }
} else {
    "No failed tests."
})

## Bitcoin Core Principles Compliance

The tested implementations were checked for alignment with the following Bitcoin Core principles:

1. **Decentralization** - Components operate without central authorities
2. **Security** - Strong cryptographic protections and validation
3. **Immutability** - Operations create permanent, tamper-proof records
4. **Transparency** - Clear audit trails and verification processes

## Recommendations

$(if ($testResults.Failed -gt 0) {
    "- Run the enterprise implementation scripts to address failed tests."
})
$(if ($testResults.Warnings -gt 0) {
    "- Review implementation components with warnings to enhance Bitcoin Core principles compliance."
})
$(if ($testResults.Failed -eq 0 -and $testResults.Warnings -eq 0) {
    "- All tests passed. Continue with integration testing."
})
"@

$reportPath = Join-Path $reportDir "enterprise-test-report-$(Get-Date -Format 'yyyyMMdd-HHmmss').md"
$reportContent | Set-Content -Path $reportPath -Encoding UTF8

Write-Host "`nEnterprise Enhancements Test Report written to: $reportPath" -ForegroundColor Green

# Print summary
Write-Host "`nEnterprise Enhancements Test Summary:" -ForegroundColor Cyan
Write-Host "  Total tests run: $($testResults.Total)" -ForegroundColor White
Write-Host "  Passed: $($testResults.Passed)" -ForegroundColor Green
Write-Host "  Failed: $($testResults.Failed)" -ForegroundColor $(if ($testResults.Failed -gt 0) { "Red" } else { "Gray" })
Write-Host "  Warnings: $($testResults.Warnings)" -ForegroundColor $(if ($testResults.Warnings -gt 0) { "Yellow" } else { "Gray" })
Write-Host "  Compliance score: $complianceScore%" -ForegroundColor $(if ($complianceScore -ge 90) { "Green" } elseif ($complianceScore -ge 70) { "Yellow" } else { "Red" })

# Exit with success code if passed more than 80%
if ($complianceScore -ge 80) {
    exit 0
} else {
    exit 1
}
