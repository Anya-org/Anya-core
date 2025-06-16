#!/bin/bash
# Script to verify the dev container setup

echo "=== Verifying Dev Container Setup ==="

# Check Git version
echo -e "\n=== Git Version ==="
git --version

# Check Rust version
echo -e "\n=== Rust Version ==="
rustc --version
cargo --version

# Check installed Rust targets
echo -e "\n=== Installed Rust Targets ==="
rustup target list --installed

# Check installed Rust components
echo -e "\n=== Installed Rust Components ==="
rustup component list --installed

# Check installed Cargo tools
echo -e "\n=== Installed Cargo Tools ==="
echo "cargo-audit: $(cargo audit --version 2>/dev/null || echo 'Not installed')"
echo "cargo-update: $(cargo install-update --version 2>/dev/null || echo 'Not installed')"
echo "cargo-outdated: $(cargo outdated --version 2>/dev/null || echo 'Not installed')"
echo "cargo-edit: $(cargo add --version 2>/dev/null || echo 'Not installed')"
echo "cargo-tarpaulin: $(cargo tarpaulin --version 2>/dev/null || echo 'Not installed')"
echo "cargo-insta: $(cargo insta --version 2>/dev/null || echo 'Not installed')"
echo "cargo-criterion: $(cargo criterion --version 2>/dev/null || echo 'Not installed')"
echo "cargo-binstall: $(cargo binstall --version 2>/dev/null || echo 'Not installed')"
echo "cargo-deny: $(cargo deny --version 2>/dev/null || echo 'Not installed')"
echo "cargo-expand: $(cargo expand --version 2>/dev/null || echo 'Not installed')"
echo "cargo-llvm-cov: $(cargo llvm-cov --version 2>/dev/null || echo 'Not installed')"
echo "cargo-make: $(cargo make --version 2>/dev/null || echo 'Not installed')"
echo "cargo-nextest: $(cargo nextest --version 2>/dev/null || echo 'Not installed')"
echo "cargo-watch: $(cargo watch --version 2>/dev/null || echo 'Not installed')"
echo "cargo-web: $(cargo web --version 2>/dev/null || echo 'Not installed')"
echo "wasm-pack: $(wasm-pack --version 2>/dev/null || echo 'Not installed')"
echo "cross: $(cross --version 2>/dev/null || echo 'Not installed')"
echo "sccache: $(sccache --version 2>/dev/null || echo 'Not installed')"

# Check Node.js version (for Bitcoin development tools)
echo -e "\n=== Node.js Version ==="
node --version 2>/dev/null || echo "Node.js not installed"
npm --version 2>/dev/null || echo "NPM not installed"

# Check Python version (for ML components)
echo -e "\n=== Python Version ==="
python --version 2>/dev/null || echo "Python not installed"
pip --version 2>/dev/null || echo "Pip not installed"

# Check Python ML libraries
echo -e "\n=== Python ML Libraries ==="
echo "NumPy: $(python -c 'import numpy; print(numpy.__version__)' 2>/dev/null || echo 'Not installed')"
echo "Pandas: $(python -c 'import pandas; print(pandas.__version__)' 2>/dev/null || echo 'Not installed')"
echo "TensorFlow: $(python -c 'import tensorflow; print(tensorflow.__version__)' 2>/dev/null || echo 'Not installed')"
echo "PyTorch: $(python -c 'import torch; print(torch.__version__)' 2>/dev/null || echo 'Not installed')"
echo "Scikit-learn: $(python -c 'import sklearn; print(sklearn.__version__)' 2>/dev/null || echo 'Not installed')"

# Check Docker version (for container-based testing)
echo -e "\n=== Docker Version ==="
docker --version 2>/dev/null || echo "Docker not installed"
docker compose version 2>/dev/null || echo "Docker compose not installed"

# Check Bitcoin development dependencies
echo -e "\n=== Bitcoin Development Dependencies ==="
for pkg in automake libevent-dev libboost-dev libboost-system-dev libboost-filesystem-dev libboost-test-dev; do
    if dpkg -s $pkg &>/dev/null; then
        echo "$pkg: Installed"
    else
        echo "$pkg: Not installed"
    fi
done

# Print success message
echo -e "\n=== Setup Verification Complete ==="
echo "If any tools are missing, run '.devcontainer/install-tools.sh' or rebuild the container"
