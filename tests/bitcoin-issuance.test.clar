;; [DEPRECATED] Test Bitcoin-style issuance - Development Version
;;
;; ⚠️  WARNING: These tests are for deprecated development contract.
;; ⚠️  For PRODUCTION testing use: contracts/dao/ test files
;;
;; PRODUCTION PARAMETERS:
;; - Block Reward: 10,000 tokens (not 5,000)
;; - Halving: 105,000 blocks (not 210,000)
;; - Distribution: 35% treasury, 25% liquidity, 20% team, 15% community, 5% security

(define-private (test-bitcoin-issuance)
  (begin
    ;; Test initial issuance - DEPRECATED VALUES
    (let ((issuance-result (contract-call? .bitcoin-issuance get-current-block-reward)))
      (asserts! (is-eq issuance-result u5000) (err "DEPRECATED: Use 10,000 in production")))

    ;; Test halving calculation - DEPRECATED VALUES
    (let ((blocks-to-halving (contract-call? .bitcoin-issuance get-blocks-to-next-halving)))
      (asserts! (is-eq blocks-to-halving u210000) (err "DEPRECATED: Use 105,000 in production")))

    ;; DEPRECATED distribution percentages - Production uses different allocation
    (let ((distribution (contract-call? .bitcoin-issuance get-distribution-percentages)))
      (asserts! (is-eq (get dex distribution) u35) (err "DEPRECATED: Production uses 25% liquidity"))
      (asserts! (is-eq (get dao distribution) u50) (err "DEPRECATED: Production uses 35% treasury + 15% community"))
      (asserts! (is-eq (get security distribution) u15) (err "DEPRECATED: Production uses 5% security + 20% team")))

    ;; Test mint distribution - DEPRECATED functionality
    (let ((mint-result (contract-call? .bitcoin-issuance mint-block-reward)))
      (asserts! (is-ok mint-result) (err "DEPRECATED contract functionality")))

    (ok "MIGRATION REQUIRED: Use contracts/dao/tokenomics.clar for production")
  )
)

;; Execute deprecated tests - Remove this after migration
(test-bitcoin-issuance)

