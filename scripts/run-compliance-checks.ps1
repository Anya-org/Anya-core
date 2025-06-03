# Compliance Checks for Bitcoin Protocol Standards
param(
    [hashtable]$Configuration,
    [string]$ResultsDirectory
)

Write-Host "==================================================================" -ForegroundColor Cyan
Write-Host "--- Running Compliance Checks                                  ---" -ForegroundColor Cyan
Write-Host "==================================================================" -ForegroundColor Cyan

# Check parameters
if (-not $ResultsDirectory) {
    $ResultsDirectory = "test-results/compliance"
}

if (-not (Test-Path $ResultsDirectory)) {
    New-Item -ItemType Directory -Path $ResultsDirectory -Force | Out-Null
}

# Initialize results
$testResults = @{
    complianceResults = @{}
    bipResults = @{}
    codeStandardsResults = @{}
    totalChecks = 0
    passedChecks = 0
    failedChecks = 0
    warningChecks = 0
    complianceScore = 0.0
    startTime = Get-Date
    endTime = $null
    duration = 0.0
}

# Define BIP compliance requirements
$bipRequirements = @(
    @{
        bip = "BIP-341"
        name = "Taproot"
        description = "Taproot: SegWit version 1 spending rules"
        required = $true
        references = @(
            "https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki"
        )
        searchPatterns = @(
            "BIP-341",
            "BIP 341",
            "Taproot",
            "verify-taproot-signature"
        )
    },
    @{
        bip = "BIP-174"
        name = "PSBT"
        description = "Partially Signed Bitcoin Transaction Format"
        required = $true
        references = @(
            "https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki"
        )
        searchPatterns = @(
            "BIP-174",
            "BIP 174",
            "PSBT",
            "Partially Signed Bitcoin Transaction"
        )
    },
    @{
        bip = "BIP-342"
        name = "Tapscript"
        description = "Validation of Taproot Scripts"
        required = $true
        references = @(
            "https://github.com/bitcoin/bips/blob/master/bip-0342.mediawiki"
        )
        searchPatterns = @(
            "BIP-342",
            "BIP 342",
            "Tapscript",
            "validate-tapscript"
        )
    },
    @{
        bip = "BIP-370"
        name = "PSBT Version 2"
        description = "PSBT Version 2 with Taproot Support"
        required = $false
        references = @(
            "https://github.com/bitcoin/bips/pull/1139"
        )
        searchPatterns = @(
            "BIP-370",
            "BIP 370",
            "PSBT v2",
            "PSBT Version 2"
        )
    }
)

# Define code standards for Clarity contracts
$codeStandards = @(
    @{
        standard = "Explicit Error Codes"
        description = "All error responses should use explicit error codes"
        required = $true
        searchPattern = '\(err u\d+'
        negativePattern = '\(err \"'
    },
    @{
        standard = "Trait Implementation"
        description = "Contracts should implement appropriate traits"
        required = $true
        searchPattern = '\(impl-trait'
    },
    @{
        standard = "Function Documentation"
        description = "All public functions should have documentation comments"
        required = $false
        searchPattern = ';;.*\n\s*\(define-public'
    },
    @{
        standard = "Proper Authorization"
        description = "Sensitive functions should include authorization checks"
        required = $true
        searchPattern = 'tx-sender|contract-caller|is-eq.*tx-sender|asserts!.*tx-sender'
    }
)

