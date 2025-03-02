 ;; Bitcoin-Style Token Issuance Module
;; Implements a hybrid issuance model with:
;; 1. Initial 6-month distribution (45% of total supply)
;; 2. Bitcoin-like halving schedule after the initial period

;; Import traits
(use-trait ft-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)
(use-trait dao-trait 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-trait.dao-trait)

;; Error codes
(define-constant ERR_UNAUTHORIZED u1001)
(define-constant ERR_ALREADY_INITIALIZED u1002)
(define-constant ERR_NOT_INITIALIZED u1003)
(define-constant ERR_INVALID_PARAMETER u1004)
(define-constant ERR_ZERO_AMOUNT u1005)
(define-constant ERR_INSUFFICIENT_BALANCE u1006)
(define-constant ERR_EXCEEDS_AVAILABLE u1007)

;; Constants - Token Distribution
(define-constant GENESIS_BLOCK u0)
(define-constant INITIAL_RELEASE_BLOCKS u240000) ;; ~6 months in blocks (assuming 10-minute blocks)
(define-constant INITIAL_RELEASE_PERCENTAGE u45) ;; 45% released in first 6 months
(define-constant TOTAL_SUPPLY u2100000000000000) ;; 21 million tokens with 8 decimal places
(define-constant HALVING_INTERVAL u210000) ;; Bitcoin halving interval (every 210,000 blocks)
(define-constant INITIAL_BLOCK_REWARD u5000000000) ;; 50 tokens per block (with 8 decimal places)

;; Data Variables
(define-data-var distribution-start-block uint u0)
(define-data-var tokens-distributed uint u0)
(define-data-var current-block-reward uint INITIAL_BLOCK_REWARD)
(define-data-var last-halving-height uint u0)
(define-data-var is-initialized bool false)
(define-data-var is-initial-distribution bool true)
(define-data-var initial-phase-released uint u0)
(define-data-var regular-phase-released uint u0)

;; Token contract reference
(define-data-var token-contract principal 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.governance_token)
(define-data-var dao-contract principal 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao)

;; Admin management
(define-data-var owner principal tx-sender)
(define-map administrators principal bool)

;; Initialize administrators
(map-set administrators tx-sender true)

;; Authorization check
(define-private (is-authorized (caller principal))
    (default-to false (map-get? administrators caller))
)

;; Initialize the issuance system
(define-public (initialize (start-block uint) (token principal) (dao principal))
    (begin
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (asserts! (not (var-get is-initialized)) (err ERR_ALREADY_INITIALIZED))
        
        ;; Set initial values
        (var-set distribution-start-block start-block)
        (var-set token-contract token)
        (var-set dao-contract dao)
        (var-set is-initialized true)
        (var-set last-halving-height start-block)
        
        ;; Calculate initial phase allocation (45% of total supply)
        (var-set initial-phase-released u0)
        
        (ok true)
    )
)

;; Calculate the current block reward based on Bitcoin halving schedule
(define-read-only (get-current-block-reward)
    (let (
        (current-block block-height)
        (start-block (var-get distribution-start-block))
        (halvings (/ (- current-block start-block) HALVING_INTERVAL))
    )
        (if (< current-block start-block)
            ;; Not started yet
            u0
            (if (var-get is-initial-distribution)
                ;; During initial 6-month distribution
                (get-initial-phase-reward)
                ;; Regular Bitcoin-style halving schedule
                (/ INITIAL_BLOCK_REWARD (pow u2 halvings))
            )
        )
    )
)

;; Calculate reward during initial phase (45% over 6 months)
(define-read-only (get-initial-phase-reward)
    (let (
        (current-block block-height)
        (start-block (var-get distribution-start-block))
        (end-block (+ start-block INITIAL_RELEASE_BLOCKS))
        (initial-allocation (/ (* TOTAL_SUPPLY INITIAL_RELEASE_PERCENTAGE) u100))
        (blocks-remaining (- end-block current-block))
        (total-blocks INITIAL_RELEASE_BLOCKS)
        (released-so-far (var-get initial-phase-released))
    )
        (if (>= current-block end-block)
            ;; Initial phase complete
            u0
            (if (>= released-so-far initial-allocation)
                ;; Already released all allocated tokens
                u0
                ;; Calculate remaining per block (linear distribution)
                (/ (- initial-allocation released-so-far) blocks-remaining)
            )
        )
    )
)

;; Check if a halving should occur
(define-private (check-halving)
    (let (
        (current-block block-height)
        (start-block (var-get distribution-start-block))
        (last-halving (var-get last-halving-height))
        (blocks-since-start (- current-block start-block))
        (current-halving-era (/ blocks-since-start HALVING_INTERVAL))
        (last-halving-era (/ (- last-halving start-block) HALVING_INTERVAL))
    )
        (if (and (> current-block start-block) (> current-halving-era last-halving-era))
            (begin
                ;; Update last halving height
                (var-set last-halving-height current-block)
                ;; Update block reward
                (var-set current-block-reward (/ INITIAL_BLOCK_REWARD (pow u2 current-halving-era)))
                true
            )
            false
        )
    )
)

;; Check if we should transition from initial to regular distribution
(define-private (check-phase-transition)
    (let (
        (current-block block-height)
        (start-block (var-get distribution-start-block))
        (initial-end-block (+ start-block INITIAL_RELEASE_BLOCKS))
    )
        (if (and (var-get is-initial-distribution) (>= current-block initial-end-block))
            (begin
                (var-set is-initial-distribution false)
                ;; Initialize the regular phase with Bitcoin-style halving
                (var-set current-block-reward INITIAL_BLOCK_REWARD)
                (var-set last-halving-height current-block)
                true
            )
            false
        )
    )
)

;; Calculate available tokens to mint
(define-read-only (get-available-to-mint)
    (let (
        (current-block block-height)
        (start-block (var-get distribution-start-block))
        (total-distributed (var-get tokens-distributed))
    )
        (if (< current-block start-block)
            ;; Distribution hasn't started
            u0
            (if (var-get is-initial-distribution)
                ;; Initial 6-month phase
                (get-initial-phase-available)
                ;; Regular Bitcoin-style phase
                (get-bitcoin-phase-available)
            )
        )
    )
)

;; Calculate available tokens in the initial phase
(define-read-only (get-initial-phase-available)
    (let (
        (current-block block-height)
        (start-block (var-get distribution-start-block))
        (end-block (+ start-block INITIAL_RELEASE_BLOCKS))
        (initial-allocation (/ (* TOTAL_SUPPLY INITIAL_RELEASE_PERCENTAGE) u100))
        (released-so-far (var-get initial-phase-released))
    )
        (if (>= current-block end-block)
            ;; Initial phase complete, return remaining allocation if any
            (if (< released-so-far initial-allocation)
                (- initial-allocation released-so-far)
                u0
            )
            ;; Initial phase still active, calculate based on block progression
            (let (
                (blocks-elapsed (- current-block start-block))
                (total-phase-blocks (- end-block start-block))
                (expected-release (/ (* initial-allocation blocks-elapsed) total-phase-blocks))
            )
                (if (< released-so-far expected-release)
                    (- expected-release released-so-far)
                    u0
                )
            )
        )
    )
)

;; Calculate available tokens in the Bitcoin-style phase
(define-read-only (get-bitcoin-phase-available)
    (let (
        (current-block block-height)
        (reward (get-current-block-reward))
    )
        ;; In Bitcoin-style issuance, only the current block reward is available
        reward
    )
)

;; Mint tokens according to the issuance schedule
(define-public (mint-tokens (recipient principal))
    (let (
        (token-contract-principal (var-get token-contract))
        (available (get-available-to-mint))
    )
        ;; Validate inputs
        (asserts! (var-get is-initialized) (err ERR_NOT_INITIALIZED))
        (asserts! (> available u0) (err ERR_ZERO_AMOUNT))
        
        ;; Check for phase transition
        (check-phase-transition)
        
        ;; Check for halving (only in regular phase)
        (if (not (var-get is-initial-distribution))
            (check-halving)
            false
        )
        
        ;; Update distribution tracking
        (if (var-get is-initial-distribution)
            (var-set initial-phase-released (+ (var-get initial-phase-released) available))
            (var-set regular-phase-released (+ (var-get regular-phase-released) available))
        )
        
        ;; Update total distributed
        (var-set tokens-distributed (+ (var-get tokens-distributed) available))
        
        ;; Mint tokens to recipient
        (as-contract (contract-call? token-contract-principal mint available recipient))
        
        (ok available)
    )
)

;; Get issuance statistics
(define-read-only (get-issuance-stats)
    {
        total-supply: TOTAL_SUPPLY,
        distributed: (var-get tokens-distributed),
        initial-phase-released: (var-get initial-phase-released),
        regular-phase-released: (var-get regular-phase-released),
        current-block-reward: (get-current-block-reward),
        is-initial-phase: (var-get is-initial-distribution),
        distribution-start: (var-get distribution-start-block),
        initial-phase-end: (+ (var-get distribution-start-block) INITIAL_RELEASE_BLOCKS),
        last-halving: (var-get last-halving-height)
    }
)

;; Calculate the next halving block
(define-read-only (get-next-halving-block)
    (let (
        (start-block (var-get distribution-start-block))
        (current-block block-height)
        (blocks-since-start (- current-block start-block))
        (current-era (/ blocks-since-start HALVING_INTERVAL))
    )
        (+ start-block (* (+ current-era u1) HALVING_INTERVAL))
    )
)

;; Calculate the current halving era
(define-read-only (get-current-halving-era)
    (let (
        (start-block (var-get distribution-start-block))
        (current-block block-height)
        (blocks-since-start (- current-block start-block))
    )
        (/ blocks-since-start HALVING_INTERVAL)
    )
)

;; Helper function for exponentiation
(define-private (pow (base uint) (exponent uint))
    (fold pow-iter (list-repeat exponent true) base)
)

;; Helper function for exponentiation iteration
(define-private (pow-iter (ignored bool) (acc uint))
    (* acc acc)
)

;; Add an administrator
(define-public (add-administrator (new-admin principal))
    (begin
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (map-set administrators new-admin true)
        (ok true)
    )
)

;; Remove an administrator
(define-public (remove-administrator (admin principal))
    (begin
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (map-set administrators admin false)
        (ok true)
    )
)

;; Set the token contract
(define-public (set-token-contract (new-token-contract principal))
    (begin
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (var-set token-contract new-token-contract)
        (ok true)
    )
)

;; Set the DAO contract
(define-public (set-dao-contract (new-dao-contract principal))
    (begin
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (var-set dao-contract new-dao-contract)
        (ok true)
    )
)