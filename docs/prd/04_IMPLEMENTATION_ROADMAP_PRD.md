# Implementation Roadmap PRD

Date: August 8, 2025
Version: 2.0.0

## Week 1-2: Minimal Productionization

- Wire Bitcoin Core RPC into state/fees (height/hash/feerates)
- Add feature flags for LN/RGB/DLC production adapters
- Provide env-driven config and defaults in example layer2.toml

## Week 3-4: LN + RGB MVP

- Implement minimal LN adapter operations (connect, invoice, pay)
- RGB: list assets, basic issue/transfer mock-through real client
- Expand health to include adapter reachability

## Weeks 5-8: DLC + Hardening

- DLC oracle client minimal
- Persistence for adapter state; retries/backoff
- CI: run verifications and warn when peers < min threshold without primary self-node

Success Criteria

- cargo check --all-features PASS
- Layer2 tests PASS + new adapter smoke tests PASS
- Warnings < 10
- Docs in docs/prd kept current

Last Updated: August 10, 2025

## Cross-Cutting: Native Dependency Migration (Added Aug 10, 2025)

Phase 0 (Complete / In Progress)
- Audit & migration guide committed (docs/audit/*)
- Quick-fix script scaffolds rustls + pure zstd path (pending execution)
- Placeholder feature flags added: kv-redb, kv-sled, git-gix

Phase 1
- Implement StorageEngine trait + RocksDbEngine wrapper (no behavior change)
- Capture baseline: build time, binary size, geiger unsafe/FFI metrics
- Apply rustls TLS switch; verify OpenSSL absence via ldd

Phase 2
- Prototype RedbEngine behind `kv-redb` (CRUD + iteration)
- Bench: rocksdb vs redb (ops/sec, p95 lat, size) with criterion
- Decision gate: adopt if perf within ±15% and complexity reduction meaningful

Phase 3
- (Optional) SledEngine if redb inconclusive
- Dual-write shadow mode + export/import + checksum verifier

Phase 4
- Git abstraction + gix exploration if git features expand

Success Metrics
- ≥15% build time reduction vs baseline
- Net decrease in unsafe/FFI lines (geiger)
- No data consistency diffs across 7 CI runs in shadow mode

Risks / Mitigations
- Perf regression → benchmark gates
- Data divergence → dual-write + checksum validation
- Complexity creep → minimal trait surface, progressive rollout
