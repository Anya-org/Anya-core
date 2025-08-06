;; [DEPRECATED] DAO Core Test - Development Version
;;
;; ⚠️  WARNING: These tests are for deprecated development contract.
;; ⚠️  For PRODUCTION testing use: contracts/dao/ test files
;;
;; MIGRATION REQUIRED: Use contracts/dao/governance.clar for production

;; Test proposal creation, voting, and execution - DEPRECATED
(define-private (test-dao-governance)
  (begin
    ;; Create a proposal - DEPRECATED functionality
    (let ((proposal-result (contract-call? .dao-core submit-proposal
                              "Test Proposal"
                              "This is a test proposal"
                              u10080)))
      (asserts! (is-ok proposal-result) (err "DEPRECATED: Proposal creation failed"))
      (let ((proposal-id (unwrap-panic proposal-result)))

        ;; Vote on the proposal - DEPRECATED
        (let ((vote-result (contract-call? .dao-core vote-on-proposal proposal-id true)))
          (asserts! (is-ok vote-result) (err "DEPRECATED: Voting failed")))

        ;; Check proposal status - DEPRECATED
        (let ((proposal-info (contract-call? .dao-core get-proposal proposal-id)))
          (asserts! (is-ok proposal-info) (err "DEPRECATED: Failed to get proposal info"))
          (let ((proposal-data (unwrap-panic proposal-info)))
            (asserts! (is-eq (get title proposal-data) "Test Proposal")
                      (err "DEPRECATED: Proposal title mismatch"))
            (asserts! (is-eq (get yes-votes proposal-data) u1)
                      (err "DEPRECATED: Yes votes not counted correctly"))))

        ;; Test proposal execution - DEPRECATED functionality
        (ok "MIGRATION REQUIRED: Use contracts/dao/governance.clar for production")))
  )
)

;; Execute deprecated tests - Remove after migration
(test-dao-governance)

