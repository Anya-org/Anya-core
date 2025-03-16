;; Test token economics rules
(define-private (test-token-economics)
  (begin
    ;; Test current distribution phase
    (let ((phase (contract-call? .token-economics get-current-phase)))
      (asserts! (is-eq phase u1) (err "Initial phase should be 1")))
    
    ;; Test halving calculation
    (let ((next-halving (contract-call? .token-economics calculate-next-halving-block)))
      (asserts! (is-eq next-halving u210000) (err "Next halving calculation incorrect")))
    
    ;; Test reward calculation
    (let ((reward (contract-call? .token-economics calculate-block-reward)))
      (asserts! (is-eq reward u5000) (err "Block reward calculation incorrect")))
    
    ;; Test developer allocation
    (let ((dev-allocation (contract-call? .token-economics calculate-developer-allocation u1000)))
      (asserts! (is-eq (get top-performer dev-allocation) u300) (err "Top performer allocation incorrect"))
      (asserts! (is-eq (get base-distribution dev-allocation) u500) (err "Base distribution incorrect"))
      (asserts! (is-eq (get bonus-pool dev-allocation) u200) (err "Bonus pool incorrect")))
    
    (ok true)
  )
)

;; Execute tests
(test-token-economics) 