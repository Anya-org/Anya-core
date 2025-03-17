# DEX Integration

[AIS-3][BPC-3][DAO-3]

## Overview

The Anya DAO integrates with a decentralized exchange (DEX) to provide liquidity, enable trading, and support token price discovery for the AGT governance token.

## Key Features

1. **Liquidity Provision**
   - DEX receives 35% of all token issuance
   - Users can provide STX/AGT liquidity to earn trading fees
   - Liquidity providers receive LP tokens representing their share
2. **Trading Operations**
   - Swap AGT for STX and vice versa
   - Constant product market maker formula (x * y = k)
   - Fee percentage: 0.3% by default (configurable)
3. **Buyback Mechanism**
   - DAO can execute buybacks through the DEX
   - Supports DAO-controlled market stabilization
4. **Price Oracle**
   - Provides reliable on-chain price information
   - Useful for other contracts needing AGT price data

## Implementation Details

### DEX Adapter Contract

The DEX Adapter serves as an interface between the DAO and the DEX:

```clarity
;; DEX Adapter implementation (simplified)
(define-trait dex-adapter-trait
  (
    ;; Add liquidity to the pool
    (add-liquidity (uint uint) (response uint uint))
    
    ;; Remove liquidity from the pool
    (remove-liquidity (uint) (response (tuple (token-a uint) (token-b uint)) uint))
    
    ;; Swap token A for token B
    (swap-a-for-b (uint) (response uint uint))
    
    ;; Swap token B for token A
    (swap-b-for-a (uint) (response uint uint))
    
    ;; Get the current price
    (get-price () (response uint uint))
  )
)

;; DEX Adapter implementation
(define-public (add-liquidity (token-a-amount uint) (token-b-amount uint))
  (begin
    ;; Transfer tokens to the pool
    (try! (contract-call? .token-a transfer token-a-amount tx-sender (as-contract tx-sender)))
    (try! (contract-call? .token-b transfer token-b-amount tx-sender (as-contract tx-sender)))
    
    ;; Calculate and mint LP tokens
    (let ((lp-amount (calculate-lp-amount token-a-amount token-b-amount)))
      (try! (mint-lp-tokens lp-amount tx-sender))
      (ok lp-amount))
  )
)
```

### Constant Product Market Maker

The DEX uses the constant product market maker formula:

```clarity
;; Calculate swap amount using constant product formula
(define-read-only (calculate-swap-output (input-amount uint) (input-reserve uint) (output-reserve uint))
  (let (
    (input-with-fee (mul input-amount u997))
    (numerator (mul input-with-fee output-reserve))
    (denominator (add (mul input-reserve u1000) input-with-fee))
  )
  (div numerator denominator))
)
```

### Liquidity Provision

The DEX supports adding and removing liquidity:

```typescript
// Example: Adding liquidity
async function addLiquidity(agtAmount, stxAmount) {
  const tx = await dexAdapter.addLiquidity({
    tokenAAmount: agtAmount,
    tokenBAmount: stxAmount
  });
  await tx.wait();
  return tx.lpTokensReceived;
}

// Example: Removing liquidity
async function removeLiquidity(lpTokenAmount) {
  const tx = await dexAdapter.removeLiquidity({
    lpTokenAmount
  });
  await tx.wait();
  return {
    agtReceived: tx.tokenAReceived,
    stxReceived: tx.tokenBReceived
  };
}
```

## DAO Integration Points

### Treasury Operations

The DAO can interact with the DEX for treasury operations:

1. **Liquidity Management**:
   ```clarity
   ;; Add liquidity from treasury
   (define-public (add-treasury-liquidity (agt-amount uint) (stx-amount uint))
     (begin
       (asserts! (is-authorized-by-governance tx-sender) (err u100))
       (contract-call? .dex-adapter add-liquidity agt-amount stx-amount)
     )
   )
   ```

2. **Buyback Execution**:
   ```clarity
   ;; Execute token buyback
   (define-public (execute-buyback (stx-amount uint))
     (begin
       (asserts! (is-authorized-by-governance tx-sender) (err u100))
       (let ((bought-amount (contract-call? .dex-adapter swap-b-for-a stx-amount)))
         (contract-call? .token-a burn (unwrap-panic (get amount-bought bought-amount)))
         (ok (unwrap-panic bought-amount))
       )
     )
   )
   ```

### Price Oracle

The DEX provides price information for governance decisions:

```clarity
;; Get current token price
(define-read-only (get-token-price)
  (contract-call? .dex-adapter get-price)
)

;; Check if price meets threshold for certain operations
(define-read-only (is-price-above-threshold (threshold uint))
  (let ((current-price (unwrap-panic (contract-call? .dex-adapter get-price))))
    (>= current-price threshold)
  )
)
```

## Special Distribution

The DEX receives 35% of all token issuance, allocated as follows:

| Category | Percentage | Purpose |
|----------|------------|---------|
| Initial Liquidity | 20% | Bootstrap trading pools |
| Liquidity Mining | 10% | Incentivize liquidity providers |
| Market Operations | 5% | Stabilize price during volatility |

## User Interactions

### Trading

Users can trade through the DEX interface:

```typescript
// Swap AGT for STX
async function swapAgtForStx(agtAmount) {
  const tx = await dexAdapter.swapAForB({
    amount: agtAmount
  });
  return tx.outputAmount;
}

// Swap STX for AGT
async function swapStxForAgt(stxAmount) {
  const tx = await dexAdapter.swapBForA({
    amount: stxAmount
  });
  return tx.outputAmount;
}
```

### Providing Liquidity

Users can provide liquidity to earn fees:

1. Approve both tokens for the DEX contract
2. Call the `addLiquidity` function with desired amounts
3. Receive LP tokens representing pool share
4. Earn 0.3% fees from all trades proportional to pool share

### Withdrawing Liquidity

To withdraw liquidity:

1. Call the `removeLiquidity` function with LP token amount
2. Receive both AGT and STX tokens proportionally
3. LP tokens are burned in the process

## Related Documents

- [Governance Token](GOVERNANCE_TOKEN.md) - Token traded on the DEX
- [Treasury Management](TREASURY_MANAGEMENT.md) - Treasury interaction with DEX
- [Governance Framework](GOVERNANCE_FRAMEWORK.md) - Governance control of DEX
- [Setup & Usage](SETUP_USAGE.md) - How to interact with the DEX

*Last updated: 2025-02-24* 