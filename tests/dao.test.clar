;; [DEPRECATED] Main DAO Contract Test - Development Version
;;
;; ⚠️  WARNING: These tests are for deprecated development contract.
;; ⚠️  For PRODUCTION testing use: contracts/dao/ test files
;;
;; MIGRATION REQUIRED: Use contracts/dao/governance.clar for production

;; Test main DAO contract - DEPRECATED functionality
(define-private (test-dao-contract)
  (begin
    ;; Test creating a proposal through main DAO contract - DEPRECATED
    (let ((proposal-result (contract-call? .dao submit-proposal
                             "Main DAO Proposal"
                             "This is a proposal through the main DAO"
                             u10080)))
      (asserts! (is-ok proposal-result) (err "DEPRECATED: Main DAO proposal creation failed")))

    ;; Test DAO-controlled DEX interaction - DEPRECATED
    (let ((add-liquidity-result (contract-call? .dao add-treasury-liquidity u1000 u500)))
      (asserts! (is-ok add-liquidity-result) (err "DEPRECATED: DAO-controlled liquidity addition failed")))

    ;; Test governance parameter updates - DEPRECATED
    (let ((update-result (contract-call? .dao update-proposal-threshold u200)))
      (asserts! (is-ok update-result) (err "DEPRECATED: Updating proposal threshold failed"))
      (let ((new-threshold (contract-call? .dao get-proposal-threshold)))
        (asserts! (is-eq new-threshold u200) (err "DEPRECATED: New threshold not reflected"))))

    (ok "MIGRATION REQUIRED: Use contracts/dao/governance.clar for production")
  )
)

;; Execute deprecated tests - Remove after migration
(test-dao-contract)

