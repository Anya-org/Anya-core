;; Treasury Diversification Contract
;; [AIR-3][AIS-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_PARAMETER u402)
(define-constant ERR_INSUFFICIENT_FUNDS u403)
(define-constant ERR_NOT_FOUND u404)
(define-constant ERR_ALREADY_EXISTS u405)
(define-constant ERR_ALLOCATION_LIMIT_EXCEEDED u406)
(define-constant ERR_BELOW_MIN_THRESHOLD u407)
(define-constant ERR_EMERGENCY_ACTIVE u408)
(define-constant ERR_TRANSACTION_FAILED u409)
(define-constant ERR_OPERATION_DISABLED u410)

;; Contract references
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance)
(define-constant TREASURY_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury-management)
(define-constant METRICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.metrics-oracle)
(define-constant OPERATIONS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.operations-manager)

;; Data vars
(define-data-var diversification-enabled bool true)
(define-data-var last-rebalance-block uint u0)
(define-data-var rebalance-interval uint u14400) ;; ~100 days with 10-min blocks
(define-data-var emergency-active bool false)
(define-data-var total-treasury-value uint u0)
(define-data-var min-reserve-ratio uint u15) ;; 15% in protocol-native assets
(define-data-var max-single-asset-allocation uint u35) ;; 35% maximum in any single asset
(define-data-var allocation-tracking-enabled bool true)
(define-data-var diversification-strategy (string-ascii 20) "moderate") ;; conservative, moderate, aggressive

;; Asset types
(define-constant ASSET_TYPE_NATIVE u1) ;; Native protocol tokens
(define-constant ASSET_TYPE_STABLE u2) ;; Stablecoins
(define-constant ASSET_TYPE_BTC u3)   ;; Bitcoin-related assets
(define-constant ASSET_TYPE_DEFI u4)  ;; DeFi protocol tokens
(define-constant ASSET_TYPE_OTHER u5) ;; Other assets

;; Risk levels
(define-constant RISK_LEVEL_LOW u1)
(define-constant RISK_LEVEL_MEDIUM u2)
(define-constant RISK_LEVEL_HIGH u3)

;; Admin list
(define-map administrators principal bool)
(define-map signers principal bool)

;; Initialize administrators
(map-set administrators tx-sender true)
(map-set signers tx-sender true)

;; Asset allocation targets
(define-map allocation-targets
  { strategy: (string-ascii 20) }
  {
    native-percent: uint,
    stable-percent: uint,
    btc-percent: uint,
    defi-percent: uint,
    other-percent: uint
  }
)

;; Asset registry
(define-map asset-registry
  { symbol: (string-ascii 10) }
  {
    name: (string-ascii 64),
    asset-type: uint,
    risk-level: uint,
    current-allocation: uint,  ;; Basis points (100 = 1%)
    target-allocation: uint,   ;; Basis points
    last-updated: uint,
    price-in-usd: uint,       ;; Price with 8 decimals
    balance: uint,            ;; Balance with native decimals
    decimal-places: uint,
    contract-address: (optional principal),
    enabled: bool
  }
)

;; Portfolio allocation tracking
(define-map allocation-tracking
  uint ;; Block height snapshot
  {
    total-value: uint,
    native-percent: uint,
    stable-percent: uint,
    btc-percent: uint,
    defi-percent: uint,
    other-percent: uint,
    timestamp: uint,
    rebalance-required: bool
  }
)

;; Rebalance operations
(define-map rebalance-operations
  uint ;; Operation ID
  {
    executed-at: uint,
    executed-by: principal,
    old-allocation: (list 10 {symbol: (string-ascii 10), allocation: uint}),
    new-allocation: (list 10 {symbol: (string-ascii 10), allocation: uint}),
    total-value: uint,
    trades-executed: (list 5 {from-asset: (string-ascii 10), to-asset: (string-ascii 10), amount: uint})
  }
)

;; Operation counter
(define-data-var operation-counter uint u0)
(define-data-var allocation-snapshot-counter uint u0)

;; Initialize default allocation targets
(map-set allocation-targets
  { strategy: "conservative" }
  {
    native-percent: u5000,  ;; 50% in native assets
    stable-percent: u3000,  ;; 30% in stablecoins
    btc-percent: u1500,     ;; 15% in BTC-related
    defi-percent: u500,     ;; 5% in DeFi
    other-percent: u0       ;; 0% in others
  }
)

(map-set allocation-targets
  { strategy: "moderate" }
  {
    native-percent: u3500,  ;; 35% in native assets
    stable-percent: u2000,  ;; 20% in stablecoins
    btc-percent: u2500,     ;; 25% in BTC-related
    defi-percent: u1500,    ;; 15% in DeFi
    other-percent: u500     ;; 5% in others
  }
)

(map-set allocation-targets
  { strategy: "aggressive" }
  {
    native-percent: u2000,  ;; 20% in native assets
    stable-percent: u1500,  ;; 15% in stablecoins
    btc-percent: u2500,     ;; 25% in BTC-related
    defi-percent: u3000,    ;; 30% in DeFi
    other-percent: u1000    ;; 10% in others
  }
)

;; Public Functions

;; Register an asset in the treasury
(define-public (register-asset 
               (symbol (string-ascii 10))
               (name (string-ascii 64))
               (asset-type uint)
               (risk-level uint)
               (target-allocation uint)
               (price-in-usd uint)
               (balance uint)
               (decimal-places uint)
               (contract-address (optional principal)))
  (begin
    ;; Check administrator authorization
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if diversification is enabled
    (asserts! (var-get diversification-enabled) (err ERR_OPERATION_DISABLED))
    
    ;; Validate asset type
    (asserts! (and (>= asset-type ASSET_TYPE_NATIVE) (<= asset-type ASSET_TYPE_OTHER)) 
              (err ERR_INVALID_PARAMETER))
    
    ;; Validate risk level
    (asserts! (and (>= risk-level RISK_LEVEL_LOW) (<= risk-level RISK_LEVEL_HIGH)) 
              (err ERR_INVALID_PARAMETER))
    
    ;; Check if asset already exists
    (asserts! (is-none (map-get? asset-registry {symbol: symbol})) (err ERR_ALREADY_EXISTS))
    
    ;; Validate allocation doesn't exceed maximum
    (asserts! (<= target-allocation (var-get max-single-asset-allocation)) 
              (err ERR_ALLOCATION_LIMIT_EXCEEDED))
    
    ;; Register asset
    (map-set asset-registry
      {symbol: symbol}
      {
        name: name,
        asset-type: asset-type,
        risk-level: risk-level,
        current-allocation: u0,
        target-allocation: target-allocation,
        last-updated: block-height,
        price-in-usd: price-in-usd,
        balance: balance,
        decimal-places: decimal-places,
        contract-address: contract-address,
        enabled: true
      }
    )
    
    ;; Create an updated allocation snapshot
    (try! (create-allocation-snapshot))
    
    (ok true)
  ))

;; Update asset information
(define-public (update-asset 
               (symbol (string-ascii 10))
               (target-allocation (optional uint))
               (price-in-usd (optional uint))
               (balance (optional uint))
               (enabled (optional bool)))
  (begin
    ;; Check administrator authorization
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if diversification is enabled
    (asserts! (var-get diversification-enabled) (err ERR_OPERATION_DISABLED))
    
    ;; Get the asset
    (let (
      (asset (unwrap! (map-get? asset-registry {symbol: symbol}) (err ERR_NOT_FOUND)))
    )
      ;; Validate target allocation if provided
      (when (is-some target-allocation)
        (asserts! (<= (default-to u0 target-allocation) (var-get max-single-asset-allocation)) 
                  (err ERR_ALLOCATION_LIMIT_EXCEEDED))
      )
      
      ;; Update asset information
      (map-set asset-registry
        {symbol: symbol}
        (merge asset {
          target-allocation: (default-to (get target-allocation asset) target-allocation),
          price-in-usd: (default-to (get price-in-usd asset) price-in-usd),
          balance: (default-to (get balance asset) balance),
          last-updated: block-height,
          enabled: (default-to (get enabled asset) enabled)
        })
      )
      
      ;; Create an updated allocation snapshot
      (try! (create-allocation-snapshot))
      
      (ok true)
    )
  ))

;; Execute rebalance of the portfolio
(define-public (execute-rebalance)
  (begin
    ;; Check authorization
    (asserts! (or (is-administrator tx-sender) (is-signer tx-sender)) (err ERR_UNAUTHORIZED))
    
    ;; Check if diversification is enabled
    (asserts! (var-get diversification-enabled) (err ERR_OPERATION_DISABLED))
    
    ;; Check if emergency mode is active
    (asserts! (not (var-get emergency-active)) (err ERR_EMERGENCY_ACTIVE))
    
    ;; Check rebalance interval
    (asserts! (>= (- block-height (var-get last-rebalance-block)) (var-get rebalance-interval)) 
              (err ERR_INVALID_PARAMETER))
    
    ;; Create allocation snapshot
    (try! (create-allocation-snapshot))
    
    ;; Calculate target allocations
    (let (
      (rebalance-plan (calculate-rebalance-plan))
      (operation-id (+ (var-get operation-counter) u1))
    )
      ;; Execute the rebalance plan
      (try! (execute-rebalance-plan rebalance-plan))
      
      ;; Store the rebalance operation
      (map-set rebalance-operations
        operation-id
        {
          executed-at: block-height,
          executed-by: tx-sender,
          old-allocation: (map asset-allocation-entry (get-all-asset-symbols)),
          new-allocation: rebalance-plan,
          total-value: (var-get total-treasury-value),
          trades-executed: (list) ;; In a real implementation, we would track actual trades
        }
      )
      
      ;; Update operation counter
      (var-set operation-counter operation-id)
      
      ;; Update last rebalance block
      (var-set last-rebalance-block block-height)
      
      ;; Create an updated allocation snapshot
      (try! (create-allocation-snapshot))
      
      (ok operation-id)
    )
  ))

;; Create a snapshot of current allocation
(define-public (create-allocation-snapshot)
  (begin
    ;; Check if allocation tracking is enabled
    (asserts! (var-get allocation-tracking-enabled) (err ERR_OPERATION_DISABLED))
    
    ;; Calculate total value and percentages
    (let (
      (allocation-data (calculate-allocation-data))
      (snapshot-id (+ (var-get allocation-snapshot-counter) u1))
      (target-allocation (unwrap! (map-get? allocation-targets {strategy: (var-get diversification-strategy)}) 
                                 (err ERR_NOT_FOUND)))
      (rebalance-required (or
                            (> (abs-diff (get native-percent allocation-data) (get native-percent target-allocation)) u500)
                            (> (abs-diff (get stable-percent allocation-data) (get stable-percent target-allocation)) u500)
                            (> (abs-diff (get btc-percent allocation-data) (get btc-percent target-allocation)) u500)
                            (> (abs-diff (get defi-percent allocation-data) (get defi-percent target-allocation)) u500)
                            (> (abs-diff (get other-percent allocation-data) (get other-percent target-allocation)) u500)
                          ))
    )
      ;; Store allocation snapshot
      (map-set allocation-tracking
        snapshot-id
        (merge allocation-data {
          timestamp: block-height,
          rebalance-required: rebalance-required
        })
      )
      
      ;; Update counter
      (var-set allocation-snapshot-counter snapshot-id)
      
      ;; Update total treasury value
      (var-set total-treasury-value (get total-value allocation-data))
      
      ;; Update metrics
      (try! (contract-call? METRICS_CONTRACT submit-treasury-metric "treasury_portfolio_value" 
                           (get total-value allocation-data) u1000 "treasury-diversification"))
      
      (ok snapshot-id)
    )
  ))

