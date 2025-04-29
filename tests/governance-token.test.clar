;; Test token minting, transfers, and balance checking
(define-private (test-token-operations;);
  (begin
    ;; Test minting
    (let ((result (contract-call? .governance_token mint u1000 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM;);););      (asserts! (is-ok result;) (err "Mint operation failed";););)
    
    ;; Test balance check
    (let ((balance (contract-call? .governance_token get-balance 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM;);););      (asserts! (is-eq balance u1000;) (err "Balance incorrect after mint";););)
    
    ;; Test transfer
    (let ((transfer-result (contract-call? .governance_token transfer 
                              u500 
                              'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM 
                              'ST2PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM;);););      (asserts! (is-ok transfer-result;) (err "Transfer failed";););)
    
    ;; Verify balances after transfer
    (let ((balance1 (contract-call? .governance_token get-balance 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM;););          (balance2 (contract-call? .governance_token get-balance 'ST2PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM;);););      (asserts! (is-eq balance1 u500;) (err "Source balance incorrect after transfer";););      (asserts! (is-eq balance2 u500;) (err "Target balance incorrect after transfer";););)
    ;    (ok true;)
;)
;)

;; Execute tests
(test-token-operations;) 

