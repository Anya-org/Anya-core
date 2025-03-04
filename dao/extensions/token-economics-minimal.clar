;; Minimal Token Economics file
;; Implements the core constants of the Bitcoin-style tokenomics model

;; Bitcoin-style tokenomics constants
(define-constant TOTAL-SUPPLY u21000000000) ;; 21 billion tokens
(define-constant INITIAL-BLOCK-REWARD u5000) ;; 5,000 tokens per block
(define-constant HALVING-INTERVAL u210000) ;; Halving every 210,000 blocks

;; Distribution percentages (must add up to 100%)
(define-constant DEX-ALLOCATION-PERCENTAGE u30) ;; 30% to DEX
(define-constant TEAM-ALLOCATION-PERCENTAGE u15) ;; 15% to team
(define-constant DAO-ALLOCATION-PERCENTAGE u55) ;; 55% to DAO/community

(define-data-var token-contract principal 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.governance_token)

(define-public (get-total-supply)
    (ok TOTAL-SUPPLY)
)

(define-public (get-block-reward)
    (ok INITIAL-BLOCK-REWARD)
)

(define-public (get-halving-interval)
    (ok HALVING-INTERVAL)
) 