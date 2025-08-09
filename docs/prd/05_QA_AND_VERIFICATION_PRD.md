---
title: QA & Verification PRD
description: Quality gates, verification strategy, and environment-honest testing
category: prd
tags: qa-testing-verification
last_updated: 2025-08-09
compliance: AIR-3 AIS-3 BPC-3 RES-3
---
---
---

# QA & Verification PRD

Date: August 8, 2025
Version: 2.0.0

## Quality Gates

- Build: cargo check --all-features
- Lint: warnings < 10
- Tests: unit + layer2 + adapter smoke tests
- Health: Self-node primary qualifies as Healthy

## Commands

```bash
cargo check --all-features
./scripts/quality_gate.sh
./scripts/verify_implementation_status.sh
```

## Acceptance Tests

- No-config boot → node primary; health OK
- With BTC RPC env → height/hash/fees present from RPC
- With bootstrap peers → discovery connects; primary preference logged

## Environment-Honest Test Execution

- Gate network-bound tests when required ports or internet are unavailable; report advisories instead of hard failures in local runs.
- ML multi-VM tests run only when the host has sufficient resources; otherwise, skip and print guidance.
- HSM simulator-based tests must initialize and unlock the simulator as needed and apply per-test timeouts.

## Known Local Limitations

- If DNS or required Bitcoin ports are closed locally, `tests/network_validation.rs` will fail; treat as advisory in developer environments.
- When running inside an async context, avoid nested Tokio runtimes; prefer async tests or refactor blocking calls.

Last Updated: August 9, 2025

## Appendix: Minimal resource check (bash)

```bash
CPUS=$(nproc || sysctl -n hw.ncpu || echo 1)
MEM_GB=$(awk '/MemTotal/ { printf "%.0f\n", $2/1024/1024 }' /proc/meminfo 2>/dev/null || echo 1)
DISK_GB=$(df -Pk . | awk 'NR==2 { printf "%.0f\n", $4/1024/1024 }' 2>/dev/null || echo 1)

REQ_CPUS=8
REQ_MEM=16
REQ_DISK=50

if [ "$CPUS" -lt "$REQ_CPUS" ] || [ "$MEM_GB" -lt "$REQ_MEM" ] || [ "$DISK_GB" -lt "$REQ_DISK" ]; then
	echo "[SKIP] ML multi-VM tests (need >=${REQ_CPUS} CPUs, >=${REQ_MEM}GB RAM, >=${REQ_DISK}GB free disk). Found: ${CPUS} CPU, ${MEM_GB}GB RAM, ${DISK_GB}GB disk."
	exit 0
fi
```
