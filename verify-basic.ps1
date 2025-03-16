# Basic verification script for Anya DAO
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "--- Anya DAO - Basic Verification Checks      ---" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan

# Test directory setup
$contractsDir = "src/contracts"
$testsDir = "tests"
$daoDir = "dao"

# Check if directories exist
Write-Host "`n--- Checking directory structure ---" -ForegroundColor Yellow
$dirChecks = @(
    @{Path=$contractsDir; Name="Contracts directory"},
    @{Path=$testsDir; Name="Tests directory"},
    @{Path=$daoDir; Name="DAO directory"},
    @{Path="$daoDir/core"; Name="DAO core directory"},
    @{Path="$daoDir/traits"; Name="DAO traits directory"},
    @{Path="$daoDir/extensions"; Name="DAO extensions directory"}
)

foreach ($dir in $dirChecks) {
    if (Test-Path $dir.Path) {
        Write-Host "  - ✅ $($dir.Name) exists" -ForegroundColor Green
    } else {
        Write-Host "  - ❌ $($dir.Name) not found" -ForegroundColor Red
        Write-Host "      Creating directory: $($dir.Path)" -ForegroundColor Yellow
        New-Item -ItemType Directory -Path $dir.Path -Force | Out-Null
    }
}

# Check if required contract files exist
Write-Host "`n--- Checking contract files ---" -ForegroundColor Yellow
$contractFiles = @(
    @{Path="$daoDir/core/dao-core.clar"; Name="DAO Core contract"},
    @{Path="$daoDir/traits/dao-trait.clar"; Name="DAO Trait contract"},
    @{Path="$contractsDir/dao.clar"; Name="Main DAO contract"},
    @{Path="$contractsDir/governance_token.clar"; Name="Governance Token contract"},
    @{Path="$contractsDir/bitcoin-issuance.clar"; Name="Bitcoin Issuance contract"},
    @{Path="$contractsDir/dex-adapter.clar"; Name="DEX Adapter contract"},
    @{Path="$daoDir/extensions/token-economics.clar"; Name="Token Economics contract"}
)

$missingContracts = 0
foreach ($file in $contractFiles) {
    if (Test-Path $file.Path) {
        Write-Host "  - ✅ $($file.Name) exists" -ForegroundColor Green
    } else {
        Write-Host "  - ❌ $($file.Name) not found" -ForegroundColor Red
        $missingContracts++
    }
}

# Check test files
Write-Host "`n--- Checking test files ---" -ForegroundColor Yellow
$testFiles = @(
    @{Path="$testsDir/governance-token.test.clar"; Name="Governance Token tests"},
    @{Path="$testsDir/dao-core.test.clar"; Name="DAO Core tests"},
    @{Path="$testsDir/bitcoin-issuance.test.clar"; Name="Bitcoin Issuance tests"},
    @{Path="$testsDir/dex-adapter.test.clar"; Name="DEX Adapter tests"},
    @{Path="$testsDir/token-economics.test.clar"; Name="Token Economics tests"},
    @{Path="$testsDir/dao.test.clar"; Name="Main DAO tests"},
    @{Path="$testsDir/dao-system.test.ts"; Name="System Integration tests"}
)

$missingTests = 0
foreach ($file in $testFiles) {
    if (Test-Path $file.Path) {
        Write-Host "  - ✅ $($file.Name) exist" -ForegroundColor Green
    } else {
        Write-Host "  - ❌ $($file.Name) not found" -ForegroundColor Red
        $missingTests++
    }
}

# Summary
Write-Host "`n--- Verification Summary ---" -ForegroundColor Yellow
if ($missingContracts -eq 0) {
    Write-Host "  - ✅ All contract files exist" -ForegroundColor Green
} else {
    Write-Host "  - ❌ Missing $missingContracts contract files" -ForegroundColor Red
}

if ($missingTests -eq 0) {
    Write-Host "  - ✅ All test files exist" -ForegroundColor Green
} else {
    Write-Host "  - ❌ Missing $missingTests test files" -ForegroundColor Red
}

Write-Host "`n==================================================" -ForegroundColor Cyan
Write-Host "Basic verification completed!" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan 