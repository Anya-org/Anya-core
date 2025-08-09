---
title: Technical Architecture PRD
description: Architecture components and integration plan
category: prd
tags: architecture-bitcoin-layer2
last_updated: 2025-08-09
compliance: AIR-3 AIS-3 BPC-3 RES-3
---
---

# Technical Architecture PRD

Date: August 8, 2025
Version: 2.0.0

## Overview

- Runtime: tokio async; error: anyhow/thiserror; serde for config/state.
- Self-node primary: NetworkState { is_primary: bool } + accessors prefers_self_as_master(), is_primary_node().
- Health model: Healthy if primary self-node; Warning when peers below threshold.

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

Branch: integration/endpoint-centralization-clean • Last Updated: August 9, 2025

Last Updated: August 9, 2025
