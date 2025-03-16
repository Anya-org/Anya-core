;; Detailed tests for Taproot implementation
(define-private (test-taproot-features)
  (begin
    ;; Test key path spending
    (let ((key-result (contract-call? .dao-core test-key-path-spending)))
      (asserts! (is-ok key-result) (err "Key path spending failed")))
    
    ;; Test script path spending
    (let ((script-result (contract-call? .dao-core test-script-path-spending)))
      (asserts! (is-ok script-result) (err "Script path spending failed")))
    
    ;; Test Schnorr signature aggregation
    (let ((aggr-result (contract-call? .dao-core test-signature-aggregation)))
      (asserts! (is-ok aggr-result) (err "Signature aggregation failed")))
    
    (ok true)
  )
)

;; Execute Taproot tests
(test-taproot-features) 