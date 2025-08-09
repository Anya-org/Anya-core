---
title: "Configuration Module"
description: "Documentation for the Anya Core configuration module"
status: "active"
last_updated: "2025-08-06"
---

# Configuration Module

This module provides configuration management for the Anya Core system.

**Compliance**: AIR-3 (AI-Readable Level 3), AIS-3 (AI-Secure Level 3),
BPC-3 (Bitcoin-Protocol-Compliant Level 3), AIT-3 (AI-Testable Level 3)

## Table of Contents

- [Configuration Module](#configuration-module)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Feature Flags (Cargo Features)](#feature-flags-cargo-features)
    - [Production Feature Profile Guidance](#production-feature-profile-guidance)
    - [Environment Variable Overrides (Selected)](#environment-variable-overrides-selected)
    - [Risk Tagging](#risk-tagging)
  - [Components](#components)
  - [API](#api)
  - [Testing](#testing)
  - [See Also](#see-also)

## Overview

The Configuration module handles system-wide configuration settings, providing
a flexible and secure way to manage application parameters across different
environments.

## Feature Flags (Cargo Features)

| Feature | Default | Depends On | Purpose | Production Stance | Risks / Notes |
|---------|---------|------------|---------|-------------------|---------------|
| std | yes (via default) | - | Enable Rust std library usage | Required | Disable only for no_std targets (unsupported currently) |
| bitcoin | yes (via default) | bitcoin, bitcoincore-rpc, miniscript, bdk_wallet | Core Bitcoin protocol integration | Required for main functionality | Disabling removes wallet/PSBT/Taproot capabilities |
| rust-bitcoin | no (alias) | bitcoin | Backward compatibility alias | Avoid (legacy) | Prefer `bitcoin` directly |
| mobile | yes (via default) | ffi, std, bitcoin | Aggregate for mobile SDK support | Optional | Increases binary size; includes ffi layer |
| ffi | no (implicit via mobile) | std | FFI bindings support | Conditional | Exposes C ABI surface; audit before enabling externally |
| ndk | no | mobile, ndk crate | Android NDK support | Conditional | Android-specific build; ensure secure JNI usage |
| bdk | no | bitcoin, bdk_wallet | BDK wallet advanced features | Conditional | Expands dependency surface |
| web5 | no | jsonwebtoken, multibase, multihash | Web5 (DID/DWN/VC) integration (scaffold) | Conditional (opt-in) | External service coupling; ensure privacy (AIP) review |
| dwn | no | web5 | Enables DWN manager + decentralized storage adapters | Conditional | Draft spec; ensure data model alignment with DIF updates |
| storage-hybrid | no | enterprise, kv-rocks, dwn | Orchestrates Postgres + RocksDB + DWN/IPFS unified API | Recommended (future) | Requires consistency & conflict resolution layer |
| memory_tracking | no | std | Memory usage tracking in tests | Non-production | Overhead; enable only for diagnostics |
| enterprise | no | sqlx | SQL persistence (Postgres/SQLite) | Conditional | Requires DB hardening & migrations |
| hsm | no | hsm-software | Base HSM (software fallback) | Recommended | Software-only; not hardware isolation |
| hsm-software | (internal) | - | Software HSM implementation | Internal | Always enabled via hsm* selections |
| hsm-bitcoin | no | bitcoin, hsm-software | Bitcoin-specific HSM ops | Conditional | Needs full Bitcoin feature for signing |
| hsm-simulator | no | hsm-software | Simulator for tests | Non-production | Must NOT be enabled in prod builds |
| hsm-external | no | hsm-software | External HSM interface (CLI/integration) | Conditional | Validate command sandboxing |
| hsm-full | no | hsm-software, hsm-bitcoin, hsm-simulator, hsm-external | Full HSM suite | Non-production aggregate (dev/test) | Remove simulator for production (use hsm-production) |
| hsm-production | no | hsm-external, hsm-bitcoin | Production HSM composition | Production | Preferred over hsm-full in prod |
| taproot | no | bitcoin | Taproot transaction & asset support | Optional (enable when needed) | Must align with BIP-341/342 test vectors |
| api | no | std | API layer gating | Optional | Disable to exclude HTTP server footprint |
| cuda | no | - | CUDA ML acceleration | Experimental | Ensure driver & security review |
| wgpu | no | - | WebGPU ML acceleration | Experimental | Review sandboxing on deployment targets |
| complete | no | bitcoin, mobile, web5, enterprise, hsm-full | Turnkey all-features bundle | Non-production (bloated) | Prefer selective enabling in prod |
| dev-sim | no | - | Development simulation pathways | Non-production | Remove before release builds |
| chaos-viz | no | - | Visualization/testing of chaos scenarios | Non-production | Could expose internal state |
| disabled | no | - | Placeholder feature (no-op) | Do not use | Reserved for future gating |

### Production Feature Profile Guidance

Minimal production (Bitcoin core + API + HSM external):

```
--features "std,bitcoin,api,hsm-production"
```

Extended enterprise (adds DB + Taproot + Web5):

```
--features "std,bitcoin,api,enterprise,taproot,web5,hsm-production"
```

Avoid `complete` and any simulator (`hsm-simulator`, `hsm-full`, `dev-sim`, `chaos-viz`) in production binaries per security hardening policy.

### Environment Variable Overrides (Selected)

| Variable | Maps To | Example | Notes |
|----------|---------|---------|-------|
| ANYA_BITCOIN_NETWORK | bitcoin.network | testnet | mainnet/testnet/regtest |
| ANYA_BITCOIN_RPC_URL | RPC endpoint | <http://localhost:18332> | Auth strongly recommended |
| ANYA_SECURITY_HSM_ENABLED | Enables HSM logic | true | Must pair with a feature (hsm or hsm-production) |
| ANYA_LAYER2_PREFER_SELF_AS_MASTER | Layer2 fallback | true | Controls self-node promotion |
| ANYA_ENABLE_HSM_CROSS | Test gating | 1 | Runs cross-provider tests (non-prod) |
| ANYA_ENABLE_HSM_HEALTH | Test gating | 1 | Enables HSM health loop tests |
| ANYA_ENABLE_HSM_CONCURRENCY | Test gating | 1 | Enables concurrency stress tests |
| ANYA_STORAGE_BACKEND | Storage backend selection | dwn \| persistent \| auto | auto | Selects DWN when feature compiled & instance ready else persistent |
| ANYA_IPFS_ENDPOINT | IPFS endpoint | <http://127.0.0.1:5001> | IPFS HTTP API used for decentralized layer |
| ANYA_WEB5_SERVICE_URL | Web5 adapter base URL | <http://localhost:8080> | DWN & DID operations endpoint |
| ANYA_DID | Pre-provisioned DID | did:example:123 | If unset autoconfig attempts generation (method=key) |
| ANYA_BITCOIN_NETWORK | Bitcoin network | regtest | Only meaningful with `bitcoin` feature (regtest/testnet/mainnet) |

### Risk Tagging

- Simulator / diagnostic features: mark commits with [DEV-NONPROD].
- Experimental acceleration (cuda/wgpu): require explicit PRD ticket reference & security review.
- Broad aggregates (`complete`): only for local exploratory builds; never ship.

*Feature table generated from Cargo.toml (version 1.3.0); ensure updates here on any feature change.*

## Components

- **config_manager.rs** - Main configuration management system
- **default_config.rs** - Default configuration values
- **environment.rs** - Environment-specific configuration handling
- **mod.rs** - Module definitions and exports
- **validation.rs** - Configuration validation utilities

## API

API documentation is available:

```bash
cargo doc --open
```

## Testing

```bash
cargo test config::
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Source Code](../../src/config/)

*Last updated: 2025-08-09*

[DEV-NONPROD]: ../../README.md "Non-production diagnostic feature guidance"
