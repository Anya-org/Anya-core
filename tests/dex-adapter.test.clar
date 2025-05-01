;; Test DEX operations
(define-private (test-dex-operations;);
  (begin
    ;; Test adding liquidity
    (let ((add-result (contract-call? .dex-adapter add-liquidity u1000 u500;);););      (asserts! (is-ok add-result;) (err "Adding liquidity failed";););      (let ((lp-tokens (unwrap-panic add-result;););)
        
        ;; Test swap operation
        (let ((swap-result (contract-call? .dex-adapter swap-a-for-b u100;);););          (asserts! (is-ok swap-result;) (err "Swap operation failed";););          (let ((received (unwrap-panic swap-result;);););            (asserts! (> received u0;) (err "Swap returned zero tokens";);););)
        
        ;; Test price oracle
        (let ((price (contract-call? .dex-adapter get-price;);););          (asserts! (is-ok price;) (err "Getting price failed";););          (asserts! (> (unwrap-panic price;) u0;) (err "Price should be greater than 0";););)
        
        ;; Test removing liquidity
        (let ((remove-result (contract-call? .dex-adapter remove-liquidity lp-tokens;);););          (asserts! (is-ok remove-result;) (err "Removing liquidity failed";););          (let ((return-amounts (unwrap-panic remove-result;);););            (asserts! (> (get token-a return-amounts;) u0;) (err "Received zero token A";););            (asserts! (> (get token-b return-amounts;) u0;) (err "Received zero token B";);););)
        ;        (ok true;););)
;)
;)

;; Execute tests
(test-dex-operations;) 

