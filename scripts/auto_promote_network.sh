#!/usr/bin/env bash
set -euo pipefail

# Auto-promotion script: runs tests and verification in testnet, then mainnet.
# If both pass, optionally deploy.
# Env:
#   DEPLOY_CMD       optional command to run on successful mainnet validation
#   ALLOW_SIMULATION set to 0 to strictly forbid simulation in testnet/mainnet (default 0)

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

run_phase() {
  local NET="$1"
  echo "===== PHASE: $NET ====="
  export ANYA_NETWORK_TYPE="$NET"
  export RUST_LOG=info
  # Strict: no simulation in testnet/mainnet
  export ALLOW_SIMULATION=${ALLOW_SIMULATION:-0}
  echo "Building with --all-features for $NET..."
  cargo check --all-features
  echo "Running unit tests (ignored skipped by default)..."
  cargo test --all-features -- --nocapture
  echo "Running verification script..."
  ./scripts/verify_implementation_status.sh
}

# 1) Testnet phase
run_phase testnet

# 2) Mainnet phase
run_phase mainnet

echo "✅ Both testnet and mainnet validations passed."
if [ -n "${DEPLOY_CMD:-}" ]; then
  echo "Deploying via DEPLOY_CMD..."
  bash -lc "$DEPLOY_CMD"
  echo "✅ Deployment command finished"
else
  echo "No DEPLOY_CMD set; skipping deployment."
fi
