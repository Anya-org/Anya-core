#!/bin/bash
# Initialize and verify Anya Core dev container

# Check if we're in a dev container
if [ ! -f /.dockerenv ]; then
    echo "This script should be run inside the dev container."
    echo "Please reopen VS Code in the dev container first."
    exit 1
fi

echo "=== Initializing Anya Core Dev Container ==="

# 1. Make sure scripts are executable
chmod +x .devcontainer/*.sh

# 2. Install required tools
echo "Installing development tools..."
./.devcontainer/install-tools.sh

# 3. Verify the installation
echo "Verifying the setup..."
./.devcontainer/verify-setup.sh

# 4. Build the project in debug mode
echo "Building the project..."
cargo build

# 5. Run tests to verify everything works
echo "Running tests..."
cargo test

# 6. Show success message
echo "=== Dev Container Setup Complete ==="
echo ""
echo "Your development environment is ready!"
echo ""
echo "Use VS Code tasks to:"
echo "  - Build Debug (default build task)"
echo "  - Build Release"
echo "  - Run"
echo "  - Test (default test task)"
echo "  - Check and Clippy"
echo "  - Coverage"
echo "  - Generate Documentation"
echo ""
echo "For more information, see .devcontainer/README.md"
