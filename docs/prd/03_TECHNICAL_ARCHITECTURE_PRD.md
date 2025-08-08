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

## Observability

- tracing spans for connect/sync/health; metrics for height/peers/fees.

Last Updated: August 8, 2025