# Function to check BIP compliance for a single contract
function Test-BipCompliance {
    param (
        [string]$ContractPath,
        [array]$Requirements
    )
    
    $contractName = (Get-Item $ContractPath).Name
    Write-Host "`nChecking BIP compliance for contract: $contractName" -ForegroundColor Yellow
    
    $complianceResults = @{
        contractName = $contractName
        contractPath = $ContractPath
        bipResults = @{}
        overallStatus = "PASS"
        complianceScore = 0.0
    }
    
    $content = Get-Content -Path $ContractPath -Raw
    
    $totalRequiredBips = ($Requirements | Where-Object { $_.required }).Count
    $foundRequiredBips = 0
    
    foreach ($req in $Requirements) {
        $found = $false
        $matchedPatterns = @()
        
        foreach ($pattern in $req.searchPatterns) {
            if ($content -match $pattern) {
                $found = $true
                $matchedPatterns += $pattern
            }
        }
        
        $status = if ($found) { "PASS" } else { if ($req.required) { "FAIL" } else { "NOT_REQUIRED" } }
        
        $complianceResults.bipResults[$req.bip] = @{
            name = $req.name
            required = $req.required
            status = $status
            matchedPatterns = $matchedPatterns
        }
        
        # Update counters
        $testResults.totalChecks++
        if ($status -eq "PASS") {
            $testResults.passedChecks++
            if ($req.required) {
                $foundRequiredBips++
            }
        } elseif ($status -eq "FAIL") {
            $testResults.failedChecks++
            $complianceResults.overallStatus = "FAIL"
        } elseif ($status -eq "NOT_REQUIRED") {
            $testResults.warningChecks++
        }
        
        # Update global BIP results
        if (-not $testResults.bipResults.ContainsKey($req.bip)) {
            $testResults.bipResults[$req.bip] = @{
                name = $req.name
                description = $req.description
                required = $req.required
                compliantContracts = @()
                nonCompliantContracts = @()
            }
        }
        
        if ($found) {
            $testResults.bipResults[$req.bip].compliantContracts += $contractName
        } else {
            $testResults.bipResults[$req.bip].nonCompliantContracts += $contractName
        }
        
        # Report findings
        Write-Host "  $($req.bip) ($($req.name)): $status" -ForegroundColor $(
            if ($status -eq "PASS") { "Green" } 
            elseif ($status -eq "FAIL") { "Red" } 
            else { "Yellow" }
        )
        
        if ($found) {
            Write-Host "    Found patterns: $($matchedPatterns -join ', ')" -ForegroundColor Gray
        }
    }
    
    # Calculate compliance score
    if ($totalRequiredBips -gt 0) {
        $complianceResults.complianceScore = ($foundRequiredBips / $totalRequiredBips) * 100
    } else {
        $complianceResults.complianceScore = 100
    }
    
    Write-Host "  Compliance score: $([Math]::Round($complianceResults.complianceScore, 2))%" -ForegroundColor $(
        if ($complianceResults.complianceScore -ge 100) { "Green" }
        elseif ($complianceResults.complianceScore -ge 80) { "Yellow" }
        else { "Red" }
    )
    
    return $complianceResults
}

# Function to check code standards
function Test-CodeStandards {
    param (
        [string]$ContractPath,
        [array]$Standards
    )
    
    $contractName = (Get-Item $ContractPath).Name
    Write-Host "`nChecking code standards for contract: $contractName" -ForegroundColor Yellow
    
    $standardsResults = @{
        contractName = $contractName
        contractPath = $ContractPath
        standardResults = @{}
        overallStatus = "PASS"
        standardsScore = 0.0
    }
    
    $content = Get-Content -Path $ContractPath -Raw
    
    $totalRequiredStandards = ($Standards | Where-Object { $_.required }).Count
    $metRequiredStandards = 0
    
    foreach ($standard in $Standards) {
        $matches = $content | Select-String -Pattern $standard.searchPattern -AllMatches
        $matchCount = if ($matches) { $matches.Matches.Count } else { 0 }
        
        $negativePassed = $true
        if ($standard.negativePattern) {
            $negativeMatches = $content | Select-String -Pattern $standard.negativePattern -AllMatches
            $negativeMatchCount = if ($negativeMatches) { $negativeMatches.Matches.Count } else { 0 }
            $negativePassed = $negativeMatchCount -eq 0
        }
        
        $passed = $matchCount -gt 0 -and $negativePassed
        $status = if ($passed) { "PASS" } else { if ($standard.required) { "FAIL" } else { "WARN" } }
        
        $standardsResults.standardResults[$standard.standard] = @{
            description = $standard.description
            required = $standard.required
            status = $status
            matchCount = $matchCount
            negativePassed = $negativePassed
        }
        
        # Update counters
        $testResults.totalChecks++
        if ($status -eq "PASS") {
            $testResults.passedChecks++
            if ($standard.required) {
                $metRequiredStandards++
            }
        } elseif ($status -eq "FAIL") {
            $testResults.failedChecks++
            $standardsResults.overallStatus = "FAIL"
        } elseif ($status -eq "WARN") {
            $testResults.warningChecks++
        }
        
        # Update global standards results
        if (-not $testResults.codeStandardsResults.ContainsKey($standard.standard)) {
            $testResults.codeStandardsResults[$standard.standard] = @{
                description = $standard.description
                required = $standard.required
                compliantContracts = @()
                nonCompliantContracts = @()
            }
        }
        
        if ($passed) {
            $testResults.codeStandardsResults[$standard.standard].compliantContracts += $contractName
        } else {
            $testResults.codeStandardsResults[$standard.standard].nonCompliantContracts += $contractName
        }
        
        # Report findings
        Write-Host "  $($standard.standard): $status" -ForegroundColor $(
            if ($status -eq "PASS") { "Green" } 
            elseif ($status -eq "FAIL") { "Red" } 
            else { "Yellow" }
        )
        
        if ($passed) {
            Write-Host "    Found $matchCount matching patterns" -ForegroundColor Gray
        } else {
            if ($matchCount -eq 0) {
                Write-Host "    No matches found for pattern" -ForegroundColor Gray
            } 
            if ($standard.negativePattern -and -not $negativePassed) {
                Write-Host "    Found $negativeMatchCount negative patterns (not allowed)" -ForegroundColor Gray
            }
        }
    }
    
    # Calculate standards score
    if ($totalRequiredStandards -gt 0) {
        $standardsResults.standardsScore = ($metRequiredStandards / $totalRequiredStandards) * 100
    } else {
        $standardsResults.standardsScore = 100
    }
    
    Write-Host "  Standards compliance score: $([Math]::Round($standardsResults.standardsScore, 2))%" -ForegroundColor $(
        if ($standardsResults.standardsScore -ge 100) { "Green" }
        elseif ($standardsResults.standardsScore -ge 80) { "Yellow" }
        else { "Red" }
    )
    
    return $standardsResults
}

# Find all Clarity contracts
$contractFiles = @()
$contractDirectories = @("dao/core", "dao/traits", "dao/extensions", "src/contracts")

foreach ($dir in $contractDirectories) {
    if (Test-Path $dir) {
        $files = Get-ChildItem -Path $dir -Filter "*.clar"
        $contractFiles += $files
    }
}

Write-Host "Found $($contractFiles.Count) Clarity contracts to check for compliance" -ForegroundColor Cyan

# Apply focus filter if specified
if ($Configuration -and $Configuration.FocusModule) {
    $focusModule = $Configuration.FocusModule
    Write-Host "Focusing compliance checks on module: $focusModule" -ForegroundColor Yellow
    $contractFiles = $contractFiles | Where-Object { 
        $_.Name -like "*$focusModule*" -or $_.Name -like "*$($focusModule -replace '-', '_')*" 
    }
    
    if ($contractFiles.Count -eq 0) {
        Write-Host "Warning: No contracts found matching focus module: $focusModule" -ForegroundColor Yellow
    } else {
        Write-Host "Filtered to $($contractFiles.Count) contract(s) for focused checking" -ForegroundColor Yellow
    }
}

# Process each contract
foreach ($contractFile in $contractFiles) {
    $contractPath = $contractFile.FullName
    
    # Check BIP compliance
    $bipResults = Test-BipCompliance -ContractPath $contractPath -Requirements $bipRequirements
    
    # Check code standards
    $standardsResults = Test-CodeStandards -ContractPath $contractPath -Requirements $codeStandards
    
    # Store combined results
    $testResults.complianceResults[$contractFile.Name] = @{
        contractPath = $contractPath
        bipResults = $bipResults
        standardsResults = $standardsResults
        overallStatus = if ($bipResults.overallStatus -eq "PASS" -and $standardsResults.overallStatus -eq "PASS") { "PASS" } else { "FAIL" }
        combinedScore = ($bipResults.complianceScore + $standardsResults.standardsScore) / 2
    }
}

# Calculate overall compliance score
$contractScores = $testResults.complianceResults.Values | ForEach-Object { $_.combinedScore }
$testResults.complianceScore = if ($contractScores.Count -gt 0) { 
    ($contractScores | Measure-Object -Average).Average 
} else {
    0
}

# Update end time and duration
$testResults.endTime = Get-Date
$testResults.duration = ($testResults.endTime - $testResults.startTime).TotalSeconds

# Generate summary
Write-Host "`n--- Compliance Checks Summary ---" -ForegroundColor Yellow
Write-Host "Total contracts checked: $($contractFiles.Count)" -ForegroundColor Cyan
Write-Host "Total checks performed: $($testResults.totalChecks)" -ForegroundColor Cyan
Write-Host "Checks passed: $($testResults.passedChecks)" -ForegroundColor $(if ($testResults.passedChecks -gt 0) { "Green" } else { "Gray" })
Write-Host "Checks failed: $($testResults.failedChecks)" -ForegroundColor $(if ($testResults.failedChecks -gt 0) { "Red" } else { "Gray" })
Write-Host "Checks with warnings: $($testResults.warningChecks)" -ForegroundColor $(if ($testResults.warningChecks -gt 0) { "Yellow" } else { "Gray" })
Write-Host "Overall compliance score: $([Math]::Round($testResults.complianceScore, 2))%" -ForegroundColor $(
    if ($testResults.complianceScore -ge 90) { "Green" }
    elseif ($testResults.complianceScore -ge 75) { "Yellow" }
    else { "Red" }
)
Write-Host "Test duration: $([Math]::Round($testResults.duration, 2)) seconds" -ForegroundColor Cyan

# Generate BIP compliance report
$bipSummary = @()
foreach ($bip in $testResults.bipResults.Keys) {
    $bipResult = $testResults.bipResults[$bip]
    $compliantCount = $bipResult.compliantContracts.Count
    $totalCount = $compliantCount + $bipResult.nonCompliantContracts.Count
    $compliance = if ($totalCount -gt 0) { ($compliantCount / $totalCount) * 100 } else { 0 }
    
    $bipSummary += [PSCustomObject]@{
        BIP = $bip
        Name = $bipResult.name
        Required = $bipResult.required
        CompliantContracts = $compliantCount
        TotalContracts = $totalCount
        CompliancePercentage = [Math]::Round($compliance, 2)
        Status = if ($bipResult.required -and $compliance -lt 100) { "NOT FULLY COMPLIANT" } else { "COMPLIANT" }
    }
}

$bipSummaryFormatted = $bipSummary | Format-Table -AutoSize | Out-String
Write-Host "`n--- BIP Compliance Summary ---" -ForegroundColor Yellow
Write-Host $bipSummaryFormatted -ForegroundColor Cyan

# Save results
$testResultsJson = $testResults | ConvertTo-Json -Depth 5
Set-Content -Path "$ResultsDirectory/compliance-results.json" -Value $testResultsJson

# Generate detailed HTML report
$htmlReportPath = "$ResultsDirectory/compliance-report.html"
$htmlContent = @"
<!DOCTYPE html>
<html>
<head>
    <title>Anya DAO Compliance Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        h1, h2, h3 { color: #333; }
        .summary { background-color: #f5f5f5; padding: 15px; border-radius: 5px; margin-bottom: 20px; }
        .pass { color: green; }
        .fail { color: red; }
        .warning { color: orange; }
        table { border-collapse: collapse; width: 100%; margin-bottom: 20px; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        tr:nth-child(even) { background-color: #f9f9f9; }
        .bip-table th { background-color: #e7f0f7; }
        .standards-table th { background-color: #f7efe7; }
        .score-bar { height: 20px; background-color: #eee; border-radius: 4px; overflow: hidden; margin-top: 5px; }
        .score-fill { height: 100%; background-color: #4CAF50; }
        .score-fill.warning { background-color: #FF9800; }
        .score-fill.fail { background-color: #F44336; }
    </style>
</head>
<body>
    <h1>Anya DAO Compliance Report</h1>
    <div class="summary">
        <h2>Summary</h2>
        <p>Generated: $((Get-Date).ToString("yyyy-MM-dd HH:mm:ss"))</p>
        <p>Total contracts checked: $($contractFiles.Count)</p>
        <p>Total checks performed: $($testResults.totalChecks)</p>
        <p>Checks passed: <span class="pass">$($testResults.passedChecks)</span></p>
        <p>Checks failed: <span class="fail">$($testResults.failedChecks)</span></p>
        <p>Checks with warnings: <span class="warning">$($testResults.warningChecks)</span></p>
        <p>Overall compliance score: $([Math]::Round($testResults.complianceScore, 2))%</p>
        <div class="score-bar">
            <div class="score-fill$(if ($testResults.complianceScore -lt 75) { " fail" } elseif ($testResults.complianceScore -lt 90) { " warning" })" style="width: $([Math]::Min(100, [Math]::Round($testResults.complianceScore, 0)))%;"></div>
        </div>
    </div>

    <h2>BIP Compliance Summary</h2>
    <table class="bip-table">
        <tr>
            <th>BIP</th>
            <th>Name</th>
            <th>Required</th>
            <th>Compliant Contracts</th>
            <th>Total Contracts</th>
            <th>Compliance %</th>
            <th>Status</th>
        </tr>
"@

foreach ($item in $bipSummary) {
    $statusClass = if ($item.Status -eq "COMPLIANT") { "pass" } else { "fail" }
    $htmlContent += @"
        <tr>
            <td>$($item.BIP)</td>
            <td>$($item.Name)</td>
            <td>$($item.Required)</td>
            <td>$($item.CompliantContracts)</td>
            <td>$($item.TotalContracts)</td>
            <td>$($item.CompliancePercentage)%</td>
            <td class="$statusClass">$($item.Status)</td>
        </tr>
"@
}

$htmlContent += @"
    </table>

    <h2>Contract Compliance Details</h2>
"@

foreach ($contractName in $testResults.complianceResults.Keys) {
    $contractResult = $testResults.complianceResults[$contractName]
    $statusClass = if ($contractResult.overallStatus -eq "PASS") { "pass" } else { "fail" }
    
    $htmlContent += @"
    <h3>$contractName <span class="$statusClass">[$($contractResult.overallStatus)]</span></h3>
    <p>File path: $($contractResult.contractPath)</p>
    <p>Combined compliance score: $([Math]::Round($contractResult.combinedScore, 2))%</p>
    <div class="score-bar">
        <div class="score-fill$(if ($contractResult.combinedScore -lt 75) { " fail" } elseif ($contractResult.combinedScore -lt 90) { " warning" })" style="width: $([Math]::Min(100, [Math]::Round($contractResult.combinedScore, 0)))%;"></div>
    </div>
    
    <h4>BIP Compliance</h4>
    <table>
        <tr>
            <th>BIP</th>
            <th>Name</th>
            <th>Required</th>
            <th>Status</th>
        </tr>
"@

    foreach ($bip in $contractResult.bipResults.bipResults.Keys) {
        $bipResult = $contractResult.bipResults.bipResults[$bip]
        $bipStatusClass = if ($bipResult.status -eq "PASS") { "pass" } elseif ($bipResult.status -eq "FAIL") { "fail" } else { "warning" }
        
        $htmlContent += @"
        <tr>
            <td>$bip</td>
            <td>$($bipResult.name)</td>
            <td>$($bipResult.required)</td>
            <td class="$bipStatusClass">$($bipResult.status)</td>
        </tr>
"@
    }
    
    $htmlContent += @"
    </table>
    
    <h4>Code Standards</h4>
    <table>
        <tr>
            <th>Standard</th>
            <th>Required</th>
            <th>Status</th>
            <th>Details</th>
        </tr>
"@

    foreach ($standard in $contractResult.standardsResults.standardResults.Keys) {
        $standardResult = $contractResult.standardsResults.standardResults[$standard]
        $standardStatusClass = if ($standardResult.status -eq "PASS") { "pass" } elseif ($standardResult.status -eq "FAIL") { "fail" } else { "warning" }
        
        $htmlContent += @"
        <tr>
            <td>$standard</td>
            <td>$($standardResult.required)</td>
            <td class="$standardStatusClass">$($standardResult.status)</td>
            <td>Found $($standardResult.matchCount) matches</td>
        </tr>
"@
    }
    
    $htmlContent += @"
    </table>
"@
}

$htmlContent += @"
</body>
</html>
"@

Set-Content -Path $htmlReportPath -Value $htmlContent
Write-Host "`nResults saved to:" -ForegroundColor Cyan
Write-Host "  JSON results: $ResultsDirectory/compliance-results.json" -ForegroundColor Cyan
Write-Host "  HTML report: $htmlReportPath" -ForegroundColor Cyan

# Return appropriate exit code
$exitCode = if ($testResults.failedChecks -gt 0) { 1 } else { 0 }
Write-Host "==================================================================" -ForegroundColor Cyan
exit $exitCode 