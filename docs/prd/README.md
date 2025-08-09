---
title: PRD Index
description: Index of Product Requirement Documents
category: prd
tags: prd-index
last_updated: 2025-08-09
compliance: AIR-3 AIS-3 BPC-3 RES-3
---
---
---

## PRD index (August 9, 2025)

Authoritative PRD set for “Self-node master-by-default + production readiness”. Legacy PRDs are retired (soft-redirects remain to avoid broken links).

Files:

- 01_SYSTEM_STATUS_PRD.md — Current status and gaps (truth-to-source)
- 02_REQUIREMENTS_AND_SCOPE_PRD.md — What’s changing and why (inputs, success criteria)
- 03_TECHNICAL_ARCHITECTURE_PRD.md — Key architecture and integration points
- 04_IMPLEMENTATION_ROADMAP_PRD.md — Short execution plan with owners
- 05_QA_AND_VERIFICATION_PRD.md — Quality gates, scripts, and exit criteria

Conventions:

- Code is the source of truth; keep PRDs aligned.
- Commands should be runnable; prefer ./scripts/*.
- Update “Last Updated” on each edit.

Handy commands:

- cargo core-health
- cargo core-validate
- cargo test --features dev-sim,hsm-full --no-fail-fast -- --nocapture
