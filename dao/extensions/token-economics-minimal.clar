;; Minimal Token Economics file
(define-constant TOTAL-SUPPLY u100000000000)
(define-data-var token-contract principal 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.governance_token)

(define-public (get-total-supply)
    (ok TOTAL-SUPPLY)
) 