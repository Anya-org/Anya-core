#!/bin/bash
# Anya Core Unix Setup Script [AIS-3][BPC-3]
echo "Setting up Anya Core for Unix-like systems..."

# Detect OS
if [[ "$OSTYPE" == "darwin"* ]]; then
  PLATFORM="macos"
else
  PLATFORM="linux"
fi

echo "Detected platform: $PLATFORM"

# Create platform configuration directories
mkdir -p config/platform

# Create Unix configuration file if it doesn't exist
if [ ! -f config/platform/unix.yaml ]; then
    echo "Creating Unix platform configuration..."
    cat > config/platform/unix.yaml << EOT
paths:
  base: "\$HOME/.anya"
  data: "\$HOME/.anya/data"
  logs: "\$HOME/.anya/logs"
  bitcoin: "\$HOME/.bitcoin"

security:
  keystore: "\$HOME/.anya/keystore"
  permissions: "0600"

network:
  default_interface: "auto"
  timeout_ms: 3000
EOT
fi

# Create Windows configuration file if it doesn't exist (for cross-platform compatibility)
if [ ! -f config/platform/windows.yaml ]; then
    echo "Creating Windows platform configuration for cross-platform compatibility..."
    cat > config/platform/windows.yaml << EOT
paths:
  base: "%USERPROFILE%\\.anya"
  data: "%USERPROFILE%\\.anya\\data"
  logs: "%USERPROFILE%\\.anya\\logs"
  bitcoin: "%APPDATA%\\Bitcoin"

security:
  keystore: "%USERPROFILE%\\.anya\\keystore"
  permissions: "0600"

network:
  default_interface: "auto"
  timeout_ms: 3000
EOT
fi

# Fix path handling in Rust files
echo "Fixing path handling in Rust files..."
python3 ./scripts/fix_path_handling.py ./src

# Create Anya configuration directories
echo "Creating Anya configuration directories..."
mkdir -p ~/.anya
mkdir -p ~/.anya/config
mkdir -p ~/.anya/data
mkdir -p ~/.anya/logs
mkdir -p ~/.anya/keystore

# Set appropriate permissions
chmod 700 ~/.anya
chmod 700 ~/.anya/keystore

# Copy default configuration
echo "Copying default configuration..."
cp config/default.yaml ~/.anya/config/

echo "Setup complete!"
echo "You can now build Anya Core with: cargo build --release" 