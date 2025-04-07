# Anya Core Fix Scripts

This utility provides cross-platform tools to fix common issues in the Anya Core codebase, particularly related to Cargo.toml files. The tool is written in Rust and works on all platforms (Windows, macOS, Linux).

## Features

- **Fix edition inheritance**: Replaces `edition.workspace = true` with explicit `edition = "2021"`
- **Fix package references**: Updates references from `anya-core-core` to `anya-core-lib`
- **Remove conflicting workspace sections**: Removes duplicate `[workspace]` sections in subpackages

## Installation

1. Make sure you have Rust installed (see [rustup.rs](https://rustup.rs))
2. Build the utility:

```bash
cd rust-fix-scripts
cargo build --release
```

The executable will be available at `target/release/anya-fix-scripts`

## Usage

### Check for issues (without making changes)

```bash
# Check for edition inheritance issues
./anya-fix-scripts fix-edition

# Check for package reference issues
./anya-fix-scripts fix-package-refs

# Check for both types of issues
./anya-fix-scripts fix-all
```

### Apply fixes

```bash
# Fix edition inheritance issues
./anya-fix-scripts fix-edition --apply

# Fix package reference issues
./anya-fix-scripts fix-package-refs --apply

# Fix both types of issues (recommended)
./anya-fix-scripts fix-all --apply
```

## Windows Usage

On Windows, you can use the tool with PowerShell:

```powershell
.\anya-fix-scripts.exe fix-all --apply
```

## Benefits over shell scripts

- Works identically on all platforms
- No need for separate scripts for Windows and Unix-like systems
- Better error handling and reporting
- Safer regular expression handling
- Dry-run capability (check without applying changes)

## Notes

Always run with the check option first before applying fixes to see what changes will be made. 