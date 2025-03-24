# Unified cleanup script
$ErrorActionPreference = "Stop"

# Clean build artifacts
Remove-Item -Recurse -Force target -ErrorAction SilentlyContinue
Remove-Item Cargo.lock -ErrorAction SilentlyContinue

# Purge problematic caches
Remove-Item -Recurse -Force $env:USERPROFILE\.cargo\git\checkouts\bips-*
Remove-Item -Recurse -Force $env:USERPROFILE\.cargo\registry\index\github.com-*

# Update dependencies
cargo update -p bitcoin --precise 0.32.1
cargo update -p secp256k1 --precise 0.28.1
cargo update -p web5 --precise 0.5.1

# Verify workspace
cargo metadata --format-version=1 | ConvertFrom-Json | Select-Object -ExpandProperty workspace_members

# Rebuild with compliance features
cargo build --workspace --features "bip174 bip341 secp256k1/std" 

# Fix line length validation
find docs/ -name "*.md" | ForEach-Object {
    markdownlint $_ -f
}

# Update AI labels
cargo run --bin anya_audit -- update-labels --scope all

# Full system check
cargo run --bin anya_audit -- check --rules bip341,bip174,secp256k1 --fix 