;; Anya DAO Token Economics Module
;; Implements the token distribution logic, including the 6-month initial release phase

;; Constants - Token Distribution
(define-constant TOKEN-GENESIS-BLOCK u0)
(define-constant INITIAL-RELEASE-BLOCKS u240000) ;; ~6 months in blocks
(define-constant INITIAL-RELEASE-PERCENTAGE u45) ;; 45% released in first 6 months
(define-constant TOTAL-SUPPLY u100000000000) ;; 100 billion tokens (with decimals)

;; Error codes
(define-constant ERR_UNAUTHORIZED u1001)
(define-constant ERR_ALREADY_INITIALIZED u1002)
(define-constant ERR_NOT_INITIALIZED u1003)
(define-constant ERR_DISTRIBUTION_NOT_STARTED u1004)
(define-constant ERR_EXCEEDS_AVAILABLE u1005)
(define-constant ERR_INVALID_PHASE u1006)
(define-constant ERR_ZERO_AMOUNT u1007)
(define-constant ERR_INSUFFICIENT_BALANCE u1008)

;; SIP-010 Token Trait
(define-trait ft-trait
  (
    (transfer (uint principal principal (optional (buff 34))) (response bool uint))
    (get-name () (response (string-ascii 32) uint))
    (get-symbol () (response (string-ascii 32) uint))
    (get-decimals () (response uint uint))
    (get-balance (principal) (response uint uint))
    (get-total-supply () (response uint uint))
    (get-token-uri () (response (optional (string-utf8 256)) uint))
    (mint (uint principal) (response bool uint))
    (burn (uint principal) (response bool uint))
  )
)

;; Data Variables - Token Distribution
(define-data-var distribution-start-block uint u0)
(define-data-var tokens-distributed uint u0)
(define-data-var buyback-reserve uint u0)
(define-data-var dex-liquidity-reserve uint u0)
(define-data-var initial-phase-released uint u0)
(define-data-var regular-phase-released uint u0)

;; Distribution Phase Tracking
(define-data-var is-initial-distribution bool true)
(define-map distribution-phases 
    uint 
    { 
        start-block: uint,
        end-block: uint,
        percentage: uint,
        tokens-allocated: uint,
        tokens-released: uint
    }
)

;; Token contract reference
(define-data-var token-contract principal 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.governance_token)

;; Admin management
(define-data-var token-owner principal tx-sender)
(define-map administrators principal bool)

;; Initialize administrators
(map-set administrators tx-sender true)

;; Authorization check
(define-private (is-authorized (caller principal))
    (default-to false (map-get? administrators caller))
)

;; Distribution Initialization
(define-public (initialize-distribution (start-block uint))
    (begin
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (asserts! (is-eq (var-get distribution-start-block) u0) (err ERR_ALREADY_INITIALIZED))
        
        ;; Set distribution start
        (var-set distribution-start-block start-block)
        
        ;; Initialize initial phase (45% in first 6 months)
        (map-set distribution-phases u1 {
            start-block: start-block,
            end-block: (+ start-block INITIAL-RELEASE-BLOCKS),
            percentage: INITIAL-RELEASE-PERCENTAGE,
            tokens-allocated: (/ (* TOTAL-SUPPLY INITIAL-RELEASE-PERCENTAGE) u100),
            tokens-released: u0
        })
        
        ;; Initialize regular distribution phase
        (map-set distribution-phases u2 {
            start-block: (+ start-block INITIAL-RELEASE-BLOCKS),
            end-block: u0, ;; indefinite
            percentage: (- u100 INITIAL-RELEASE-PERCENTAGE),
            tokens-allocated: (/ (* TOTAL-SUPPLY (- u100 INITIAL-RELEASE-PERCENTAGE)) u100),
            tokens-released: u0
        })
        
        (ok true)
    )
)

;; Get current distribution phase
(define-read-only (get-current-phase)
    (let (
        (current-block block-height)
        (distribution-start (var-get distribution-start-block))
        (phase-1 (unwrap-panic (map-get? distribution-phases u1)))
        (phase-2 (unwrap-panic (map-get? distribution-phases u2)))
    )
        (if (<= current-block (get end-block phase-1))
            ;; Still in phase 1
            {
                phase-id: u1,
                start-block: (get start-block phase-1),
                end-block: (get end-block phase-1),
                percentage: (get percentage phase-1),
                tokens-allocated: (get tokens-allocated phase-1),
                tokens-released: (get tokens-released phase-1)
            }
            ;; In phase 2
            {
                phase-id: u2,
                start-block: (get start-block phase-2),
                end-block: (get end-block phase-2),
                percentage: (get percentage phase-2),
                tokens-allocated: (get tokens-allocated phase-2),
                tokens-released: (get tokens-released phase-2)
            }
        )
    )
)

