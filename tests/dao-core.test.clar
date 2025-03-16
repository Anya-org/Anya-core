;; Test proposal creation, voting, and execution
(define-private (test-dao-governance)
  (begin
    ;; Create a proposal
    (let ((proposal-result (contract-call? .dao-core submit-proposal 
                              "Test Proposal" 
                              "This is a test proposal" 
                              u10080)))
      (asserts! (is-ok proposal-result) (err "Proposal creation failed"))
      (let ((proposal-id (unwrap-panic proposal-result)))
        
        ;; Vote on the proposal
        (let ((vote-result (contract-call? .dao-core vote-on-proposal proposal-id true)))
          (asserts! (is-ok vote-result) (err "Voting failed")))
        
        ;; Check proposal status
        (let ((proposal-info (contract-call? .dao-core get-proposal proposal-id)))
          (asserts! (is-ok proposal-info) (err "Failed to get proposal info"))
          (let ((proposal-data (unwrap-panic proposal-info)))
            (asserts! (is-eq (get title proposal-data) "Test Proposal") 
                      (err "Proposal title mismatch"))
            (asserts! (is-eq (get yes-votes proposal-data) u1) 
                      (err "Yes votes not counted correctly"))))
        
        ;; Test proposal execution (would need block manipulation in a real test)
        (ok true)))
  )
)

;; Execute tests
(test-dao-governance) 