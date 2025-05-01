# API Standardization Test Script
# Verifies that API endpoints conform to standardized conventions and Bitcoin Core principles

param(
    [switch]$Verbose
)

# Script configuration
$scriptName = "API Standardization Tests"
$scriptVersion = "1.0.0"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent (Split-Path -Parent $scriptRoot)

# Directories to process
$apiDirs = @(
    (Join-Path $projectRoot "src\api"),
    (Join-Path $projectRoot "anya-bitcoin\src\api"),
    (Join-Path $projectRoot "core\src\api")
)

Write-Host "===== $scriptName v$scriptVersion =====" -ForegroundColor Cyan
Write-Host "Starting API standardization tests..."

# Test results tracking
$testResults = @{
    Total = 0
    Passed = 0
    Failed = 0
    Warnings = 0
    FailedEndpoints = @()
}

# Standard compliance patterns
$endpointPatterns = @{
    "GET" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
    "POST" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
    "PUT" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
    "DELETE" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
    "PATCH" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
}

# Bitcoin Core principle verification
$securityRules = @(
    @{ 
        Name = "Immutability Protection"
        Pattern = "immutable|audit|log|track|history|trace"
        Required = $true
        AppliesTo = @("POST", "PUT", "DELETE", "PATCH")
    },
    @{ 
        Name = "Authentication Check"
        Pattern = "auth|verify|token|sign|security"
        Required = $true
        AppliesTo = @("POST", "PUT", "DELETE", "PATCH")
    },
    @{ 
        Name = "Input Validation"
        Pattern = "valid|sanitize|check|verify"
        Required = $true
        AppliesTo = @("GET", "POST", "PUT", "DELETE", "PATCH")
    },
    @{ 
        Name = "Idempotency Support"
        Pattern = "idempotent|idempotency|idempotence"
        Required = $false
        AppliesTo = @("POST", "PUT", "PATCH")
    }
)

# Function to validate if an endpoint follows standards
function Test-StandardEndpoint {
    param(
        [string]$Endpoint,
        [string]$Method,
        [string]$FilePath,
        [string]$FunctionName
    )
    
    $testResults.Total++
    $fileName = Split-Path -Leaf $FilePath
    $passed = $true
    $warnings = 0
    
    Write-Host "  Testing endpoint: $Method $Endpoint ($fileName)" -ForegroundColor Yellow
    
    # Check endpoint pattern
    if ($endpointPatterns.ContainsKey($Method)) {
        $pattern = $endpointPatterns[$Method]
        $patternMatch = $Endpoint -match $pattern
        
        if ($patternMatch) {
            Write-Host "    - Endpoint pattern: PASSED" -ForegroundColor Green
        } else {
            Write-Host "    - Endpoint pattern: FAILED - Does not match standard pattern" -ForegroundColor Red
            $passed = $false
        }
    } else {
        Write-Host "    - Endpoint pattern: FAILED - Invalid HTTP method" -ForegroundColor Red
        $passed = $false
    }
    
    # Check if endpoint includes version
    if ($Endpoint -notmatch "\/v\d+\/") {
        Write-Host "    - API versioning: FAILED - Missing version (v1, v2, etc.)" -ForegroundColor Red
        $passed = $false
    } else {
        Write-Host "    - API versioning: PASSED" -ForegroundColor Green
    }
    
    # Check kebab-case for path segments
    $segments = $Endpoint -split "/"
    $nonKebabSegments = $segments | Where-Object { $_ -match "^[a-zA-Z0-9]+[A-Z][a-zA-Z0-9]*$" }
    
    if ($nonKebabSegments.Count -gt 0) {
        Write-Host "    - Naming convention: FAILED - Non kebab-case segments: $($nonKebabSegments -join ', ')" -ForegroundColor Red
        $passed = $false
    } else {
        Write-Host "    - Naming convention: PASSED" -ForegroundColor Green
    }
    
    # Check implementation file for security rule compliance
    if (Test-Path $FilePath) {
        $fileContent = Get-Content $FilePath -Raw
        
        # Try to find the specific function
        $functionContent = ""
        if (-not [string]::IsNullOrEmpty($FunctionName)) {
            $functionMatch = [regex]::Match($fileContent, "(?:fn|async\s+fn)\s+$FunctionName.*?(?:}(?:\s*\})*\s*$|(?=(?:fn|async\s+fn)))", [System.Text.RegularExpressions.RegexOptions]::Singleline)
            if ($functionMatch.Success) {
                $functionContent = $functionMatch.Value
            }
        }
        
        # If function content couldn't be extracted, use whole file
        if ([string]::IsNullOrEmpty($functionContent)) {
            $functionContent = $fileContent
        }
        
        # Check for security rules
        foreach ($rule in $securityRules) {
            if ($rule.AppliesTo -contains $Method) {
                $ruleMatch = $functionContent -match $rule.Pattern
                
                if ($ruleMatch) {
                    Write-Host "    - $($rule.Name): PASSED" -ForegroundColor Green
                } else {
                    if ($rule.Required) {
                        Write-Host "    - $($rule.Name): FAILED - Required pattern not found" -ForegroundColor Red
                        $passed = $false
                    } else {
                        Write-Host "    - $($rule.Name): WARNING - Recommended pattern not found" -ForegroundColor Yellow
                        $warnings++
                    }
                }
            }
        }
    }
    
    # Update test results
    if ($passed) {
        $testResults.Passed++
        if ($warnings -gt 0) {
            $testResults.Warnings++
        }
    } else {
        $testResults.Failed++
        $testResults.FailedEndpoints += "$Method $Endpoint ($fileName)"
    }
    
    return $passed
}

