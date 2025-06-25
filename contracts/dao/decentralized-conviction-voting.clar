;; Decentralized Conviction Voting Contract
;; [AIR-3][AIS-3][AIT-3][BPC-3][DAO-3]

;; Imports
(use-trait ft-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)
(use-trait governance-trait .governance-traits.governance-trait)
(use-trait multi-sig-trait .governance-traits.multi-sig-trait)

;; Import shared constants
(use-contract dao-constants .shared.dao-constants)

;; Contract references
(define-constant GOVERNANCE_CONTRACT .multi-sig-governance)
(define-constant TOKEN_CONTRACT .token)
(define-constant TREASURY_CONTRACT .treasury-management)

;; Proposal Types
(define-constant PROPOSAL_TYPE_GENERAL u0)
(define-constant PROPOSAL_TYPE_FUNDING u1)
(define-constant PROPOSAL_TYPE_PARAMETER u2)
(define-constant PROPOSAL_TYPE_CONTRACT u3)

;; Proposal Status
(define-constant STATUS_ACTIVE u0)
(define-constant STATUS_EXECUTED u1)
(define-constant STATUS_REJECTED u2)
(define-constant STATUS_EXPIRED u3)

;; Data vars
(define-data-var conviction-enabled bool true)
(define-data-var proposal-count uint u0)
(define-data-var decay-constant uint u9500) ;; 0.95 * 10000, decay factor per block
(define-data-var conviction-threshold uint u1000000) ;; Threshold to pass proposals
(define-data-var min-stake uint u10000000) ;; Minimum stake required to create proposal
(define-data-var max-inactive-blocks uint u144000) ;; Max blocks a proposal can be inactive (approximately 1000 days)

;; Proposals map
(define-map proposals
  uint ;; proposal ID
  {
    title: (string-ascii 100),
    description: (string-utf8 1000),
    link: (string-ascii 256),
    proposer: principal,
    created-at-block: uint,
    proposal-type: uint,
    status: uint,
    current-conviction: uint,
    last-update-block: uint,
    required-conviction: uint,
    executed-at-block: (optional uint),
    execution-params: (optional (list 5 {key: (string-ascii 64), value: (string-ascii 256)})),
    max-conviction: uint
  })

;; Votes map - tracks conviction for each voter on each proposal
(define-map votes
  { proposal-id: uint, voter: principal }
  {
    stake: uint,
    conviction: uint,
    last-update-block: uint,
    vote-added-at-block: uint
  })

;; Total conviction by voter
(define-map voter-conviction
  principal
  {
    total-conviction: uint,
    total-stake: uint,
    available-tokens: uint
  })

;; Public Functions

;; Create a new proposal
(define-public (create-proposal 
               (title (string-ascii 100))
               (description (string-utf8 1000))
               (link (string-ascii 256))
               (proposal-type uint)
               (execution-params (optional (list 5 {key: (string-ascii 64), value: (string-ascii 256)}))))
  (let (
    (proposal-id (+ (var-get proposal-count) u1))
    (token-balance (default-to u0 (get-balance tx-sender)))
    (threshold (calculate-threshold proposal-type token-balance)))
    
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled) (contract-call? dao-constants get-error-unauthorized))
    
    ;; Check token balance meets minimum requirement
    (asserts! (>= token-balance (var-get min-stake)) (contract-call? dao-constants get-error-insufficient-balance))
    
    ;; Validate proposal type
    (asserts! (<= proposal-type PROPOSAL_TYPE_CONTRACT) (contract-call? dao-constants get-error-invalid-parameter))
    
    ;; Create the proposal
    (map-set proposals
      proposal-id
      {
        title: title,
        description: description,
        link: link,
        proposer: tx-sender,
        created-at-block: block-height,
        proposal-type: proposal-type,
        status: STATUS_ACTIVE,
        current-conviction: u0,
        last-update-block: block-height,
        required-conviction: threshold,
        executed-at-block: none,
        execution-params: execution-params,
        max-conviction: u0
      })
    
    ;; Increment proposal count
    (var-set proposal-count proposal-id)
    
    (ok proposal-id)
  ))

;; Add conviction to a proposal
(define-public (add-conviction (proposal-id uint) (stake uint))
  (let (
    (proposal (unwrap! (map-get? proposals proposal-id) (contract-call? dao-constants get-error-not-found)))
    (token-balance (default-to u0 (get-balance tx-sender)))
    (voter-data (default-to 
                  { total-conviction: u0, total-stake: u0, available-tokens: token-balance } 
                  (map-get? voter-conviction tx-sender)))
    (existing-vote (map-get? votes { proposal-id: proposal-id, voter: tx-sender })))
    
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled) (contract-call? dao-constants get-error-unauthorized))
    
    ;; Check proposal is active
    (asserts! (is-eq (get status proposal) STATUS_ACTIVE) (contract-call? dao-constants get-error-invalid-state))
    
    ;; Check user has sufficient available tokens
    (asserts! (>= (get available-tokens voter-data) stake) (contract-call? dao-constants get-error-insufficient-balance))
    
    ;; If vote already exists, update it
    (if (is-some existing-vote)
        (update-existing-conviction proposal-id stake (unwrap-panic existing-vote) proposal voter-data)
        (add-new-conviction proposal-id stake proposal voter-data))
  ))

;; Remove conviction from a proposal
(define-public (remove-conviction (proposal-id uint))
  (let (
    (proposal (unwrap! (map-get? proposals proposal-id) (contract-call? dao-constants get-error-not-found)))
    (vote (unwrap! (map-get? votes { proposal-id: proposal-id, voter: tx-sender }) (contract-call? dao-constants get-error-not-found)))
    (voter-data (default-to 
                  { total-conviction: u0, total-stake: u0, available-tokens: u0 } 
                  (map-get? voter-conviction tx-sender)))
    (current-block block-height)
    (elapsed-blocks (- current-block (get last-update-block vote)))
    (updated-conviction (calculate-updated-conviction (get conviction vote) (get stake vote) elapsed-blocks)))
    
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled) (contract-call? dao-constants get-error-unauthorized))
    
    ;; Update proposal conviction
    (map-set proposals
      proposal-id
      (merge proposal {
        current-conviction: (- (get current-conviction proposal) updated-conviction),
        last-update-block: current-block
      }))
    
    ;; Update voter's available tokens and total stake
    (map-set voter-conviction
      tx-sender
      {
        total-conviction: (- (get total-conviction voter-data) updated-conviction),
        total-stake: (- (get total-stake voter-data) (get stake vote)),
        available-tokens: (+ (get available-tokens voter-data) (get stake vote))
      })
    
    ;; Remove the vote
    (map-delete votes { proposal-id: proposal-id, voter: tx-sender })
    
    (ok updated-conviction)
  ))

;; Execute a proposal that has reached the conviction threshold
(define-public (execute-proposal (proposal-id uint))
  (let (
    (proposal (unwrap! (map-get? proposals proposal-id) (contract-call? dao-constants get-error-not-found))))
    
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled) (contract-call? dao-constants get-error-unauthorized))
    
    ;; Check proposal is active
    (asserts! (is-eq (get status proposal) STATUS_ACTIVE) (contract-call? dao-constants get-error-invalid-state))
    
    ;; Check proposal hasn't already been executed
    (asserts! (is-none (get executed-at-block proposal)) (contract-call? dao-constants get-error-already-exists))
    
    ;; Check if conviction is above threshold
    (asserts! (>= (get current-conviction proposal) (get required-conviction proposal))
              (contract-call? dao-constants get-error-threshold-not-met))
    
    ;; Execute the proposal based on its type
    (match (get proposal-type proposal)
      PROPOSAL_TYPE_GENERAL (execute-general-proposal proposal-id proposal)
      PROPOSAL_TYPE_FUNDING (execute-funding-proposal proposal-id proposal)
      PROPOSAL_TYPE_PARAMETER (execute-parameter-proposal proposal-id proposal)
      PROPOSAL_TYPE_CONTRACT (execute-contract-proposal proposal-id proposal)
      (contract-call? dao-constants get-error-invalid-parameter))
  ))

;; Update all proposals and votes to calculate current conviction
(define-public (update-convictions (proposal-ids (list 50 uint)))
  (begin
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled) (contract-call? dao-constants get-error-unauthorized))
    
    ;; Update each proposal
    (map update-proposal-conviction proposal-ids)
    
    ;; Check for expired proposals
    (map check-proposal-expiry proposal-ids)
    
    (ok true)
  ))

;; Helper Functions

;; Get token balance for a user
(define-read-only (get-balance (user principal))
  (ok u0)) ;; Placeholder - would call token contract in actual implementation

;; Calculate conviction threshold based on proposal type and stake
(define-private (calculate-threshold (proposal-type uint) (stake uint))
  (var-get conviction-threshold)) ;; Simplified - would have more complex logic in real implementation

;; Calculate updated conviction for a vote
(define-private (calculate-updated-conviction (initial-conviction uint) (stake uint) (blocks uint))
  (let (
    (decay-factor (calculate-decay-factor blocks)))
    
    ;; Apply decay formula: conviction * (decay ^ blocks)
    (+ (/ (* initial-conviction decay-factor) u10000) stake)
  ))

;; Calculate decay factor based on blocks elapsed
(define-private (calculate-decay-factor (blocks uint))
  (let (
    (decay (var-get decay-constant))
    (base-factor u10000)) ;; 10000 represents 1.0
    
    ;; Simplified decay calculation: (decay ^ blocks) using repeated multiplication
    ;; In a real implementation, we would use a more efficient algorithm
    (fold calculate-decay blocks decay base-factor)
  ))

;; Helper for calculating decay over multiple blocks
(define-private (calculate-decay (i uint) (result uint) (base uint))
  (/ (* result (var-get decay-constant)) u10000))

;; Update existing conviction
(define-private (update-existing-conviction (proposal-id uint) (additional-stake uint) (vote {stake: uint, conviction: uint, last-update-block: uint, vote-added-at-block: uint}) (proposal {title: (string-ascii 100), description: (string-utf8 1000), link: (string-ascii 256), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint), execution-params: (optional (list 5 {key: (string-ascii 64), value: (string-ascii 256)})), max-conviction: uint}) (voter-data {total-conviction: uint, total-stake: uint, available-tokens: uint}))
  (let (
    (current-block block-height)
    (elapsed-blocks (- current-block (get last-update-block vote)))
    (updated-conviction (calculate-updated-conviction (get conviction vote) (get stake vote) elapsed-blocks))
    (new-stake (+ (get stake vote) additional-stake))
    (new-conviction (+ updated-conviction additional-stake)))
    
    ;; Update the vote
    (map-set votes
      { proposal-id: proposal-id, voter: tx-sender }
      {
        stake: new-stake,
        conviction: new-conviction,
        last-update-block: current-block,
        vote-added-at-block: (get vote-added-at-block vote)
      })
    
    ;; Update the proposal
    (map-set proposals
      proposal-id
      (merge proposal {
        current-conviction: (+ (- (get current-conviction proposal) updated-conviction) new-conviction),
        last-update-block: current-block,
        max-conviction: (max (get max-conviction proposal) 
                             (+ (- (get current-conviction proposal) updated-conviction) new-conviction))
      }))
    
    ;; Update voter conviction data
    (map-set voter-conviction
      tx-sender
      {
        total-conviction: (+ (- (get total-conviction voter-data) updated-conviction) new-conviction),
        total-stake: (+ (get total-stake voter-data) additional-stake),
        available-tokens: (- (get available-tokens voter-data) additional-stake)
      })
    
    (ok new-conviction)
  ))

;; Add new conviction
(define-private (add-new-conviction (proposal-id uint) (stake uint) (proposal {title: (string-ascii 100), description: (string-utf8 1000), link: (string-ascii 256), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint), execution-params: (optional (list 5 {key: (string-ascii 64), value: (string-ascii 256)})), max-conviction: uint}) (voter-data {total-conviction: uint, total-stake: uint, available-tokens: uint}))
  (let (
    (current-block block-height)
    (new-conviction stake))
    
    ;; Add the vote
    (map-set votes
      { proposal-id: proposal-id, voter: tx-sender }
      {
        stake: stake,
        conviction: new-conviction,
        last-update-block: current-block,
        vote-added-at-block: current-block
      })
    
    ;; Update the proposal
    (map-set proposals
      proposal-id
      (merge proposal {
        current-conviction: (+ (get current-conviction proposal) new-conviction),
        last-update-block: current-block,
        max-conviction: (max (get max-conviction proposal) 
                             (+ (get current-conviction proposal) new-conviction))
      }))
    
    ;; Update voter conviction data
    (map-set voter-conviction
      tx-sender
      {
        total-conviction: (+ (get total-conviction voter-data) new-conviction),
        total-stake: (+ (get total-stake voter-data) stake),
        available-tokens: (- (get available-tokens voter-data) stake)
      })
    
    (ok new-conviction)
  ))

;; Update proposal conviction
(define-private (update-proposal-conviction (proposal-id uint))
  (match (map-get? proposals proposal-id)
    proposal (if (is-eq (get status proposal) STATUS_ACTIVE)
                (let (
                  (current-block block-height)
                  (elapsed-blocks (- current-block (get last-update-block proposal))))
                  (if (> elapsed-blocks u0)
                      (let (
                        (decay-factor (calculate-decay-factor elapsed-blocks))
                        (updated-conviction (/ (* (get current-conviction proposal) decay-factor) u10000)))
                        ;; Update the proposal
                        (map-set proposals
                          proposal-id
                          (merge proposal {
                            current-conviction: updated-conviction,
                            last-update-block: current-block
                          }))
                        true)
                      true))
                true)
    false))

;; Check if a proposal has expired
(define-private (check-proposal-expiry (proposal-id uint))
  (match (map-get? proposals proposal-id)
    proposal (if (and (is-eq (get status proposal) STATUS_ACTIVE)
                      (> (- block-height (get last-update-block proposal)) (var-get max-inactive-blocks)))
                (begin
                  ;; Update the proposal to expired status
                  (map-set proposals
                    proposal-id
                    (merge proposal {
                      status: STATUS_EXPIRED
                    }))
                  true)
                true)
    false))

;; Execute different types of proposals

;; Execute a general proposal (informational)
(define-private (execute-general-proposal (proposal-id uint) (proposal {title: (string-ascii 100), description: (string-utf8 1000), link: (string-ascii 256), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint), execution-params: (optional (list 5 {key: (string-ascii 64), value: (string-ascii 256)})), max-conviction: uint}))
  (begin
    ;; Update proposal status
    (map-set proposals
      proposal-id
      (merge proposal {
        status: STATUS_EXECUTED,
        executed-at-block: (some block-height)
      }))
    
    ;; No specific execution needed for general proposals
    (ok true)
  ))

;; Execute a funding proposal
(define-private (execute-funding-proposal (proposal-id uint) (proposal {title: (string-ascii 100), description: (string-utf8 1000), link: (string-ascii 256), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint), execution-params: (optional (list 5 {key: (string-ascii 64), value: (string-ascii 256)})), max-conviction: uint}))
  (let (
    (params (unwrap! (get execution-params proposal) (contract-call? dao-constants get-error-invalid-parameter)))
    (recipient (unwrap! (get-param-value params "recipient") (contract-call? dao-constants get-error-invalid-parameter)))
    (amount-str (unwrap! (get-param-value params "amount") (contract-call? dao-constants get-error-invalid-parameter)))
    (amount (unwrap! (string-to-uint amount-str) (contract-call? dao-constants get-error-invalid-parameter)))
    (recipient-principal (unwrap! (string-to-principal recipient) (contract-call? dao-constants get-error-invalid-parameter))))
    
    ;; Update proposal status
    (map-set proposals
      proposal-id
      (merge proposal {
        status: STATUS_EXECUTED,
        executed-at-block: (some block-height)
      }))
    
    ;; Execute treasury transfer via governance contract
    (contract-call? GOVERNANCE_CONTRACT propose-transaction (concat "transfer-funds-to-" recipient))
  ))

;; Execute a parameter change proposal
(define-private (execute-parameter-proposal (proposal-id uint) (proposal {title: (string-ascii 100), description: (string-utf8 1000), link: (string-ascii 256), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint), execution-params: (optional (list 5 {key: (string-ascii 64), value: (string-ascii 256)})), max-conviction: uint}))
  (let (
    (params (unwrap! (get execution-params proposal) (contract-call? dao-constants get-error-invalid-parameter)))
    (param-name (unwrap! (get-param-value params "parameter") (contract-call? dao-constants get-error-invalid-parameter)))
    (param-value-str (unwrap! (get-param-value params "value") (contract-call? dao-constants get-error-invalid-parameter)))
    (param-value (unwrap! (string-to-uint param-value-str) (contract-call? dao-constants get-error-invalid-parameter))))
    
    ;; Update proposal status
    (map-set proposals
      proposal-id
      (merge proposal {
        status: STATUS_EXECUTED,
        executed-at-block: (some block-height)
      }))
    
    ;; Execute parameter change via governance contract
    (contract-call? GOVERNANCE_CONTRACT propose-transaction (concat "update-parameter-" param-name))
  ))

;; Execute a contract call proposal
(define-private (execute-contract-proposal (proposal-id uint) (proposal {title: (string-ascii 100), description: (string-utf8 1000), link: (string-ascii 256), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint), execution-params: (optional (list 5 {key: (string-ascii 64), value: (string-ascii 256)})), max-conviction: uint}))
  (let (
    (params (unwrap! (get execution-params proposal) (contract-call? dao-constants get-error-invalid-parameter)))
    (contract-name (unwrap! (get-param-value params "contract") (contract-call? dao-constants get-error-invalid-parameter)))
    (function-name (unwrap! (get-param-value params "function") (contract-call? dao-constants get-error-invalid-parameter))))
    
    ;; Update proposal status
    (map-set proposals
      proposal-id
      (merge proposal {
        status: STATUS_EXECUTED,
        executed-at-block: (some block-height)
      }))
    
    ;; Since Clarity doesn't support dynamic contract calls, we use the governance contract
    (contract-call? GOVERNANCE_CONTRACT propose-transaction (concat (concat contract-name "-") function-name))
  ))

;; Helper function to get parameter value from list
(define-private (get-param-value (params (list 5 {key: (string-ascii 64), value: (string-ascii 256)})) (key-to-find (string-ascii 64)))
  (get value (unwrap! (find filter-by-key params) none)))

;; Helper function to filter parameters by key
(define-private (filter-by-key (param {key: (string-ascii 64), value: (string-ascii 256)}))
  (is-eq (get key param) key-to-find))

;; Helper function to convert string to uint
(define-private (string-to-uint (str (string-ascii 256)))
  (some u0)) ;; Placeholder - would need actual string parsing in real implementation

;; Helper function to convert string to principal
(define-private (string-to-principal (str (string-ascii 256)))
  (some 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)) ;; Placeholder - would need actual conversion in real implementation

;; Governance Functions

;; Toggle conviction voting (governance-controlled)
(define-public (toggle-conviction-voting (enabled bool))
  (begin
    ;; Check if caller is the governance contract or authenticated by it
    (asserts! (contract-call? GOVERNANCE_CONTRACT is-valid-signer tx-sender) 
              (contract-call? dao-constants get-error-unauthorized))
    
    (var-set conviction-enabled enabled)
    (ok true)
  ))

;; Update conviction parameters (governance-controlled)
(define-public (update-conviction-parameters 
               (new-decay-constant (optional uint))
               (new-conviction-threshold (optional uint))
               (new-min-stake (optional uint))
               (new-max-inactive-blocks (optional uint)))
  (begin
    ;; Check if caller is the governance contract or authenticated by it
    (asserts! (contract-call? GOVERNANCE_CONTRACT is-valid-signer tx-sender) 
              (contract-call? dao-constants get-error-unauthorized))
    
    ;; Update decay constant if provided
    (match new-decay-constant
      decay (var-set decay-constant decay)
      true)
    
    ;; Update conviction threshold if provided
    (match new-conviction-threshold
      threshold (var-set conviction-threshold threshold)
      true)
    
    ;; Update minimum stake if provided
    (match new-min-stake
      stake (var-set min-stake stake)
      true)
    
    ;; Update max inactive blocks if provided
    (match new-max-inactive-blocks
      blocks (var-set max-inactive-blocks blocks)
      true)
    
    (ok true)
  ))

;; Read-only functions

;; Get proposal details
(define-read-only (get-proposal (proposal-id uint))
  (map-get? proposals proposal-id))

;; Get vote details
(define-read-only (get-vote (proposal-id uint) (voter principal))
  (map-get? votes { proposal-id: proposal-id, voter: voter }))

;; Get voter conviction data
(define-read-only (get-voter-data (voter principal))
  (map-get? voter-conviction voter))

;; Get current conviction parameters
(define-read-only (get-conviction-parameters)
  {
    enabled: (var-get conviction-enabled),
    decay-constant: (var-get decay-constant),
    conviction-threshold: (var-get conviction-threshold),
    min-stake: (var-get min-stake),
    max-inactive-blocks: (var-get max-inactive-blocks)
  })

;; Get the total count of proposals
(define-read-only (get-proposal-count)
  (var-get proposal-count))
