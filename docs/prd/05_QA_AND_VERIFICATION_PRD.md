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

Last Updated: August 8, 2025
