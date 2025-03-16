;; Test main DAO contract
(define-private (test-dao-contract)
  (begin
    ;; Test creating a proposal through main DAO contract
    (let ((proposal-result (contract-call? .dao submit-proposal 
                             "Main DAO Proposal" 
                             "This is a proposal through the main DAO" 
                             u10080)))
      (asserts! (is-ok proposal-result) (err "Main DAO proposal creation failed")))
    
    ;; Test DAO-controlled DEX interaction
    (let ((add-liquidity-result (contract-call? .dao add-treasury-liquidity u1000 u500)))
      (asserts! (is-ok add-liquidity-result) (err "DAO-controlled liquidity addition failed")))
    
    ;; Test governance parameter updates
    (let ((update-result (contract-call? .dao update-proposal-threshold u200)))
      (asserts! (is-ok update-result) (err "Updating proposal threshold failed"))
      (let ((new-threshold (contract-call? .dao get-proposal-threshold)))
        (asserts! (is-eq new-threshold u200) (err "New threshold not reflected"))))
    
    (ok true)
  )
)

;; Execute tests
(test-dao-contract) 