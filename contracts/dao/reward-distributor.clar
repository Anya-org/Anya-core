;; Reward Distributor Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;;
;; This contract handles the actual distribution of rewards to contributors
;; based on the calculations from the reward-controller.

;; Import required traits
(use-trait ft-trait .token.ft-token-trait)
(use-trait reward-trait .reward-controller.reward-controller-trait)

;; Constants
(define-constant CONTRACT_OWNER 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant ORACLE_CONTRACT .contribution-oracle)
(define-constant REWARD_CONTRACT .reward-controller)
(define-constant TOKEN_CONTRACT .token)

;; Error codes
(define-constant ERR_UNAUTHORIZED (err u401))
(define-constant ERR_INVALID_PERIOD (err u402))
(define-constant ERR_PERIOD_NOT_REWARDED (err u403))
(define-constant ERR_ALREADY_CLAIMED (err u404))
(define-constant ERR_ZERO_REWARD (err u405))
(define-constant ERR_TOKEN_TRANSFER (err u406))

;; Data structures
(define-map reward-claims
  { contributor: principal, period: (string-ascii 20) }
  { claimed: bool, amount: uint, timestamp: uint }
)

(define-map authorized-distributors principal bool)

;; Initialize authorized distributors
(map-set authorized-distributors CONTRACT_OWNER true)

;; Public functions

;; Distribute rewards for a specific period to a list of contributors
(define-public (distribute-rewards 
    (token-contract <ft-trait>) 
    (period (string-ascii 20))
    (contributors (list 200 principal))
  )
  (let (
    ;; Get period info from reward controller
    (period-info (unwrap! (contract-call? REWARD_CONTRACT get-rewarded-period-info period) ERR_INVALID_PERIOD))
    ;; Check if period has been rewarded
    (period-rewarded (get rewarded period-info))
  )
    ;; Verify conditions
    (asserts! (is-authorized-distributor tx-sender) ERR_UNAUTHORIZED)
    (asserts! period-rewarded ERR_PERIOD_NOT_REWARDED)
    
    ;; Distribute rewards to each contributor
    (map (process-contributor-reward token-contract period) contributors)
    
    ;; Return success
    (ok (tuple 
      (period period) 
      (contributors-count (len contributors))))
  )
)

;; Process reward for a single contributor
(define-private (process-contributor-reward (token-contract <ft-trait>) (period (string-ascii 20)) (contributor principal))
  (let (
    ;; Check if reward was already claimed
    (claim-info (default-to 
      { claimed: false, amount: u0, timestamp: u0 } 
      (map-get? reward-claims { contributor: contributor, period: period })))
    (already-claimed (get claimed claim-info))
    ;; Calculate reward amount
    (reward-amount (contract-call? REWARD_CONTRACT calculate-contributor-reward contributor period))
  )
    (if (and (not already-claimed) (> reward-amount u0))
      (begin
        ;; Record reward claim
        (map-set reward-claims
          { contributor: contributor, period: period }
          { claimed: true, amount: reward-amount, timestamp: block-height }
        )
        
        ;; Transfer tokens to contributor
        (unwrap! (contract-call? token-contract transfer 
          reward-amount 
          CONTRACT_OWNER
          contributor 
          none) 
          reward-amount)
      )
      reward-amount
    )
  )
)

;; Allow a contributor to claim their own rewards
(define-public (claim-reward 
    (token-contract <ft-trait>) 
    (period (string-ascii 20))
  )
  (let (
    ;; Check if reward was already claimed
    (claim-info (default-to 
      { claimed: false, amount: u0, timestamp: u0 } 
      (map-get? reward-claims { contributor: tx-sender, period: period })))
    (already-claimed (get claimed claim-info))
    ;; Get period info from reward controller
    (period-info (unwrap! (contract-call? REWARD_CONTRACT get-rewarded-period-info period) ERR_INVALID_PERIOD))
    ;; Check if period has been rewarded
    (period-rewarded (get rewarded period-info))
    ;; Calculate reward amount
    (reward-amount (contract-call? REWARD_CONTRACT calculate-contributor-reward tx-sender period))
  )
    ;; Verify conditions
    (asserts! period-rewarded ERR_PERIOD_NOT_REWARDED)
    (asserts! (not already-claimed) ERR_ALREADY_CLAIMED)
    (asserts! (> reward-amount u0) ERR_ZERO_REWARD)
    
    ;; Record reward claim
    (map-set reward-claims
      { contributor: tx-sender, period: period }
      { claimed: true, amount: reward-amount, timestamp: block-height }
    )
    
    ;; Transfer tokens to contributor
    (unwrap! (contract-call? token-contract transfer 
      reward-amount 
      CONTRACT_OWNER
      tx-sender 
      none) 
      ERR_TOKEN_TRANSFER)
      
    ;; Return success with amount
    (ok reward-amount)
  )
)

;; Read functions

;; Check if a contributor has claimed rewards for a period
(define-read-only (get-claim-status (contributor principal) (period (string-ascii 20)))
  (default-to 
    { claimed: false, amount: u0, timestamp: u0 }
    (map-get? reward-claims { contributor: contributor, period: period })
  )
)

;; Check if an address is an authorized distributor
(define-read-only (is-authorized-distributor (address principal))
  (default-to false (map-get? authorized-distributors address))
)

;; Admin functions

;; Add a new authorized distributor
(define-public (add-distributor (distributor principal))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (map-set authorized-distributors distributor true)
    (ok true)
  )
)

;; Remove an authorized distributor
(define-public (remove-distributor (distributor principal))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (map-delete authorized-distributors distributor)
    (ok true)
  )
)
