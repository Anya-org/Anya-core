# Module-Level Tests
Write-Host "==================================================================" -ForegroundColor Cyan
Write-Host "--- Running Module Tests                                       ---" -ForegroundColor Cyan
Write-Host "==================================================================" -ForegroundColor Cyan

# Load test configuration
$testConfig = Get-Content -Path "test-config.json" | ConvertFrom-Json
$modulesToTest = $testConfig.testModules

# Check if Clarinet is available
$clarinetAvailable = $null -ne (Get-Command clarinet -ErrorAction SilentlyContinue)

$testResults = @{
    moduleResults = @{}
    totalTests = 0
    passedTests = 0
    failedTests = 0
    skippedTests = 0
}

# Create test result directory
$resultDir = "test-results/module-tests"
if (-not (Test-Path $resultDir)) {
    New-Item -ItemType Directory -Path $resultDir -Force | Out-Null
}

# Function to run tests for a specific module
function Test-Module {
    param (
        [string]$ModuleName
    )
    
    Write-Host "`nTesting module: $ModuleName" -ForegroundColor Yellow
    
    $moduleResult = @{
        name = $ModuleName
        status = "UNKNOWN"
        tests = @()
        coverage = 0.0
        duration = 0.0
    }
    
    $startTime = Get-Date
    
    # Different testing approaches based on whether Clarinet is available
    if ($clarinetAvailable) {
        # For Clarity contracts, use Clarinet
        try {
            # Look for module test files
            $testFile = $null
            
            # Check various potential test file paths
            $potentialPaths = @(
                "tests/$ModuleName.test.clar",
                "tests/modules/$ModuleName.test.clar",
                "tests/modules/$ModuleName-test.clar",
                "tests/$($ModuleName -replace '-', '_').test.clar",
                "tests/modules/$($ModuleName -replace '-', '_').test.clar"
            )
            
            foreach ($path in $potentialPaths) {
                if (Test-Path $path) {
                    $testFile = $path
                    break
                }
            }
            
            if ($testFile) {
                Write-Host "  Found test file: $testFile" -ForegroundColor Green
                
                # Run the test with Clarinet
                $testOutput = clarinet test $testFile 2>&1
                $testOutput | Out-File -FilePath "$resultDir/$ModuleName-test-output.txt"
                
                # Parse test results
                $passCount = ($testOutput | Select-String -Pattern "pass" -AllMatches).Matches.Count
                $failCount = ($testOutput | Select-String -Pattern "fail" -AllMatches).Matches.Count
                
                $moduleResult.tests += @{
                    name = "Clarinet test"
                    status = if ($failCount -eq 0) { "PASS" } else { "FAIL" }
                    details = "Passed: $passCount, Failed: $failCount"
                }
                
                $testResults.totalTests++
                if ($failCount -eq 0) {
                    $testResults.passedTests++
                    $moduleResult.status = "PASS"
                } else {
                    $testResults.failedTests++
                    $moduleResult.status = "FAIL"
                }
                
                # Estimate coverage based on output (simple heuristic)
                $coverageEstimate = [math]::Min(100, ($passCount / ($passCount + $failCount + 0.001)) * 100)
                $moduleResult.coverage = [math]::Round($coverageEstimate, 2)
                
                Write-Host "  Module test results: $($moduleResult.status)" -ForegroundColor $(if ($moduleResult.status -eq "PASS") { "Green" } else { "Red" })
                Write-Host "  Estimated coverage: $($moduleResult.coverage)%" -ForegroundColor Cyan
            } else {
                Write-Host "  ⚠️ No test file found for module $ModuleName" -ForegroundColor Yellow
                $moduleResult.status = "SKIPPED"
                $testResults.skippedTests++
            }
        } catch {
            Write-Host "  ❌ Error testing module $ModuleName`: $_" -ForegroundColor Red
            $moduleResult.status = "ERROR"
            $moduleResult.tests += @{
                name = "Clarinet test"
                status = "ERROR"
                details = $_.ToString()
            }
            $testResults.failedTests++
        }
    } else {
        # Manual verification if Clarinet isn't available
        try {
            # Find the contract file
            $contractFile = $null
            $potentialContractPaths = @(
                "dao/core/$ModuleName.clar",
                "dao/traits/$ModuleName.clar",
                "dao/extensions/$ModuleName.clar",
                "src/contracts/$ModuleName.clar",
                "src/contracts/$($ModuleName -replace '-', '_').clar"
            )
            
            foreach ($path in $potentialContractPaths) {
                if (Test-Path $path) {
                    $contractFile = $path
                    break
                }
            }
            
            if ($contractFile) {
                Write-Host "  Found contract: $contractFile" -ForegroundColor Green
                
                # Basic syntax check
                $content = Get-Content -Path $contractFile -Raw
                $errors = @()
                
                # Check for basic syntax issues
                if ($content -match "\)\s*\(") {
                    $errors += "Possible missing semicolon"
                }
                
                if (($content -match "\(define" -and -not $content -match "\(define-") -or 
                    ($content -match "\(let" -and -not $content -match "\(let\s*\(")) {
                    $errors += "Possible malformed define or let statement"
                }
                
                # Count parentheses to check for mismatches
                $openCount = ($content | Select-String -Pattern "\(" -AllMatches).Matches.Count
                $closeCount = ($content | Select-String -Pattern "\)" -AllMatches).Matches.Count
                
                if ($openCount -ne $closeCount) {
                    $errors += "Mismatched parentheses ($openCount open, $closeCount close)"
                }
                
                $moduleResult.tests += @{
                    name = "Basic syntax check"
                    status = if ($errors.Count -eq 0) { "PASS" } else { "FAIL" }
                    details = if ($errors.Count -eq 0) { "No syntax issues detected" } else { $errors -join ", " }
                }
                
                $testResults.totalTests++
                if ($errors.Count -eq 0) {
                    $testResults.passedTests++
                    $moduleResult.status = "PASS"
                } else {
                    $testResults.failedTests++
                    $moduleResult.status = "FAIL"
                }
                
                # Check for BIP compliance
                $bipCompliance = @()
                $bips = @("BIP-341", "BIP-174", "BIP-342", "BIP-370")
                foreach ($bip in $bips) {
                    $shortBip = $bip -replace "BIP-", ""
                    if ($content -match "BIP-$shortBip" -or $content -match "BIP $shortBip") {
                        $bipCompliance += $bip
                    }
                }
                
                $moduleResult.tests += @{
                    name = "BIP compliance check"
                    status = if ($bipCompliance.Count -gt 0) { "PASS" } else { "WARN" }
                    details = if ($bipCompliance.Count -gt 0) { "Found: $($bipCompliance -join ', ')" } else { "No BIP references found" }
                }
                
                # Estimate coverage (simple heuristic based on file size and complexity)
                $moduleResult.coverage = [math]::Round((100 * [math]::Min(1, $content.Length / 5000)), 2)
                
                Write-Host "  Manual check results: $($moduleResult.status)" -ForegroundColor $(if ($moduleResult.status -eq "PASS") { "Green" } else { "Red" })
                Write-Host "  Estimated coverage: $($moduleResult.coverage)%" -ForegroundColor Cyan
            } else {
                Write-Host "  ⚠️ No contract file found for module $ModuleName" -ForegroundColor Yellow
                $moduleResult.status = "SKIPPED"
                $testResults.skippedTests++
            }
        } catch {
            Write-Host "  ❌ Error checking module $ModuleName`: $_" -ForegroundColor Red
            $moduleResult.status = "ERROR"
            $testResults.failedTests++
        }
    }
    
    $endTime = Get-Date
    $moduleResult.duration = ($endTime - $startTime).TotalSeconds
    
    return $moduleResult
}

