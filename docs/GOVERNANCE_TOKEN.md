---
title: "Governance_token"
description: "Documentation for Governance_token"
---

[AIR-3][AIS-3][BPC-3][RES-3]


# Anya Governance Token (AGT)

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


[AIS-3][BPC-3][DAO-3]

## Overview

The Anya Governance Token (AGT) is the core utility and governance token of the Anya DAO platform, enabling participation in protocol governance and ecosystem incentives.

## Token Economics

- **Total Supply**: 21,000,000,000 AGT (Fixed)
- **Initial Block Reward**: 10,000 AGT
- **Emission Schedule**: Adaptive Bitcoin-inspired halving mechanism
  - Minimum halving interval: 105,000 blocks
  - Halving controlled by governance parameters

## Token Distribution

The AGT token is distributed according to the following model:

- **40% Protocol Treasury** (8.4B AGT)
  - 20% Strategic Reserves (BIP-341 compliant)
  - 20% Ecosystem Development (DAO-4 managed)
- **30% Liquidity Provision** (6.3B AGT)
  - 20% Initial DEX Liquidity (Taproot-enabled)
  - 10% Ongoing Liquidity Mining (BIP-174 PSBT)
- **15% Team & Development** (3.15B AGT)
  - 5-year vesting with 2-year cliff
  - Performance milestones (BPC-3 verified)
- **10% Community Incentives** (2.1B AGT)
  - Governance participation rewards
  - Protocol usage incentives
- **5% Strategic Partners** (1.05B AGT)
  - 3-year vesting schedule

## Bitcoin-Style Tokenomics

### Issuance Model

The AGT token follows a Bitcoin-style issuance model:

- **Total Supply**: 21 billion AGT (with 8 decimal places)
- **Initial Block Reward**: 5,000 AGT per block (higher than Bitcoin)
- **Halving Interval**: Every 210,000 blocks (~4 years with 10-minute blocks)
- **Halving Schedule**:
  - First 210,000 blocks: 5,000 AGT per block
  - Next 210,000 blocks: 2,500 AGT per block
  - Next 210,000 blocks: 1,250 AGT per block
  - And so on...

### Distribution Allocation

Each block reward is distributed strategically:

- **DEX Allocation**: 35% (aligned with liquidity provision)
- **DAO/Community**: 50% (aligned with governance needs)
- **Network Security Fund**: 15% (enables protocol safety)

### Developer Team Allocation

The team allocation is further distributed:

- **Top Performer**: 30% of the team allocation
- **Base Distribution**: 50% evenly split
- **Performance Bonus Pool**: 20%

## Token Utility

AGT tokens serve multiple functions within the ecosystem:

1. **Governance Rights**: Vote on protocol decisions
2. **Proposal Creation**: Submit governance proposals
3. **Protocol Fee Discounts**: Reduced fees for token holders
4. **Staking Rewards**: Earn rewards for protocol support
5. **Liquidity Mining**: Earn tokens by providing liquidity

## Token Contract Implementation

The AGT token is implemented as a standard-compliant token with additional governance functionality:

```clarity
;; AGT token implementation (simplified)
(define-fungible-token agt 21000000000000000)

;; Check token balance
(define-read-only (get-balance (account principal))
  (ft-get-balance agt account)
)

;; Transfer tokens
(define-public (transfer (amount uint) (sender principal) (recipient principal))
  (ft-transfer? agt amount sender recipient)
)

;; Mint tokens (only callable by authorized minters)
(define-public (mint (amount uint) (recipient principal))
  (begin
    (asserts! (is-authorized-minter tx-sender) (err u100))
    (ft-mint? agt amount recipient)
  )
)
```

## Tokenomics Parameters

| Parameter | Value | Description |
|-----------|-------|-------------|
| Total Supply | 21,000,000,000 AGT | Maximum supply cap |
| Initial Block Reward | 5,000 AGT | Block reward with 8 decimal places |
| Halving Interval | 210,000 blocks | ~4 years with 10-minute blocks |
| DEX Allocation | 35% | Percentage of block rewards allocated to DEX |
| DAO Allocation | 50% | Percentage of block rewards allocated to DAO/community |
| Network Security Fund | 15% | Percentage allocated to security operations |
| DEX Fee | 0.3% | Trading fee percentage |
| Proposal Threshold | 100 AGT | Minimum tokens to submit a proposal |
| Voting Threshold | 60% | Percentage needed to pass a proposal |
| Quorum | 30% | Minimum participation required |

## Related Documents

- [Governance Framework](GOVERNANCE_FRAMEWORK.md) - How tokens are used in governance
- [DEX Integration](DEX_INTEGRATION.md) - Liquidity provision for tokens
- [Treasury Management](TREASURY_MANAGEMENT.md) - Token treasury operations
- [Tokenomics Flowchart](TOKENOMICS_FLOWCHART.md) - Visual representation of token flows

*Last updated: 2025-02-24* 
## See Also

- [Related Document](#related-document)

