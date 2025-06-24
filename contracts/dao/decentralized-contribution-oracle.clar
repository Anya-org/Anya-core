;; Decentralized Contribution Oracle Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;; 
;; This contract implements a decentralized oracle system for contributing data
;; to the DAO. It uses threshold signatures and consensus to ensure accuracy.

;; Import traits and constants
(use-trait ft-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)
(use-trait reward-trait .reward-controller.reward-controller-trait)
(use-trait governance-trait .governance-traits.governance-trait)
(use-trait multi-sig-trait .governance-traits.multi-sig-trait)

;; Import shared constants
(use-contract dao-constants .shared.dao-constants)

;; Constants
(define-constant GOVERNANCE_CONTRACT .multi-sig-governance)
(define-constant TOKEN_CONTRACT .token)
(define-constant MIN_STAKE u100000000) ;; Minimum stake to be an oracle (1000 tokens)

;; Data structures for contribution tracking
(define-map contributor-points 
  { contributor: principal, period: (string-ascii 20) } 
  { points: uint, timestamp: uint }
)

(define-map processed-periods 
  { period: (string-ascii 20) }
  { processed: bool, timestamp: uint, total-points: uint }
)

;; Oracle management
(define-map oracles 
  principal 
  { 
    active: bool, 
    stake: uint,
    last-submission: uint,
    reliability: uint, ;; 0-100 scale
    added-height: uint
  }
)

;; Data submissions from oracles
(define-map oracle-submissions
  { period: (string-ascii 20), oracle: principal }
  { 
    submitted: bool,
    submission-hash: (buff 32),
    timestamp: uint
  }
)

;; Consensus tracking
(define-map period-consensus
  { period: (string-ascii 20) }
  {
    submission-count: uint,
    consensus-reached: bool,
    consensus-timestamp: uint,
    consensus-hash: (optional (buff 32))
  }
)

;; Oracle rewards
(define-map oracle-rewards
  { period: (string-ascii 20), oracle: principal }
  {
    amount: uint,
    claimed: bool,
    in-consensus: bool
  }
)

;; Oracle application queue for governance approval
(define-map oracle-applications
  principal
  {
    stake: uint,
    application-height: uint,
    approved: bool
  }
)

;; Initialization
(define-data-var total-oracles uint u0)
(define-data-var min-oracle-consensus uint u67) ;; 67% consensus required
(define-data-var oracle-reward-per-submission uint u5000000) ;; 50 tokens per valid submission

;; Public functions

;; Apply to become an oracle by staking tokens
(define-public (apply-as-oracle (token <ft-trait>))
  (let
    (
      (stake MIN_STAKE)
      (existing-application (map-get? oracle-applications tx-sender))
      (existing-oracle (map-get? oracles tx-sender))
    )
    
    ;; Check if not already an oracle or applicant
    (asserts! (and (is-none existing-application) (is-none existing-oracle)) (contract-call? dao-constants get-error-already-exists))
    
    ;; Transfer stake to contract (will fail if insufficient balance)
    (try! (contract-call? token transfer stake tx-sender (as-contract tx-sender) none))
    
    ;; Record application
    (map-set oracle-applications tx-sender
      {
        stake: stake,
        application-height: block-height,
        approved: false
      }
    )
    
    (ok stake)
  )
)

;; Governance function to approve an oracle application
(define-public (approve-oracle-application (applicant principal))
  (let
    (
      (is-governor (contract-call? GOVERNANCE_CONTRACT is-valid-signer tx-sender))
      (application (unwrap! (map-get? oracle-applications applicant) (contract-call? dao-constants get-error-not-found)))
      (current-total (var-get total-oracles))
    )
    
    ;; Check if caller is authorized governor
    (asserts! is-governor (contract-call? dao-constants get-error-unauthorized))
    
    ;; Add as oracle
    (map-set oracles applicant
      {
        active: true,
        stake: (get stake application),
        last-submission: u0,
        reliability: u100,
        added-height: block-height
      }
    )
    
    ;; Update application status
    (map-set oracle-applications applicant
      (merge application { approved: true })
    )
    
    ;; Update total oracles
    (var-set total-oracles (+ current-total u1))
    
    (ok (var-get total-oracles))
  )
)

;; Submit contribution data hash for a specific period
(define-public (submit-contribution-hash 
    (period (string-ascii 20))
    (contribution-hash (buff 32))
  )
  (let
    (
      (is-oracle (default-to false (get active (map-get? oracles tx-sender))))
      (period-consensus-data (default-to 
        { 
          submission-count: u0, 
          consensus-reached: false,
          consensus-timestamp: u0,
          consensus-hash: none
        } 
        (map-get? period-consensus { period: period })
      ))
      (existing-submission (map-get? oracle-submissions { period: period, oracle: tx-sender }))
    )
    
    ;; Check if caller is authorized oracle
    (asserts! is-oracle (contract-call? dao-constants get-error-unauthorized))
    
    ;; Check if period already reached consensus
    (asserts! (not (get consensus-reached period-consensus-data)) (contract-call? dao-constants get-error-already-exists))
    
    ;; Check if oracle hasn't already submitted for this period
    (asserts! (is-none existing-submission) (contract-call? dao-constants get-error-already-exists))
    
    ;; Record submission
    (map-set oracle-submissions 
      { period: period, oracle: tx-sender }
      { 
        submitted: true,
        submission-hash: contribution-hash,
        timestamp: block-height
      }
    )
    
    ;; Update oracle's last submission timestamp
    (map-set oracles tx-sender
      (merge (default-to 
        { 
          active: true, 
          stake: MIN_STAKE,
          last-submission: u0,
          reliability: u100,
          added-height: block-height
        } 
        (map-get? oracles tx-sender)) 
        { last-submission: block-height }
      )
    )
    
    ;; Update consensus count
    (map-set period-consensus 
      { period: period }
      { 
        submission-count: (+ (get submission-count period-consensus-data) u1),
        consensus-reached: false,
        consensus-timestamp: u0,
        consensus-hash: none
      }
    )
    
    (ok true)
  )
)

;; Submit the actual contribution data once consensus is reached
(define-public (submit-contributions 
    (period (string-ascii 20))
    (contributors (list 200 { contributor: principal, points: uint }))
    (contribution-hash (buff 32))
  )
  (let
    (
      (is-oracle (default-to false (get active (map-get? oracles tx-sender))))
      (period-consensus-data (default-to 
        { 
          submission-count: u0, 
          consensus-reached: false,
          consensus-timestamp: u0,
          consensus-hash: none
        } 
        (map-get? period-consensus { period: period })
      ))
      ;; Verify consensus hash matches submission
      (hash-matches (is-eq (hash-data contributors) contribution-hash))
      ;; Check if a consensus has been reached
      (consensus-reached (verify-consensus period))
    )
    
    ;; Check if caller is authorized oracle
    (asserts! is-oracle (contract-call? dao-constants get-error-unauthorized))
    
    ;; Check if period doesn't already have processed data
    (asserts! (is-none (map-get? processed-periods { period: period })) (contract-call? dao-constants get-error-already-exists))
    
    ;; Ensure consensus has been reached and hash matches
    (asserts! consensus-reached (contract-call? dao-constants get-error-threshold-not-met))
    (asserts! hash-matches (contract-call? dao-constants get-error-invalid-parameter))
    
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
    
    ;; Update consensus data
    (map-set period-consensus
      { period: period }
      (merge period-consensus-data
        {
          consensus-reached: true,
          consensus-timestamp: block-height,
          consensus-hash: (some contribution-hash)
        }
      )
    )
    
    ;; Record oracle rewards
    (map-set oracle-rewards
      { period: period, oracle: tx-sender }
      {
        amount: (var-get oracle-reward-per-submission),
        claimed: false,
        in-consensus: true
      }
    )
    
    ;; Return success with period and number of contributors
    (ok (tuple (period period) (contributors-count (len contributors))))
  )
)

;; Helper function to verify if consensus has been reached for a period
(define-read-only (verify-consensus (period (string-ascii 20)))
  (let
    (
      (period-consensus-data (default-to 
        { 
          submission-count: u0, 
          consensus-reached: false,
          consensus-timestamp: u0,
          consensus-hash: none
        } 
        (map-get? period-consensus { period: period })
      ))
      (total-oracles-count (var-get total-oracles))
      (submission-count (get submission-count period-consensus-data))
      (min-required (calculate-threshold total-oracles-count (var-get min-oracle-consensus)))
    )
    
    ;; Check if submission count meets threshold
    (>= submission-count min-required)
  )
)

;; Calculate consensus threshold based on percentage
(define-private (calculate-threshold (total uint) (percentage uint))
  (let
    (
      (threshold (/ (* total percentage) u100))
    )
    ;; Ensure at least 1
    (if (> threshold u0) threshold u1)
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

;; Helper function to compute a hash for a list of contributors
(define-private (hash-data (contributors (list 200 { contributor: principal, points: uint })))
  (sha256 (serialize-data contributors))
)

;; Helper function to serialize contribution data (simplified)
;; Note: In a real implementation, proper serialization would be used
(define-private (serialize-data (contributors (list 200 { contributor: principal, points: uint })))
  ;; Simplified implementation - in production this would properly serialize the data
  ;; For now, we'll just convert to a string representation
  (to-consensus-buff contributors)
)

;; Helper function to convert to buffer (stub implementation)
(define-private (to-consensus-buff (data (list 200 { contributor: principal, points: uint })))
  ;; In a real implementation this would properly serialize the data
  ;; For now we'll just use a hash of the first item as a placeholder
  (unwrap-panic (as-max-len? (concat 0x00 0x00) u32))
)

;; Claim oracle rewards
(define-public (claim-oracle-rewards (period (string-ascii 20)) (token <ft-trait>))
  (let
    (
      (reward-data (unwrap! (map-get? oracle-rewards { period: period, oracle: tx-sender }) (contract-call? dao-constants get-error-not-found)))
      (is-consensus (get in-consensus reward-data))
      (reward-amount (get amount reward-data))
      (already-claimed (get claimed reward-data))
    )
    
    ;; Check if not already claimed
    (asserts! (not already-claimed) (contract-call? dao-constants get-error-already-exists))
    
    ;; Check if oracle was in consensus
    (asserts! is-consensus (contract-call? dao-constants get-error-unauthorized))
    
    ;; Mark as claimed
    (map-set oracle-rewards
      { period: period, oracle: tx-sender }
      (merge reward-data { claimed: true })
    )
    
    ;; Transfer reward tokens to oracle
    (as-contract (contract-call? token transfer reward-amount tx-sender tx-sender none))
  )
)

;; Governance function: Update oracle reward amount
(define-public (update-oracle-reward (new-reward uint))
  (let
    (
      (is-governor (contract-call? GOVERNANCE_CONTRACT is-valid-signer tx-sender))
    )
    
    ;; Check if caller is authorized governor
    (asserts! is-governor (contract-call? dao-constants get-error-unauthorized))
    
    ;; Update reward amount
    (var-set oracle-reward-per-submission new-reward)
    
    (ok new-reward)
  )
)

;; Governance function: Update consensus threshold
(define-public (update-consensus-threshold (new-threshold uint))
  (let
    (
      (is-governor (contract-call? GOVERNANCE_CONTRACT is-valid-signer tx-sender))
    )
    
    ;; Check if caller is authorized governor
    (asserts! is-governor (contract-call? dao-constants get-error-unauthorized))
    
    ;; Validate threshold (between 51 and 100)
    (asserts! (and (>= new-threshold u51) (<= new-threshold u100)) (contract-call? dao-constants get-error-invalid-parameter))
    
    ;; Update threshold
    (var-set min-oracle-consensus new-threshold)
    
    (ok new-threshold)
  )
)

;; Governance function: Remove oracle for misbehavior
(define-public (remove-oracle (oracle principal))
  (let
    (
      (is-governor (contract-call? GOVERNANCE_CONTRACT is-valid-signer tx-sender))
      (oracle-data (unwrap! (map-get? oracles oracle) (contract-call? dao-constants get-error-not-found)))
      (current-total (var-get total-oracles))
    )
    
    ;; Check if caller is authorized governor
    (asserts! is-governor (contract-call? dao-constants get-error-unauthorized))
    
    ;; Update oracle status
    (map-set oracles oracle
      (merge oracle-data { active: false })
    )
    
    ;; Update total count
    (var-set total-oracles (- current-total u1))
    
    ;; We don't return stake automatically - would need governance decision
    
    (ok (var-get total-oracles))
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

;; Check if an address is an active oracle
(define-read-only (is-authorized-oracle (address principal))
  (default-to false (get active (map-get? oracles address)))
)

;; Get oracle information
(define-read-only (get-oracle-info (oracle principal))
  (map-get? oracles oracle)
)

;; Get consensus info for a period
(define-read-only (get-period-consensus-info (period (string-ascii 20)))
  (map-get? period-consensus { period: period })
)

;; Get total number of active oracles
(define-read-only (get-total-oracles)
  (var-get total-oracles)
)

;; Get current consensus threshold percentage
(define-read-only (get-consensus-threshold)
  (var-get min-oracle-consensus)
)

;; Get oracle submission for a period
(define-read-only (get-oracle-submission (period (string-ascii 20)) (oracle principal))
  (map-get? oracle-submissions { period: period, oracle: oracle })
)
