;; Reward Scheduler Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;;
;; This contract handles the automatic scheduling of reward distributions
;; based on predefined periods and integrates with blockchain events.
;; Enhanced with multi-chain bridge functionality for SIP-010, SRC-20, and tBTC.

;; Import required traits
(use-trait ft-trait .token.ft-token-trait)
(use-trait reward-trait .reward-controller.reward-controller-trait)
(use-trait bridge-trait .cross-chain-bridge.bridge-trait)

;; Constants
(define-constant CONTRACT_OWNER 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant ORACLE_CONTRACT .contribution-oracle)
(define-constant REWARD_CONTRACT .reward-controller)
(define-constant DISTRIBUTOR_CONTRACT .reward-distributor)
(define-constant TOKEN_CONTRACT .token)
(define-constant BRIDGE_CONTRACT .cross-chain-bridge)
(define-constant TREASURY_CONTRACT .treasury-management)

;; Error codes
(define-constant ERR_UNAUTHORIZED (err u401))
(define-constant ERR_INVALID_PERIOD (err u402))
(define-constant ERR_PERIOD_NOT_DUE (err u403))
(define-constant ERR_ALREADY_PROCESSED (err u404))
(define-constant ERR_INVALID_SCHEDULE (err u405))
(define-constant ERR_BRIDGE_DISABLED (err u406))
(define-constant ERR_INSUFFICIENT_AMOUNT (err u407))
(define-constant ERR_BRIDGE_FAILED (err u408))

;; Cross-chain bridge fee configuration
;; Using standardized 5% fee across all bridges for simplicity and profitability
(define-data-var bridge-fee-rate uint u50000) ;; 5% (denominator is 1,000,000)
(define-data-var bridge-fee-treasury-share uint u800000) ;; 80% of fees to treasury
(define-data-var bridge-fee-community-share uint u200000) ;; 20% of fees to community incentives

;; Bridge minimum amounts (in smallest units)
(define-data-var stacks-to-bitcoin-min uint u100000000) ;; 1000 tokens (with 8 decimals)
(define-data-var stacks-to-ethereum-min uint u50000000) ;; 500 tokens (with 8 decimals)
(define-data-var bitcoin-to-stacks-min uint u100000000) ;; 1000 tokens (with 8 decimals)
(define-data-var ethereum-to-stacks-min uint u50000000) ;; 500 tokens (with 8 decimals)

;; Bridge status flags
(define-map bridge-status 
  { bridge-id: (string-ascii 20) }
  { enabled: bool }
)

;; Bridge records for auditing
(define-map bridge-records
  { tx-id: (buff 32) }
  {
    sender: principal,
    recipient: (buff 33),
    amount: uint,
    fee: uint,
    timestamp: uint,
    bridge-type: (string-ascii 20),
    status: (string-ascii 10)
  }
)

;; Initialize bridge status
(map-set bridge-status { bridge-id: "stacks-to-bitcoin" } { enabled: true })
(map-set bridge-status { bridge-id: "stacks-to-ethereum" } { enabled: true })
(map-set bridge-status { bridge-id: "bitcoin-to-stacks" } { enabled: true })
(map-set bridge-status { bridge-id: "ethereum-to-stacks" } { enabled: true })

;; Scheduling parameters
(define-data-var period-interval uint u4320) ;; Default to ~30 days in blocks (assuming ~10 min blocks)
(define-data-var next-scheduled-height uint u0)
(define-data-var auto-schedule-enabled bool true)

;; Period tracking
(define-map scheduled-periods
  { period: (string-ascii 20) }
  { 
    scheduled-height: uint, 
    processed: bool, 
    processing-height: uint,
    status: (string-ascii 20)
  }
)

;; Define a map for period definitions
(define-map period-definitions
  { period: (string-ascii 20) }
  { 
    start-date: (string-ascii 20), 
    end-date: (string-ascii 20),
    type: (string-ascii 10)
  }
)

;; Public functions

;; Schedule the next reward period based on current block height and interval
(define-public (schedule-next-period (period (string-ascii 20)) (scheduled-height uint))
  (begin
    ;; Only contract owner or authorized parties can schedule
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    
    ;; Validate schedule height is in the future
    (asserts! (> scheduled-height block-height) ERR_INVALID_SCHEDULE)
    
    ;; Check if period is already scheduled
    (asserts! (is-none (map-get? scheduled-periods { period: period })) ERR_ALREADY_PROCESSED)
    
    ;; Schedule the period
    (map-set scheduled-periods
      { period: period }
      { 
        scheduled-height: scheduled-height, 
        processed: false, 
        processing-height: u0,
        status: "scheduled"
      }
    )
    
    ;; Update next scheduled height if this is sooner
    (if (or 
          (is-eq (var-get next-scheduled-height) u0)
          (< scheduled-height (var-get next-scheduled-height)))
      (var-set next-scheduled-height scheduled-height)
      true
    )
    
    ;; Return success
    (ok (tuple 
      (period period) 
      (scheduled-height scheduled-height)))
  )
)

;; Define a period with metadata
(define-public (define-period (period (string-ascii 20)) (start-date (string-ascii 20)) (end-date (string-ascii 20)) (period-type (string-ascii 10)))
  (begin
    ;; Only contract owner can define periods
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    
    ;; Define the period
    (map-set period-definitions
      { period: period }
      { 
        start-date: start-date, 
        end-date: end-date,
        type: period-type
      }
    )
    
    ;; Return success
    (ok (tuple 
      (period period) 
      (start-date start-date)
      (end-date end-date)
      (type period-type)))
  )
)

;; Execute scheduled reward distribution if due
(define-public (execute-scheduled-rewards (period (string-ascii 20)))
  (let (
    ;; Get schedule info for the period
    (schedule-info (unwrap! (map-get? scheduled-periods { period: period }) ERR_INVALID_PERIOD))
    ;; Check if scheduled height has been reached
    (is-due (>= block-height (get scheduled-height schedule-info)))
    ;; Check if already processed
    (is-processed (get processed schedule-info))
  )
    ;; Verify conditions
    (asserts! is-due ERR_PERIOD_NOT_DUE)
    (asserts! (not is-processed) ERR_ALREADY_PROCESSED)
    
    ;; Update period status to processing
    (map-set scheduled-periods
      { period: period }
      (merge schedule-info 
        { 
          processing-height: block-height,
          status: "processing"
        }
      )
    )
    
    ;; Process rewards via reward controller
    (match (contract-call? REWARD_CONTRACT process-period period)
      success-result 
        (begin
          ;; Update period status to processed
          (map-set scheduled-periods
            { period: period }
            (merge schedule-info 
              { 
                processed: true,
                processing-height: block-height,
                status: "processed"
              }
            )
          )
          
          ;; Return success
          (ok success-result)
        )
      error (begin
        ;; Update period status to failed
        (map-set scheduled-periods
          { period: period }
          (merge schedule-info 
            { 
              processing-height: block-height,
              status: "failed"
            }
          )
        )
        (err error)
      ))
    )
  )
)

;; Admin functions

;; Set the period interval for auto-scheduling
(define-public (set-period-interval (interval uint))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (var-set period-interval interval)
    (ok interval)
  )
)

;; Toggle automatic scheduling
(define-public (toggle-auto-schedule)
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (var-set auto-schedule-enabled (not (var-get auto-schedule-enabled)))
    (ok (var-get auto-schedule-enabled))
  )
)

;; Update bridge fee rate
(define-public (set-bridge-fee-rate (new-fee-rate uint))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    ;; Ensure fee rate is reasonable (max 20%)
    (asserts! (<= new-fee-rate u200000) ERR_INVALID_PARAMETER)
    (var-set bridge-fee-rate new-fee-rate)
    (ok new-fee-rate)
  )
)

;; Update fee distribution between treasury and community
(define-public (set-fee-distribution (treasury-share uint) (community-share uint))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    ;; Ensure shares add up to 100%
    (asserts! (is-eq (+ treasury-share community-share) u1000000) ERR_INVALID_PARAMETER)
    (var-set bridge-fee-treasury-share treasury-share)
    (var-set bridge-fee-community-share community-share)
    (ok {treasury: treasury-share, community: community-share})
  )
)

;; Set bridge status (enable/disable)
(define-public (set-bridge-status (bridge-id (string-ascii 20)) (enabled bool))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (map-set bridge-status {bridge-id: bridge-id} {enabled: enabled})
    (ok enabled)
  )
)

;; Set minimum bridge amounts
(define-public (set-min-bridge-amount (bridge-id (string-ascii 20)) (min-amount uint))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    
    ;; Set the appropriate minimum based on bridge ID
    (match bridge-id
      "stacks-to-bitcoin" (var-set stacks-to-bitcoin-min min-amount)
      "stacks-to-ethereum" (var-set stacks-to-ethereum-min min-amount)
      "bitcoin-to-stacks" (var-set bitcoin-to-stacks-min min-amount)
      "ethereum-to-stacks" (var-set ethereum-to-stacks-min min-amount)
      (err ERR_INVALID_PARAMETER)
    )
    
    (ok min-amount)
  )
)

;; Read functions

;; Get information about a scheduled period
(define-read-only (get-scheduled-period (period (string-ascii 20)))
  (map-get? scheduled-periods { period: period })
)

;; Get period definition information
(define-read-only (get-period-definition (period (string-ascii 20)))
  (map-get? period-definitions { period: period })
)

;; Get current scheduling parameters
(define-read-only (get-scheduler-info)
  (tuple 
    (period-interval (var-get period-interval))
    (next-scheduled-height (var-get next-scheduled-height))
    (auto-schedule-enabled (var-get auto-schedule-enabled)))
)

;; Get current bridge fee configuration
(define-read-only (get-bridge-fee-config)
  (tuple 
    (fee-rate (var-get bridge-fee-rate))
    (treasury-share (var-get bridge-fee-treasury-share))
    (community-share (var-get bridge-fee-community-share)))
)

;; Get bridge minimum amounts
(define-read-only (get-bridge-minimum-amounts)
  (tuple 
    (stacks-to-bitcoin (var-get stacks-to-bitcoin-min))
    (stacks-to-ethereum (var-get stacks-to-ethereum-min))
    (bitcoin-to-stacks (var-get bitcoin-to-stacks-min))
    (ethereum-to-stacks (var-get ethereum-to-stacks-min)))
)

;; Get bridge status
(define-read-only (get-bridge-status (bridge-id (string-ascii 20)))
  (default-to { enabled: false } (map-get? bridge-status {bridge-id: bridge-id}))
)

;; Get bridge transaction record
(define-read-only (get-bridge-record (tx-id (buff 32)))
  (map-get? bridge-records {tx-id: tx-id})
)

;; Calculate fee for a given amount
(define-read-only (calculate-bridge-fee (amount uint))
  (let (
    (fee-amount (/ (* amount (var-get bridge-fee-rate)) u1000000))
    (net-amount (- amount fee-amount))
    (treasury-fee (/ (* fee-amount (var-get bridge-fee-treasury-share)) u1000000))
    (community-fee (/ (* fee-amount (var-get bridge-fee-community-share)) u1000000))
  )
    (tuple 
      (gross-amount amount)
      (fee fee-amount)
      (net-amount net-amount)
      (treasury-fee treasury-fee)
      (community-fee community-fee))
  )
)

;; Bridge functions for cross-chain reward distribution

;; Bridge tokens from Stacks to Bitcoin (SIP-010 to SRC-20)
(define-public (bridge-to-bitcoin 
    (token-contract <ft-trait>) 
    (amount uint) 
    (bitcoin-recipient (buff 33))
  )
  (let (
    ;; Check if bridge is enabled
    (bridge-enabled (default-to { enabled: false } (map-get? bridge-status { bridge-id: "stacks-to-bitcoin" })))
    ;; Calculate fee amount (5% of amount)
    (fee-amount (/ (* amount (var-get bridge-fee-rate)) u1000000))
    ;; Amount after fee
    (net-amount (- amount fee-amount))
    ;; Calculate treasury share of fee
    (treasury-fee (/ (* fee-amount (var-get bridge-fee-treasury-share)) u1000000))
    ;; Calculate community share of fee
    (community-fee (/ (* fee-amount (var-get bridge-fee-community-share)) u1000000))
  )
    ;; Verify conditions
    (asserts! (get enabled bridge-enabled) ERR_BRIDGE_DISABLED)
    (asserts! (>= amount (var-get stacks-to-bitcoin-min)) ERR_INSUFFICIENT_AMOUNT)
    
    ;; Transfer tokens from sender to contract first
    (unwrap! (contract-call? token-contract transfer amount tx-sender (as-contract tx-sender) none) ERR_BRIDGE_FAILED)
    
    ;; Distribute fee
    (unwrap! (as-contract (contract-call? token-contract transfer treasury-fee tx-sender TREASURY_CONTRACT none)) ERR_BRIDGE_FAILED)
    (unwrap! (as-contract (contract-call? token-contract transfer community-fee tx-sender COMMUNITY_PRINCIPAL none)) ERR_BRIDGE_FAILED)
    
    ;; Record the bridge transaction
    (map-set bridge-records 
      { tx-id: (tx-hash) }
      {
        sender: tx-sender,
        recipient: bitcoin-recipient,
        amount: amount,
        fee: fee-amount,
        timestamp: block-height,
        bridge-type: "stacks-to-bitcoin",
        status: "pending"
      }
    )
    
    ;; Call the bridge contract
    (contract-call? BRIDGE_CONTRACT initiate-bitcoin-transfer net-amount bitcoin-recipient)
  )
)

;; Bridge tokens from Stacks to Ethereum (SIP-010 to tBTC)
(define-public (bridge-to-ethereum 
    (token-contract <ft-trait>) 
    (amount uint) 
    (ethereum-recipient (buff 20))
  )
  (let (
    ;; Check if bridge is enabled
    (bridge-enabled (default-to { enabled: false } (map-get? bridge-status { bridge-id: "stacks-to-ethereum" })))
    ;; Calculate fee amount (5% of amount)
    (fee-amount (/ (* amount (var-get bridge-fee-rate)) u1000000))
    ;; Amount after fee
    (net-amount (- amount fee-amount))
    ;; Calculate treasury share of fee
    (treasury-fee (/ (* fee-amount (var-get bridge-fee-treasury-share)) u1000000))
    ;; Calculate community share of fee
    (community-fee (/ (* fee-amount (var-get bridge-fee-community-share)) u1000000))
  )
    ;; Verify conditions
    (asserts! (get enabled bridge-enabled) ERR_BRIDGE_DISABLED)
    (asserts! (>= amount (var-get stacks-to-ethereum-min)) ERR_INSUFFICIENT_AMOUNT)
    
    ;; Transfer tokens from sender to contract first
    (unwrap! (contract-call? token-contract transfer amount tx-sender (as-contract tx-sender) none) ERR_BRIDGE_FAILED)
    
    ;; Distribute fee
    (unwrap! (as-contract (contract-call? token-contract transfer treasury-fee tx-sender TREASURY_CONTRACT none)) ERR_BRIDGE_FAILED)
    (unwrap! (as-contract (contract-call? token-contract transfer community-fee tx-sender COMMUNITY_PRINCIPAL none)) ERR_BRIDGE_FAILED)
    
    ;; Record the bridge transaction
    (map-set bridge-records 
      { tx-id: (tx-hash) }
      {
        sender: tx-sender,
        recipient: ethereum-recipient,
        amount: amount,
        fee: fee-amount,
        timestamp: block-height,
        bridge-type: "stacks-to-ethereum",
        status: "pending"
      }
    )
    
    ;; Call the bridge contract
    (contract-call? BRIDGE_CONTRACT initiate-ethereum-transfer net-amount ethereum-recipient)
  )
)
