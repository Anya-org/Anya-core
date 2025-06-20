---
redirect_to: /docs/ARCHITECTURE.md
title: "System_map - Development Status (June 17, 2025)"
description: "Documentation for System_map - Bitcoin Integration Progress"
last_updated: 2025-06-17
---

[AIR-3][AIS-3][BPC-3][RES-3]

# Anya Core System Map

## Visual System Map (Mermaid)

```mermaid
flowchart TD
    A[User/Contributor]
    subgraph Docs[Documentation]
        D1[INDEX_CORRECTED.md]
        D2[docs/INDEX.md]
        D3[docs/SYSTEM_MAP.md]
        D4[docs/ML_SYSTEM_ARCHITECTURE.md]
        D5[docs/SECURITY_ARCHITECTURE.md]
        D6[docs/PERFORMANCE_ARCHITECTURE.md]
    end
    subgraph Core[Core System]
        C1[core/ - Consensus, Mempool, Network]
        C2[bitcoin/ - Primitives, Wallet, Taproot]
        C3[layer2/ - RGB, DLC, RSK, Lightning, BitVM]
        C4[dao/ - Governance, Voting, Tokenomics]
        C5[infrastructure/ - Dev Rewards, Monitoring, HA]
        C6[ml/ - Agents, Federated Learning]
        C7[security/ - HSM, Crypto, Hardening]
        C8[extensions/ - Alignment, Audit, Protocol]
    end
    subgraph Contracts[Smart Contracts]
        S1[contracts/dao/vesting.clar]
        S2[contracts/dao/treasury-management.clar]
        S3[contracts/dao/license-manager.clar]
    end
    subgraph Tests[Testing]
        T1[tests/ - Unit & Integration]
        T2[tests/integration/]
        T3[tests/modules/]
    end
    A-->|Reads|Docs
    Docs-->|Guides|Core
    Core-->|Implements|Contracts
    Core-->|Tested by|Tests
    A-->|Runs|Core
    A-->|Proposes|dao/
    dao/-->|Rewards|infrastructure/dev_rewards/
    ml/-->|AI/ML|Core
    security/-->|Secures|Core
```

*This diagram provides a high-level overview of the Anya-core system, showing the relationships between documentation, core modules, smart contracts, and testing.*

---

## Improvements Implemented
- Added a visual system map in Mermaid format for onboarding and documentation clarity.
- Ensured the main index references this map for quick navigation.

---

*To view this diagram, use a Mermaid-enabled Markdown viewer or VS Code extension.*

## Development Status (June 17, 2025)

**UPDATE:** Branch management and consolidation completed via PR #44 (branch-management-20250616). The project has made significant progress but is still under active development.

## Overview

The Anya Core System Map provides a high-level visual and descriptive overview of the system's architecture, major components, and their interactions. It serves as a reference for understanding the modular structure, integration points, and relationships between subsystems such as Bitcoin, Web5, ML, and security modules.

**Current Status:** Development in progress with branch consolidation completed as of June 17, 2025.

## Bitcoin Implementation Status - In Progress

### Core Integration Progress
- **Bitcoin Core Compilation:** Most critical errors resolved, some warnings remain
- **Build System:** Functional with ongoing improvements
- **Layer2 Protocols:** Various stages of development:
  - 🔄 BOB Protocol - In development
  - ⚠️ Lightning Network - Partially integrated
  - 🔄 RSK (Rootstock) - In development
  - 🔄 RGB Protocol - Framework defined
  - 🔄 DLC Support - Base implementation
  - 🔄 Taproot Assets - Implementation in progress

## Table of Contents

- [System Architecture](#system-architecture)
- [Component Interactions](#component-interactions)
- [Integration Points](#integration-points)
- [See Also](#see-also)

## System Architecture

(Section to be completed: describe the modular hexagonal architecture, core, adapters, and ports.)

## Component Interactions

(Section to be completed: describe how modules like DAO, Layer2, ML, and Security interact.)

## Integration Points

(Section to be completed: describe integration with Bitcoin, Stacks, Web5, and external APIs.)

## See Also

- [ARCHITECTURE.md](./ARCHITECTURE.md) – Detailed architecture documentation
- [ROOT_INDEX.md](../ROOT_INDEX.md) – Root documentation index
- [DAO_SYSTEM_MAP.md](./DAO_SYSTEM_MAP.md) – DAO system map

<!-- AI Labeling references -->
[AIR-3]: ./standards/AI_LABELING.md
[AIS-3]: ./standards/AI_LABELING.md
[BPC-3]: ./standards/AI_LABELING.md
[RES-3]: ./standards/AI_LABELING.md
