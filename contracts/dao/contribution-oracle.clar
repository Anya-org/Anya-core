;; Contribution Oracle Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;; 
;; This contract serves as an oracle that bridges off-chain contribution data to the on-chain reward system.
;; It securely receives contribution data from trusted sources and makes it available to the reward-controller.

(use-trait reward-trait .reward-controller.reward-controller-trait)

;; Constants for access control
(define-constant CONTRACT_OWNER 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant ERR_UNAUTHORIZED (err u401))
(define-constant ERR_INVALID_PERIOD (err u402))
(define-constant ERR_ALREADY_PROCESSED (err u403))
(define-constant ERR_INVALID_CONTRIBUTOR (err u404))

;; Data structures for contribution tracking
(define-map contributor-points 
  { contributor: principal, period: (string-ascii 20) } 
  { points: uint, timestamp: uint }
)

(define-map processed-periods 
  { period: (string-ascii 20) }
  { processed: bool, timestamp: uint, total-points: uint }
)

(define-map authorized-oracles principal bool)

;; Initialize authorized oracles
(map-set authorized-oracles CONTRACT_OWNER true)

;; Public functions

;; Submit contribution data for a specific period
(define-public (submit-contributions 
    (period (string-ascii 20))
    (contributors (list 200 { contributor: principal, points: uint }))
  )
  (begin
    ;; Check if caller is authorized
    (asserts! (default-to false (map-get? authorized-oracles tx-sender)) ERR_UNAUTHORIZED)
    
    ;; Check if period has already been processed
    (asserts! (is-none (map-get? processed-periods { period: period })) ERR_ALREADY_PROCESSED)
    
    ;; Process each contributor's points
    (map record-contribution-points 
      (map 
        (lambda (entry)
          (tuple 
            (period period) 
            (contributor (get contributor entry)) 
            (points (get points entry))
          )
        )
        contributors
      )
    )
    
    ;; Mark period as processed and record total points
    (map-set processed-periods 
      { period: period }
      { 
        processed: true, 
        timestamp: block-height, 
        total-points: (fold + (map get-points contributors) u0)
      }
    )
    
    ;; Return success with period and number of contributors
    (ok (tuple (period period) (contributors-count (len contributors))))
  )
)

;; Helper function to get points from a contributor entry
(define-private (get-points (entry { contributor: principal, points: uint }))
  (get points entry)
)

;; Helper function to record contribution points for a single contributor
(define-private (record-contribution-points (data { period: (string-ascii 20), contributor: principal, points: uint }))
  (begin
    (map-set contributor-points
      { contributor: (get contributor data), period: (get period data) }
      { points: (get points data), timestamp: block-height }
    )
    true
  )
)

;; Read functions

;; Get contribution points for a specific contributor and period
(define-read-only (get-contributor-points (contributor principal) (period (string-ascii 20)))
  (default-to { points: u0, timestamp: u0 }
    (map-get? contributor-points { contributor: contributor, period: period })
  )
)

;; Get information about a processed period
(define-read-only (get-period-info (period (string-ascii 20)))
  (default-to { processed: false, timestamp: u0, total-points: u0 }
    (map-get? processed-periods { period: period })
  )
)

;; Check if an address is an authorized oracle
(define-read-only (is-authorized-oracle (address principal))
  (default-to false (map-get? authorized-oracles address))
)

;; Admin functions

;; Add a new authorized oracle
(define-public (add-oracle (oracle principal))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (map-set authorized-oracles oracle true)
    (ok true)
  )
)

;; Remove an authorized oracle
(define-public (remove-oracle (oracle principal))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (map-delete authorized-oracles oracle)
    (ok true)
  )
)
