;; Token Economics Contract
;; Contains the Bitcoin-style tokenomics implementation for the Anya protocol

;; =========================================
;; Imports
;; =========================================
(use-trait ft-trait .sip-010-trait.sip-010-trait)

;; =========================================
;; Constants & Token Supply Configuration
;; =========================================

;; Bitcoin-style tokenomics constants
(define-constant TOTAL_SUPPLY u21000000000) ;; 21 billion tokens
(define-constant INITIAL_BLOCK_REWARD u5000) ;; 5,000 tokens per block
(define-constant HALVING_INTERVAL u210000) ;; Halving every 210,000 blocks
(define-constant DEX_ALLOCATION_PERCENTAGE u30) ;; 30% to DEX
(define-constant TEAM_ALLOCATION_PERCENTAGE u15) ;; 15% to team
(define-constant DAO_ALLOCATION_PERCENTAGE u55) ;; 55% to DAO/community
(define-constant CONTRACT_OWNER tx-sender) ;; Initial contract deployer

;; Error codes
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_ALLOCATION u402)
(define-constant ERR_DISTRIBUTION_FAILED u403)

;; =========================================
;; Data Maps
;; =========================================

;; Track token distribution to different stakeholders
(define-map token-distribution
    { stakeholder: (string-ascii 24) }
    { amount: uint, last-distribution-height: uint }
)

;; Metrics tracking for system observability
(define-map economic-metrics
    { metric-name: (string-ascii 24) }
    { value: uint, last-updated-height: uint }
)

;; =========================================
;; Read-only Functions
;; =========================================

;; Get the total maximum supply of tokens
(define-read-only (get-total-supply)
    (ok TOTAL_SUPPLY)
)

;; Get the initial block reward amount
(define-read-only (get-initial-block-reward)
    (ok INITIAL_BLOCK_REWARD)
)

;; Get the interval between reward halvings
(define-read-only (get-halving-interval)
    (ok HALVING_INTERVAL)
)

;; Get the allocation percentages for different stakeholders
(define-read-only (get-allocation-percentages)
    (ok {
        dex: DEX_ALLOCATION_PERCENTAGE,
        team: TEAM_ALLOCATION_PERCENTAGE,
        dao: DAO_ALLOCATION_PERCENTAGE
    })
)

;; Calculate the current block reward based on the block height
(define-read-only (calculate-block-reward (block-height uint))
    (let 
        (
            (halvings (/ block-height HALVING_INTERVAL))
            (reward INITIAL_BLOCK_REWARD)
        )
        (if (>= halvings u64)
            (ok u0)
            (ok (/ reward (pow u2 halvings)))
        )
    )
)

;; Get the total tokens issued at a specific block height
(define-read-only (get-tokens-issued-at-height (block-height uint))
    (let
        (
            (full-halvings (/ block-height HALVING_INTERVAL))
            (remainder-blocks (mod block-height HALVING_INTERVAL))
        )
        (ok (+
            (fold + u0 
                (map 
                    (lambda (halving-index uint)
                        (* 
                            (/ INITIAL_BLOCK_REWARD (pow u2 halving-index))
                            HALVING_INTERVAL
                        )
                    )
                    (list u0 u1 u2 u3 u4 u5 u6 u7 u8)
                )
            )
            (* 
                (unwrap-panic (calculate-block-reward (* full-halvings HALVING_INTERVAL)))
                remainder-blocks
            )
        ))
    )
)

;; Get distribution metrics for a specific stakeholder
(define-read-only (get-distribution-info (stakeholder (string-ascii 24)))
    (match (map-get? token-distribution { stakeholder: stakeholder })
        success (ok success)
        (ok { amount: u0, last-distribution-height: u0 })
    )
)

;; =========================================
;; Verification Functions
;; =========================================

;; Verify that token allocations add up to 100%
(define-read-only (verify-allocation-percentages)
    (let 
        (
            (total-percentage (+ (+ DEX_ALLOCATION_PERCENTAGE TEAM_ALLOCATION_PERCENTAGE) DAO_ALLOCATION_PERCENTAGE))
        )
        (ok (is-eq total-percentage u100))
    )
)

;; Verify current token issuance is within defined limits
(define-read-only (verify-issuance (current-supply uint))
    (ok (<= current-supply TOTAL_SUPPLY))
)

;; =========================================
;; Public Functions
;; =========================================

;; Record a token distribution to a specific stakeholder
(define-public (record-distribution (stakeholder (string-ascii 24)) (amount uint))
    (begin
        (asserts! (is-eq tx-sender CONTRACT_OWNER) (err ERR_UNAUTHORIZED))
        
        (let 
            (
                (previous-info (default-to { amount: u0, last-distribution-height: u0 } 
                                (map-get? token-distribution { stakeholder: stakeholder })))
                (new-amount (+ (get amount previous-info) amount))
            )
            (map-set token-distribution
                { stakeholder: stakeholder }
                { 
                    amount: new-amount, 
                    last-distribution-height: block-height 
                }
            )
            
            (map-set economic-metrics
                { metric-name: "total-distributed" }
                {
                    value: (+ amount (default-to u0 (get value (map-get? economic-metrics { metric-name: "total-distributed" })))),
                    last-updated-height: block-height
                }
            )
            
            (ok new-amount)
        )
    )
)

;; Update a system metric
(define-public (update-metric (metric-name (string-ascii 24)) (value uint))
    (begin
        (asserts! (is-eq tx-sender CONTRACT_OWNER) (err ERR_UNAUTHORIZED))
        
        (map-set economic-metrics
            { metric-name: metric-name }
            { value: value, last-updated-height: block-height }
        )
        
        (ok true)
    )
)

;; =========================================
;; Helper Functions
;; =========================================

;; Simple integer power function for halving calculations
(define-private (pow (base uint) (exp uint))
    (fold * u1 (list-repeat exp base))
)

;; Create a list with the same value repeated n times
(define-private (list-repeat (n uint) (val uint))
    (if (<= n u0)
        (list)
        (unwrap-panic (as-max-len? (append (list-repeat (- n u1) val) val) u64))
    )
)