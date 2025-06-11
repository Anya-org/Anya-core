#!/bin/sh
set -euo pipefail

# Verify Bitcoin Core metrics endpoint
if [ -n "${BITCOIN_METRICS_URL:-}" ]; then
    echo "[INFO] Verifying Bitcoin Core metrics endpoint..."
    if curl -sf --connect-timeout 10 "${BITCOIN_METRICS_URL}" >/dev/null 2>&1; then
        echo "[INFO] Bitcoin metrics endpoint is accessible"
    else
        echo "[ERROR] Bitcoin metrics endpoint check failed"
        exit 1
    fi
else
    echo "[WARN] BITCOIN_METRICS_URL not set; skipping metrics verification"
fi
