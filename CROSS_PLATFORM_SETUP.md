# Anya-Core Cross-Platform Setup Guide [AIS-3][BPC-3]

This guide provides step-by-step instructions for setting up the Anya-Core development environment across Windows, macOS, and Linux platforms.

## Common Prerequisites

- Git
- Rust 1.70+ (via rustup)
- Bitcoin Core 24.0+

## Windows Setup

### System Requirements
- Windows 10/11 (64-bit)
- PowerShell 5.1+
- Visual Studio Build Tools 2019+ (with C++ build tools)

### Installation Steps

1. **Install Rust**:
   ```powershell
   # Download and run rustup-init.exe from https://rustup.rs
   # Or use winget
   winget install --id Rustlang.Rustup
   ```

2. **Install Visual Studio Build Tools**:
   ```powershell
   # Install Build Tools
   winget install Microsoft.VisualStudio.2022.BuildTools
   # Or download from: https://visualstudio.microsoft.com/downloads/
   ```

3. **Install Bitcoin Core**:
   ```powershell
   # Download from bitcoin.org and install
   # Or use chocolatey
   choco install bitcoin-core
   ```

4. **Clone Repository**:
   ```powershell
   git clone https://github.com/anya-org/anya-core.git
   cd anya-core
   ```

5. **Configure Build Environment**:
   ```powershell
   # Run the new Windows-specific setup script
   .\scripts\setup_windows.bat
   ```

6. **Build**:
   ```powershell
   cargo build --release
   ```

7. **Run Tests**:
   ```powershell
   cargo test
   ```

### Windows-Specific Notes

- **Path Length Limits**: Windows has a 260-character path limit. Enable long paths:
  ```powershell
  # Run as administrator
  Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1
  ```

- **Antivirus Exclusions**: Add the repository directory to Windows Defender exclusions to prevent build performance issues.

- **Environment Variables**: Ensure that Rust binaries are in your PATH:
  ```powershell
  $env:Path += ";$env:USERPROFILE\.cargo\bin"
  ```

## macOS Setup

### System Requirements
- macOS 11.0+ (Big Sur or newer)
- Xcode Command Line Tools
- Homebrew (recommended)

### Installation Steps

1. **Install Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```

2. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Install Dependencies via Homebrew**:
   ```bash
   # Install Homebrew if not already installed
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   
   # Install dependencies
   brew install openssl pkg-config bitcoin
   ```

4. **Clone Repository**:
   ```bash
   git clone https://github.com/anya-org/anya-core.git
   cd anya-core
   ```

5. **Configure Build Environment**:
   ```bash
   # Run the new Unix-specific setup script
   ./scripts/unix/setup.sh
   ```

6. **Build**:
   ```bash
   cargo build --release
   ```

7. **Run Tests**:
   ```bash
   cargo test
   ```

### macOS-Specific Notes

- **Apple Silicon (M1/M2)**: Ensure Rust is configured for the correct architecture:
  ```bash
  rustup target add aarch64-apple-darwin
  ```

- **OpenSSL Configuration**: If you encounter OpenSSL-related build issues:
  ```bash
  export OPENSSL_DIR=$(brew --prefix openssl)
  ```

- **File System Case-Sensitivity**: macOS file system is case-insensitive by default, which may cause issues. Be consistent with case in filenames.

## Linux Setup

### System Requirements
- Modern Linux distribution (Ubuntu 20.04+, Fedora 35+, etc.)
- GCC or Clang
- Required development libraries

### Installation Steps

1. **Install Dependencies**:
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install -y build-essential pkg-config libssl-dev clang git curl
   
   # Fedora
   sudo dnf install -y gcc gcc-c++ make openssl-devel pkg-config git curl
   ```

2. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Install Bitcoin Core**:
   ```bash
   # Ubuntu/Debian (using Bitcoin PPA)
   sudo add-apt-repository ppa:bitcoin/bitcoin
   sudo apt update
   sudo apt install -y bitcoind
   
   # Fedora
   sudo dnf install -y bitcoin-core
   ```

