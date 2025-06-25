;; Reward Controller Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;;
;; This contract implements the on-chain reward management system based on Bitcoin-style tokenomics.
;; It interacts with the contribution-oracle to calculate and distribute rewards.

;; Import token trait for integration
(use-trait ft-trait .token.ft-token-trait)

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
(define-constant CONTRACT_OWNER 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant ORACLE_CONTRACT .contribution-oracle)
(define-constant TOKEN_CONTRACT .token)
(define-constant COMMUNITY_CONTRACT 'ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC)

;; Error codes
(define-constant ERR_UNAUTHORIZED (err u401))
(define-constant ERR_INVALID_PERIOD (err u402))
(define-constant ERR_ALREADY_REWARDED (err u403))
(define-constant ERR_NOT_PROCESSED (err u404))
(define-constant ERR_TOKEN_TRANSFER (err u405))
(define-constant ERR_ZERO_POINTS (err u406))

;; Bitcoin-style tokenomics constants
(define-constant MAX_SUPPLY u21000000000000000) ;; 21B with 8 decimals
(define-constant HALVING_INTERVAL u210000)
(define-constant INITIAL_BLOCK_REWARD u1000000000) ;; 10,000 tokens per block
(define-constant COMMUNITY_PERCENTAGE u15) ;; 15% to Community Incentives

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
    timestamp: uint
  }
)

;; Public functions

;; Calculate and distribute rewards for a given period
(define-public (process-period (period (string-ascii 20)))
  (let (
    ;; Get period info from oracle
    (period-info (unwrap! (contract-call? ORACLE_CONTRACT get-period-info period) ERR_INVALID_PERIOD))
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
    (asserts! period-processed ERR_NOT_PROCESSED)
    (asserts! (not period-rewarded) ERR_ALREADY_REWARDED)
    (asserts! (> total-points u0) ERR_ZERO_POINTS)
    
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
    (halvings (/ current-block-val HALVING_INTERVAL))
  )
    ;; Check if we've reached the maximum halvings (64)
    (if (>= halvings u64)
      u0
      ;; Divide the initial block reward by 2^halvings
      (/ INITIAL_BLOCK_REWARD (pow u2 halvings))
    )
  )
)

;; Calculate total mined supply until current block
(define-read-only (calculate-total-mined-supply)
  (let (
    (current-block-val (var-get current-block))
    (reward INITIAL_BLOCK_REWARD)
    (supply u0)
    (blocks-per-era HALVING_INTERVAL)
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
    (blocks-in-era (if (> (* (+ era u1) HALVING_INTERVAL) current-block-val)
      (- current-block-val (* era HALVING_INTERVAL))
      HALVING_INTERVAL))
    (era-reward (/ INITIAL_BLOCK_REWARD (pow u2 era)))
  )
    (if (> (* era HALVING_INTERVAL) current-block-val)
      current-supply
      (+ current-supply (* blocks-in-era era-reward))
    )
  )
)

;; Calculate community incentive allocation (15% of total)
(define-read-only (calculate-community-incentive)
  (let (
    (total-mined (calculate-total-mined-supply))
  )
    (/ (* total-mined COMMUNITY_PERCENTAGE) u100)
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
    (period-info (unwrap! (contract-call? ORACLE_CONTRACT get-period-info period) ERR_INVALID_PERIOD))
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

;; Admin functions

;; Set current block for testing and simulation
(define-public (set-current-block (block uint))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
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

;; Get current block
(define-read-only (get-current-block)
  (var-get current-block)
)

;; Get total community rewards distributed
(define-read-only (get-total-community-rewards-distributed)
  (var-get total-community-rewards-distributed)
)
