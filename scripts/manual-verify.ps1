# Manual verification script that doesn't require Clarinet
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "--- Anya DAO - Manual Contract Verification                  ---" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan

# Ensure the basic directory structure exists
./scripts/setup-directories.ps1

# Check for required files
Write-Host "`n--- Checking Contract Files ---" -ForegroundColor Yellow

$requiredFiles = @(
    @{Path="dao/core/dao-core.clar"; Description="Core DAO implementation"},
    @{Path="dao/traits/dao-trait.clar"; Description="DAO trait definition"},
    @{Path="src/contracts/dao.clar"; Description="Main DAO contract"},
    @{Path="src/contracts/governance_token.clar"; Description="Governance token contract"},
    @{Path="src/contracts/bitcoin-issuance.clar"; Description="Bitcoin-style issuance contract"},
    @{Path="src/contracts/dex-adapter.clar"; Description="DEX adapter contract"},
    @{Path="dao/extensions/token-economics.clar"; Description="Token economics contract"}
)

$missingFiles = 0
foreach ($file in $requiredFiles) {
    if (Test-Path $file.Path) {
        Write-Host "  - ✅ Found $($file.Path) ($($file.Description))" -ForegroundColor Green
    } else {
        Write-Host "  - ❌ Missing $($file.Path) ($($file.Description))" -ForegroundColor Red
        $missingFiles++
    }
}

if ($missingFiles -gt 0) {
    Write-Host "`n⚠️ There are $missingFiles missing contract files. Creating templates..." -ForegroundColor Yellow
    
    # Call the file creation script
    ./scripts/create-contract-templates.ps1
}

# Run the basic contract verification
Write-Host "`n--- Running Basic Contract Verification ---" -ForegroundColor Yellow
./scripts/verify-contracts-basic.ps1

# Generate compliance report
Write-Host "`n--- Generating Compliance Report ---" -ForegroundColor Yellow
if (Test-Path "scripts/generate-compliance-report.ts") {
    try {
        npx ts-node scripts/generate-compliance-report.ts
        Write-Host "  - ✅ Compliance report generated successfully" -ForegroundColor Green
    } catch {
        Write-Host "  - ❌ Error generating compliance report: $_" -ForegroundColor Red
    }
} else {
    Write-Host "  - ❌ Missing compliance report script" -ForegroundColor Red
}

Write-Host "`nManual verification complete!" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan 