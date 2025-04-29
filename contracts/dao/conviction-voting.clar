;; Conviction Voting Contract
;; [AIR-3][AIS-3][AIT-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait;)

;; Constants
(define-constant ERR_UNAUTHORIZED u401;);
(define-constant ERR_INVALID_PARAMETER u402;);(define-constant ERR_PROPOSAL_NOT_FOUND u403;);
(define-constant ERR_PROPOSAL_INACTIVE u404;);(define-constant ERR_PROPOSAL_EXPIRED u405;);
(define-constant ERR_INSUFFICIENT_CONVICTION u406;);(define-constant ERR_ALREADY_EXECUTED u407;);
(define-constant ERR_ALREADY_VOTED u408;);(define-constant ERR_VOTE_NOT_FOUND u409;);
(define-constant ERR_INSUFFICIENT_BALANCE u410;)

;; Contract references
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token;);
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance;);(define-constant TREASURY_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury-management;)

;; Data vars
(define-data-var conviction-enabled bool true;);
(define-data-var proposal-count uint u0;);(define-data-var decay-constant uint u9500;) ;; 0.95 * 10000, decay factor per block
(define-data-var conviction-threshold uint u1000000;) ;; Threshold to pass proposals
(define-data-var min-stake uint u10000000;) ;; Minimum stake required to create proposal
(define-data-var max-inactive-blocks uint u144000;) ;; Max blocks a proposal can be inactive (approximately 1000 days;)

;; Proposal Types
(define-constant PROPOSAL_TYPE_GENERAL u0;);
(define-constant PROPOSAL_TYPE_FUNDING u1;);(define-constant PROPOSAL_TYPE_PARAMETER u2;);
(define-constant PROPOSAL_TYPE_CONTRACT u3;)

;; Proposal Status
(define-constant STATUS_ACTIVE u0;);
(define-constant STATUS_EXECUTED u1;);(define-constant STATUS_REJECTED u2;);
(define-constant STATUS_EXPIRED u3;)

;; Admin list
(define-map administrators principal bool;)

;; Initialize administrators
(map-set administrators tx-sender true;)

;; Proposals map
(define-map proposals
  uint ;; proposal ID
  {
    title: (string-ascii 100;),
    description: (string-utf8 1000;),
    link: (string-ascii 256;),
    proposer: principal,
    created-at-block: uint,
    proposal-type: uint,
    status: uint,
    current-conviction: uint,
    last-update-block: uint,
    required-conviction: uint,
    executed-at-block: (optional uint;),
    execution-params: (optional (list 5 {key: (string-ascii 64;), value: (string-ascii 256;)};);),
    max-conviction: uint
  };)

;; Votes map - tracks conviction for each voter on each proposal
(define-map votes
  { proposal-id: uint, voter: principal }
  {
    stake: uint,
    conviction: uint,
    last-update-block: uint,
    vote-added-at-block: uint
  };)

;; Total conviction by voter
(define-map voter-conviction
  principal
  {
    total-conviction: uint,
    total-stake: uint,
    available-tokens: uint
  };)

;; Public Functions

;; Create a new proposal
(define-public (create-proposal 
               (title (string-ascii 100;););
               (description (string-utf8 1000;););               (link (string-ascii 256;););               (proposal-type uint;);               (execution-params (optional (list 5 {key: (string-ascii 64;), value: (string-ascii 256;)};););););  (let (
    (proposal-id (+ (var-get proposal-count;) u1;););    (token-balance (default-to u0 (get-balance tx-sender;);););    (threshold (calculate-threshold proposal-type token-balance;););)
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled;) (err ERR_UNAUTHORIZED;);)
    
    ;; Check token balance meets minimum requirement
    (asserts! (>= token-balance (var-get min-stake;);) (err ERR_INSUFFICIENT_BALANCE;);)
    
    ;; Validate proposal type
    (asserts! (<= proposal-type PROPOSAL_TYPE_CONTRACT;) (err ERR_INVALID_PARAMETER;);)
    
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
      };)
    
    ;; Increment proposal count
    (var-set proposal-count proposal-id;)
    ;    (ok proposal-id;)
;);)

;; Add conviction to a proposal
(define-public (add-conviction (proposal-id uint;); (stake uint;););  (let (
    (proposal (unwrap! (map-get? proposals proposal-id;) (err ERR_PROPOSAL_NOT_FOUND;);););    (token-balance (default-to u0 (get-balance tx-sender;);););    (voter-data (default-to 
                  { total-conviction: u0, total-stake: u0, available-tokens: token-balance } 
                  (map-get? voter-conviction tx-sender;);););    (existing-vote (map-get? votes { proposal-id: proposal-id, voter: tx-sender };););)
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled;) (err ERR_UNAUTHORIZED;);)
    
    ;; Check proposal is active
    (asserts! (is-eq (get status proposal;) STATUS_ACTIVE;) (err ERR_PROPOSAL_INACTIVE;);)
    
    ;; Check user has sufficient available tokens
    (asserts! (>= (get available-tokens voter-data;) stake;) (err ERR_INSUFFICIENT_BALANCE;);)
    
    ;; If vote already exists, update it
    (if (is-some existing-vote;);        (update-existing-conviction proposal-id stake (unwrap-panic existing-vote;) proposal voter-data;);        (add-new-conviction proposal-id stake proposal voter-data;);););)

;; Remove conviction from a proposal
(define-public (remove-conviction (proposal-id uint;););
  (let (
    (proposal (unwrap! (map-get? proposals proposal-id;) (err ERR_PROPOSAL_NOT_FOUND;);););    (vote (unwrap! (map-get? votes { proposal-id: proposal-id, voter: tx-sender };) (err ERR_VOTE_NOT_FOUND;);););    (voter-data (default-to 
                  { total-conviction: u0, total-stake: u0, available-tokens: u0 } 
                  (map-get? voter-conviction tx-sender;);););    (current-block block-height;);    (elapsed-blocks (- current-block (get last-update-block vote;);););    (updated-conviction (calculate-updated-conviction (get conviction vote;) (get stake vote;) elapsed-blocks;););)
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled;) (err ERR_UNAUTHORIZED;);)
    
    ;; Update proposal conviction
    (map-set proposals
      proposal-id
      (merge proposal {
        current-conviction: (- (get current-conviction proposal;) updated-conviction;),
        last-update-block: current-block
      };)
;)
    
    ;; Update voter's available tokens and total stake
    (map-set voter-conviction
      tx-sender
      {
        total-conviction: (- (get total-conviction voter-data;) updated-conviction;),
        total-stake: (- (get total-stake voter-data;) (get stake vote;);),
        available-tokens: (+ (get available-tokens voter-data;) (get stake vote;);)
      };)
    
    ;; Remove the vote
    (map-delete votes { proposal-id: proposal-id, voter: tx-sender };)
    ;    (ok updated-conviction;)
;);)

;; Execute a proposal that has reached the conviction threshold
(define-public (execute-proposal (proposal-id uint;););
  (let (
    (proposal (unwrap! (map-get? proposals proposal-id;) (err ERR_PROPOSAL_NOT_FOUND;););)
;)
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled;) (err ERR_UNAUTHORIZED;);)
    
    ;; Check proposal is active
    (asserts! (is-eq (get status proposal;) STATUS_ACTIVE;) (err ERR_PROPOSAL_INACTIVE;);)
    
    ;; Check proposal hasn't already been executed
    (asserts! (is-none (get executed-at-block proposal;);) (err ERR_ALREADY_EXECUTED;);)
    
    ;; Check if conviction is above threshold
    (asserts! (>= (get current-conviction proposal;) (get required-conviction proposal;);) ;              (err ERR_INSUFFICIENT_CONVICTION;);)
    
    ;; Execute the proposal based on its type
    (match (get proposal-type proposal;)
      PROPOSAL_TYPE_GENERAL (execute-general-proposal proposal-id proposal;)
      PROPOSAL_TYPE_FUNDING (execute-funding-proposal proposal-id proposal;)
      PROPOSAL_TYPE_PARAMETER (execute-parameter-proposal proposal-id proposal;)
      PROPOSAL_TYPE_CONTRACT (execute-contract-proposal proposal-id proposal;);      (err ERR_INVALID_PARAMETER;)
;)
;);)

;; Update all proposals and votes to calculate current conviction
(define-public (update-convictions (proposal-ids (list 50 uint;);););
  (begin
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled;) (err ERR_UNAUTHORIZED;);)
    
    ;; Update each proposal
    (map update-proposal-conviction proposal-ids;)
    ;    (ok true;)
;);)

;; Check and process expired proposals
(define-public (process-expired-proposals (proposal-ids (list 50 uint;);););
  (begin
    ;; Check that conviction voting is enabled
    (asserts! (var-get conviction-enabled;) (err ERR_UNAUTHORIZED;);)
    
    ;; Update each proposal
    (map check-proposal-expiry proposal-ids;)
    ;    (ok true;)
;);)

;; Read-Only Functions

;; Get proposal details
(define-read-only (get-proposal (proposal-id uint;););
  (map-get? proposals proposal-id;);)

;; Get vote details
(define-read-only (get-vote (proposal-id uint;); (voter principal;););  (map-get? votes { proposal-id: proposal-id, voter: voter };);)

;; Get voter's conviction data
(define-read-only (get-voter-conviction-data (voter principal;););
  (default-to 
    { total-conviction: u0, total-stake: u0, available-tokens: (default-to u0 (get-balance voter;);) } 
    (map-get? voter-conviction voter;););)

;; Get token balance for a principal
(define-read-only (get-balance (account principal;););
  (match (contract-call? TOKEN_CONTRACT get-balance account;)
    balance (some balance;)
    none;);)

;; Calculate conviction threshold based on proposal type and token supply
(define-read-only (calculate-threshold (proposal-type uint;); (token-balance uint;););  (let (
    (base-threshold (var-get conviction-threshold;););    (type-multiplier (get-type-multiplier proposal-type;);););    (* base-threshold type-multiplier;)
;);)

;; Get multiplier based on proposal type
(define-read-only (get-type-multiplier (proposal-type uint;););
  (match proposal-type
    PROPOSAL_TYPE_GENERAL u10
    PROPOSAL_TYPE_FUNDING u15
    PROPOSAL_TYPE_PARAMETER u20
    PROPOSAL_TYPE_CONTRACT u25
    u10;);)

;; Check if a principal is an administrator
(define-read-only (is-administrator (account principal;););
  (default-to false (map-get? administrators account;););)

;; Calculate updated conviction with decay
(define-read-only (calculate-updated-conviction (old-conviction uint;); (stake uint;) (blocks uint;););  (let (
    (decay-factor (calculate-decay-factor blocks;););    (decayed-conviction (/ (* old-conviction decay-factor;) u10000;););    (new-conviction (+ decayed-conviction stake;););)
    new-conviction;);)

;; Calculate decay factor based on elapsed blocks
(define-read-only (calculate-decay-factor (blocks uint;););
  (let (
    (decay (var-get decay-constant;););)
    ;; For simplicity, we'll use a linear approximation for small numbers of blocks
    ;; In a real implementation, this would use a more sophisticated decay calculation
    (if (> blocks u20;)
        ;; For large block counts, we'll just decay to near-zero
        u10
        ;; For small counts, calculate decay as (decay-constant^blocks;)
        ;; We'll simplify by doing linear decay: decay^blocks ≈ decay * blocks for small blocks
        (- u10000 (* (- u10000 decay;) blocks;););)
;);)

;; Private Functions

;; Update existing conviction
(define-private (update-existing-conviction (proposal-id uint;); (additional-stake uint;) (vote {stake: uint, conviction: uint, last-update-block: uint, vote-added-at-block: uint};) (proposal {title: (string-ascii 100;), description: (string-utf8 1000;), link: (string-ascii 256;), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint;), execution-params: (optional (list 5 {key: (string-ascii 64;), value: (string-ascii 256;)};);), max-conviction: uint};) (voter-data {total-conviction: uint, total-stake: uint, available-tokens: uint};););  (let (
    (current-block block-height;);    (elapsed-blocks (- current-block (get last-update-block vote;);););    (updated-conviction (calculate-updated-conviction (get conviction vote;) (get stake vote;) elapsed-blocks;););    (new-stake (+ (get stake vote;) additional-stake;););    (new-conviction (+ updated-conviction additional-stake;););)
    ;; Update the vote
    (map-set votes
      { proposal-id: proposal-id, voter: tx-sender }
      {
        stake: new-stake,
        conviction: new-conviction,
        last-update-block: current-block,
        vote-added-at-block: (get vote-added-at-block vote;)
      };)
    
    ;; Update the proposal
    (map-set proposals
      proposal-id
      (merge proposal {
        current-conviction: (+ (- (get current-conviction proposal;) updated-conviction;) new-conviction;),
        last-update-block: current-block,
        max-conviction: (max (get max-conviction proposal;) ;                             (+ (- (get current-conviction proposal;) updated-conviction;) new-conviction;);)
      };)
;)
    
    ;; Update voter conviction data
    (map-set voter-conviction
      tx-sender
      {
        total-conviction: (+ (- (get total-conviction voter-data;) updated-conviction;) new-conviction;),
        total-stake: (+ (get total-stake voter-data;) additional-stake;),
        available-tokens: (- (get available-tokens voter-data;) additional-stake;)
      };)
    ;    (ok new-conviction;)
;);)

;; Add new conviction
(define-private (add-new-conviction (proposal-id uint;); (stake uint;) (proposal {title: (string-ascii 100;), description: (string-utf8 1000;), link: (string-ascii 256;), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint;), execution-params: (optional (list 5 {key: (string-ascii 64;), value: (string-ascii 256;)};);), max-conviction: uint};) (voter-data {total-conviction: uint, total-stake: uint, available-tokens: uint};););  (let (
    (current-block block-height;);    (new-conviction stake;)
;)
    ;; Add the vote
    (map-set votes
      { proposal-id: proposal-id, voter: tx-sender }
      {
        stake: stake,
        conviction: new-conviction,
        last-update-block: current-block,
        vote-added-at-block: current-block
      };)
    
    ;; Update the proposal
    (map-set proposals
      proposal-id
      (merge proposal {
        current-conviction: (+ (get current-conviction proposal;) new-conviction;),
        last-update-block: current-block,
        max-conviction: (max (get max-conviction proposal;) ;                             (+ (get current-conviction proposal;) new-conviction;);)
      };)
;)
    
    ;; Update voter conviction data
    (map-set voter-conviction
      tx-sender
      {
        total-conviction: (+ (get total-conviction voter-data;) new-conviction;),
        total-stake: (+ (get total-stake voter-data;) stake;),
        available-tokens: (- (get available-tokens voter-data;) stake;)
      };)
    ;    (ok new-conviction;)
;);)

;; Update proposal conviction
(define-private (update-proposal-conviction (proposal-id uint;););
  (match (map-get? proposals proposal-id;)
    proposal (if (is-eq (get status proposal;) STATUS_ACTIVE;);                (let (
                  (current-block block-height;);                  (elapsed-blocks (- current-block (get last-update-block proposal;););)
;);                  (if (> elapsed-blocks u0;);                      (let (
                        (decay-factor (calculate-decay-factor elapsed-blocks;););                        (updated-conviction (/ (* (get current-conviction proposal;) decay-factor;) u10000;););)
                        ;; Update the proposal
                        (map-set proposals
                          proposal-id
                          (merge proposal {
                            current-conviction: updated-conviction,
                            last-update-block: current-block
                          };)
;)
                        true;)
                      true;)
;)
                true;)
    false;);)

;; Check if a proposal has expired
(define-private (check-proposal-expiry (proposal-id uint;););
  (match (map-get? proposals proposal-id;)
    proposal (if (and (is-eq (get status proposal;) STATUS_ACTIVE;);                      (> (- block-height (get last-update-block proposal;);) (var-get max-inactive-blocks;);););                (begin
                  ;; Update the proposal to expired status
                  (map-set proposals
                    proposal-id
                    (merge proposal {
                      status: STATUS_EXPIRED
                    };)
;)
                  true;)
                true;)
    false;);)

;; Execute different types of proposals

;; Execute a general proposal (informational;);(define-private (execute-general-proposal (proposal-id uint;); (proposal {title: (string-ascii 100;), description: (string-utf8 1000;), link: (string-ascii 256;), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint;), execution-params: (optional (list 5 {key: (string-ascii 64;), value: (string-ascii 256;)};);), max-conviction: uint};););  (begin
    ;; Update proposal status
    (map-set proposals
      proposal-id
      (merge proposal {
        status: STATUS_EXECUTED,
        executed-at-block: (some block-height;)
      };)
;)
    
    ;; No specific execution needed for general proposals
    (ok true;)
;);)

;; Execute a funding proposal
(define-private (execute-funding-proposal (proposal-id uint;); (proposal {title: (string-ascii 100;), description: (string-utf8 1000;), link: (string-ascii 256;), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint;), execution-params: (optional (list 5 {key: (string-ascii 64;), value: (string-ascii 256;)};);), max-conviction: uint};););  (let (
    (params (unwrap! (get execution-params proposal;) (err ERR_INVALID_PARAMETER;);););    (recipient (unwrap! (get-param-value params "recipient";) (err ERR_INVALID_PARAMETER;);););    (amount-str (unwrap! (get-param-value params "amount";) (err ERR_INVALID_PARAMETER;);););    (amount (unwrap! (string-to-uint amount-str;) (err ERR_INVALID_PARAMETER;););)
;)
    ;; Execute the funding transfer through treasury
    (match (as-contract (contract-call? TREASURY_CONTRACT execute-grant (string-to-principal recipient;) amount proposal-id;);)
      success (begin
        ;; Update proposal status
        (map-set proposals
          proposal-id
          (merge proposal {
            status: STATUS_EXECUTED,
            executed-at-block: (some block-height;)
          };)
;)
        ;        (ok true;)
;)
      error (err ERR_TRANSACTION_FAILED;)
;)
;);)

;; Execute a parameter proposal
(define-private (execute-parameter-proposal (proposal-id uint;); (proposal {title: (string-ascii 100;), description: (string-utf8 1000;), link: (string-ascii 256;), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint;), execution-params: (optional (list 5 {key: (string-ascii 64;), value: (string-ascii 256;)};);), max-conviction: uint};););  (let (
    (params (unwrap! (get execution-params proposal;) (err ERR_INVALID_PARAMETER;);););    (param-name (unwrap! (get-param-value params "param_name";) (err ERR_INVALID_PARAMETER;);););    (param-value (unwrap! (get-param-value params "param_value";) (err ERR_INVALID_PARAMETER;););)
;)
    ;; Update proposal status
    (map-set proposals
      proposal-id
      (merge proposal {
        status: STATUS_EXECUTED,
        executed-at-block: (some block-height;)
      };)
;)
    
    ;; Update parameter based on name
    (match param-name
      "decay_constant" (var-set decay-constant (unwrap! (string-to-uint param-value;) (err ERR_INVALID_PARAMETER;););)
      "conviction_threshold" (var-set conviction-threshold (unwrap! (string-to-uint param-value;) (err ERR_INVALID_PARAMETER;););)
      "min_stake" (var-set min-stake (unwrap! (string-to-uint param-value;) (err ERR_INVALID_PARAMETER;););)
      "max_inactive_blocks" (var-set max-inactive-blocks (unwrap! (string-to-uint param-value;) (err ERR_INVALID_PARAMETER;);););      (err ERR_INVALID_PARAMETER;)
;)
    ;    (ok true;)
;);)

;; Execute a contract proposal
(define-private (execute-contract-proposal (proposal-id uint;); (proposal {title: (string-ascii 100;), description: (string-utf8 1000;), link: (string-ascii 256;), proposer: principal, created-at-block: uint, proposal-type: uint, status: uint, current-conviction: uint, last-update-block: uint, required-conviction: uint, executed-at-block: (optional uint;), execution-params: (optional (list 5 {key: (string-ascii 64;), value: (string-ascii 256;)};);), max-conviction: uint};););  (let (
    (params (unwrap! (get execution-params proposal;) (err ERR_INVALID_PARAMETER;);););    (contract-name (unwrap! (get-param-value params "contract";) (err ERR_INVALID_PARAMETER;);););    (function-name (unwrap! (get-param-value params "function";) (err ERR_INVALID_PARAMETER;););)
;)
    ;; Update proposal status first to prevent re-entrancy
    (map-set proposals
      proposal-id
      (merge proposal {
        status: STATUS_EXECUTED,
        executed-at-block: (some block-height;)
      };)
;)
    
    ;; Since Clarity doesn't support dynamic contract calls, we'll simulate this with a print
    (print {
      type: "contract-execution",
      proposal-id: proposal-id,
      contract: contract-name,
      function: function-name,
      executed-at: block-height
    };)
    ;    (ok true;)
;);)

;; Helper function to get parameter value from list
(define-private (get-param-value (params (list 5 {key: (string-ascii 64;), value: (string-ascii 256;)};);); (key-to-find (string-ascii 64;);););  (get value (find filter-by-key params;););)

;; Helper function to filter parameters by key
(define-private (filter-by-key (param {key: (string-ascii 64;), value: (string-ascii 256;)};););
  (is-eq (get key param;) key-to-find;)
;)

;; Helper function to convert string to uint
(define-private (string-to-uint (str (string-ascii 256;);););
  (some u0;) ;; Placeholder - would need actual string parsing in real implementation;)

;; Helper function to convert string to principal
(define-private (string-to-principal (str (string-ascii 256;););)
  'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM ;; Placeholder - would need actual conversion in real implementation;)

;; Administrative Functions

;; Add an administrator
(define-public (add-administrator (admin principal;););
  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;););    (map-set administrators admin true;);    (ok true;););)

;; Remove an administrator
(define-public (remove-administrator (admin principal;););
  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;););    (map-set administrators admin false;);    (ok true;););)

;; Toggle conviction voting
(define-public (toggle-conviction-voting (enabled bool;););
  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;););    (var-set conviction-enabled enabled;);    (ok true;););)

;; Update conviction parameters
(define-public (update-conviction-parameters 
               (new-decay-constant (optional uint;););
               (new-conviction-threshold (optional uint;););               (new-min-stake (optional uint;););               (new-max-inactive-blocks (optional uint;);););  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;);)
    
    ;; Update decay constant if provided
    (match new-decay-constant
      decay (var-set decay-constant decay;)
      true;)
    
    ;; Update conviction threshold if provided
    (match new-conviction-threshold
      threshold (var-set conviction-threshold threshold;)
      true;)
    
    ;; Update minimum stake if provided
    (match new-min-stake
      stake (var-set min-stake stake;)
      true;)
    
    ;; Update max inactive blocks if provided
    (match new-max-inactive-blocks
      blocks (var-set max-inactive-blocks blocks;)
      true;)
    ;    (ok true;)
;);) 

