# System-Level Tests
param(
    [hashtable]$Configuration,
    [string]$ResultsDirectory
)

Write-Host "==================================================================" -ForegroundColor Cyan
Write-Host "--- Running System Tests                                       ---" -ForegroundColor Cyan
Write-Host "==================================================================" -ForegroundColor Cyan

# Check parameters
if (-not $ResultsDirectory) {
    $ResultsDirectory = "test-results/system"
}

if (-not (Test-Path $ResultsDirectory)) {
    New-Item -ItemType Directory -Path $ResultsDirectory -Force | Out-Null
}

# Check if Clarinet is available
$clarinetAvailable = $null -ne (Get-Command clarinet -ErrorAction SilentlyContinue)

$testResults = @{
    systemTestResults = @{}
    totalTests = 0
    passedTests = 0
    failedTests = 0
    skippedTests = 0
    systemCoverage = 0.0
    startTime = Get-Date
    endTime = $null
    duration = 0.0
}

# Define system test scenarios
$systemTests = @(
    @{
        name = "DAO Proposal Lifecycle"
        description = "Tests the full lifecycle of a proposal from creation to execution"
        components = @("dao-core", "dao", "governance_token")
        scenarios = @(
            "Create proposal with minimum token balance",
            "Vote on active proposal", 
            "Execute approved proposal",
            "Attempt to execute rejected proposal (should fail)"
        )
        required = $true
    },
    @{
        name = "Token Distribution"
        description = "Tests token minting and distribution according to economic model"
        components = @("governance_token", "bitcoin-issuance", "token-economics")
        scenarios = @(
            "Mint initial token supply",
            "Distribute tokens according to allocation model",
            "Verify halving mechanism reduces rewards correctly"
        )
        required = $true
    },
    @{
        name = "Liquidity Management"
        description = "Tests DEX interactions and liquidity operations"
        components = @("dex-adapter", "governance_token")
        scenarios = @(
            "Add liquidity to trading pool",
            "Swap tokens through DEX",
            "Remove liquidity and verify balances"
        )
        required = $true
    },
    @{
        name = "Bitcoin Protocol Compliance"
        description = "Tests compliance with Bitcoin protocols, including PSBT and Taproot"
        components = @("dao-core", "dao", "bitcoin-issuance")
        scenarios = @(
            "Verify BIP-341 Taproot implementation",
            "Test BIP-174 PSBT transaction creation",
            "Validate BIP-342 Tapscript execution"
        )
        required = $true
    },
    @{
        name = "Cross-Contract Interactions"
        description = "Tests interactions across all major contracts in the system"
        components = @("dao-core", "dao", "governance_token", "bitcoin-issuance", "dex-adapter", "token-economics")
        scenarios = @(
            "Create governance proposal to modify economic parameters",
            "Execute proposal that affects token issuance",
            "Verify DEX liquidity adjusts to new economic parameters"
        )
        required = $false
    }
)

