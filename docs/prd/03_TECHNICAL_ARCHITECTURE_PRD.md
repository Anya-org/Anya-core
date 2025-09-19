---
title: Technical Architecture PRD
description: Architecture components and integration plan (updated with native dependency migration strategy)
category: prd
tags: architecture-bitcoin-layer2
last_updated: 2025-08-10
compliance: AIR-3 AIS-3 BPC-3 RES-3
---
---

# Technical Architecture PRD

Date: August 8, 2025
Version: 2.0.1

## Overview

- Runtime: tokio async; error: anyhow/thiserror; serde for config/state.
- Self-node primary: NetworkState { is_primary: bool } + accessors prefers_self_as_master(), is_primary_node().
- Health model: Healthy if primary self-node; Warning when peers below threshold.
- Storage Architecture: `StorageRouter` mediates between PersistentStorage and DecentralizedStorage (feature-gated `dwn`). Autoconfig builds IPFS + Web5 adapter + optional Bitcoin anchoring network then selects backend via ANYA_STORAGE_BACKEND.

## Bitcoin Core RPC Integration

- Adapter: src/bitcoin/adapters/rpc/mod.rs (existing)
- Usage: When BTC_RPC_URL (+ auth) is set, use RPC for:
  - getblockchaininfo → height/besthash
  - estimatesmartfee → feerate mapping
- Fallback: If RPC unavailable, retain previous simulated estimations.

## Layer2 Production Adapters (Phase-in)

- Lightning: choose LDK (native) or LND gRPC; feature flag: feature_ln_prod
- RGB: rgb-node client crate; feature flag: feature_rgb_prod
- DLC: oracle client minimal; feature flag: feature_dlc_prod

## Config Surface

- TOML + env override:
  - prefer_self_as_master=true (default)
  - enable_self_node_fallback=true (default)
  - btc.rpc_url, btc.rpc_user, btc.rpc_pass (or cookie)
  - min_peers, bootstrap_peers[]

Environment mappings (examples):

- ANYA_LAYER2_PREFER_SELF_AS_MASTER=true|false
- ANYA_LAYER2_ENABLE_SELF_NODE_FALLBACK=true|false
- ANYA_BITCOIN_RPC_URL, ANYA_BITCOIN_RPC_USER, ANYA_BITCOIN_RPC_PASS
- ANYA_STORAGE_BACKEND=auto|dwn|persistent
- ANYA_IPFS_ENDPOINT=<http://127.0.0.1:5001>
- ANYA_WEB5_SERVICE_URL=<http://localhost:8080>
- ANYA_DID=did:... (optional; generated if absent)
- ANYA_BITCOIN_NETWORK=regtest|testnet|mainnet (anchoring)

## Observability

- tracing spans for connect/sync/health; metrics for height/peers/fees.

## HSM Integration Notes

- Dev simulator is locked-by-default; tests must initialize and unlock via Custom("unlock") with pin "1234".
- Per-test timeouts are enforced to avoid hangs under async runtime.
- Factory fallback order: primary -> software -> simulator (debug + dev-sim feature only).

## Environment-Honest Testing

- Tests for ML/system integrations check for required CLIs (`anya-ml`, `anya-cli`, `web5`) and models; if missing, they log a skip and exit OK.
- Network-bound checks use public testnet defaults when config is absent and warn if unreachable.

## CLI & Developer Ergonomics

- Default run target: `anya-core` (cargo run maps to core binary).
- Aliases provided in `.cargo/config.toml`:
  - `cargo core-health`, `cargo core-validate`, `cargo core-start`

Branch: integration/endpoint-centralization-clean • Last Updated: August 9, 2025 (storage autoconfig incorporated)

Last Updated: August 10, 2025

## Native Dependency Migration Strategy (Added Aug 10, 2025)

Objective: Reduce build complexity, security exposure, and toolchain friction by phasing out high-risk C/C++ system dependencies in favor of pure Rust crates.

Targets & Rationale:
- OpenSSL (via reqwest default TLS) → rustls (DONE: quick-fix script prepared)
- zstd-sys (C wrapper) → pure Rust zstd crate with `pure_rust` (Phase 1 follow-up)
- RocksDB (librocksdb-sys) → Evaluate `redb` vs `sled` (Phase 2 experiment)
- libgit2-sys (if introduced) → `gix` (git-oxide) (Phase 3 if Git integration expands)
- secp256k1-sys → retain (audited domain-specific; revisit only if parity alternative emerges)

Feature Flag Placeholders (Cargo.toml): `kv-redb`, `kv-sled`, `git-gix`.

Integration Pattern:
1. Introduce trait abstraction (StorageEngine) wrapping current RocksDB calls.
2. Prototype alternative engines behind feature flags + runtime selector (ANYA_STORAGE_BACKEND).
3. Dual-write shadow mode with consistency assertions before cutover.
4. Benchmark dimensions: init latency, write throughput, p95 get latency, compaction overhead, binary size delta.

Security & Compliance:
- Pure Rust reduces FFI/unsafe surface (geiger delta tracked per PR).
- Removes OpenSSL CVE churn from critical path (rustls memory-safe).
- Supply chain simplification for SBOM & license scanning.

Instrumentation:
- tracing spans: storage.init, storage.put, storage.get, storage.batch.
- metrics: storage_engine_active{engine}, storage_op_latency_seconds_bucket.

Rollback:
- Single feature flag toggle; retain RocksDB state untouched until migration complete.
- Export/import tool with checksums for irreversible engine transitions.

Next Actions:
1. Add StorageEngine trait + RocksDbEngine wrapper.
2. Add redb prototype behind `kv-redb`.
3. Criterion benchmarks harness.
4. Integrate geiger report into quality gate.
