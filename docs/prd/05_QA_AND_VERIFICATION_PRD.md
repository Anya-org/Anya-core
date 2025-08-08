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

Last Updated: August 8, 2025
