# DAO System Migration Guide

## Overview

This guide documents the migration from alternative/development tokenomics systems to the **official production system**. All alternative implementations have been deprecated and should be replaced with the production system.

## Migration Status

### ✅ **Official Production System** (`contracts/dao/`)

**Location**: `/contracts/dao/tokenomics.clar`
**Status**: ✅ **ACTIVE - Use for all production deployments**

**Production Parameters**:

- Initial Block Reward: **10,000 AGT tokens per block**
- Halving Interval: **105,000 blocks** (adaptive, minimum interval)
- Total Supply: **21 billion tokens** (8 decimal precision)
- Distribution: **35%/25%/20%/15%/5%**
  - 35% Protocol Treasury
  - 25% Liquidity Provision
  - 20% Team & Development
  - 15% Community Incentives
  - 5% Strategic Partners

### ❌ **Deprecated Alternative Systems**

#### 1. Development System (`src/contracts/bitcoin-issuance.clar`)

**Status**: ❌ **DEPRECATED - Do not use**
**Reason**: Wrong parameters (5,000 block reward, 210,000 halving, incorrect distribution)

#### 2. Extension System (`dao/extensions/token-economics.clar`)

**Status**: ❌ **DEPRECATED - Do not use**
**Reason**: Alternative implementation with different parameters

#### 3. Minimal System (`dao/extensions/token-economics-minimal.clar`)

**Status**: ❌ **DEPRECATED - Do not use**
**Reason**: Simplified implementation for testing only

#### 4. Legacy Governance (`src/contracts/governance_token.clar`)

**Status**: ❌ **DEPRECATED - Do not use**
**Reason**: Old implementation, replaced by production system

## Critical Parameter Changes

### Block Reward Changes

- **Old**: 5,000 tokens per block
- **New**: 10,000 tokens per block
- **Impact**: Doubled emission rate for faster ecosystem growth

### Halving Interval Changes

- **Old**: 210,000 blocks (Bitcoin standard)
- **New**: 105,000 blocks (adaptive, minimum interval)
- **Impact**: More frequent halvings, accelerated supply curve

### Distribution Changes

- **Old**: 30% DEX / 15% Team / 55% DAO/Community
- **New**: 35% Treasury / 25% Liquidity / 20% Team / 15% Community / 5% Partners
- **Impact**: More balanced distribution, added strategic partners allocation

## Migration Steps

### For Developers

1. **Update Contract References**:

   ```clarity
   ;; OLD (WRONG)
   (use-trait dao-trait .dao-extensions.dao-trait)

   ;; NEW (CORRECT)
   (use-trait dao-trait .dao-governance.dao-trait)
   ```

2. **Update Parameter References**:

   ```clarity
   ;; OLD (WRONG)
   (define-constant INITIAL_BLOCK_REWARD u5000)
   (define-constant HALVING_INTERVAL u210000)

   ;; NEW (CORRECT)
   (define-constant INITIAL_BLOCK_REWARD u1000000000) ;; 10,000 with 8 decimals
   (define-constant MIN_HALVING_INTERVAL u105000)
   ```

3. **Update Distribution Logic**:

   ```clarity
   ;; OLD (WRONG)
   (define-constant DEX_ALLOCATION_PERCENTAGE u30)
   (define-constant DAO_ALLOCATION_PERCENTAGE u55)

   ;; NEW (CORRECT)
   (define-constant TREASURY_PERCENTAGE u35)
   (define-constant LIQUIDITY_PERCENTAGE u25)
   (define-constant TEAM_PERCENTAGE u20)
   (define-constant COMMUNITY_PERCENTAGE u15)
   (define-constant PARTNERS_PERCENTAGE u5)
   ```

### For Tools and Scripts

1. **Update Configuration Files**:

   ```javascript
   // OLD (WRONG)
   const HALVING_INTERVAL = 210000;
   const INITIAL_BLOCK_REWARD = 5000;

   // NEW (CORRECT)
   const HALVING_INTERVAL = 105000;
   const INITIAL_BLOCK_REWARD = 10000;
   ```

2. **Update Import Paths**:

   ```javascript
   // OLD (WRONG)
   import { tokenomics } from './dao/extensions/token-economics.clar';

   // NEW (CORRECT)
   import { tokenomics } from './contracts/dao/tokenomics.clar';
   ```

### For Documentation

1. **Remove Alternative System References**
2. **Update All Parameter Documentation**
3. **Add Migration Notices to Deprecated Files**

## Verification Checklist

- [ ] All contract references point to `contracts/dao/`
- [ ] Block reward is 10,000 tokens per block (1,000,000,000 base units)
- [ ] Halving interval is 105,000 blocks minimum
- [ ] Distribution uses 35%/25%/20%/15%/5% model
- [ ] No references to deprecated systems remain
- [ ] All tools use production parameters
- [ ] Tests validate production tokenomics

## File Status Reference

### ✅ Production Files (Use These)

```
contracts/dao/
├── tokenomics.clar              ✅ ACTIVE
├── dao-governance.clar          ✅ ACTIVE
├── token.clar                   ✅ ACTIVE
├── vesting.clar                 ✅ ACTIVE
├── treasury-management.clar     ✅ ACTIVE
└── [all other contracts/dao/]   ✅ ACTIVE
```

### ❌ Deprecated Files (Do Not Use)

```
dao/extensions/
├── token-economics.clar         ❌ DEPRECATED
└── token-economics-minimal.clar ❌ DEPRECATED

src/contracts/
├── bitcoin-issuance.clar        ❌ DEPRECATED
├── dao.clar                     ❌ DEPRECATED
├── governance_token.clar        ❌ DEPRECATED
└── token-economics.clar         ❌ DEPRECATED
```

## Support

For migration support:

1. Check production system documentation in `contracts/dao/`
2. Reference this migration guide
3. Test thoroughly before mainnet deployment
4. Verify all parameters match production specifications

**Remember**: Only use the production system (`contracts/dao/`) for mainnet deployments.
