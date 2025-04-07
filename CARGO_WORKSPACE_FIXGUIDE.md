# Cargo Workspace Edition Inheritance Fix Guide

The error about inheriting `edition` from workspace root manifest's `workspace.package.edition` appears to be occurring because:

1. You're using `edition.workspace = true` in your package definitions
2. Your root Cargo.toml correctly defines `[workspace.package] edition = "2021"`, but something is preventing it from being inherited properly

## Solution Options

### Option 1: Explicitly define edition in each crate (Recommended)

Replace `edition.workspace = true` with `edition = "2021"` in each crate's Cargo.toml:

```toml
[package]
name = "your-crate-name"
version.workspace = true
edition = "2021"  # Explicitly set instead of using inheritance
authors.workspace = true
# ...other fields...
```

This is the most reliable approach and has been applied to the `core/Cargo.toml` file.

### Option 2: Fix workspace inheritance in the root Cargo.toml

There may be an issue with your workspace configuration. Check:

1. Ensure the root Cargo.toml has properly formatted workspace.package section:

```toml
[workspace.package]
version = "2.5.0"
edition = "2021"
authors = ["Anya Core Team"]
# ...other fields...
```

2. Make sure you're using a Rust version that supports workspace inheritance (1.64.0+)

3. Check that your Cargo.toml files don't have conflicting workspace definitions

### Files to Fix

Based on the examination of your project, these files need to be updated:

- ✅ `core/Cargo.toml` (already fixed)
- `packages/core/Cargo.toml`
- `packages/bitcoin-network/Cargo.toml`
- `packages/metrics/Cargo.toml`
- `packages/protocol-adapters/Cargo.toml`
- `packages/mcp-interface/Cargo.toml`
- `packages/bin/Cargo.toml`
- `packages/privacy/Cargo.toml`
- `packages/test_bip353/Cargo.toml`

## PowerShell Script to Apply Fix

You can use this PowerShell script to apply the fix to all Cargo.toml files:

```powershell
$cargoFiles = Get-ChildItem -Path "." -Filter "Cargo.toml" -Recurse
foreach ($file in $cargoFiles) {
    $content = Get-Content $file.FullName -Raw
    $newContent = $content -replace "edition\.workspace = true", "edition = `"2021`""
    Set-Content -Path $file.FullName -Value $newContent
    Write-Host "Updated $($file.FullName)"
}
```

## Verification

After applying the fix, run:

```bash
cargo check
```

This should resolve the edition inheritance error. 