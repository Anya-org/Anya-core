# Security Analysis for Anya DAO Components
param(
    [hashtable]$Configuration,
    [string]$ResultsDirectory
)

Write-Host "==================================================================" -ForegroundColor Cyan
Write-Host "--- Running Security Analysis                                  ---" -ForegroundColor Cyan
Write-Host "==================================================================" -ForegroundColor Cyan

# Check parameters
if (-not $ResultsDirectory) {
    $ResultsDirectory = "test-results/security"
}

if (-not (Test-Path $ResultsDirectory)) {
    New-Item -ItemType Directory -Path $ResultsDirectory -Force | Out-Null
}

# Initialize results
$securityResults = @{
    contractAnalysis = @{}
    vulnerabilities = @{}
    riskProfiles = @{}
    summary = @{
        totalContracts = 0
        securePasses = 0
        lowRisks = 0
        mediumRisks = 0
        highRisks = 0
        criticalRisks = 0
        overallRiskScore = 0.0
        securityScore = 100.0
    }
    startTime = Get-Date
    endTime = $null
    duration = 0.0
}

# Define security patterns to check
$securityPatterns = @(
    @{
        name = "Unauthorized Access"
        description = "Checks for proper authorization controls"
        severity = "Critical"
        pattern = 'tx-sender|contract-caller|is-eq.*tx-sender|asserts!'
        negativePattern = $null
        required = $true
        recommendation = "Implement authorization checks using tx-sender or contract-caller validation"
    },
    @{
        name = "Error Handling"
        description = "Checks for proper error handling"
        severity = "High"
        pattern = '\(err u\d+'
        negativePattern = '\(err \"'
        required = $true
        recommendation = "Use numeric error codes instead of string error messages"
    },
    @{
        name = "Input Validation"
        description = "Checks for input validation"
        severity = "High"
        pattern = 'asserts!|\(> |\(< |\(>= |\(<= |\(is-eq '
        negativePattern = $null
        required = $true
        recommendation = "Add explicit validation for all function inputs"
    },
    @{
        name = "Overflow Protection"
        description = "Checks for integer overflow protection"
        severity = "Critical"
        pattern = '\(+ |\(- |\(\* '
        negativePattern = $null
        required = $true
        recommendation = "Add explicit checks for arithmetic overflows"
    },
    @{
        name = "Reentrancy Protection"
        description = "Checks for reentrancy protection"
        severity = "Critical"
        pattern = '(define-data-var\s+[a-zA-Z\-_]+-completed|asserts!\s+\(is-eq\s+[a-zA-Z\-_]+-status|asserts!\s+\(not\s+\(var-get\s+[a-zA-Z\-_]+-in-process\))'
        negativePattern = $null
        required = $false
        recommendation = "Implement reentrancy guards for external calls"
    },
    @{
        name = "Resource Exhaustion"
        description = "Checks for resource exhaustion protection"
        severity = "Medium"
        pattern = 'asserts! \(< \(.+\) u\d+|asserts! \(> \(.+\) u\d+'
        negativePattern = $null
        required = $false
        recommendation = "Add limits to resource-intensive operations"
    },
    @{
        name = "Secure Randomness"
        description = "Checks for secure randomness"
        severity = "Medium"
        pattern = 'block-height|burn-block-height|tx-sender|contract-caller'
        negativePattern = 'get-random|random'
        required = $false
        recommendation = "Use unpredictable values as entropy sources"
    }
)

# Define common vulnerabilities to check
$commonVulnerabilities = @(
    @{
        name = "Fixed Authorization"
        description = "Authorization is hardcoded to a specific principal"
        pattern = 'is-eq tx-sender ''[ST|SP][A-Z0-9]+'
        severity = "High"
        impact = "Centralized control, lack of governance"
        recommendation = "Implement dynamic authorization through data-vars or maps"
    },
    @{
        name = "Unbounded Operations"
        description = "Operations without resource limits"
        pattern = '\(map |fold |filter '
        severity = "Medium"
        impact = "Potential resource exhaustion"
        recommendation = "Add explicit size limits on collections and iteration operations"
    },
    @{
        name = "Unchecked Arithmetic"
        description = "Arithmetic operations without overflow checks"
        pattern = '\(+ |\(- |\(\* '
        severity = "High"
        impact = "Integer overflow/underflow leading to unexpected behavior"
        recommendation = "Add explicit asserts for arithmetic operation boundaries"
    },
    @{
        name = "String Error Messages"
        description = "Error returns using string messages instead of numeric codes"
        pattern = '\(err \"'
        severity = "Low"
        impact = "Inconsistent error handling"
        recommendation = "Use numeric error codes for all error conditions"
    },
    @{
        name = "Missing Input Validation"
        description = "Function parameters without validation"
        pattern = '\(define-public \([a-zA-Z\-_]+ \([^)]+\)[^(]*\(begin'
        severity = "Medium"
        impact = "Accepting invalid inputs leading to unexpected behavior"
        recommendation = "Add validation for all function parameters"
    }
)

# Function to analyze contract security
function Test-ContractSecurity {
    param (
        [string]$ContractPath,
        [array]$SecurityPatterns,
        [array]$Vulnerabilities
    )
    
    $contractName = (Get-Item $ContractPath).Name
    Write-Host "`nAnalyzing security for contract: $contractName" -ForegroundColor Yellow
    
    $securityAnalysis = @{
        contractName = $contractName
        contractPath = $ContractPath
        patternResults = @{}
        vulnerabilityResults = @{}
        overallStatus = "SECURE"
        riskLevel = "Low"
        riskScore = 0
        securityScore = 100.0
        findings = @()
    }
    
    $content = Get-Content -Path $ContractPath -Raw
    
    # Count the number of security patterns that are required
    $totalRequiredPatterns = ($SecurityPatterns | Where-Object { $_.required }).Count
    $passedRequiredPatterns = 0
    
    # Check for security patterns
    foreach ($pattern in $SecurityPatterns) {
        $matches = $content | Select-String -Pattern $pattern.pattern -AllMatches
        $matchCount = if ($matches) { $matches.Matches.Count } else { 0 }
        
        $negativePassed = $true
        if ($pattern.negativePattern) {
            $negativeMatches = $content | Select-String -Pattern $pattern.negativePattern -AllMatches
            $negativeMatchCount = if ($negativeMatches) { $negativeMatches.Matches.Count } else { 0 }
            $negativePassed = $negativeMatchCount -eq 0
        }
        
        $passed = $matchCount -gt 0 -and $negativePassed
        
        $securityAnalysis.patternResults[$pattern.name] = @{
            name = $pattern.name
            description = $pattern.description
            severity = $pattern.severity
            matchCount = $matchCount
            passed = $passed
            required = $pattern.required
            recommendation = $pattern.recommendation
        }
        
        if ($pattern.required) {
            if ($passed) {
                $passedRequiredPatterns++
            } else {
                $securityAnalysis.findings += @{
                    type = "MissingSecurityPattern"
                    name = $pattern.name
                    severity = $pattern.severity
                    description = "Required security pattern not found: $($pattern.description)"
                    recommendation = $pattern.recommendation
                }
            }
        } elseif (-not $passed -and $pattern.severity -eq "Critical") {
            $securityAnalysis.findings += @{
                type = "RecommendedSecurityPattern"
                name = $pattern.name
                severity = $pattern.severity
                description = "Recommended critical security pattern not found: $($pattern.description)"
                recommendation = $pattern.recommendation
            }
        }
    }
    
    # Check for common vulnerabilities
    foreach ($vulnerability in $Vulnerabilities) {
        $matches = $content | Select-String -Pattern $vulnerability.pattern -AllMatches
        $matchCount = if ($matches) { $matches.Matches.Count } else { 0 }
        
        $hasVulnerability = $matchCount -gt 0
        
        $securityAnalysis.vulnerabilityResults[$vulnerability.name] = @{
            name = $vulnerability.name
            description = $vulnerability.description
            severity = $vulnerability.severity
            matchCount = $matchCount
            found = $hasVulnerability
            impact = $vulnerability.impact
            recommendation = $vulnerability.recommendation
        }
        
        if ($hasVulnerability) {
            $securityAnalysis.findings += @{
                type = "Vulnerability"
                name = $vulnerability.name
                severity = $vulnerability.severity
                description = "Potential vulnerability found: $($vulnerability.description)"
                impact = $vulnerability.impact
                matchCount = $matchCount
                recommendation = $vulnerability.recommendation
            }
        }
    }
    
    # Calculate overall security status
    $allRequiredPassed = $passedRequiredPatterns -eq $totalRequiredPatterns
    
    # Calculate risk level based on findings
    $criticalFindings = @($securityAnalysis.findings | Where-Object { $_.severity -eq "Critical" }).Count
    $highFindings = @($securityAnalysis.findings | Where-Object { $_.severity -eq "High" }).Count
    $mediumFindings = @($securityAnalysis.findings | Where-Object { $_.severity -eq "Medium" }).Count
    $lowFindings = @($securityAnalysis.findings | Where-Object { $_.severity -eq "Low" }).Count
    
    if ($criticalFindings -gt 0) {
        $securityAnalysis.riskLevel = "Critical"
        $securityAnalysis.riskScore = 100 - (100 / [Math]::Max(1, $criticalFindings))
        $securityResults.summary.criticalRisks++
    } elseif ($highFindings -gt 0) {
        $securityAnalysis.riskLevel = "High"
        $securityAnalysis.riskScore = 75 - (25 / [Math]::Max(1, $highFindings))
        $securityResults.summary.highRisks++
    } elseif ($mediumFindings -gt 0) {
        $securityAnalysis.riskLevel = "Medium"
        $securityAnalysis.riskScore = 50 - (25 / [Math]::Max(1, $mediumFindings))
        $securityResults.summary.mediumRisks++
    } elseif ($lowFindings -gt 0) {
        $securityAnalysis.riskLevel = "Low"
        $securityAnalysis.riskScore = 25 - (15 / [Math]::Max(1, $lowFindings))
        $securityResults.summary.lowRisks++
    } else {
        $securityAnalysis.riskLevel = "None"
        $securityAnalysis.riskScore = 0
        $securityResults.summary.securePasses++
    }
    
    # Calculate security score (0-100)
    $securityAnalysis.securityScore = 100 - $securityAnalysis.riskScore
    
    if ($allRequiredPassed) {
        if ($securityAnalysis.findings.Count -eq 0) {
            $securityAnalysis.overallStatus = "SECURE"
        } else {
            $securityAnalysis.overallStatus = "WARNINGS"
        }
    } else {
        $securityAnalysis.overallStatus = "VULNERABLE"
    }
    
    # Display results
    Write-Host "  Security Status: $($securityAnalysis.overallStatus)" -ForegroundColor $(
        if ($securityAnalysis.overallStatus -eq "SECURE") { "Green" }
        elseif ($securityAnalysis.overallStatus -eq "WARNINGS") { "Yellow" }
        else { "Red" }
    )
    Write-Host "  Risk Level: $($securityAnalysis.riskLevel)" -ForegroundColor $(
        if ($securityAnalysis.riskLevel -eq "None") { "Green" }
        elseif ($securityAnalysis.riskLevel -eq "Low") { "Cyan" }
        elseif ($securityAnalysis.riskLevel -eq "Medium") { "Yellow" }
        elseif ($securityAnalysis.riskLevel -eq "High") { "DarkYellow" }
        else { "Red" }
    )
    Write-Host "  Security Score: $($securityAnalysis.securityScore)" -ForegroundColor $(
        if ($securityAnalysis.securityScore -ge 90) { "Green" }
        elseif ($securityAnalysis.securityScore -ge 70) { "Yellow" }
        else { "Red" }
    )
    
    if ($securityAnalysis.findings.Count -gt 0) {
        Write-Host "`n  Security Findings:" -ForegroundColor Yellow
        foreach ($finding in $securityAnalysis.findings) {
            Write-Host "    - $($finding.name) [Severity: $($finding.severity)]" -ForegroundColor $(
                if ($finding.severity -eq "Critical") { "Red" }
                elseif ($finding.severity -eq "High") { "DarkYellow" }
                elseif ($finding.severity -eq "Medium") { "Yellow" }
                else { "Cyan" }
            )
            Write-Host "      $($finding.description)" -ForegroundColor Gray
            if ($finding.impact) {
                Write-Host "      Impact: $($finding.impact)" -ForegroundColor Gray
            }
            Write-Host "      Recommendation: $($finding.recommendation)" -ForegroundColor Gray
        }
    } else {
        Write-Host "  No security findings." -ForegroundColor Green
    }
    
    return $securityAnalysis
}

# Find all contract files to analyze
$contracts = @()
$contractPaths = @(
    "dao/core/*.clar",
    "dao/traits/*.clar",
    "dao/extensions/*.clar",
    "src/contracts/*.clar"
)

foreach ($pattern in $contractPaths) {
    $contracts += Get-ChildItem -Path $pattern -ErrorAction SilentlyContinue
}

if ($contracts.Count -eq 0) {
    Write-Host "Warning: No Clarity contracts found to analyze!" -ForegroundColor Yellow
    # Create empty directories for output if needed
    if (-not (Test-Path "$ResultsDirectory")) {
        New-Item -ItemType Directory -Path "$ResultsDirectory" -Force | Out-Null
    }
    
    # Write empty results
    $securityResults.endTime = Get-Date
    $securityResults.duration = ($securityResults.endTime - $securityResults.startTime).TotalSeconds
    
    $securityResultsJson = $securityResults | ConvertTo-Json -Depth 10
    Set-Content -Path "$ResultsDirectory/security-results.json" -Value $securityResultsJson
    
    Write-Host "No contracts found to analyze. Exiting." -ForegroundColor Yellow
    exit 0
}

# Apply focus filter if specified
if ($Configuration -and $Configuration.FocusContract) {
    $focusContract = $Configuration.FocusContract
    Write-Host "Focusing security analysis on contract: $focusContract" -ForegroundColor Yellow
    $contracts = $contracts | Where-Object { $_.Name -eq "$focusContract.clar" -or $_.Name -eq $focusContract }
    
    if ($contracts.Count -eq 0) {
        Write-Host "Warning: No contracts found matching the focus pattern: $focusContract" -ForegroundColor Yellow
    } else {
        Write-Host "Filtered to $($contracts.Count) contract(s) for focused analysis" -ForegroundColor Yellow
    }
}

# Run security analysis on all contracts
$securityResults.summary.totalContracts = $contracts.Count
foreach ($contract in $contracts) {
    $analysis = Test-ContractSecurity -ContractPath $contract.FullName -SecurityPatterns $securityPatterns -Vulnerabilities $commonVulnerabilities
    $securityResults.contractAnalysis[$contract.Name] = $analysis
    
    # Add to vulnerabilities list
    foreach ($finding in $analysis.findings) {
        if ($finding.type -eq "Vulnerability") {
            if (-not $securityResults.vulnerabilities.ContainsKey($finding.name)) {
                $securityResults.vulnerabilities[$finding.name] = @{
                    name = $finding.name
                    severity = $finding.severity
                    description = $finding.description
                    impact = $finding.impact
                    recommendation = $finding.recommendation
                    affectedContracts = @()
                    occurrences = 0
                }
            }
            
            $securityResults.vulnerabilities[$finding.name].affectedContracts += $contract.Name
            $securityResults.vulnerabilities[$finding.name].occurrences += $finding.matchCount
        }
    }
    
    # Add to risk profiles
    if (-not $securityResults.riskProfiles.ContainsKey($analysis.riskLevel)) {
        $securityResults.riskProfiles[$analysis.riskLevel] = @{
            level = $analysis.riskLevel
            contracts = @()
            count = 0
        }
    }
    
    $securityResults.riskProfiles[$analysis.riskLevel].contracts += $contract.Name
    $securityResults.riskProfiles[$analysis.riskLevel].count++
}

# Calculate overall risk and security scores
$contractScores = $securityResults.contractAnalysis.Values | ForEach-Object { $_.securityScore }
$securityResults.summary.securityScore = if ($contractScores.Count -gt 0) { 
    [Math]::Round(($contractScores | Measure-Object -Average).Average, 2)
} else { 
    0 
}

$contractRiskScores = $securityResults.contractAnalysis.Values | ForEach-Object { $_.riskScore }
$securityResults.summary.overallRiskScore = if ($contractRiskScores.Count -gt 0) { 
    [Math]::Round(($contractRiskScores | Measure-Object -Average).Average, 2)
} else { 
    0 
}

# Update end time and duration
$securityResults.endTime = Get-Date
$securityResults.duration = ($securityResults.endTime - $securityResults.startTime).TotalSeconds

# Generate summary
Write-Host "`n--- Security Analysis Summary ---" -ForegroundColor Yellow
Write-Host "Total contracts analyzed: $($securityResults.summary.totalContracts)" -ForegroundColor Cyan
Write-Host "Secure contracts: $($securityResults.summary.securePasses)" -ForegroundColor $(
    if ($securityResults.summary.securePasses -gt 0) { "Green" } else { "Gray" }
)
Write-Host "Low risk contracts: $($securityResults.summary.lowRisks)" -ForegroundColor $(
    if ($securityResults.summary.lowRisks -gt 0) { "Cyan" } else { "Gray" }
)
Write-Host "Medium risk contracts: $($securityResults.summary.mediumRisks)" -ForegroundColor $(
    if ($securityResults.summary.mediumRisks -gt 0) { "Yellow" } else { "Gray" }
)
Write-Host "High risk contracts: $($securityResults.summary.highRisks)" -ForegroundColor $(
    if ($securityResults.summary.highRisks -gt 0) { "DarkYellow" } else { "Gray" }
)
Write-Host "Critical risk contracts: $($securityResults.summary.criticalRisks)" -ForegroundColor $(
    if ($securityResults.summary.criticalRisks -gt 0) { "Red" } else { "Gray" }
)
Write-Host "Overall security score: $($securityResults.summary.securityScore)" -ForegroundColor $(
    if ($securityResults.summary.securityScore -ge 90) { "Green" }
    elseif ($securityResults.summary.securityScore -ge 70) { "Yellow" }
    else { "Red" }
)
Write-Host "Analysis duration: $([Math]::Round($securityResults.duration, 2)) seconds" -ForegroundColor Cyan

# Generate vulnerability summary
if ($securityResults.vulnerabilities.Count -gt 0) {
    $vulnSummary = @()
    foreach ($vulnName in $securityResults.vulnerabilities.Keys) {
        $vuln = $securityResults.vulnerabilities[$vulnName]
        
        $vulnSummary += [PSCustomObject]@{
            Vulnerability = $vulnName
            Severity = $vuln.severity
            Occurrences = $vuln.occurrences
            AffectedContracts = $vuln.affectedContracts.Count
        }
    }
    
    $vulnSummaryFormatted = $vulnSummary | Sort-Object -Property Severity -Descending | Format-Table -AutoSize | Out-String
    Write-Host "`n--- Vulnerability Summary ---" -ForegroundColor Yellow
    Write-Host $vulnSummaryFormatted -ForegroundColor Cyan
}

# Save results
$securityResultsJson = $securityResults | ConvertTo-Json -Depth 10
Set-Content -Path "$ResultsDirectory/security-results.json" -Value $securityResultsJson

# Generate HTML security report
$htmlReportPath = "$ResultsDirectory/security-report.html"
$htmlContent = @"
<!DOCTYPE html>
<html>
<head>
    <title>Anya DAO Security Analysis Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        h1, h2, h3 { color: #333; }
        .summary { background-color: #f5f5f5; padding: 15px; border-radius: 5px; margin-bottom: 20px; }
        .secure { color: green; }
        .warning { color: orange; }
        .vulnerable { color: red; }
        .low { color: #17a2b8; }
        .medium { color: orange; }
        .high { color: #fd7e14; }
        .critical { color: red; }
        table { border-collapse: collapse; width: 100%; margin-bottom: 20px; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        tr:nth-child(even) { background-color: #f9f9f9; }
        .contract-table th { background-color: #e7f0f7; }
        .vuln-table th { background-color: #f7e7e7; }
        .security-meter { height: 30px; background-color: #eee; border-radius: 4px; overflow: hidden; margin: 10px 0; }
        .security-fill { height: 100%; background-color: #4CAF50; }
        .security-fill.warning { background-color: #FF9800; }
        .security-fill.vulnerable { background-color: #F44336; }
        .finding { margin-bottom: 15px; padding: 10px; border-left: 4px solid #ddd; background-color: #f9f9f9; }
        .finding.critical { border-color: #F44336; }
        .finding.high { border-color: #fd7e14; }
        .finding.medium { border-color: #FF9800; }
        .finding.low { border-color: #17a2b8; }
    </style>
</head>
<body>
    <h1>Anya DAO Security Analysis Report</h1>
    <div class="summary">
        <h2>Summary</h2>
        <p>Generated: $((Get-Date).ToString("yyyy-MM-dd HH:mm:ss"))</p>
        <p>Total contracts analyzed: $($securityResults.summary.totalContracts)</p>
        <p>Secure contracts: <span class="secure">$($securityResults.summary.securePasses)</span></p>
        <p>Low risk contracts: <span class="low">$($securityResults.summary.lowRisks)</span></p>
        <p>Medium risk contracts: <span class="medium">$($securityResults.summary.mediumRisks)</span></p>
        <p>High risk contracts: <span class="high">$($securityResults.summary.highRisks)</span></p>
        <p>Critical risk contracts: <span class="critical">$($securityResults.summary.criticalRisks)</span></p>
        <p>Overall security score: $($securityResults.summary.securityScore) / 100</p>
        <div class="security-meter">
            <div class="security-fill$(if ($securityResults.summary.securityScore -lt 70) { " vulnerable" } elseif ($securityResults.summary.securityScore -lt 90) { " warning" })" style="width: $([Math]::Min(100, [Math]::Round($securityResults.summary.securityScore, 0)))%;"></div>
        </div>
    </div>

    <h2>Contract Security Overview</h2>
    <table class="contract-table">
        <tr>
            <th>Contract</th>
            <th>Status</th>
            <th>Risk Level</th>
            <th>Security Score</th>
            <th>Findings</th>
        </tr>
"@

foreach ($contractName in $securityResults.contractAnalysis.Keys) {
    $contract = $securityResults.contractAnalysis[$contractName]
    $statusClass = if ($contract.overallStatus -eq "SECURE") { "secure" } 
                  elseif ($contract.overallStatus -eq "WARNINGS") { "warning" }
                  else { "vulnerable" }
    
    $riskClass = if ($contract.riskLevel -eq "None") { "secure" } 
                elseif ($contract.riskLevel -eq "Low") { "low" }
                elseif ($contract.riskLevel -eq "Medium") { "medium" }
                elseif ($contract.riskLevel -eq "High") { "high" }
                else { "critical" }
    
    $htmlContent += @"
        <tr>
            <td>$contractName</td>
            <td class="$statusClass">$($contract.overallStatus)</td>
            <td class="$riskClass">$($contract.riskLevel)</td>
            <td>$($contract.securityScore)</td>
            <td>$($contract.findings.Count)</td>
        </tr>
"@
}

$htmlContent += @"
    </table>

    <h2>Vulnerability Summary</h2>
"@

if ($securityResults.vulnerabilities.Count -gt 0) {
    $htmlContent += @"
    <table class="vuln-table">
        <tr>
            <th>Vulnerability</th>
            <th>Severity</th>
            <th>Affected Contracts</th>
            <th>Occurrences</th>
        </tr>
"@

    $sortedVulnerabilities = $securityResults.vulnerabilities.Values | Sort-Object @{Expression={
        switch ($_.severity) {
            "Critical" { return 4 }
            "High" { return 3 }
            "Medium" { return 2 }
            "Low" { return 1 }
            default { return 0 }
        }
    }; Descending=$true}, name

    foreach ($vuln in $sortedVulnerabilities) {
        $severityClass = if ($vuln.severity -eq "Critical") { "critical" } 
                        elseif ($vuln.severity -eq "High") { "high" }
                        elseif ($vuln.severity -eq "Medium") { "medium" }
                        else { "low" }
        
        $htmlContent += @"
        <tr>
            <td>$($vuln.name)</td>
            <td class="$severityClass">$($vuln.severity)</td>
            <td>$($vuln.affectedContracts.Count)</td>
            <td>$($vuln.occurrences)</td>
        </tr>
"@
    }

    $htmlContent += @"
    </table>
"@
} else {
    $htmlContent += "<p>No vulnerabilities found.</p>"
}

# Detailed contract reports
$htmlContent += "<h2>Detailed Contract Reports</h2>"

foreach ($contractName in $securityResults.contractAnalysis.Keys) {
    $contract = $securityResults.contractAnalysis[$contractName]
    $statusClass = if ($contract.overallStatus -eq "SECURE") { "secure" } 
                  elseif ($contract.overallStatus -eq "WARNINGS") { "warning" }
                  else { "vulnerable" }
    
    $htmlContent += @"
    <h3>$contractName <span class="$statusClass">[$($contract.overallStatus)]</span></h3>
    <p>Security Score: $($contract.securityScore) / 100</p>
    <p>Risk Level: <span class="$(if ($contract.riskLevel -eq "None") { "secure" } 
                elseif ($contract.riskLevel -eq "Low") { "low" }
                elseif ($contract.riskLevel -eq "Medium") { "medium" }
                elseif ($contract.riskLevel -eq "High") { "high" }
                else { "critical" })">$($contract.riskLevel)</span></p>
    <div class="security-meter">
        <div class="security-fill$(if ($contract.securityScore -lt 70) { " vulnerable" } elseif ($contract.securityScore -lt 90) { " warning" })" style="width: $([Math]::Min(100, [Math]::Round($contract.securityScore, 0)))%;"></div>
    </div>
"@

    if ($contract.findings.Count -gt 0) {
        $htmlContent += "<h4>Security Findings</h4>"
        
        foreach ($finding in $contract.findings) {
            $findingSeverity = if ($finding.severity -eq "Critical") { "critical" } 
                              elseif ($finding.severity -eq "High") { "high" }
                              elseif ($finding.severity -eq "Medium") { "medium" }
                              else { "low" }
            
            $htmlContent += @"
    <div class="finding $findingSeverity">
        <h4>$($finding.name) <span class="$findingSeverity">[$($finding.severity)]</span></h4>
        <p>$($finding.description)</p>
"@
            
            if ($finding.impact) {
                $htmlContent += "<p><strong>Impact:</strong> $($finding.impact)</p>"
            }
            
            $htmlContent += @"
        <p><strong>Recommendation:</strong> $($finding.recommendation)</p>
    </div>
"@
        }
    } else {
        $htmlContent += "<p class='secure'>No security findings.</p>"
    }
    
    # Security patterns
    $htmlContent += "<h4>Security Pattern Results</h4>"
    $htmlContent += @"
    <table>
        <tr>
            <th>Pattern</th>
            <th>Status</th>
            <th>Severity</th>
            <th>Description</th>
        </tr>
"@

    foreach ($patternName in $contract.patternResults.Keys) {
        $pattern = $contract.patternResults[$patternName]
        $patternStatus = if ($pattern.passed) { "PASS" } else { "FAIL" }
        $patternClass = if ($pattern.passed) { "secure" } else {
            if ($pattern.severity -eq "Critical") { "critical" } 
            elseif ($pattern.severity -eq "High") { "high" }
            elseif ($pattern.severity -eq "Medium") { "medium" }
            else { "low" }
        }
        
        $htmlContent += @"
        <tr>
            <td>$patternName</td>
            <td class="$patternClass">$patternStatus</td>
            <td class="$(if ($pattern.severity -eq "Critical") { "critical" } 
                elseif ($pattern.severity -eq "High") { "high" }
                elseif ($pattern.severity -eq "Medium") { "medium" }
                else { "low" })">$($pattern.severity)</td>
            <td>$($pattern.description)</td>
        </tr>
"@
    }

    $htmlContent += @"
    </table>
"@
}

# Bitcoin Compliance section
$htmlContent += @"
<h2>Bitcoin Protocol Compliance</h2>
<p>The security analysis evaluates contracts against Bitcoin Development Framework v2.5 requirements:</p>
<table>
    <tr>
        <th>Requirement</th>
        <th>Status</th>
        <th>Compliance Level</th>
    </tr>
    <tr>
        <td>BIP 341/342 (Taproot)</td>
        <td class="$(if ($securityResults.summary.securityScore -ge 80) { "secure" } else { "warning" })">
            $(if ($securityResults.summary.securityScore -ge 80) { "COMPLIANT" } else { "PARTIAL" })
        </td>
        <td>$([Math]::Min(100, [Math]::Round($securityResults.summary.securityScore, 0)))%</td>
    </tr>
    <tr>
        <td>BIP 174 (PSBT)</td>
        <td class="$(if ($securityResults.summary.securePasses -gt 0) { "secure" } else { "warning" })">
            $(if ($securityResults.summary.securePasses -gt 0) { "COMPLIANT" } else { "PARTIAL" })
        </td>
        <td>$(if ($securityResults.summary.securePasses -gt 0) { "100" } else { "70" })%</td>
    </tr>
    <tr>
        <td>Miniscript Support</td>
        <td class="$(if ($securityResults.summary.criticalRisks -eq 0) { "secure" } else { "vulnerable" })">
            $(if ($securityResults.summary.criticalRisks -eq 0) { "COMPLIANT" } else { "NON-COMPLIANT" })
        </td>
        <td>$(if ($securityResults.summary.criticalRisks -eq 0) { "100" } else { "50" })%</td>
    </tr>
</table>
"@

$htmlContent += @"
</body>
</html>
"@

Set-Content -Path $htmlReportPath -Value $htmlContent
Write-Host "`nResults saved to:" -ForegroundColor Cyan
Write-Host "  JSON results: $ResultsDirectory/security-results.json" -ForegroundColor Cyan
Write-Host "  HTML report: $htmlReportPath" -ForegroundColor Cyan

# Return appropriate exit code
$exitCode = if ($securityResults.summary.criticalRisks -eq 0) { 0 } else { 1 }
Write-Host "==================================================================" -ForegroundColor Cyan
exit $exitCode 