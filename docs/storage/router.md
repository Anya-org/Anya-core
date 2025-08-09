# Storage Router

Provides runtime selection between decentralized (DWN/IPFS) storage and persistent (Postgres/RocksDB) backend.

## Overview

The `StorageRouter` enum chooses the authoritative storage layer at runtime while preserving compile-time feature gating.

```rust
let router = StorageRouter::from_env(persistent, Some(decentralized));
```

## Environment Variable

| Variable | Values | Default | Behavior |
|----------|--------|---------|----------|
| ANYA_STORAGE_BACKEND | dwn \| persistent \| auto | auto | auto selects DWN when compiled with `dwn` feature & a decentralized instance is available; otherwise persistent |
| ANYA_IPFS_ENDPOINT | URL | <http://127.0.0.1:5001> | IPFS API endpoint used for content addressing |
| ANYA_WEB5_SERVICE_URL | URL | <http://localhost:8080> | Web5 service base URL for DWN/DID ops |
| ANYA_DID | DID string | (auto-generated) | If absent, autoconfig attempts `create_did("key")` via adapter |
| ANYA_BITCOIN_NETWORK | regtest \| testnet \| mainnet | regtest | Anchoring network (only effective when bitcoin feature enabled) |

## Feature Interaction

- `dwn` feature must be enabled to construct the decentralized variant.
- `storage-hybrid` may layer additional orchestration logic (future).

## Fallback Behavior

When `dwn` feature is disabled or a decentralized instance isn't supplied, router transparently uses persistent storage.

## Security Notes

- Ensure DWN data model integrity; persistent fallback should not expose unvalidated DWN-originated records.
- For high-assurance deployments, pin ANYA_STORAGE_BACKEND="dwn" and verify via startup logs.

_Last updated: 2025-08-09 (autoconfig env doc fix)_
