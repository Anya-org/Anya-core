;; DAO Tokenomics Contract
;; [AIR-3][AIS-3][AIT-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_PARAMETER u402)
(define-constant ERR_BELOW_THRESHOLD u403)

;; Tokenomics Constants (Updated)
(define-constant TOTAL_SUPPLY u21000000000000000) ;; 21B tokens with 8 decimals
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance)

;; Updated distribution percentages
(define-constant TREASURY_PERCENTAGE u35) ;; 35% Protocol Treasury
(define-constant LIQUIDITY_PERCENTAGE u25) ;; 25% Liquidity Provision
(define-constant TEAM_PERCENTAGE u20) ;; 20% Team & Development
(define-constant COMMUNITY_PERCENTAGE u15) ;; 15% Community Incentives
(define-constant PARTNERS_PERCENTAGE u5) ;; 5% Strategic Partners

;; Updated allocation amounts
(define-constant TREASURY_ALLOCATION (/ (* TOTAL_SUPPLY TREASURY_PERCENTAGE) u100))
(define-constant LIQUIDITY_ALLOCATION (/ (* TOTAL_SUPPLY LIQUIDITY_PERCENTAGE) u100))
(define-constant TEAM_ALLOCATION (/ (* TOTAL_SUPPLY TEAM_PERCENTAGE) u100))
(define-constant COMMUNITY_ALLOCATION (/ (* TOTAL_SUPPLY COMMUNITY_PERCENTAGE) u100))
(define-constant PARTNERS_ALLOCATION (/ (* TOTAL_SUPPLY PARTNERS_PERCENTAGE) u100))

;; Updated emission parameters
(define-constant INITIAL_BLOCK_REWARD u1000000000) ;; 10,000 AGT per block
(define-constant MIN_HALVING_INTERVAL u105000) ;; Minimum halving interval
(define-constant DEFAULT_HALVING_INTERVAL u105000) ;; Default halving interval

;; Emission data
(define-data-var current-block-reward uint INITIAL_BLOCK_REWARD)
(define-data-var halving-interval uint DEFAULT_HALVING_INTERVAL)
(define-data-var last-halving-block uint u0)
(define-data-var total-minted uint u0)

;; Treasury management parameters
(define-data-var reserve-ratio uint u15) ;; 15% minimum reserve ratio
(define-data-var pol-ratio uint u15) ;; 15% target protocol-owned liquidity
(define-data-var strategic-reserves uint u0) ;; Current strategic reserves
(define-data-var protocol-owned-liquidity uint u0) ;; Current protocol-owned liquidity

;; Distribution tracking
(define-data-var treasury-released uint u0)
(define-data-var liquidity-released uint u0)
(define-data-var team-released uint u0)
(define-data-var community-released uint u0)
(define-data-var partners-released uint u0)

;; Vesting timestamps
(define-data-var launch-timestamp uint u0)

;; Admin list
(define-map administrators principal bool)

;; Initialize administrators
(map-set administrators tx-sender true)

;; Public Functions

;; Initialize distribution
(define-public (initialize-distribution)
  (begin
    ;; Only admins can initialize
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Can only be called once
    (asserts! (is-eq (var-get launch-timestamp) u0) (err ERR_UNAUTHORIZED))
    
    ;; Set launch timestamp
    (var-set launch-timestamp block-height)
    
    ;; Calculate initial releases
    (let (
      (treasury-initial (/ (* TREASURY_ALLOCATION u20) u100)) ;; 20% initial
      (liquidity-initial (/ (* LIQUIDITY_ALLOCATION u50) u100)) ;; 50% initial
      (team-initial u0) ;; 0% initial (cliff)
      (community-initial (/ (* COMMUNITY_ALLOCATION u10) u100)) ;; 10% initial
      (partners-initial (/ (* PARTNERS_ALLOCATION u10) u100)) ;; 10% initial
    )
      ;; Track released amounts
      (var-set treasury-released treasury-initial)
      (var-set liquidity-released liquidity-initial)
      (var-set team-released team-initial)
      (var-set community-released community-initial)
      (var-set partners-released partners-initial)
      
      ;; Initialize strategic reserves
      (var-set strategic-reserves (/ treasury-initial u2))
      
      ;; Initialize protocol-owned liquidity
      (var-set protocol-owned-liquidity (/ liquidity-initial u3))
      
      ;; Update total minted
      (var-set total-minted (+ (+ (+ (+ treasury-initial liquidity-initial) team-initial) community-initial) partners-initial))
      
      (ok true)
    )
  ))