4. **Clone Repository**:
   ```bash
   git clone https://github.com/anya-org/anya-core.git
   cd anya-core
   ```

5. **Configure Build Environment**:
   ```bash
   # Run the new Unix-specific setup script
   ./scripts/unix/setup.sh
   ```

6. **Build**:
   ```bash
   cargo build --release
   ```

7. **Run Tests**:
   ```bash
   cargo test
   ```

### Linux-Specific Notes

- **File Permissions**: Ensure the user has appropriate permissions:
  ```bash
  # For Bitcoin Core data directory
  mkdir -p ~/.bitcoin
  chmod 700 ~/.bitcoin
  
  # For Anya Core data
  mkdir -p ~/.anya
  chmod 700 ~/.anya
  ```

- **SELinux Configuration**: If you're running SELinux (e.g., Fedora), you may need to configure policies:
  ```bash
  sudo semanage fcontext -a -t bin_t "/path/to/anya-core/target/release/anya(/.*)?"
  sudo restorecon -Rv /path/to/anya-core/target/release
  ```

## Cross-Platform Development Tips

### Using Docker

For consistent development environments across platforms:

```bash
# Build Docker image
docker build -t anya-core-dev .

# Run development container
docker run -it --rm -v "$(pwd):/code" anya-core-dev
```

### CI/CD Configuration

The repository includes GitHub Actions workflows for all platforms:
- `.github/workflows/linux.yml`
- `.github/workflows/macos.yml`
- `.github/workflows/windows.yml`

### Path Handling

When contributing code, remember to use platform-agnostic path handling:

```rust
// Good - works on all platforms
let config_path = std::path::Path::new(base_dir).join("config").join("settings.yaml");

// Bad - only works on Unix-like systems
let config_path = format!("{}/config/settings.yaml", base_dir);
```

### Platform Configuration

The project now includes platform-specific configuration files:

- `config/platform/windows.yaml` - Windows-specific paths and settings
- `config/platform/unix.yaml` - Unix-specific paths and settings (Linux/macOS)

These files are automatically created by the setup scripts, but you can modify them as needed.

### Environment Variables

Set environment variables according to your platform:

**Windows (PowerShell)**:
```powershell
$env:RUST_LOG="debug"
$env:ANYA_CONFIG_DIR="$env:APPDATA\Anya\config"
```

**macOS/Linux (Bash/Zsh)**:
```bash
export RUST_LOG="debug"
export ANYA_CONFIG_DIR="$HOME/.anya/config"
```

## Troubleshooting

### Common Issues

#### Windows
- **'cl.exe' not found**: Visual Studio Build Tools are not properly installed or not in PATH.
- **Long path errors**: Enable long paths as described above.
- **Permission issues**: Run PowerShell as Administrator or check file permissions.

#### macOS
- **OpenSSL errors**: Set OPENSSL_DIR as described above.
- **Xcode license**: Accept the Xcode license agreement by running `sudo xcodebuild -license accept`.
- **M1/M2 compatibility**: Ensure you're using the right architecture for dependencies.

#### Linux
- **Permission denied**: Check file permissions or try with sudo for system directories.
- **Missing libraries**: Install required development packages for your distribution.
- **SELinux denials**: Check audit logs with `sudo ausearch -m avc -ts recent`.

### Getting Help

- **Discord**: Join our community Discord server: https://discord.gg/anya-core
- **GitHub Issues**: Report bugs or ask questions on GitHub
- **Documentation**: Refer to the docs directory for more detailed documentation

## Appendix: Feature Flags

Control enabled features for your platform:

```bash
# Build with minimum features
cargo build --no-default-features

# Build with specific features
cargo build --features="taproot,silent-payments"

# Build with platform-specific features
cargo build --features="silent-payments,$(uname | tr '[:upper:]' '[:lower:]')-specific"
```

## Next Steps

Once you've set up your development environment, refer to:
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Architecture overview
- [BIP353_IMPLEMENTATION_GUIDE.md](docs/BIP353_IMPLEMENTATION_GUIDE.md) - Silent Payments guide 