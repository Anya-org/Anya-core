;; Decentralized Reward Controller Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;;
;; This contract implements the on-chain reward management system based on Bitcoin-style tokenomics.
;; It interacts with the decentralized oracle system to calculate and distribute rewards.

;; Import traits and shared libraries
(use-trait ft-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)
(use-trait governance-trait .governance-traits.governance-trait)
(use-trait multi-sig-trait .governance-traits.multi-sig-trait)

;; Import shared constants
(use-contract dao-constants .shared.dao-constants)

;; Define trait for this contract
(define-trait reward-controller-trait
  (
    ;; Process rewards for a specific period
    (process-period ((string-ascii 20)) (response (tuple (period (string-ascii 20)) (success bool) (rewarded-contributors uint)) uint))
    ;; Get reward calculation info for a period
    (get-period-reward-info ((string-ascii 20)) (response (tuple (period (string-ascii 20)) (total-points uint) (reward-per-point uint) (total-reward uint)) uint))
  )
)

;; Constants
(define-constant GOVERNANCE_CONTRACT .multi-sig-governance)
(define-constant ORACLE_CONTRACT .decentralized-contribution-oracle)
(define-constant TOKEN_CONTRACT .token)
(define-constant COMMUNITY_CONTRACT 'ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC)

;; Data structures
(define-data-var current-block uint u1)
(define-data-var total-community-rewards-distributed uint u0)

;; Track rewarded periods to avoid double payouts
(define-map rewarded-periods 
  { period: (string-ascii 20) }
  { 
    rewarded: bool, 
    timestamp: uint, 
    total-points: uint,
    reward-per-point: uint,
    total-reward: uint
  }
)

;; Track rewards by contributor
(define-map contributor-rewards 
  { contributor: principal, period: (string-ascii 20) }
  { 
    points: uint, 
    reward-amount: uint,
    timestamp: uint,
    claimed: bool
  }
)

;; Public functions

;; Calculate and distribute rewards for a given period
(define-public (process-period (period (string-ascii 20)))
  (let (
    ;; Get period info from oracle
    (period-info (unwrap! (contract-call? ORACLE_CONTRACT get-period-info period) (contract-call? dao-constants get-error-invalid-parameter)))
    ;; Check if period has been processed by the oracle
    (period-processed (get processed period-info))
    ;; Get total points for the period
    (total-points (get total-points period-info))
    ;; Check if period has already been rewarded
    (period-rewarded (default-to false (get rewarded (map-get? rewarded-periods { period: period }))))
    ;; Calculate reward per point
    (reward-per-point (calculate-reward-per-point total-points))
    ;; Calculate total reward for this period
    (total-reward (* reward-per-point total-points))
  )
    ;; Verify conditions
    (asserts! period-processed (contract-call? dao-constants get-error-not-found))
    (asserts! (not period-rewarded) (contract-call? dao-constants get-error-already-exists))
    (asserts! (> total-points u0) (contract-call? dao-constants get-error-invalid-parameter))
    
    ;; Mark period as rewarded and store reward data
    (map-set rewarded-periods 
      { period: period }
      { 
        rewarded: true, 
        timestamp: block-height, 
        total-points: total-points,
        reward-per-point: reward-per-point,
        total-reward: total-reward
      }
    )
    
    ;; Increment the block counter (simulating blockchain advancement)
    (var-set current-block (+ (var-get current-block) u1))
    
    ;; Update total community rewards distributed
    (var-set total-community-rewards-distributed 
      (+ (var-get total-community-rewards-distributed) total-reward)
    )
    
    ;; Return success data
    (ok (tuple 
      (period period) 
      (success true) 
      (rewarded-contributors total-points)))
  )
)

;; Calculate current block reward based on Bitcoin-style halving
(define-read-only (calculate-block-reward)
  (let (
    (current-block-val (var-get current-block))
    (halvings (/ current-block-val (contract-call? dao-constants get-halving-interval)))
  )
    ;; Check if we've reached the maximum halvings (64)
    (if (>= halvings u64)
      u0
      ;; Divide the initial block reward by 2^halvings
      (/ (contract-call? dao-constants get-initial-block-reward) (pow u2 halvings))
    )
  )
)

;; Calculate total mined supply until current block
(define-read-only (calculate-total-mined-supply)
  (let (
    (current-block-val (var-get current-block))
    (reward (contract-call? dao-constants get-initial-block-reward))
    (supply u0)
    (blocks-per-era (contract-call? dao-constants get-halving-interval))
    (remaining-blocks current-block-val)
    (era u0)
  )
    ;; Iteratively calculate supply through each halving era
    (fold calculate-era-supply 
      ;; Generate a list of eras based on the current block
      (list u0 u1 u2 u3 u4 u5 u6 u7 u8 u9 u10 u11 u12 u13 u14 u15)
      u0
    )
  )
)

;; Helper function to calculate supply for each halving era
(define-private (calculate-era-supply (era uint) (current-supply uint))
  (let (
    (current-block-val (var-get current-block))
    (halving-interval (contract-call? dao-constants get-halving-interval))
    (initial-reward (contract-call? dao-constants get-initial-block-reward))
    (blocks-in-era (if (> (* (+ era u1) halving-interval) current-block-val)
      (- current-block-val (* era halving-interval))
      halving-interval))
    (era-reward (/ initial-reward (pow u2 era)))
  )
    (if (> (* era halving-interval) current-block-val)
      current-supply
      (+ current-supply (* blocks-in-era era-reward))
    )
  )
)

;; Calculate community incentive allocation
(define-read-only (calculate-community-incentive)
  (let (
    (total-mined (calculate-total-mined-supply))
    (community-percentage (contract-call? dao-constants get-community-percentage))
  )
    (/ (* total-mined community-percentage) u100)
  )
)

;; Calculate reward per contribution point
(define-private (calculate-reward-per-point (total-points uint))
  (let (
    ;; Calculate maximum available community rewards
    (available-community-rewards (calculate-community-incentive))
    ;; Subtract already distributed rewards
    (remaining-rewards (- available-community-rewards (var-get total-community-rewards-distributed)))
    ;; Calculate reward per point (handle zero points case)
    (reward-per-point (if (> total-points u0)
      (/ remaining-rewards total-points)
      u0))
  )
    (if (> reward-per-point u0)
      reward-per-point
      u1) ;; Minimum reward of 1 token unit if calculation is zero
  )
)

;; Get reward calculation info for a period without processing rewards
(define-public (get-period-reward-info (period (string-ascii 20)))
  (let (
    ;; Get period info from oracle
    (period-info (unwrap! (contract-call? ORACLE_CONTRACT get-period-info period) (contract-call? dao-constants get-error-invalid-parameter)))
    ;; Check if period has been processed by the oracle
    (period-processed (get processed period-info))
    ;; Get total points for the period
    (total-points (get total-points period-info))
    ;; Calculate reward per point
    (reward-per-point (calculate-reward-per-point total-points))
    ;; Calculate total reward for this period
    (total-reward (* reward-per-point total-points))
  )
    (ok (tuple 
      (period period) 
      (total-points total-points) 
      (reward-per-point reward-per-point)
      (total-reward total-reward)))
  )
)

;; Calculate reward for a specific contributor in a given period
(define-read-only (calculate-contributor-reward (contributor principal) (period (string-ascii 20)))
  (let (
    ;; Get contributor points from oracle
    (contributor-info (contract-call? ORACLE_CONTRACT get-contributor-points contributor period))
    ;; Get period info if already rewarded
    (period-info (default-to 
      { 
        rewarded: false, 
        timestamp: u0, 
        total-points: u0,
        reward-per-point: u0,
        total-reward: u0
      }
      (map-get? rewarded-periods { period: period })))
    ;; Get points for this contributor
    (points (get points contributor-info))
    ;; Get reward-per-point from period info or calculate it
    (reward-per-point (if (get rewarded period-info)
      (get reward-per-point period-info)
      (calculate-reward-per-point (get total-points (contract-call? ORACLE_CONTRACT get-period-info period)))
    ))
  )
    (* points reward-per-point)
  )
)

;; Claim rewards for a contributor
(define-public (claim-rewards (period (string-ascii 20)) (token <ft-trait>))
  (let (
    ;; Get contributor reward info
    (contributor-info (contract-call? ORACLE_CONTRACT get-contributor-points tx-sender period))
    ;; Get period info
    (period-info (unwrap! (map-get? rewarded-periods { period: period }) (contract-call? dao-constants get-error-not-found)))
    ;; Get points
    (points (get points contributor-info))
    ;; Get reward per point
    (reward-per-point (get reward-per-point period-info))
    ;; Calculate reward amount
    (reward-amount (* points reward-per-point))
    ;; Get existing claim record
    (existing-claim (default-to
      {
        points: points,
        reward-amount: reward-amount,
        timestamp: u0,
        claimed: false
      }
      (map-get? contributor-rewards { contributor: tx-sender, period: period })
    ))
  )
    ;; Check if not already claimed
    (asserts! (not (get claimed existing-claim)) (contract-call? dao-constants get-error-already-exists))
    
    ;; Check if period has been rewarded
    (asserts! (get rewarded period-info) (contract-call? dao-constants get-error-not-found))
    
    ;; Check if has points
    (asserts! (> points u0) (contract-call? dao-constants get-error-invalid-parameter))
    
    ;; Update claim record
    (map-set contributor-rewards
      { contributor: tx-sender, period: period }
      (merge existing-claim
        {
          timestamp: block-height,
          claimed: true
        }
      )
    )
    
    ;; Transfer reward tokens to contributor
    (as-contract (contract-call? token transfer reward-amount tx-sender tx-sender none))
  )
)

;; Governance functions

;; Set current block for testing and simulation
(define-public (set-current-block (block uint))
  (let
    (
      (is-governor (contract-call? GOVERNANCE_CONTRACT is-valid-signer tx-sender))
    )
    
    ;; Check if caller is authorized governor
    (asserts! is-governor (contract-call? dao-constants get-error-unauthorized))
    
    ;; Update block
    (var-set current-block block)
    
    (ok block)
  )
)

;; Read functions

;; Get info about a rewarded period
(define-read-only (get-rewarded-period-info (period (string-ascii 20)))
  (default-to 
    { 
      rewarded: false, 
      timestamp: u0, 
      total-points: u0,
      reward-per-point: u0,
      total-reward: u0
    }
    (map-get? rewarded-periods { period: period })
  )
)

;; Get contributor reward info
(define-read-only (get-contributor-reward-info (contributor principal) (period (string-ascii 20)))
  (default-to 
    { 
      points: u0, 
      reward-amount: u0,
      timestamp: u0,
      claimed: false
    }
    (map-get? contributor-rewards { contributor: contributor, period: period })
  )
)

;; Get current block
(define-read-only (get-current-block)
  (var-get current-block)
)

;; Get total community rewards distributed
(define-read-only (get-total-community-rewards-distributed)
  (var-get total-community-rewards-distributed)
)