# Function to process a single API file
function Test-ApiFile {
    param(
        [string]$FilePath
    )
    
    $fileName = Split-Path -Leaf $FilePath
    Write-Host "Processing API file: $fileName" -ForegroundColor Magenta
    
    # Read file content
    $content = Get-Content $FilePath -Raw
    
    # Look for endpoint definitions
    # Pattern matches common endpoint definition formats in Rust
    $endpointMatches = [regex]::Matches($content, "#\[\s*(get|post|put|delete|patch|route)\s*\(\s*[""']([^""']+)[""']\s*\)")
    
    if ($endpointMatches.Count -eq 0) {
        Write-Host "  No endpoints found in $fileName" -ForegroundColor Gray
        return
    }
    
    foreach ($match in $endpointMatches) {
        $method = $match.Groups[1].Value.Trim().ToUpper()
        $endpoint = $match.Groups[2].Value
        
        # Get function name associated with this endpoint
        $functionNameMatch = [regex]::Match($content.Substring($match.Index), "(?:fn|async\s+fn)\s+([a-zA-Z_][a-zA-Z0-9_]*)")
        $functionName = if ($functionNameMatch.Success) { $functionNameMatch.Groups[1].Value } else { "" }
        
        # Test the endpoint
        Test-StandardEndpoint -Endpoint $endpoint -Method $method -FilePath $FilePath -FunctionName $functionName
    }
}

# Main execution
$apiFiles = @()

# Find all API files
foreach ($dir in $apiDirs) {
    if (Test-Path $dir) {
        $files = Get-ChildItem -Path $dir -Filter "*.rs" -Recurse
        $apiFiles += $files
    }
}

Write-Host "Found $($apiFiles.Count) API files to test" -ForegroundColor Cyan

# Process each API file
foreach ($file in $apiFiles) {
    try {
        Test-ApiFile -FilePath $file.FullName
    } catch {
        Write-Host "Error testing $($file.Name): $_" -ForegroundColor Red
    }
}

# Generate test report
$reportDir = Join-Path $projectRoot "test-results\api"
if (!(Test-Path $reportDir)) {
    New-Item -ItemType Directory -Path $reportDir -Force | Out-Null
}

# Create test report markdown
$reportContent = @"
# API Standardization Test Report

Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Summary

- **Total endpoints tested:** $($testResults.Total)
- **Passed:** $($testResults.Passed)
- **Failed:** $($testResults.Failed)
- **With warnings:** $($testResults.Warnings)

## Compliance Score

**$(if ($testResults.Total -gt 0) { [math]::Round(($testResults.Passed / $testResults.Total) * 100, 2) } else { 0 })%** of endpoints fully comply with standardization requirements.

## Failed Endpoints

$(if ($testResults.FailedEndpoints.Count -gt 0) {
    $testResults.FailedEndpoints | ForEach-Object { "- $_" }
} else {
    "No failed endpoints."
})

## Bitcoin Core Principles Compliance

The tested APIs were checked for alignment with the following Bitcoin Core principles:

1. **Immutability** - Operations that modify data must create immutable audit records
2. **Authentication** - All modification operations must require proper authentication
3. **Input Validation** - All inputs must be strictly validated before processing
4. **Idempotency** - Operations should be safely retriable with identical results

## Recommendations

$(if ($testResults.Failed -gt 0) {
    "- Run the API standardization implementation script to address failed endpoints."
})
$(if ($testResults.Warnings -gt 0) {
    "- Review endpoints with warnings to enhance Bitcoin Core principles compliance."
})
$(if ($testResults.Failed -eq 0 -and $testResults.Warnings -eq 0) {
    "- All endpoints comply with standardization requirements. No immediate action needed."
})
"@

$reportPath = Join-Path $reportDir "api-standards-report-$(Get-Date -Format 'yyyyMMdd-HHmmss').md"
$reportContent | Set-Content -Path $reportPath -Encoding UTF8

Write-Host "`nAPI Standardization Test Report written to: $reportPath" -ForegroundColor Green

# Print summary
Write-Host "`nAPI Standardization Test Summary:" -ForegroundColor Cyan
Write-Host "  Total endpoints tested: $($testResults.Total)" -ForegroundColor White
Write-Host "  Passed: $($testResults.Passed)" -ForegroundColor Green
Write-Host "  Failed: $($testResults.Failed)" -ForegroundColor $(if ($testResults.Failed -gt 0) { "Red" } else { "Gray" })
Write-Host "  With warnings: $($testResults.Warnings)" -ForegroundColor $(if ($testResults.Warnings -gt 0) { "Yellow" } else { "Gray" })

# Calculate compliance percentage
$complianceScore = if ($testResults.Total -gt 0) { [math]::Round(($testResults.Passed / $testResults.Total) * 100, 2) } else { 0 }
Write-Host "  Compliance score: $complianceScore%" -ForegroundColor $(if ($complianceScore -ge 90) { "Green" } elseif ($complianceScore -ge 70) { "Yellow" } else { "Red" })

# Exit with success code if passed more than 80%
if ($complianceScore -ge 80) {
    exit 0
} else {
    exit 1
}
