# Python Setup for Anya-Core [AIS-3][BPC-3]

This guide provides instructions for setting up Python for Anya-Core development across different platforms.

## Windows Setup

### Installation

1. **Download Python 3.11+** from [python.org](https://www.python.org/downloads/)
   - Ensure you check "Add Python to PATH" during installation

2. **Verify installation**:
   ```powershell
   python --version
   # Should show Python 3.11.x or higher
   ```

3. **Configure Python Path**:
   If you receive an error message like:
   ```
   Python was not found; run without arguments to install from the Microsoft Store, or disable this shortcut from Settings > Apps > Advanced app settings > App execution aliases.
   ```
   
   Use one of these solutions:
   
   a. **Disable the Microsoft Store Python alias**:
      - Open Windows Settings
      - Go to Apps > Apps & features > App execution aliases
      - Turn off the Python aliases
   
   b. **Use full path to Python**:
      ```powershell
      # Find your Python installation
      $pythonPath = Get-Command python | Select-Object -ExpandProperty Source
      # If the above fails, try the following common locations:
      # C:\Users\{username}\AppData\Local\Programs\Python\Python311\python.exe
      # C:\Program Files\Python311\python.exe
      
      # Use the full path when running scripts
      & "$pythonPath" .\scripts\fix_path_handling.py .\src
      ```
   
   c. **Add Python to your PATH permanently**:
      ```powershell
      # Run PowerShell as Administrator
      $pythonDir = Split-Path -Parent (Get-Command python | Select-Object -ExpandProperty Source)
      [Environment]::SetEnvironmentVariable("Path", $env:Path + ";$pythonDir", [EnvironmentVariableTarget]::Machine)
      ```

## macOS Setup

### Installation

1. **Install using Homebrew** (recommended):
   ```bash
   brew install python@3.11
   ```

2. **Or download from Python.org**:
   - Download from [python.org](https://www.python.org/downloads/macos/)
   - Run the installer package

3. **Verify installation**:
   ```bash
   python3 --version
   # Should show Python 3.11.x or higher
   ```

4. **Set up alias** (optional):
   ```bash
   # Add to your ~/.zshrc or ~/.bash_profile
   alias python=python3
   ```

## Linux Setup

### Installation

#### Ubuntu/Debian:
```bash
sudo apt update
sudo apt install python3.11 python3-pip
```

#### Fedora:
```bash
sudo dnf install python3.11 python3-pip
```

#### Arch Linux:
```bash
sudo pacman -S python python-pip
```

### Verify installation:
```bash
python3 --version
# Should show Python 3.11.x or higher
```

## Required Python Packages

The following packages are required for Anya-Core development:

```bash
# Install using pip (Windows)
pip install pyyaml toml requests cryptography

# Install using pip (macOS/Linux)
pip3 install pyyaml toml requests cryptography
```

Or use the requirements file:

```bash
# Windows
pip install -r requirements.txt

# macOS/Linux
pip3 install -r requirements.txt
```

## Troubleshooting

### Multiple Python Versions

If you have multiple Python versions installed, be specific about which one to use:

#### Windows:
```powershell
py -3.11 .\scripts\fix_path_handling.py .\src
```

#### macOS/Linux:
```bash
python3.11 ./scripts/fix_path_handling.py ./src
```

### Virtual Environments

For a clean development environment, consider using virtual environments:

#### Windows:
```powershell
python -m venv venv
.\venv\Scripts\Activate.ps1
pip install -r requirements.txt
```

#### macOS/Linux:
```bash
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

### Python Not Found in PATH

#### Windows:
The Python installer on Windows sometimes fails to add Python to the PATH environment variable correctly. If `python` command is not found, try:

```powershell
# Locate Python installation directory
$pythonDir = "$env:LOCALAPPDATA\Programs\Python\Python311"
if (Test-Path $pythonDir) {
    $env:Path += ";$pythonDir;$pythonDir\Scripts"
} else {
    # Try other common locations
    $pythonDir = "C:\Program Files\Python311"
    if (Test-Path $pythonDir) {
        $env:Path += ";$pythonDir;$pythonDir\Scripts"
    }
}
```

#### macOS:
```bash
export PATH=$PATH:/usr/local/bin:/Library/Frameworks/Python.framework/Versions/3.11/bin
```

#### Linux:
```bash
export PATH=$PATH:/usr/bin:/usr/local/bin
``` 