;; Process vesting release
(define-public (process-vesting-release)
  (begin
    ;; Only admins can process vesting
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Ensure distribution was initialized
    (asserts! (> (var-get launch-timestamp) u0) (err ERR_UNAUTHORIZED))
    
    ;; Calculate months since launch
    (let (
      (blocks-since-launch (- block-height (var-get launch-timestamp)))
      (months-since-launch (/ blocks-since-launch u4320)) ;; ~30 days with 10-min blocks
    )
      ;; Process each allocation's vesting
      (process-treasury-vesting months-since-launch)
      (process-liquidity-vesting months-since-launch)
      (process-team-vesting months-since-launch)
      (process-community-vesting months-since-launch)
      (process-partners-vesting months-since-launch)
      
      (ok true)
    )
  ))

;; Update emission parameters
(define-public (update-emission-parameters (new-halving-interval uint))
  (begin
    ;; Only the DAO contract can update emission parameters
    (asserts! (is-eq contract-caller DAO_CONTRACT) (err ERR_UNAUTHORIZED))
    
    ;; Ensure new interval is at or above minimum
    (asserts! (>= new-halving-interval MIN_HALVING_INTERVAL) (err ERR_INVALID_PARAMETER))
    
    ;; Update the halving interval
    (var-set halving-interval new-halving-interval)
    
    (ok true)
  ))

;; Process block rewards
(define-public (process-block-rewards)
  (begin
    ;; Only admins can process rewards
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if halving should occur
    (if (should-halve)
        (perform-halving)
        true)
    
    ;; Update total minted with current block reward
    (var-set total-minted (+ (var-get total-minted) (var-get current-block-reward)))
    
    (ok true)
  ))

;; Update treasury management parameters
(define-public (update-treasury-parameters (new-reserve-ratio uint) (new-pol-ratio uint))
  (begin
    ;; Only the DAO contract can update treasury parameters
    (asserts! (is-eq contract-caller DAO_CONTRACT) (err ERR_UNAUTHORIZED))
    
    ;; Validate parameters
    (asserts! (and (>= new-reserve-ratio u5) (<= new-reserve-ratio u30)) (err ERR_INVALID_PARAMETER))
    (asserts! (and (>= new-pol-ratio u5) (<= new-pol-ratio u30)) (err ERR_INVALID_PARAMETER))
    
    ;; Update parameters
    (var-set reserve-ratio new-reserve-ratio)
    (var-set pol-ratio new-pol-ratio)
    
    (ok true)
  ))

;; Update strategic reserves amount
(define-public (update-strategic-reserves (amount uint) (operation uint))
  (begin
    ;; Only the DAO contract can update treasury reserves
    (asserts! (is-eq contract-caller DAO_CONTRACT) (err ERR_UNAUTHORIZED))
    
    ;; 1 = add, 2 = subtract
    (if (is-eq operation u1)
        (var-set strategic-reserves (+ (var-get strategic-reserves) amount))
        (var-set strategic-reserves (- (var-get strategic-reserves) amount))
    )
    
    (ok true)
  ))

;; Update protocol-owned liquidity amount
(define-public (update-protocol-liquidity (amount uint) (operation uint))
  (begin
    ;; Only the DAO contract can update protocol liquidity
    (asserts! (is-eq contract-caller DAO_CONTRACT) (err ERR_UNAUTHORIZED))
    
    ;; 1 = add, 2 = subtract
    (if (is-eq operation u1)
        (var-set protocol-owned-liquidity (+ (var-get protocol-owned-liquidity) amount))
        (var-set protocol-owned-liquidity (- (var-get protocol-owned-liquidity) amount))
    )
    
    (ok true)
  ))

;; Read-Only Functions

;; Get distribution data
(define-read-only (get-distribution-data)
  {
    treasury-percentage: TREASURY_PERCENTAGE,
    liquidity-percentage: LIQUIDITY_PERCENTAGE,
    team-percentage: TEAM_PERCENTAGE,
    community-percentage: COMMUNITY_PERCENTAGE,
    partners-percentage: PARTNERS_PERCENTAGE,
    
    treasury-allocation: TREASURY_ALLOCATION,
    liquidity-allocation: LIQUIDITY_ALLOCATION,
    team-allocation: TEAM_ALLOCATION,
    community-allocation: COMMUNITY_ALLOCATION,
    partners-allocation: PARTNERS_ALLOCATION,
    
    treasury-released: (var-get treasury-released),
    liquidity-released: (var-get liquidity-released),
    team-released: (var-get team-released),
    community-released: (var-get community-released),
    partners-released: (var-get partners-released)
  })

;; Get emission data
(define-read-only (get-emission-data)
  {
    initial-block-reward: INITIAL_BLOCK_REWARD,
    current-block-reward: (var-get current-block-reward),
    halving-interval: (var-get halving-interval),
    min-halving-interval: MIN_HALVING_INTERVAL,
    last-halving-block: (var-get last-halving-block),
    total-minted: (var-get total-minted),
    next-halving-block: (+ (var-get last-halving-block) (var-get halving-interval))
  })

;; Get treasury data
(define-read-only (get-treasury-data)
  {
    reserve-ratio: (var-get reserve-ratio),
    pol-ratio: (var-get pol-ratio),
    strategic-reserves: (var-get strategic-reserves),
    protocol-owned-liquidity: (var-get protocol-owned-liquidity)
  })