;; Calculate available tokens in the initial phase
(define-read-only (get-initial-phase-available)
    (let (
        (current-block block-height)
        (phase-1 (unwrap-panic (map-get? distribution-phases u1)))
        (start-block (get start-block phase-1))
        (end-block (get end-block phase-1))
        (tokens-allocated (get tokens-allocated phase-1))
        (tokens-released (get tokens-released phase-1))
    )
        (if (> current-block end-block)
            ;; Phase 1 is complete, return remaining allocation if any
            (if (< tokens-released tokens-allocated)
                (- tokens-allocated tokens-released)
                u0
            )
            ;; Phase 1 is still active, calculate based on block progression
            (let (
                (blocks-elapsed (- current-block start-block))
                (total-phase-blocks (- end-block start-block))
                (expected-release (/ (* tokens-allocated blocks-elapsed) total-phase-blocks))
            )
                (if (< tokens-released expected-release)
                    (- expected-release tokens-released)
                    u0
                )
            )
        )
    )
)

;; Calculate available tokens in the regular phase
(define-read-only (get-regular-phase-available)
    (let (
        (current-block block-height)
        (phase-2 (unwrap-panic (map-get? distribution-phases u2)))
        (start-block (get start-block phase-2))
        (tokens-allocated (get tokens-allocated phase-2))
        (tokens-released (get tokens-released phase-2))
    )
        (if (< current-block start-block)
            ;; Regular phase hasn't started yet
            u0
            ;; Linear release after initial phase
            (let (
                (blocks-elapsed (- current-block start-block))
                ;; Approximately 10% per year after initial phase (adjust as needed)
                (annual-blocks u1051200) ;; ~52560 blocks * 20 years
                (expected-release (/ (* tokens-allocated blocks-elapsed) annual-blocks))
            )
                (if (< tokens-released expected-release)
                    (- expected-release tokens-released)
                    u0
                )
            )
        )
    )
)

;; Calculate total available tokens to mint based on distribution schedule
(define-read-only (get-available-to-mint)
    (let (
        (current-block block-height)
        (distribution-start (var-get distribution-start-block))
        (total-distributed (var-get tokens-distributed))
    )
        (if (< current-block distribution-start)
            ;; Distribution hasn't started
            u0
            (if (< current-block (+ distribution-start INITIAL-RELEASE-BLOCKS))
                ;; Initial 6-month phase
                (get-initial-phase-available)
                ;; We're in the regular distribution phase
                (+ (get-initial-phase-available) (get-regular-phase-available))
            )
        )
    )
)

;; Update initial phase distribution tracking
(define-private (update-initial-phase-distribution (amount uint))
    (let (
        (phase-1 (unwrap-panic (map-get? distribution-phases u1)))
        (tokens-released (get tokens-released phase-1))
    )
        (map-set distribution-phases u1 (merge phase-1 { tokens-released: (+ tokens-released amount) }))
        
        ;; Check if we need to transition to regular phase
        (if (>= (+ tokens-released amount) (get tokens-allocated phase-1))
            (var-set is-initial-distribution false)
            true
        )
    )
)

;; Update regular phase distribution tracking
(define-private (update-regular-phase-distribution (amount uint))
    (let (
        (phase-2 (unwrap-panic (map-get? distribution-phases u2)))
        (tokens-released (get tokens-released phase-2))
    )
        (map-set distribution-phases u2 (merge phase-2 { tokens-released: (+ tokens-released amount) }))
        true
    )
)

;; Set buyback reserve amount
(define-public (set-buyback-reserve (amount uint))
    (begin
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (var-set buyback-reserve amount)
        (ok true)
    )
)

;; Set DEX liquidity reserve amount
(define-public (set-dex-liquidity-reserve (amount uint))
    (begin
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (var-set dex-liquidity-reserve amount)
        (ok true)
    )
)

;; Get buyback reserve amount
(define-read-only (get-buyback-reserve)
    (var-get buyback-reserve)
)

;; Get DEX liquidity reserve amount
(define-read-only (get-dex-liquidity-reserve)
    (var-get dex-liquidity-reserve)
)

;; Get total tokens distributed
(define-read-only (get-tokens-distributed)
    (var-get tokens-distributed)
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

;; Check if a principal is an administrator
(define-read-only (is-admin (principal-to-check principal))
    (ok (is-authorized principal-to-check))
)

;; Set the token contract
(define-public (set-token-contract (new-token-contract principal))
    (begin
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (var-set token-contract new-token-contract)
        (ok true)
    )
)