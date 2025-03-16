# Basic contract verification script without Clarinet
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "--- Anya DAO - Basic Contract Verification                    ---" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan

# Track overall status
$overallStatus = $true

# Verify BIP implementations in contracts
Write-Host "`n--- Checking BIP Implementation in Contracts ---" -ForegroundColor Yellow

$contractPaths = @("dao", "src/contracts")
$contractFiles = @()

foreach ($path in $contractPaths) {
    if (Test-Path $path) {
        $files = Get-ChildItem -Path $path -Recurse -Filter "*.clar" -ErrorAction SilentlyContinue
        $contractFiles += $files
    }
}

$bipReqs = @{
    "BIP-341" = @{Name = "Taproot"; Required = $true; Found = $false}
    "BIP-174" = @{Name = "PSBT"; Required = $true; Found = $false}
    "BIP-370" = @{Name = "PSBT v2"; Required = $false; Found = $false}
    "BIP-342" = @{Name = "Tapscript"; Required = $true; Found = $false}
}

# Process each contract file
Write-Host "Found $($contractFiles.Count) contract files to check" -ForegroundColor Gray

foreach ($file in $contractFiles) {
    Write-Host "Checking file: $($file.Name)" -ForegroundColor Gray
    
    $content = Get-Content -Path $file.FullName -Raw
    
    foreach ($bip in $bipReqs.Keys) {
        $shortBip = $bip -replace "BIP-", ""
        
        # Check if BIP is mentioned in the file
        if ($content -match "BIP-$shortBip" -or $content -match "BIP $shortBip") {
            Write-Host "  - ✅ $bip ($($bipReqs[$bip].Name)) found in $($file.Name)" -ForegroundColor Green
            $bipReqs[$bip].Found = $true
        }
    }
}

# Check if all required BIPs were found
foreach ($bip in $bipReqs.Keys) {
    if ($bipReqs[$bip].Required -and -not $bipReqs[$bip].Found) {
        Write-Host "  - ❌ Required $bip ($($bipReqs[$bip].Name)) not found in any contract" -ForegroundColor Red
        $overallStatus = $false
    }
}

# Verify contract syntax (basic checks)
Write-Host "`n--- Basic Contract Syntax Check ---" -ForegroundColor Yellow

foreach ($file in $contractFiles) {
    Write-Host "Checking syntax: $($file.Name)" -ForegroundColor Gray
    
    $content = Get-Content -Path $file.FullName -Raw
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
    
    # Report results
    if ($errors.Count -eq 0) {
        Write-Host "  - ✅ No basic syntax issues detected in $($file.Name)" -ForegroundColor Green
    } else {
        Write-Host "  - ❌ Potential issues in $($file.Name):" -ForegroundColor Red
        foreach ($error in $errors) {
            Write-Host "      - $error" -ForegroundColor Red
        }
        $overallStatus = $false
    }
}

# Generate a simple compliance report in JSON format
$report = @{
    timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    contracts = @{}
    bipCompliance = $bipReqs
    overallStatus = if ($overallStatus) { "PASS" } else { "FAIL" }
}

# Add contract information
foreach ($file in $contractFiles) {
    $contractName = $file.Name
    $report.contracts[$contractName] = @{
        path = $file.FullName
        size = (Get-Item $file.FullName).Length
        bips = @{}
    }
    
    $content = Get-Content -Path $file.FullName -Raw
    foreach ($bip in $bipReqs.Keys) {
        $shortBip = $bip -replace "BIP-", ""
        $report.contracts[$contractName].bips[$bip] = ($content -match "BIP-$shortBip" -or $content -match "BIP $shortBip")
    }
}

# Save report as JSON
$reportJson = $report | ConvertTo-Json -Depth 5
Set-Content -Path "basic-compliance-report.json" -Value $reportJson

# Final summary
Write-Host "`n--- Verification Summary ---" -ForegroundColor Yellow

if ($overallStatus) {
    Write-Host "  - ✅ All basic checks passed" -ForegroundColor Green
} else {
    Write-Host "  - ❌ Some checks failed (see details above)" -ForegroundColor Red
}

Write-Host "`nBasic compliance report generated: basic-compliance-report.json" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan 