---
title: System Status PRD
description: Current system status, gaps, and master-by-default policy
category: prd
tags: status-architecture-readiness
last_updated: 2025-08-09
compliance: AIR-3 AIS-3 BPC-3 RES-3
---
---

# System Status PRD

Date: August 8, 2025
Version: 2.0.0 — Master-by-default + Productionization
Repository: Anya-core (branch: bugfix/config-and-main-readme-fixes)
Status: ✅ Core stable • 🔄 Production integrations in progress

## 🎯 Executive Summary

- Default behavior: If no external nodes are configured or reachable, Anya Core becomes the primary (master) self-node automatically, while still auto-configuring peers when found.
- State/health: Health is OK when self-node is primary even if peers < min_peers; warns when discovery fails while prefer_self_as_master=true.
- Next milestone: Replace remaining simulation paths with production adapters backed by Bitcoin Core RPC.

## 📊 Compilation & Quality

- cargo check: PASS (as of latest local check)
- warnings: Target < 10 (track via scripts/quality_gate.sh)
- tests: All Layer2 tests previously passing; re-run before release

## 🧩 Components Snapshot

- Security/HSM: Production-ready (software HSM; audit logging; AES-GCM/RSA/Ed25519); integration tests stabilized with simulator unlock + timeouts.
- Bitcoin Core: RPC client available; wire state/fees (height/hash/estimatesmartfee)
- Layer2: Manager/coordinator stable; production adapters pending for LN/RGB/DLC
- Storage: SQLite + RocksDB operational; metrics enabled
           WEB5 Data storage : Beta
- AI/ML: Enhanced agentic system present; integration stabilization pending

## 🚧 Gaps To Close (P1)

1) Bitcoin Core RPC-backed state/fees plumbed into ProductionLayer2Protocol
2) Lightning adapter (LDK or LND gRPC) minimal operations
3) RGB client integration for basic schema/list/query
4) DLC oracle client wiring

## 🧪 Recent Branch Outcomes

- CLI health and validation succeed via default `anya-core` target (cargo core-health/core-validate).
- HSM integration tests updated to avoid hangs; network-bound tests remain environment-sensitive.
- Env-honest gating added: ML and system integration tests self-skip when external CLIs/resources are absent (`anya-ml`, `anya-cli`, `web5`), preventing false failures locally.

## ✅ Verification Commands

```bash
cargo check --all-features
./scripts/quality_gate.sh
./scripts/verify_implementation_status.sh
```

Last Updated: August 9, 2025