;; Check if vesting is initialized
(define-read-only (is-vesting-initialized)
  (> (var-get launch-timestamp) u0))

;; Calculate months since launch
(define-read-only (get-months-since-launch)
  (if (> (var-get launch-timestamp) u0)
      (/ (- block-height (var-get launch-timestamp)) u4320)
      u0))

;; Check if account is an administrator
(define-read-only (is-administrator (account principal))
  (default-to false (map-get? administrators account)))

;; Check if halving should occur
(define-read-only (should-halve)
  (>= block-height (+ (var-get last-halving-block) (var-get halving-interval))))

;; Private and Helper Functions

;; Perform halving operation
(define-private (perform-halving)
  (begin
    ;; Reduce block reward by 50%
    (var-set current-block-reward (/ (var-get current-block-reward) u2))
    ;; Update last halving block
    (var-set last-halving-block block-height)
    true))

;; Process vesting for each allocation
(define-private (process-treasury-vesting (months uint))
  (let (
    (initial-amount (/ (* TREASURY_ALLOCATION u20) u100)) ;; 20% at launch
    (vesting-amount (/ (* TREASURY_ALLOCATION u80) u100)) ;; 80% vesting
    (vesting-months u48) ;; 48 month vesting
    (vested-amount (if (>= months vesting-months)
                       vesting-amount
                       (/ (* vesting-amount months) vesting-months)))
    (total-vested (+ initial-amount vested-amount))
    (to-release (- total-vested (var-get treasury-released)))
  )
    (if (> to-release u0)
        (begin
          (var-set treasury-released total-vested)
          (var-set total-minted (+ (var-get total-minted) to-release))
          true)
        true)))

(define-private (process-liquidity-vesting (months uint))
  (let (
    (initial-amount (/ (* LIQUIDITY_ALLOCATION u50) u100)) ;; 50% at launch
    (vesting-amount (/ (* LIQUIDITY_ALLOCATION u50) u100)) ;; 50% vesting
    (vesting-months u18) ;; 18 month vesting
    (vested-amount (if (>= months vesting-months)
                       vesting-amount
                       (/ (* vesting-amount months) vesting-months)))
    (total-vested (+ initial-amount vested-amount))
    (to-release (- total-vested (var-get liquidity-released)))
  )
    (if (> to-release u0)
        (begin
          (var-set liquidity-released total-vested)
          (var-set total-minted (+ (var-get total-minted) to-release))
          true)
        true)))

(define-private (process-team-vesting (months uint))
  (let (
    (initial-amount u0) ;; 0% at launch
    (vesting-amount TEAM_ALLOCATION) ;; 100% vesting
    (cliff-months u12) ;; 12 month cliff
    (vesting-months-after-cliff u36) ;; 36 month vesting after cliff
    (effective-months (if (>= months cliff-months)
                          (- months cliff-months)
                          u0))
    (vested-amount (if (>= effective-months vesting-months-after-cliff)
                       vesting-amount
                       (/ (* vesting-amount effective-months) vesting-months-after-cliff)))
    (total-vested (+ initial-amount vested-amount))
    (to-release (- total-vested (var-get team-released)))
  )
    (if (> to-release u0)
        (begin
          (var-set team-released total-vested)
          (var-set total-minted (+ (var-get total-minted) to-release))
          true)
        true)))

(define-private (process-community-vesting (months uint))
  (let (
    (initial-amount (/ (* COMMUNITY_ALLOCATION u10) u100)) ;; 10% at launch
    (vesting-amount (/ (* COMMUNITY_ALLOCATION u90) u100)) ;; 90% vesting
    (vesting-months u48) ;; 48 month vesting
    (vested-amount (if (>= months vesting-months)
                       vesting-amount
                       (/ (* vesting-amount months) vesting-months)))
    (total-vested (+ initial-amount vested-amount))
    (to-release (- total-vested (var-get community-released)))
  )
    (if (> to-release u0)
        (begin
          (var-set community-released total-vested)
          (var-set total-minted (+ (var-get total-minted) to-release))
          true)
        true)))

(define-private (process-partners-vesting (months uint))
  (let (
    (initial-amount (/ (* PARTNERS_ALLOCATION u10) u100)) ;; 10% at launch
    (vesting-amount (/ (* PARTNERS_ALLOCATION u90) u100)) ;; 90% vesting
    (vesting-months u36) ;; 36 month vesting
    (vested-amount (if (>= months vesting-months)
                       vesting-amount
                       (/ (* vesting-amount months) vesting-months)))
    (total-vested (+ initial-amount vested-amount))
    (to-release (- total-vested (var-get partners-released)))
  )
    (if (> to-release u0)
        (begin
          (var-set partners-released total-vested)
          (var-set total-minted (+ (var-get total-minted) to-release))
          true)
        true)))

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