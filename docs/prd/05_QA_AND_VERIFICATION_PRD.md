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

## Quality Gates (Strict Enforcement)

All gates MUST pass on every merge into `main` (no soft fails). Any temporary exception requires an inline justification comment and MUST be resolved in the next merge window.

| Gate | Requirement | Command | Failure Policy |
|------|-------------|---------|----------------|
| Formatting | No diff vs rustfmt | `cargo fmt -- --check` | Hard fail |
| Build | PASS (no errors) | `cargo build --all --all-features` | Hard fail |
| Lint | Zero warnings (clippy -D warnings) | `cargo clippy --all-targets --all-features -- -D warnings` | Hard fail |
| Tests | 100% pass, skips explicitly reported | `cargo test --all --all-features --no-fail-fast` | Hard fail (except approved skips) |
| Test Skip Accounting | All skipped / ignored tests enumerated & justified | `cargo test -- --list` + parse | Hard fail if unaccounted |
| Security: Advisories | Zero vulnerabilities | `cargo audit -q` | Hard fail |
| Security: Licenses/Bans | All deny rules pass | `cargo deny check` | Hard fail |
| Dependency Drift | No duplicate crate major versions (critical libs) | `cargo tree -d` | Hard fail if duplicates in security surface |
| Docs Sync | PRDs & README updated when behavior changes | Manual diff + `git diff --name-only` | Hard fail if drift detected |
| Scripts | `quality_gate.sh` & `verify_implementation_status.sh` SUCCESS | `./quality_gate.sh --ci` | Hard fail |
| Unimplemented | Zero `unimplemented!()` macros | grep scan | Hard fail |
| TODO Stubs | Tracked & shrinking trend | grep scan | Warn (fail when <=5 remain target) |
| Warnings | Zero (build + clippy) | included in clippy gate | Hard fail |
| PRD Drift Check | Hash of PRD set stable for unchanged code | `scripts/verify_implementation_status.sh --json` | Hard fail if mismatch |

Metrics captured per CI run MUST be appended (or referenced) in the merge commit message: unimplemented_count, todo_count, warnings, test_total, test_passed, test_failed, test_skipped, audit_vulns, deny_failures.

## Canonical Verification Command Set

```bash
cargo fmt -- --check
cargo build --all --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --all-features --no-fail-fast
cargo deny check
cargo audit -q
cargo tree -d
./quality_gate.sh --ci
scripts/verify_implementation_status.sh --auto-run --json
```

## Acceptance Tests

- No-config boot → node primary; health OK
- With BTC RPC env → height/hash/fees present from RPC
- With bootstrap peers → discovery connects; primary preference logged

## Environment-Honest Test Execution & Skip Policy

Skips are ONLY permitted under controlled, enumerated conditions and must surface as structured metrics. All skips must print a line beginning with `[skip-metric]` in the form:

`[skip-metric] <category>=<id> reason="<concise explanation>" condition="<detected condition>"`

Approved skip categories:

1. network-env: External network or Bitcoin RPC endpoint unavailable (connection refused / DNS failure)
2. resource-insufficient: Host resources below required thresholds (CPU / RAM / disk) for ML multi-VM tests
3. hsm-hardware-missing: Hardware HSM not present; software simulator used instead
4. feature-flag-disabled: Test guarded by feature flag not enabled in current build

Behavior:
• Network-bound tests: If endpoints unreachable → emit skip metric, not silent ignore.
• ML multi-VM tests: Use resource preflight (see Appendix script) and skip with metric if below thresholds.
• HSM tests: If real hardware unavailable, fallback to simulator tests must still run; only hardware‑specific cases may skip.
• Any unclassified skip = hard fail.

CI Aggregation:
Parsing script will count lines and ensure every `ignored` test reported by `cargo test -- --list` has corresponding skip metric or is explicitly feature-gated.

## Known Local Limitations

- If DNS or required Bitcoin ports are closed locally, `tests/network_validation.rs` will fail; treat as advisory in developer environments.
- When running inside an async context, avoid nested Tokio runtimes; prefer async tests or refactor blocking calls.

Last Updated: August 10, 2025 (Strict gating & skip policy revision)

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
