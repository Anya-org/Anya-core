# Build Requirements for Anya Core

This document outlines the requirements for building Anya Core on different operating systems.

## Common Requirements

- Rust 1.70.0 or later (use [rustup](https://rustup.rs/) for installation)
- Git for cloning the repository

## Windows

### Required Tools

- Visual Studio 2019 or 2022 with "Desktop development with C++" workload
  - Or install Build Tools for Visual Studio with the "C++ build tools" option
- Windows 10 or 11 (64-bit)

### Installation Steps

1. Install Rust using [rustup-init.exe](https://rustup.rs/)
2. Install [Visual Studio 2022 Community](https://visualstudio.microsoft.com/vs/community/) with "Desktop development with C++" workload
3. Open a new terminal (restart after installation)
4. Run `rustup default stable`
5. Run `rustup target add x86_64-pc-windows-msvc`

### Building

```powershell
# Fix Cargo.toml files if needed
.\fix_cargo_edition.ps1
.\fix_core_package_refs.ps1

# Build the project
cargo build --release
```

## macOS

### Required Tools

- Xcode Command Line Tools
- macOS 10.15 Catalina or newer

### Installation Steps

1. Install Xcode Command Line Tools:
   ```bash
   xcode-select --install
   ```
2. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Add Rust to your PATH:
   ```bash
   source $HOME/.cargo/env
   ```

### Building

```bash
# Fix Cargo.toml files if needed
chmod +x fix_cargo_edition.sh fix_core_package_refs.sh
./fix_cargo_edition.sh
./fix_core_package_refs.sh

# Build the project
cargo build --release
```

## Linux (Ubuntu/Debian)

### Required Tools

- Basic development tools (gcc, etc.)
- libssl-dev for cryptographic operations

### Installation Steps

1. Install required packages:
   ```bash
   sudo apt update
   sudo apt install build-essential pkg-config libssl-dev
   ```
2. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Add Rust to your PATH:
   ```bash
   source $HOME/.cargo/env
   ```

### Building

```bash
# Fix Cargo.toml files if needed
chmod +x fix_cargo_edition.sh fix_core_package_refs.sh
./fix_cargo_edition.sh
./fix_core_package_refs.sh

# Build the project
cargo build --release
```

## Linux (Fedora/RHEL/CentOS)

### Required Tools

- Basic development tools (gcc, etc.)
- openssl-devel for cryptographic operations

### Installation Steps

1. Install required packages:
   ```bash
   sudo dnf install gcc make openssl-devel
   ```
2. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Add Rust to your PATH:
   ```bash
   source $HOME/.cargo/env
   ```

### Building

Same as Ubuntu/Debian build steps.

## Troubleshooting

### Common Issues

1. **Cargo.toml inheritance errors**: Run the provided fix scripts for your platform.

2. **Linker not found (Windows)**: Ensure Visual Studio with C++ tools is installed properly.

3. **Dependency errors**: Some dependencies might have incorrect feature flags. Run the fix scripts and check the error output for specific dependency issues.

4. **SSL/TLS libraries not found**: Ensure you have the appropriate development packages installed for your platform.

### Getting Help

If you encounter any build issues, please:

1. Check the error message carefully
2. Ensure you have all required dependencies installed
3. Try running the appropriate fix scripts
4. Report issues with full error output to the project issue tracker 