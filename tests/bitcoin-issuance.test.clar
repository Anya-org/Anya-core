;; Test Bitcoin-style issuance with halving
(define-private (test-bitcoin-issuance)
  (begin
    ;; Test initial issuance
    (let ((issuance-result (contract-call? .bitcoin-issuance get-current-block-reward)))
      (asserts! (is-eq issuance-result u5000) (err "Initial block reward incorrect")))
    
    ;; Test halving calculation
    (let ((blocks-to-halving (contract-call? .bitcoin-issuance get-blocks-to-next-halving)))
      (asserts! (is-eq blocks-to-halving u210000) (err "Blocks to halving incorrect")))
    
    ;; Test distribution percentages
    (let ((distribution (contract-call? .bitcoin-issuance get-distribution-percentages)))
      (asserts! (is-eq (get dex distribution) u35) (err "DEX percentage incorrect"))
      (asserts! (is-eq (get dao distribution) u50) (err "DAO percentage incorrect"))
      (asserts! (is-eq (get security distribution) u15) (err "Security fund percentage incorrect")))
    
    ;; Test mint distribution
    (let ((mint-result (contract-call? .bitcoin-issuance mint-block-reward)))
      (asserts! (is-ok mint-result) (err "Block reward minting failed")))
    
    (ok true)
  )
)

;; Execute tests
(test-bitcoin-issuance) 