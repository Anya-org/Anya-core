# Create all necessary directories for Anya DAO
$directories = @(
    "dao/core",
    "dao/traits",
    "dao/extensions",
    "src/contracts",
    "tests",
    "scripts"
)

foreach ($dir in $directories) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Host "Created directory: $dir" -ForegroundColor Green
    } else {
        Write-Host "Directory already exists: $dir" -ForegroundColor Yellow
    }
}

# Create Clarinet.toml if it doesn't exist
if (-not (Test-Path "Clarinet.toml")) {
    $clarinetConfig = @"
[project]
name = "anya-dao"
authors = []
description = ""
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

Write-Host "`nSetup completed successfully!" -ForegroundColor Cyan 