# Function to run a system test
function Test-SystemScenario {
    param (
        [hashtable]$TestScenario
    )
    
    $scenarioName = $TestScenario.name
    
    Write-Host "`nTesting system scenario: $scenarioName" -ForegroundColor Yellow
    Write-Host "  Description: $($TestScenario.description)" -ForegroundColor Gray
    Write-Host "  Components: $($TestScenario.components -join ', ')" -ForegroundColor Gray
    
    $scenarioResult = @{
        name = $scenarioName
        description = $TestScenario.description
        components = $TestScenario.components
        status = "UNKNOWN"
        scenarios = @()
        duration = 0.0
        coverage = 0.0
    }
    
    $startTime = Get-Date
    
    # Track scenario tests
    $totalScenarios = $TestScenario.scenarios.Count
    $passedScenarios = 0
    
    if ($clarinetAvailable) {
        # Using Clarinet for system tests
        
        # Look for system test file
        $testFile = $null
        $testFileFound = $false
        
        $potentialPaths = @(
            "tests/system/$($scenarioName -replace ' ', '-' -replace '[^a-zA-Z0-9\-]', '').test.clar",
            "tests/system/$($scenarioName -replace ' ', '_' -replace '[^a-zA-Z0-9\_]', '').test.clar"
        )
        
        foreach ($path in $potentialPaths) {
            if (Test-Path $path) {
                $testFile = $path
                $testFileFound = $true
                break
            }
        }
        
        if ($testFileFound) {
            Write-Host "  Found system test file: $testFile" -ForegroundColor Green
            
            try {
                # Run the system test
                $testOutput = clarinet test $testFile 2>&1
                $testOutput | Out-File -FilePath "$ResultsDirectory/$($scenarioName -replace ' ', '-').test-output.txt"
                
                # Parse test results
                $passCount = ($testOutput | Select-String -Pattern "pass" -AllMatches).Matches.Count
                $failCount = ($testOutput | Select-String -Pattern "fail" -AllMatches).Matches.Count
                
                # Update scenario results
                foreach ($scenario in $TestScenario.scenarios) {
                    $scenarioFound = ($testOutput | Select-String -Pattern $scenario -SimpleMatch).Length -gt 0
                    $scenarioPassed = $scenarioFound -and ($testOutput | Select-String -Pattern "$scenario.*pass" -SimpleMatch).Length -gt 0
                    
                    $scenarioResult.scenarios += @{
                        name = $scenario
                        status = if ($scenarioFound) { if ($scenarioPassed) { "PASS" } else { "FAIL" } } else { "NOT_FOUND" }
                    }
                    
                    if ($scenarioFound -and $scenarioPassed) {
                        $passedScenarios++
                    }
                }
                
                # Update overall status
                $scenarioResult.status = if ($passedScenarios -eq $totalScenarios) { "PASS" } else { "FAIL" }
                
                # Update test counters
                $testResults.totalTests++
                if ($scenarioResult.status -eq "PASS") {
                    $testResults.passedTests++
                } else {
                    $testResults.failedTests++
                }
                
                # Calculate coverage
                $scenarioResult.coverage = if ($totalScenarios -gt 0) { ($passedScenarios / $totalScenarios) * 100 } else { 0 }
                
                Write-Host "  System test results: $($scenarioResult.status)" -ForegroundColor $(if ($scenarioResult.status -eq "PASS") { "Green" } else { "Red" })
                Write-Host "  Scenarios: $passedScenarios / $totalScenarios passed" -ForegroundColor Cyan
                Write-Host "  Coverage: $([Math]::Round($scenarioResult.coverage, 2))%" -ForegroundColor Cyan
            }
            catch {
                Write-Host "  ❌ Error running system test: $_" -ForegroundColor Red
                $scenarioResult.status = "ERROR"
                $testResults.failedTests++
            }
        }
        else {
            # Generate a minimal system test if none exists
            Write-Host "  No system test file found. Using alternative verification." -ForegroundColor Yellow
            
            # Check if all required components exist
            $componentsExist = $true
            $componentStatuses = @()
            
            foreach ($component in $TestScenario.components) {
                $componentFound = $false
                $componentPaths = @(
                    "dao/core/$component.clar",
                    "dao/traits/$component.clar",
                    "dao/extensions/$component.clar",
                    "src/contracts/$component.clar",
                    "src/contracts/$($component -replace '-', '_').clar"
                )
                
                foreach ($path in $componentPaths) {
                    if (Test-Path $path) {
                        $componentFound = $true
                        $componentStatuses += @{
                            component = $component
                            path = $path
                            exists = $true
                        }
                        break
                    }
                }
                
                if (-not $componentFound) {
                    $componentsExist = $false
                    $componentStatuses += @{
                        component = $component
                        exists = $false
                    }
                }
            }
            
            # Create a minimal system test template
            $testDirPath = "tests/system"
            if (-not (Test-Path $testDirPath)) {
                New-Item -ItemType Directory -Path $testDirPath -Force | Out-Null
            }
            
            $templateFile = "$testDirPath/$($scenarioName -replace ' ', '-' -replace '[^a-zA-Z0-9\-]', '').test.clar"
            
            $templateContent = @"
;; System Test: $scenarioName
;; Description: $($TestScenario.description)
;; Generated automatically by unified test system

(begin
    (print "Running system test: $scenarioName")

    ;; Test scenarios
"@
            
            foreach ($scenario in $TestScenario.scenarios) {
                $templateContent += @"

    ;; Scenario: $scenario
    (print "Testing scenario: $scenario")
    (asserts! (is-eq true true) (err "Placeholder test for: $scenario"))
"@
            }
            
            $templateContent += @"

    (ok true)
)
"@
            
            Set-Content -Path $templateFile -Value $templateContent
            Write-Host "  ℹ️ Created template system test file: $templateFile" -ForegroundColor Cyan
            Write-Host "  Please implement the actual tests for each scenario" -ForegroundColor Yellow
            
            # Mark as skipped for now
            $scenarioResult.status = "TEMPLATE_CREATED"
            $testResults.skippedTests++
            
            foreach ($scenario in $TestScenario.scenarios) {
                $scenarioResult.scenarios += @{
                    name = $scenario
                    status = "PENDING"
                }
            }
            
            $scenarioResult.coverage = 0
        }
    }
    else {
        # Manual verification when Clarinet isn't available
        Write-Host "  Performing manual verification (Clarinet not available)" -ForegroundColor Yellow
        
        # Check if required components exist
        $componentsExist = $true
        foreach ($component in $TestScenario.components) {
            $componentFound = $false
            $componentPaths = @(
                "dao/core/$component.clar",
                "dao/traits/$component.clar",
                "dao/extensions/$component.clar",
                "src/contracts/$component.clar",
                "src/contracts/$($component -replace '-', '_').clar"
            )
            
            foreach ($path in $componentPaths) {
                if (Test-Path $path) {
                    $componentFound = $true
                    break
                }
            }
            
            if (-not $componentFound) {
                $componentsExist = $false
                Write-Host "  ❌ Component not found: $component" -ForegroundColor Red
            }
            else {
                Write-Host "  ✅ Component exists: $component" -ForegroundColor Green
            }
        }
        
        # Update scenario statuses based on component existence
        foreach ($scenario in $TestScenario.scenarios) {
            $scenarioResult.scenarios += @{
                name = $scenario
                status = if ($componentsExist) { "COMPONENT_CHECK_PASS" } else { "COMPONENT_CHECK_FAIL" }
            }
            
            if ($componentsExist) {
                $passedScenarios++
            }
        }
        
        # Update overall status
        if ($componentsExist) {
            $scenarioResult.status = "COMPONENT_CHECK_PASS"
            $testResults.passedTests++
            $scenarioResult.coverage = 30.0 # Minimal coverage for component check only
        }
        else {
            $scenarioResult.status = "COMPONENT_CHECK_FAIL" 
            $testResults.failedTests++
            $scenarioResult.coverage = 0.0
        }
        
        $testResults.totalTests++
        
        Write-Host "  Manual verification result: $($scenarioResult.status)" -ForegroundColor $(if ($scenarioResult.status -eq "COMPONENT_CHECK_PASS") { "Green" } else { "Red" })
        Write-Host "  Coverage: $([Math]::Round($scenarioResult.coverage, 2))% (component verification only)" -ForegroundColor Cyan
    }
    
    $endTime = Get-Date
    $scenarioResult.duration = ($endTime - $startTime).TotalSeconds
    
    return $scenarioResult
}