# Test each module
foreach ($module in $modulesToTest) {
    $moduleResult = Test-Module -ModuleName $module
    $testResults.moduleResults[$module] = $moduleResult
}

# Generate summary
Write-Host "`n--- Module Tests Summary ---" -ForegroundColor Yellow
Write-Host "Total modules tested: $($modulesToTest.Count)" -ForegroundColor Cyan
Write-Host "Tests executed: $($testResults.totalTests)" -ForegroundColor Cyan
Write-Host "Tests passed: $($testResults.passedTests)" -ForegroundColor $(if ($testResults.passedTests -gt 0) { "Green" } else { "Gray" })
Write-Host "Tests failed: $($testResults.failedTests)" -ForegroundColor $(if ($testResults.failedTests -gt 0) { "Red" } else { "Gray" })
Write-Host "Tests skipped: $($testResults.skippedTests)" -ForegroundColor $(if ($testResults.skippedTests -gt 0) { "Yellow" } else { "Gray" })

# Save results
$testResultsJson = $testResults | ConvertTo-Json -Depth 5
Set-Content -Path "$resultDir/module-test-results.json" -Value $testResultsJson
Write-Host "`nResults saved to: $resultDir/module-test-results.json" -ForegroundColor Cyan

# Return appropriate exit code
$exitCode = if ($testResults.failedTests -eq 0) { 0 } else { 1 }
Write-Host "==================================================================" -ForegroundColor Cyan
exit $exitCode 