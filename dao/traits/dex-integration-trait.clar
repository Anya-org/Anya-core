;; DEX Integration Trait
;; Defines the interface for DEX interactions with the DAO

(define-trait dex-integration-trait
    (
        ;; Liquidity Management
        (provide-liquidity (uint uint) (response bool uint))
        (remove-liquidity (uint) (response { stx-amount: uint, token-amount: uint } uint))
        
        ;; Trading Operations
        (swap-tokens-for-stx (uint) (response uint uint))
        (swap-stx-for-tokens (uint) (response uint uint))
        
        ;; Price Oracle
        (get-token-price () (response uint uint))
        (get-liquidity-info () (response { 
            stx-reserve: uint, 
            token-reserve: uint, 
            liquidity-tokens: uint 
        } uint))
        
        ;; Market Making
        (update-fee-percentage (uint) (response bool uint))
        (get-fee-percentage () (response uint uint))
        
        ;; DAO Integration
        (set-dao-contract (principal) (response bool uint))
        (execute-dao-instruction ((string-ascii 20) (list 10 {key: (string-ascii 64), value: (optional-none)})) (response bool uint))
        
        ;; Analytics
        (get-volume-24h () (response uint uint))
        (get-trades-count () (response uint uint))
        (get-price-impact (uint) (response uint uint))
    )
) 