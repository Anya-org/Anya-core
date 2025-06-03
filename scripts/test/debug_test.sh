#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
set -euo pipefail

# ==== SELF-DIAGNOSTICS FUNCTIONS ====
self_diagnose() {
    echo "=== TEST SCRIPT SELF-DIAGNOSTICS ==="
    echo "Date and time: $(date)"
    echo "Script path: $0"
    echo "Working directory: $(pwd)"
    echo "User: $(whoami)"
    echo "Environment variables:"
    env | grep -E 'ANYA|RUST|CARGO|PATH' | sort

    echo -e "\nSystem resources:"
    echo "- CPU: $(nproc) cores"
    echo "- Memory: $(free -h | awk '/^Mem:/ {print $2}') total / $(free -h | awk '/^Mem:/ {print $7}') available"
    echo "- Disk: $(df -h . | awk 'NR==2 {print $4}') available"

    echo -e "\nCompiled features:"
    local core_binary
    for bin_path in "${PROJECT_ROOT}/target/release/anya-core" "$(which anya-core 2>/dev/null)" "/usr/local/bin/anya-core"; do
        if [ -x "$bin_path" ]; then
            core_binary="$bin_path"
            break
        fi
    done

    if [ -n "$core_binary" ]; then
        echo "Using binary: $core_binary"
        "$core_binary" --version || echo "Failed to get version"
        "$core_binary" --show-features 2>/dev/null || echo "Feature info not available"
    else
        echo "Could not find anya-core binary"
    fi

    echo -e "\nDetecting hardware security capabilities:"
    if [ -e "/dev/tpm0" ] || [ -e "/dev/tpmrm0" ]; then
        echo "- TPM: Available"
    else
        echo "- TPM: Not detected"
    fi

    if lsusb 2>/dev/null | grep -qi "yubikey"; then
        echo "- YubiKey: Connected"
    else
        echo "- YubiKey: Not detected"
    fi

    if lsusb 2>/dev/null | grep -qi "ledger"; then
        echo "- Ledger: Connected"
    else
        echo "- Ledger: Not detected"
    fi

    local pkcs11_lib=false
    for lib in /usr/lib*/libykcs11.so* /usr/lib*/libtpm2_pkcs11.so* /usr/local/lib*/libykcs11.so*; do
        if [ -f "$lib" ]; then
            echo "- PKCS#11: Available ($lib)"
            pkcs11_lib=true
            break
        fi
    done
    if [ "$pkcs11_lib" = false ]; then
        echo "- PKCS#11: No libraries detected"
    fi

    echo -e "\nFeature flag detection:"
    # Detect hardware and recommend feature flags
    local recommended_flags="std"
    
    if [ -e "/dev/tpm0" ] || [ -e "/dev/tpmrm0" ] || \
       lsusb 2>/dev/null | grep -qi "yubikey" || \
       lsusb 2>/dev/null | grep -qi "ledger" || \
       ls /usr/lib*/libykcs11.so* &>/dev/null; then
        recommended_flags="${recommended_flags},hsm"
        echo "- Hardware security detected: recommend including 'hsm' feature"
    else
        echo "- No hardware security detected: software HSM will be used if 'hsm' feature enabled"
    fi
    
    echo "- Recommended feature flags: ${recommended_flags},bitcoin_integration"

    # Check if the Cargo.toml has the expected features
    if [ -f "${PROJECT_ROOT}/Cargo.toml" ]; then
        echo -e "\nCargo feature flags defined in Cargo.toml:"
        grep -A 20 "\[features\]" "${PROJECT_ROOT}/Cargo.toml" | grep -v "^#" || echo "No features section found"
    fi

    echo -e "\nService status:"
    systemctl status anya-core.service 2>/dev/null || echo "Service not available"

    echo -e "\nNetwork configuration:"
    # Check if API is responding
    echo -n "- API health endpoint: "
    curl -s -o /dev/null -w "%{http_code}" http://localhost:3300/health 2>/dev/null || echo "not responding"
    
    echo -e "\n=== END SELF-DIAGNOSTICS ===\n"
}

# Main script logic
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEST_TYPE=${1:-basic}
FEATURE_FLAGS=${2:-std}

echo "Running Anya Core test script in DEBUG MODE (type: $TEST_TYPE, features: $FEATURE_FLAGS)"
echo "This is a standalone version for debugging purposes"

# Always run self-diagnostics in debug mode
self_diagnose

# Check installation scripts for feature flags handling
echo -e "\n=== SCRIPT FEATURE FLAGS ANALYSIS ==="
echo "Checking auto_install.sh for feature flags handling:"
grep -n "FEATURE_FLAGS" "${PROJECT_ROOT}/scripts/install/auto_install.sh" | head -10

echo -e "\nChecking linux_install.sh for feature flags handling:"
grep -n "FEATURE_FLAGS\|features=" "${PROJECT_ROOT}/scripts/install/linux_install.sh" | head -10

# Check if the anya-core binary supports the --show-features option
if [ -x "${PROJECT_ROOT}/target/release/anya-core" ]; then
    echo -e "\n=== BINARY FEATURE FLAGS TEST ==="
    echo "Testing if anya-core binary supports --show-features:"
    "${PROJECT_ROOT}/target/release/anya-core" --help | grep -i "feature" || echo "No feature flag help found"
    
    # Try to display features anyway
    echo "Attempting to show compiled features:"
    "${PROJECT_ROOT}/target/release/anya-core" --show-features 2>&1 || echo "Feature display not supported"
fi

echo -e "\n=== DEBUG TEST COMPLETED ==="
echo "You can now run the actual tests with:"
echo "  sudo ./scripts/install/auto_install.sh --run-tests --test-type=$TEST_TYPE --features=$FEATURE_FLAGS"