;; Change diversification strategy
(define-public (change-strategy (new-strategy (string-ascii 20)))
  (begin
    ;; Check authorization
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if diversification is enabled
    (asserts! (var-get diversification-enabled) (err ERR_OPERATION_DISABLED))
    
    ;; Validate strategy
    (asserts! (is-some (map-get? allocation-targets {strategy: new-strategy})) 
              (err ERR_INVALID_PARAMETER))
    
    ;; Update strategy
    (var-set diversification-strategy new-strategy)
    
    ;; Create an updated allocation snapshot
    (try! (create-allocation-snapshot))
    
    (ok true)
  ))

;; Execute a targeted rebalance for specific assets
(define-public (execute-targeted-rebalance (assets (list 10 (string-ascii 10))))
  (begin
    ;; Check authorization
    (asserts! (or (is-administrator tx-sender) (is-signer tx-sender)) (err ERR_UNAUTHORIZED))
    
    ;; Check if diversification is enabled
    (asserts! (var-get diversification-enabled) (err ERR_OPERATION_DISABLED))
    
    ;; Check if emergency mode is active
    (asserts! (not (var-get emergency-active)) (err ERR_EMERGENCY_ACTIVE))
    
    ;; Create rebalance plan for target assets
    (let (
      (rebalance-plan (calculate-targeted-rebalance-plan assets))
      (operation-id (+ (var-get operation-counter) u1))
    )
      ;; Execute the rebalance plan
      (try! (execute-rebalance-plan rebalance-plan))
      
      ;; Store the rebalance operation
      (map-set rebalance-operations
        operation-id
        {
          executed-at: block-height,
          executed-by: tx-sender,
          old-allocation: (map asset-allocation-entry assets),
          new-allocation: rebalance-plan,
          total-value: (var-get total-treasury-value),
          trades-executed: (list) ;; In a real implementation, we would track actual trades
        }
      )
      
      ;; Update operation counter
      (var-set operation-counter operation-id)
      
      ;; Create an updated allocation snapshot
      (try! (create-allocation-snapshot))
      
      (ok operation-id)
    )
  ))

;; Read-Only Functions

;; Get asset details
(define-read-only (get-asset (symbol (string-ascii 10)))
  (map-get? asset-registry {symbol: symbol}))

;; Get current allocation snapshot
(define-read-only (get-current-allocation)
  (let ((snapshot-id (var-get allocation-snapshot-counter)))
    (map-get? allocation-tracking snapshot-id)))

;; Get allocation targets for a strategy
(define-read-only (get-allocation-targets (strategy (string-ascii 20)))
  (map-get? allocation-targets {strategy: strategy}))

;; Get diversification status
(define-read-only (get-diversification-status)
  {
    enabled: (var-get diversification-enabled),
    strategy: (var-get diversification-strategy),
    last-rebalance-block: (var-get last-rebalance-block),
    blocks-since-rebalance: (- block-height (var-get last-rebalance-block)),
    rebalance-interval: (var-get rebalance-interval),
    emergency-active: (var-get emergency-active),
    total-value: (var-get total-treasury-value),
    min-reserve-ratio: (var-get min-reserve-ratio),
    max-single-asset-allocation: (var-get max-single-asset-allocation)
  })

;; Get rebalance operation details
(define-read-only (get-rebalance-operation (operation-id uint))
  (map-get? rebalance-operations operation-id))

;; Check if account is an administrator
(define-read-only (is-administrator (account principal))
  (default-to false (map-get? administrators account)))

;; Check if account is an authorized signer
(define-read-only (is-signer (account principal))
  (default-to false (map-get? signers account)))

;; Get all registered asset symbols
(define-read-only (get-all-asset-symbols)
  ;; This would be dynamically generated in a real implementation
  ;; For demo, we'll return a static list
  (list "AGT" "USDT" "BTC" "ETH" "DOT")
)

;; Helper Functions

