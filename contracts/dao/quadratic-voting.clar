;; Quadratic Voting Contract
;; [AIR-3][AIS-3][AIT-3][AIP-2][AIE-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_PARAMETER u402)
(define-constant ERR_INSUFFICIENT_TOKENS u403)
(define-constant ERR_SYSTEM_DISABLED u404)
(define-constant ERR_PROPOSAL_NOT_FOUND u405)
(define-constant ERR_ALREADY_VOTED u406)
(define-constant ERR_VOTING_PERIOD_EXPIRED u407)
(define-constant ERR_VOTING_PERIOD_NOT_STARTED u408)
(define-constant ERR_VOTING_PERIOD_ACTIVE u409)
(define-constant ERR_NO_VOTES_CAST u410)
(define-constant ERR_PROPOSAL_ALREADY_EXISTS u411)
(define-constant ERR_BELOW_THRESHOLD u412)
(define-constant ERR_EXECUTION_FAILED u413)
(define-constant ERR_CREDIT_LIMIT_REACHED u414)

;; Contract References
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance)
(define-constant METRICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.metrics-oracle)
(define-constant TREASURY_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury-management)
(define-constant OPERATIONS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.operations-manager)

;; Data Variables
(define-data-var quadratic-voting-enabled bool true)
(define-data-var proposal-count uint u0)
(define-data-var vote-cost-coefficient uint u5) ;; cost is coefficient * sqrt(votes)^2
(define-data-var identity-verification-required bool false)
(define-data-var minimum-proposal-threshold uint u100000000) ;; 1 AGT token (with 8 decimals)
(define-data-var minimum-quorum uint u500000000) ;; 5 AGT tokens worth of effective votes
(define-data-var voting-period-blocks uint u14400) ;; ~100 days with 10-min blocks
(define-data-var execution-delay-blocks uint u1440) ;; ~10 days with 10-min blocks
(define-data-var max-credits-per-period uint u10000) ;; Maximum voting credits per period
(define-data-var credits-reset-blocks uint u144000) ;; Reset credits every ~1000 days

;; Admin and identity verifier lists
(define-map administrators principal bool)
(define-map identity-verifiers principal bool)
(define-map verified-identities principal bool)

;; Initialize administrators
(map-set administrators tx-sender true)

;; Proposal Types
(define-constant PROPOSAL_TYPE_GENERAL u1)
(define-constant PROPOSAL_TYPE_PARAMETER u2)
(define-constant PROPOSAL_TYPE_CONTRACT u3)
(define-constant PROPOSAL_TYPE_BUDGET u4)
(define-constant PROPOSAL_TYPE_EMERGENCY u5)

;; Proposal Statuses
(define-constant PROPOSAL_STATUS_ACTIVE u1)
(define-constant PROPOSAL_STATUS_PASSED u2)
(define-constant PROPOSAL_STATUS_FAILED u3)
(define-constant PROPOSAL_STATUS_EXECUTING u4)
(define-constant PROPOSAL_STATUS_EXECUTED u5)
(define-constant PROPOSAL_STATUS_CANCELLED u6)

;; Proposal Data Structure
(define-map proposals
  uint ;; proposal-id
  {
    title: (string-ascii 100),
    description: (string-utf8 4096),
    url: (optional (string-utf8 256)),
    proposer: principal,
    proposal-type: uint,
    start-block: uint,
    end-block: uint,
    execution-block: uint,
    total-votes-for: uint,
    total-votes-against: uint,
    total-sqrt-votes-for: uint,
    total-sqrt-votes-against: uint,
    status: uint,
    executed-at: (optional uint),
    executed-by: (optional principal),
    action-target: (optional principal),
    action-data: (optional (string-utf8 1024))
  }
)

;; Vote Records
(define-map votes
  { proposal-id: uint, voter: principal }
  {
    vote-count: uint,
    vote-cost: uint,
    vote-direction: bool, ;; true = for, false = against
    block-height: uint,
    effective-vote-power: uint,
    sqrt-votes: uint
  }
)

;; Voter Credits
(define-map voter-credits
  principal
  {
    available-credits: uint,
    last-reset-block: uint,
    total-spent: uint
  }
)

;; Voting Stats
(define-data-var total-votes-cast uint u0)
(define-data-var unique-voters uint u0)
(define-map voter-participation principal uint)

;; Public Functions

;; Create a new proposal
(define-public (create-proposal 
    (title (string-ascii 100))
    (description (string-utf8 4096))
    (url (optional (string-utf8 256)))
    (proposal-type uint)
    (action-target (optional principal))
    (action-data (optional (string-utf8 1024))))
  (begin
    ;; Check if quadratic voting is enabled
    (asserts! (var-get quadratic-voting-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Validate proposal type
    (asserts! (and (>= proposal-type PROPOSAL_TYPE_GENERAL) (<= proposal-type PROPOSAL_TYPE_EMERGENCY)) 
              (err ERR_INVALID_PARAMETER))
    
    ;; Check if proposer has enough tokens for proposal threshold
    (asserts! (>= (unwrap! (contract-call? TOKEN_CONTRACT get-balance tx-sender) (err ERR_INVALID_PARAMETER))
                 (var-get minimum-proposal-threshold))
             (err ERR_BELOW_THRESHOLD))
    
    ;; Check identity verification if required
    (when (var-get identity-verification-required)
      (asserts! (is-identity-verified tx-sender) (err ERR_UNAUTHORIZED)))
    
    ;; Increment proposal count
    (let ((new-proposal-id (+ (var-get proposal-count) u1)))
      
      ;; Create new proposal
      (map-set proposals new-proposal-id
        {
          title: title,
          description: description,
          url: url,
          proposer: tx-sender,
          proposal-type: proposal-type,
          start-block: block-height,
          end-block: (+ block-height (var-get voting-period-blocks)),
          execution-block: (+ block-height (var-get voting-period-blocks) (var-get execution-delay-blocks)),
          total-votes-for: u0,
          total-votes-against: u0,
          total-sqrt-votes-for: u0,
          total-sqrt-votes-against: u0,
          status: PROPOSAL_STATUS_ACTIVE,
          executed-at: none,
          executed-by: none,
          action-target: action-target,
          action-data: action-data
        }
      )
      
      ;; Update proposal count
      (var-set proposal-count new-proposal-id)
      
      ;; Log proposal creation metric
      (try! (contract-call? METRICS_CONTRACT submit-governance-metric "proposals_created" u1 u1000 "quadratic-voting"))
      
      ;; Return proposal ID
      (ok new-proposal-id)
    )
  ))

;; Cast a quadratic vote
(define-public (cast-quadratic-vote 
    (proposal-id uint)
    (vote-count uint)
    (vote-direction bool))
  (begin
    ;; Check if quadratic voting is enabled
    (asserts! (var-get quadratic-voting-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check if the proposal exists
    (let ((proposal (unwrap! (map-get? proposals proposal-id) (err ERR_PROPOSAL_NOT_FOUND))))
      
      ;; Check if user has already voted on this proposal
      (asserts! (is-none (map-get? votes {proposal-id: proposal-id, voter: tx-sender}))
               (err ERR_ALREADY_VOTED))
      
      ;; Check if voting period is active
      (asserts! (>= block-height (get start-block proposal)) (err ERR_VOTING_PERIOD_NOT_STARTED))
      (asserts! (<= block-height (get end-block proposal)) (err ERR_VOTING_PERIOD_EXPIRED))
      
      ;; Check if proposal is still active
      (asserts! (is-eq (get status proposal) PROPOSAL_STATUS_ACTIVE) (err ERR_VOTING_PERIOD_EXPIRED))
      
      ;; Ensure vote count is positive
      (asserts! (> vote-count u0) (err ERR_INVALID_PARAMETER))
      
      ;; Check identity verification if required
      (when (var-get identity-verification-required)
        (asserts! (is-identity-verified tx-sender) (err ERR_UNAUTHORIZED)))
      
      ;; Calculate quadratic voting cost and power
      (let (
        (sqrt-votes (get-sqrt-votes vote-count))
        (vote-cost (* (var-get vote-cost-coefficient) (* sqrt-votes sqrt-votes)))
        ;; Get token balance
        (token-balance (unwrap! (contract-call? TOKEN_CONTRACT get-balance tx-sender) (err ERR_INVALID_PARAMETER)))
        ;; Get or initialize voter credits
        (credits (default-to {available-credits: (var-get max-credits-per-period), last-reset-block: block-height, total-spent: u0} 
                           (map-get? voter-credits tx-sender)))
      )
        ;; Check if user has enough credits
        (asserts! (>= (get available-credits credits) vote-cost) (err ERR_CREDIT_LIMIT_REACHED))
        
        ;; Calculate effective vote power (sqrt of vote-count)
        (let (
          (effective-vote-power sqrt-votes)
          ;; Update voter credits
          (new-available-credits (- (get available-credits credits) vote-cost))
          (new-total-spent (+ (get total-spent credits) vote-cost))
        )
          ;; Update votes map
          (map-set votes 
            {proposal-id: proposal-id, voter: tx-sender}
            {
              vote-count: vote-count,
              vote-cost: vote-cost,
              vote-direction: vote-direction,
              block-height: block-height,
              effective-vote-power: effective-vote-power,
              sqrt-votes: sqrt-votes
            }
          )
          
          ;; Update voter credits
          (map-set voter-credits tx-sender
            {
              available-credits: new-available-credits,
              last-reset-block: (get last-reset-block credits),
              total-spent: new-total-spent
            }
          )
          
          ;; Update proposal vote counts
          (if vote-direction
              ;; Vote FOR
              (map-set proposals proposal-id
                (merge proposal {
                  total-votes-for: (+ (get total-votes-for proposal) vote-count),
                  total-sqrt-votes-for: (+ (get total-sqrt-votes-for proposal) sqrt-votes)
                })
              )
              ;; Vote AGAINST
              (map-set proposals proposal-id
                (merge proposal {
                  total-votes-against: (+ (get total-votes-against proposal) vote-count),
                  total-sqrt-votes-against: (+ (get total-sqrt-votes-against proposal) sqrt-votes)
                })
              )
          )
          
          ;; Update voting participation stats
          (if (is-eq (default-to u0 (map-get? voter-participation tx-sender)) u0)
              (begin
                (var-set unique-voters (+ (var-get unique-voters) u1))
                (map-set voter-participation tx-sender u1))
              (map-set voter-participation tx-sender (+ (default-to u0 (map-get? voter-participation tx-sender)) u1))
          )
          
          ;; Update total votes cast
          (var-set total-votes-cast (+ (var-get total-votes-cast) u1))
          
          ;; Log voting metrics
          (try! (contract-call? METRICS_CONTRACT submit-governance-metric "quadratic_votes_cast" u1 u1000 "quadratic-voting"))
          (try! (contract-call? METRICS_CONTRACT submit-governance-metric "voting_participation" 
                               (var-get unique-voters) u1000 "quadratic-voting"))
          
          (ok {
            proposal-id: proposal-id,
            vote-count: vote-count,
            vote-cost: vote-cost,
            effective-power: effective-vote-power
          })
        )
      )
    )
  ))

;; Execute a proposal that has passed
(define-public (execute-proposal (proposal-id uint))
  (begin
    ;; Check if quadratic voting is enabled
    (asserts! (var-get quadratic-voting-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check if the proposal exists
    (let ((proposal (unwrap! (map-get? proposals proposal-id) (err ERR_PROPOSAL_NOT_FOUND))))
      
      ;; Check if proposal passed and is ready for execution
      (asserts! (>= block-height (get execution-block proposal)) (err ERR_VOTING_PERIOD_ACTIVE))
      (asserts! (is-eq (get status proposal) PROPOSAL_STATUS_ACTIVE) (err ERR_INVALID_PARAMETER))
      
      ;; Check quorum
      (asserts! (>= (+ (get total-sqrt-votes-for proposal) (get total-sqrt-votes-against proposal)) 
                   (var-get minimum-quorum))
               (err ERR_NO_VOTES_CAST))
      
      ;; Check if proposal passed (more FOR than AGAINST)
      (if (> (get total-sqrt-votes-for proposal) (get total-sqrt-votes-against proposal))
          ;; Proposal passed
          (begin
            ;; Update proposal status to executing
            (map-set proposals proposal-id
              (merge proposal {
                status: PROPOSAL_STATUS_EXECUTING,
                executed-at: (some block-height),
                executed-by: (some tx-sender)
              })
            )
            
            ;; Execute proposal action if provided
            (match (get action-target proposal)
              target 
              (begin
                ;; Call operations contract to execute
                (match (get action-data proposal)
                  action-data (try! (contract-call? OPERATIONS_CONTRACT execute-operation target action-data))
                  (err ERR_INVALID_PARAMETER)
                )
                
                ;; Update status to executed
                (map-set proposals proposal-id
                  (merge 
                    (unwrap! (map-get? proposals proposal-id) (err ERR_PROPOSAL_NOT_FOUND))
                    { status: PROPOSAL_STATUS_EXECUTED }
                  )
                )
              )
              ;; No action target, just mark as executed
              (map-set proposals proposal-id (merge proposal { status: PROPOSAL_STATUS_EXECUTED }))
            )
            
            ;; Log execution metric
            (try! (contract-call? METRICS_CONTRACT submit-governance-metric "proposals_executed" u1 u1000 "quadratic-voting"))
            
            (ok true)
          )
          ;; Proposal failed
          (begin
            (map-set proposals proposal-id (merge proposal { status: PROPOSAL_STATUS_FAILED }))
            (ok false)
          )
      )
    )
  ))

;; Cancel a proposal (only proposer or admin)
(define-public (cancel-proposal (proposal-id uint))
  (begin
    ;; Check if the proposal exists
    (let ((proposal (unwrap! (map-get? proposals proposal-id) (err ERR_PROPOSAL_NOT_FOUND))))
      
      ;; Check if caller is proposer or admin
      (asserts! (or (is-eq tx-sender (get proposer proposal)) (is-administrator tx-sender))
               (err ERR_UNAUTHORIZED))
      
      ;; Check if proposal is still active
      (asserts! (is-eq (get status proposal) PROPOSAL_STATUS_ACTIVE) (err ERR_INVALID_PARAMETER))
      
      ;; Update status to cancelled
      (map-set proposals proposal-id (merge proposal { status: PROPOSAL_STATUS_CANCELLED }))
      
      (ok true)
    )
  ))

;; Reset voting credits for a user
(define-public (reset-voting-credits)
  (begin
    ;; Check if quadratic voting is enabled
    (asserts! (var-get quadratic-voting-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Get current credits
    (let ((credits (default-to {available-credits: (var-get max-credits-per-period), last-reset-block: block-height, total-spent: u0}
                             (map-get? voter-credits tx-sender))))
      
      ;; Check if enough blocks have passed since last reset
      (asserts! (>= (- block-height (get last-reset-block credits)) (var-get credits-reset-blocks))
               (err ERR_INVALID_PARAMETER))
      
      ;; Reset credits
      (map-set voter-credits tx-sender
        {
          available-credits: (var-get max-credits-per-period),
          last-reset-block: block-height,
          total-spent: (get total-spent credits)
        }
      )
      
      (ok true)
    )
  ))

;; Read-Only Functions

;; Get proposal details
(define-read-only (get-proposal (proposal-id uint))
  (map-get? proposals proposal-id))

;; Get proposal result
(define-read-only (get-proposal-result (proposal-id uint))
  (match (map-get? proposals proposal-id)
    proposal {
      status: (get status proposal),
      total-votes-for: (get total-votes-for proposal),
      total-votes-against: (get total-votes-against proposal),
      total-sqrt-votes-for: (get total-sqrt-votes-for proposal),
      total-sqrt-votes-against: (get total-sqrt-votes-against proposal),
      quorum-reached: (>= (+ (get total-sqrt-votes-for proposal) (get total-sqrt-votes-against proposal)) 
                         (var-get minimum-quorum)),
      execution-block: (get execution-block proposal),
      executed: (not (is-eq (get status proposal) PROPOSAL_STATUS_ACTIVE))
    }
    none
  ))

;; Get user's vote on a proposal
(define-read-only (get-vote (proposal-id uint) (voter principal))
  (map-get? votes {proposal-id: proposal-id, voter: voter}))

;; Get available voting credits for a user
(define-read-only (get-user-credits (user principal))
  (default-to 
    {
      available-credits: (var-get max-credits-per-period),
      last-reset-block: u0,
      total-spent: u0
    }
    (map-get? voter-credits user)
  ))

;; Calculate the current participation rate
(define-read-only (get-participation-rate)
  (let (
    (unique-voters-count (var-get unique-voters))
    (token-holders (unwrap-panic (contract-call? TOKEN_CONTRACT get-token-holders-count)))
  )
    (if (is-eq token-holders u0)
        u0
        (/ (* unique-voters-count u10000) token-holders) ;; Returns percentage with 2 decimals (e.g., 2350 = 23.50%)
    )
  ))

;; Get quadratic voting parameters
(define-read-only (get-voting-parameters)
  {
    quadratic-voting-enabled: (var-get quadratic-voting-enabled),
    vote-cost-coefficient: (var-get vote-cost-coefficient),
    identity-verification-required: (var-get identity-verification-required),
    minimum-proposal-threshold: (var-get minimum-proposal-threshold),
    minimum-quorum: (var-get minimum-quorum),
    voting-period-blocks: (var-get voting-period-blocks),
    execution-delay-blocks: (var-get execution-delay-blocks),
    max-credits-per-period: (var-get max-credits-per-period),
    credits-reset-blocks: (var-get credits-reset-blocks)
  })

;; Check if account is an administrator
(define-read-only (is-administrator (account principal))
  (default-to false (map-get? administrators account)))

;; Check if account is an identity verifier
(define-read-only (is-identity-verifier (account principal))
  (default-to false (map-get? identity-verifiers account)))

;; Check if account has verified identity
(define-read-only (is-identity-verified (account principal))
  (default-to false (map-get? verified-identities account)))

;; Get system statistics
(define-read-only (get-voting-stats)
  {
    total-proposals: (var-get proposal-count),
    total-votes-cast: (var-get total-votes-cast),
    unique-voters: (var-get unique-voters),
    participation-rate: (get-participation-rate)
  })

;; Helper Functions

;; Calculate square root of a number using binary search (for quadratic voting)
(define-private (get-sqrt-votes (num uint))
  (let ((max-iterations u32))
    (get result (sqrt-internal num u0 num max-iterations))
  ))

;; Binary search implementation for square root calculation
(define-private (sqrt-internal (num uint) (low uint) (high uint) (iterations uint))
  (if (or (<= iterations u0) (< high low))
      {result: low, iterations-used: (- u32 iterations)}
      (let (
        (mid (/ (+ low high) u2))
        (mid-squared (* mid mid))
      )
        (if (> mid-squared num)
            (sqrt-internal num low (- mid u1) (- iterations u1))
            (if (< mid-squared num)
                (sqrt-internal num (+ mid u1) high (- iterations u1))
                {result: mid, iterations-used: (- u32 iterations)}
            )
        )
      )
  ))

;; Admin Functions

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
    (asserts! (not (is-eq admin tx-sender)) (err ERR_INVALID_PARAMETER))
    (map-set administrators admin false)
    (ok true)))

;; Add an identity verifier
(define-public (add-identity-verifier (verifier principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set identity-verifiers verifier true)
    (ok true)))

;; Remove an identity verifier
(define-public (remove-identity-verifier (verifier principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set identity-verifiers verifier false)
    (ok true)))

;; Verify a user's identity
(define-public (verify-identity (user principal))
  (begin
    (asserts! (or (is-administrator tx-sender) (is-identity-verifier tx-sender)) (err ERR_UNAUTHORIZED))
    (map-set verified-identities user true)
    (ok true)))

;; Revoke a user's identity verification
(define-public (revoke-identity-verification (user principal))
  (begin
    (asserts! (or (is-administrator tx-sender) (is-identity-verifier tx-sender)) (err ERR_UNAUTHORIZED))
    (map-set verified-identities user false)
    (ok true)))

;; Toggle quadratic voting
(define-public (toggle-quadratic-voting (enabled bool))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (var-set quadratic-voting-enabled enabled)
    (ok true)))

;; Toggle identity verification requirement
(define-public (toggle-identity-verification (required bool))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (var-set identity-verification-required required)
    (ok true)))

;; Update quadratic voting parameters
(define-public (update-voting-parameters
    (cost-coefficient (optional uint))
    (proposal-threshold (optional uint))
    (quorum (optional uint))
    (voting-period (optional uint))
    (execution-delay (optional uint))
    (max-credits (optional uint))
    (reset-blocks (optional uint)))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Update each parameter if provided
    (match cost-coefficient
      val (var-set vote-cost-coefficient val)
      true)
    
    (match proposal-threshold
      val (var-set minimum-proposal-threshold val)
      true)
    
    (match quorum
      val (var-set minimum-quorum val)
      true)
    
    (match voting-period
      val (var-set voting-period-blocks val)
      true)
    
    (match execution-delay
      val (var-set execution-delay-blocks val)
      true)
    
    (match max-credits
      val (var-set max-credits-per-period val)
      true)
    
    (match reset-blocks
      val (var-set credits-reset-blocks val)
      true)
    
    (ok true)
  )) 