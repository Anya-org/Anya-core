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
    "test-results",
    "src/protocols"
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
    searchParams = @{
        maxKeywords = 10
        responseLimit = 50
        indexPatterns = @(
            "creator",
            "category",
            "date-range"
        )
    }
    testTypes = @(
        "basic",
        "compliance",
        "security",
        "search",
        "indexing"
    )
    testPatterns = @{
        "bip-compliance" = @{
            patterns = @("verify-taproot-signature", "process-psbt")
            required = $true
        }
    }
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

# Create test configuration template
$testConfigPath = "test-config.json"
if (-not (Test-Path $testConfigPath)) {
    $testConfig = @{
        modules = @(
            @{ 
                name = "dao-core"
                path = "dao/core/dao-core.clar"
                testTypes = @("basic", "compliance", "security")
            },
            @{
                name = "governance-token" 
                path = "src/contracts/governance_token.clar"
                testTypes = @("basic", "bip-compliance")
            }
        )
        testPatterns = @{
            "bip-compliance" = @{
                patterns = @("verify-taproot-signature", "process-psbt")
                required = $true
            }
        }
    }
    $testConfig | ConvertTo-Json -Depth 4 | Out-File $testConfigPath
    Write-Host "Created test configuration template" -ForegroundColor Green
}

# Create BIP protocol templates if missing
$protocolTemplates = @{
    "src/protocols/bip-341.clar" = @"
;; BIP-341 (Taproot) Protocol Adapter
(define-read-only (verify-taproot-signature
    (msg-hash (buff 32))
    (sig (buff 64))
    (pubkey (buff 33))
)
    ;; Implementation should use Bitcoin Core validation logic
    (ok true)
)
"@

    "src/protocols/bip-370.clar" = @"
;; BIP-370 (PSBT v2) Protocol Adapter
(define-public (process-psbt-v2 (psbt (buff 1024)))
    ;; Implementation should validate PSBT v2 structure
    (ok psbt)
)
"@

    "src/protocols/bip-174.clar" = @"
;; BIP-174 (PSBT) Protocol Adapter
(define-public (process-psbt-v0 (psbt (buff 1024)))
    ;; Implementation should validate PSBT v0 structure
    (ok psbt)
)
"@
}

foreach ($proto in $protocolTemplates.GetEnumerator()) {
    if (-not (Test-Path $proto.Key)) {
        Set-Content -Path $proto.Key -Value $proto.Value
        Write-Host "Created protocol template: $($proto.Key)" -ForegroundColor Yellow
    }
}

# Validate Hexagonal Structure
$hexComponents = @{
    "Adapter Layer" = @("src/adapters/psbt-adapter.clar", "src/adapters/taproot-adapter.clar")
    "Core Logic" = @("dao/core/dao-core.clar", "src/contracts/governance_token.clar")
    "Protocol Adapters" = @("src/protocols/bip-341.clar", "src/protocols/bip-370.clar", "src/protocols/bip-174.clar")
}

foreach ($component in $hexComponents.GetEnumerator()) {
    Write-Host "Validating $($component.Key) components..."
    foreach ($file in $component.Value) {
        if (-not (Test-Path $file)) {
            throw "Hexagonal architecture violation: Missing $($component.Key) component - $file"
        }
    }
}

# Create BIP-341 secured symlinks
New-Item -ItemType SymbolicLink -Path "test-results/security" -Target "tests/security" -Force | Out-Null

# Create adapter layer templates if missing
$adapterTemplates = @{
    "src/adapters/psbt-adapter.clar" = @"
;; PSBT Adapter (BIP-174/370)
(define-public (process-psbt (psbt (buff 1024)) (version uint))
    (if (> version u1)
        (contract-call? .bip-370 process-psbt-v2 psbt)
        (contract-call? .bip-174 process-psbt-v0 psbt)
    )
)
"@
    
    "src/adapters/taproot-adapter.clar" = @"
;; Taproot Adapter (BIP-341)
(define-read-only (verify-taproot-sig 
    (msg-hash (buff 32)) 
    (sig (buff 64)) 
    (pubkey (buff 33))
)
    (contract-call? .bip-341 verify-taproot-signature msg-hash sig pubkey)
)
"@
}

# Create adapters directory if missing
if (-not (Test-Path "src/adapters")) {
    New-Item -ItemType Directory -Path "src/adapters" -Force | Out-Null
}

foreach ($adapter in $adapterTemplates.GetEnumerator()) {
    if (-not (Test-Path $adapter.Key)) {
        Set-Content -Path $adapter.Key -Value $adapter.Value
        Write-Host "Created adapter template: $($adapter.Key)" -ForegroundColor Yellow
    }
}

# Report success
Write-Host "Test environment setup complete!" -ForegroundColor Green
Write-Host "==================================================================" -ForegroundColor Cyan

# Return success
exit 0 