;; Detailed tests for PSBT implementation
(define-private (test-psbt-features)
  (begin
    ;; Test PSBT creation
    (let ((creation-result (contract-call? .dao-core test-create-psbt)))
      (asserts! (is-ok creation-result) (err "PSBT creation failed")))
    
    ;; Test PSBT signing
    (let ((signing-result (contract-call? .dao-core test-sign-psbt)))
      (asserts! (is-ok signing-result) (err "PSBT signing failed")))
    
    ;; Test PSBT finalization
    (let ((final-result (contract-call? .dao-core test-finalize-psbt)))
      (asserts! (is-ok final-result) (err "PSBT finalization failed")))
    
    ;; Test PSBT extraction
    (let ((extract-result (contract-call? .dao-core test-extract-psbt)))
      (asserts! (is-ok extract-result) (err "PSBT extraction failed")))
    
    (ok true)
  )
)

;; Execute PSBT tests
(test-psbt-features) 