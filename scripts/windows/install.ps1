# Anya Core Windows Installer

# OS detection
 = "windows"

# System checks
 = Get-CimInstance -ClassName Win32_ComputerSystem
 = [math]::Round(.TotalPhysicalMemory / 1GB)
 = .NumberOfLogicalProcessors

# Configure based on system resources
if ( -ge 8) {
   = "--features=full-stack"
} else {
   = "--features=minimal"
}

# Build
cargo build --release 

# Install configuration
 = Join-Path :USERPROFILE ".anya\config"
New-Item -ItemType Directory -Force -Path 
Copy-Item -Path ".\config\default.yaml" -Destination 
