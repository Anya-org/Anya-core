# Requirements & Scope PRD

Date: August 8, 2025
Version: 2.0.0

## ✅ Scope
- Master-by-default: When no external nodes configured/reachable, run as primary self-node.
- Auto-peering: If peers are found, auto-configure and join while retaining primary preference when configured.
- Health semantics: Primary self-node considered healthy; warn on low peers.
- Productionization: Replace simulation layers with real adapters gradually, feature-gated.

## 🎯 Requirements (Testable)
- R1: prefer_self_as_master (default=true) flag controls primary role bias.
- R2: enable_self_node_fallback (default=true) ensures self-node boot when discovery fails.
- R3: is_primary flag in network state exposed via accessor is_primary_node().
- R4: health_check returns Healthy when primary self-node even if peers < min_peers.
- R5: When Bitcoin Core RPC endpoint is set, height/hash and fee estimates come from RPC.
- R6: New adapters are optional behind features; builds succeed without external services.

## 🧪 Acceptance Criteria
- AC1: Fresh run without config → node starts as primary, no panic, health=Healthy.
- AC2: With reachable peers → node discovers and connects, logs primary preference status.
- AC3: Setting RPC env → state.height>0 and fees populated from estimatesmartfee.
- AC4: All unit tests pass; cargo check --all-features passes with <10 warnings.

## 🔒 Non-Goals
- Full LN/RGB/DLC parity in this phase; target minimal viable adapters first.
- Removing simulation entirely; keep as fallback while production paths harden.

Last Updated: August 8, 2025
