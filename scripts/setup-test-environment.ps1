# Test Environment Setup
Write-Host "==================================================================" -ForegroundColor Cyan
Write-Host "--- Setting up Test Environment                                ---" -ForegroundColor Cyan
Write-Host "==================================================================" -ForegroundColor Cyan

# Create necessary directories
$directories = @(
    "dao/core",
    "dao/traits",
    "dao/extensions",
    "src/contracts",
    "tests",
    "tests/modules",
    "tests/integration",
    "tests/system",
    "tests/performance",
    "tests/security",
    "test-results"
)

foreach ($dir in $directories) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Host "Created directory: $dir" -ForegroundColor Green
    } else {
        Write-Host "Directory already exists: $dir" -ForegroundColor Gray
    }
}

# Check if Clarinet.toml exists, create if not
if (-not (Test-Path "Clarinet.toml")) {
    $clarinetConfig = @"
[project]
name = "anya-dao"
authors = []
description = "Anya DAO - Bitcoin Development Framework Compliant"
telemetry = false

[[project.requirements]]
contract_id = "SP000000000000000000002Q6VF78.pox"

[contracts.dao-core]
path = "dao/core/dao-core.clar"
depends_on = ["dao-trait"]

[contracts.dao-trait]
path = "dao/traits/dao-trait.clar"

[contracts.dao]
path = "src/contracts/dao.clar"
depends_on = ["dao-core"]

[contracts.governance_token]
path = "src/contracts/governance_token.clar"

[contracts.bitcoin-issuance]
path = "src/contracts/bitcoin-issuance.clar"
depends_on = ["governance_token"]

[contracts.dex-adapter]
path = "src/contracts/dex-adapter.clar"

[contracts.token-economics]
path = "dao/extensions/token-economics.clar"
depends_on = ["bitcoin-issuance"]
"@

    Set-Content -Path "Clarinet.toml" -Value $clarinetConfig
    Write-Host "Created Clarinet.toml configuration" -ForegroundColor Green
}

# Create or update test configuration
$testConfig = @{
    testModules = @(
        "dao-core",
        "dao-trait",
        "governance-token",
        "bitcoin-issuance",
        "dex-adapter",
        "token-economics"
    )
    integrationGroups = @(
        @{
            name = "core-governance"
            modules = @("dao-core", "dao-trait", "dao")
        },
        @{
            name = "token-economics"
            modules = @("governance_token", "bitcoin-issuance", "token-economics")
        },
        @{
            name = "liquidity"
            modules = @("governance_token", "dex-adapter")
        }
    )
    complianceChecks = @(
        "BIP-341",
        "BIP-174",
        "BIP-342",
        "BIP-370" 
    )
    performanceBenchmarks = @(
        "proposal-creation",
        "voting",
        "token-transfers"
    )
}

$testConfigJson = $testConfig | ConvertTo-Json -Depth 5
Set-Content -Path "test-config.json" -Value $testConfigJson
Write-Host "Created/updated test configuration" -ForegroundColor Green

# Create missing contract templates if needed
$missingContracts = 0
$requiredContracts = @(
    "dao/core/dao-core.clar",
    "dao/traits/dao-trait.clar",
    "src/contracts/dao.clar",
    "src/contracts/governance_token.clar",
    "src/contracts/bitcoin-issuance.clar", 
    "src/contracts/dex-adapter.clar",
    "dao/extensions/token-economics.clar"
)

foreach ($contract in $requiredContracts) {
    if (-not (Test-Path $contract)) {
        $missingContracts++
    }
}

if ($missingContracts -gt 0) {
    Write-Host "$missingContracts contracts are missing. Creating templates..." -ForegroundColor Yellow
    if (Test-Path "scripts/create-contract-templates.ps1") {
        & "scripts/create-contract-templates.ps1"
    } else {
        Write-Host "❌ Contract template creator script not found" -ForegroundColor Red
        exit 1
    }
}

# Report success
Write-Host "Test environment setup complete!" -ForegroundColor Green
Write-Host "==================================================================" -ForegroundColor Cyan

# Return success
exit 0 