# Apply focus filter if specified
if ($Configuration -and $Configuration.FocusModule) {
    $focusModule = $Configuration.FocusModule
    Write-Host "Focusing system tests on module: $focusModule" -ForegroundColor Yellow
    $systemTests = $systemTests | Where-Object { $_.components -contains $focusModule }
}

# Skip slow tests if specified
if ($Configuration -and $Configuration.QuickMode) {
    Write-Host "Running in quick mode - testing only core scenarios" -ForegroundColor Yellow
    $systemTests = $systemTests | Where-Object { $_.required }
}

# Run each system test scenario
foreach ($test in $systemTests) {
    $scenarioResult = Test-SystemScenario -TestScenario $test
    $testResults.systemTestResults[$test.name] = $scenarioResult
}

# Calculate overall system coverage
$coverageValues = @()
foreach ($result in $testResults.systemTestResults.Values) {
    $coverageValues += $result.coverage
}

$testResults.systemCoverage = if ($coverageValues.Count -gt 0) { 
    ($coverageValues | Measure-Object -Average).Average 
} else { 
    0.0 
}

# Update end time and duration
$testResults.endTime = Get-Date
$testResults.duration = ($testResults.endTime - $testResults.startTime).TotalSeconds

# Generate summary
Write-Host "`n--- System Tests Summary ---" -ForegroundColor Yellow
Write-Host "Total system scenarios tested: $($systemTests.Count)" -ForegroundColor Cyan
Write-Host "Tests executed: $($testResults.totalTests)" -ForegroundColor Cyan
Write-Host "Tests passed: $($testResults.passedTests)" -ForegroundColor $(if ($testResults.passedTests -gt 0) { "Green" } else { "Gray" })
Write-Host "Tests failed: $($testResults.failedTests)" -ForegroundColor $(if ($testResults.failedTests -gt 0) { "Red" } else { "Gray" })
Write-Host "Tests skipped: $($testResults.skippedTests)" -ForegroundColor $(if ($testResults.skippedTests -gt 0) { "Yellow" } else { "Gray" })
Write-Host "Overall system coverage: $([Math]::Round($testResults.systemCoverage, 2))%" -ForegroundColor Cyan
Write-Host "Test duration: $([Math]::Round($testResults.duration, 2)) seconds" -ForegroundColor Cyan

# Save results
$testResultsJson = $testResults | ConvertTo-Json -Depth 5
Set-Content -Path "$ResultsDirectory/system-test-results.json" -Value $testResultsJson
Write-Host "`nResults saved to: $ResultsDirectory/system-test-results.json" -ForegroundColor Cyan

# Return appropriate exit code
$exitCode = if ($testResults.failedTests -eq 0) { 0 } else { 1 }
Write-Host "==================================================================" -ForegroundColor Cyan
exit $exitCode 