;; Calculate current allocation data
(define-private (calculate-allocation-data)
  (let (
    (assets (get-all-asset-symbols))
    (total-value u0)
    (native-value u0)
    (stable-value u0)
    (btc-value u0)
    (defi-value u0)
    (other-value u0)
  )
    ;; In a real implementation, we would iterate through assets to calculate these values
    ;; For simplicity, we'll use placeholder values
    {
      total-value: u1000000000000, ;; $10M with 8 decimals
      native-percent: u3500,       ;; 35%
      stable-percent: u2000,       ;; 20%
      btc-percent: u2500,          ;; 25%
      defi-percent: u1500,         ;; 15%
      other-percent: u500          ;; 5%
    }
  ))

;; Calculate rebalance plan for all assets
(define-private (calculate-rebalance-plan)
  (let (
    (assets (get-all-asset-symbols))
  )
    ;; In a real implementation, we would calculate specific target allocations
    ;; For simplicity, we'll return placeholder targets
    (list
      {symbol: "AGT", allocation: u3500}
      {symbol: "USDT", allocation: u2000}
      {symbol: "BTC", allocation: u2500}
      {symbol: "ETH", allocation: u1500}
      {symbol: "DOT", allocation: u500}
    )
  ))

;; Calculate targeted rebalance plan for specific assets
(define-private (calculate-targeted-rebalance-plan (assets (list 10 (string-ascii 10))))
  ;; In a real implementation, we would calculate specific target allocations for the provided assets
  ;; For simplicity, we'll return placeholder targets
  (list
    {symbol: "AGT", allocation: u3500}
    {symbol: "USDT", allocation: u2000}
  )
)

;; Execute a rebalance plan
(define-private (execute-rebalance-plan (plan (list 10 {symbol: (string-ascii 10), allocation: uint})))
  ;; In a real implementation, we would execute actual trades to achieve the target allocation
  ;; For demo, we'll just simulate success
  (ok true)
)

;; Get asset allocation entry for a symbol
(define-private (asset-allocation-entry (symbol (string-ascii 10)))
  (match (map-get? asset-registry {symbol: symbol})
    asset {symbol: symbol, allocation: (get current-allocation asset)}
    {symbol: symbol, allocation: u0}
  )
)

;; Calculate absolute difference between two values
(define-private (abs-diff (a uint) (b uint))
  (if (>= a b)
      (- a b)
      (- b a)
  )
)

;; Administrative Functions

;; Add an administrator
(define-public (add-administrator (admin principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set administrators admin true)
    (ok true)))

;; Remove an administrator
(define-public (remove-administrator (admin principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set administrators admin false)
    (ok true)))

;; Add a signer
(define-public (add-signer (signer principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set signers signer true)
    (ok true)))

;; Remove a signer
(define-public (remove-signer (signer principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set signers signer false)
    (ok true)))

;; Toggle diversification
(define-public (toggle-diversification (enabled bool))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (var-set diversification-enabled enabled)
    (ok true)))

;; Toggle allocation tracking
(define-public (toggle-allocation-tracking (enabled bool))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (var-set allocation-tracking-enabled enabled)
    (ok true)))

;; Toggle emergency mode
(define-public (toggle-emergency-mode (enabled bool))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (var-set emergency-active enabled)
    (ok true)))

;; Update diversification parameters
(define-public (update-diversification-parameters
               (new-rebalance-interval (optional uint))
               (new-min-reserve-ratio (optional uint))
               (new-max-single-asset-allocation (optional uint)))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Update rebalance interval if provided
    (match new-rebalance-interval
      interval (var-set rebalance-interval interval)
      true)
    
    ;; Update minimum reserve ratio if provided
    (match new-min-reserve-ratio
      ratio (var-set min-reserve-ratio ratio)
      true)
    
    ;; Update maximum single asset allocation if provided
    (match new-max-single-asset-allocation
      allocation (var-set max-single-asset-allocation allocation)
      true)
    
    (ok true)
  )) 