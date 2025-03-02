# PowerShell script to resolve merge conflicts
Write-Host "Resolving merge conflicts..."

# Accept our changes for all conflicted files
$conflicts = @(
    "AGENT_ARCHITECTURE.md",
    "NEW_FEATURES.md",
    "ROADMAP.md",
    "SECURITY.md",
    "docs/README.md",
    "docs/api/README.md",
    "docs/development.md",
    "lib/src/api/server.dart",
    "pubspec.yaml",
    "scripts/install.py",
    "scripts/migrate_to_web5.dart",
    "scripts/requirements.txt",
    "src/enterprise/anya-enterprise/Cargo.toml",
    "src/extensions/anya-extensions/Cargo.toml",
    "src/governance/dao.rs",
    "test/bitcoin/wallet_test.dart",
    "test/storage/dwn_store_test.dart",
    "test/web5/identity_test.dart"
)

foreach ($file in $conflicts) {
    if (Test-Path $file) {
        Write-Host "Resolving conflict in $file"
        git checkout --ours $file
        git add $file
    }
}

# Handle special cases
if (Test-Path "dash33") {
    Write-Host "Resolving dash33 directory conflict"
    git add dash33
}
if (Test-Path "enterprise~HEAD") {
    Write-Host "Resolving enterprise~HEAD directory conflict"
    git add enterprise~HEAD
}
if (Test-Path "mobile") {
    Write-Host "Resolving mobile directory conflict"
    git add mobile
}

Write-Host "All conflicts resolved. Ready to commit."
