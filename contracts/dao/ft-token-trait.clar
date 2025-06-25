;; FT Token Trait
;; [AIR-3][AIS-3][AIT-3][BPC-3][DAO-3]
;;
;; This trait defines the interface for fungible tokens used in the DAO reward system.

(define-trait ft-token-trait
  (
    ;; Transfer tokens between accounts
    (transfer (uint principal principal (optional (buff 34))) (response bool uint))
    
    ;; Get the token balance for a specific account
    (get-balance (principal) (response uint uint))
    
    ;; Get the total supply of tokens
    (get-total-supply () (response uint uint))
    
    ;; Get the name of the token
    (get-name () (response (string-ascii 32) uint))
    
    ;; Get the symbol for the token
    (get-symbol () (response (string-ascii 10) uint))
    
    ;; Get the number of decimals for the token
    (get-decimals () (response uint uint))
    
    ;; Get the URI for the token metadata
    (get-token-uri () (response (optional (string-utf8 256)) uint))
  )
)
