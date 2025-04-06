;; Ethics Framework Tests
(use-trait auth-trait .authorization-trait.authorization-trait)

;; Test setup
(define-constant TEST_ADMIN 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant TEST_AGENT 'ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG)
(define-constant TEST_REVIEWER 'ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC)

;; Test Cases

;; Add Ethical Principle Tests
(define-public (test-add-ethical-principle)
    (begin
        ;; Setup: Grant admin permissions
        (try! (contract-call? .authorization grant-permission TEST_ADMIN "manage-ethics"))
        
        ;; Test: Add valid principle
        (let 
            (
                (result (contract-call? .ethics-framework add-ethical-principle
                    "fairness"
                    "Ensure fair treatment of all participants"
                    (list "check-bias" "verify-equality")
                    u80
                    "governance"))
            )
            (asserts! (is-ok result) (err "Failed to add principle"))
            (asserts! (is-eq (unwrap-panic result) u1) (err "Invalid principle ID"))
            
            ;; Verify principle was added correctly
            (let 
                (
                    (stored-principle (contract-call? .ethics-framework get-ethical-principle u1))
                )
                (asserts! (is-ok stored-principle) (err "Failed to retrieve principle"))
                (asserts! (is-eq (get name (unwrap-panic stored-principle)) "fairness")
                    (err "Invalid principle name"))
            )
            (ok true)
        )
    )
)

;; Record Decision Tests
(define-public (test-record-decision)
    (begin
        ;; Setup: Grant agent permissions
        (try! (contract-call? .authorization grant-permission TEST_AGENT "record-decisions"))
        
        ;; Test: Record valid decision
        (let
            (
                (result (as-contract TEST_AGENT
                    (contract-call? .ethics-framework record-decision
                        "allocate-resources"
                        (list u1)
                        "approved"
                        false)))
            )
            (asserts! (is-ok result) (err "Failed to record decision"))
            
            ;; Verify decision was recorded
            (let
                (
                    (stored-decision (contract-call? .ethics-framework get-decision 
                        (unwrap-panic result)))
                )
                (asserts! (is-ok stored-decision) (err "Failed to retrieve decision"))
                (asserts! (is-eq (get outcome (unwrap-panic stored-decision)) "approved")
                    (err "Invalid decision outcome"))
            )
            (ok true)
        )
    )
)

;; Update Bias Metric Tests
(define-public (test-update-bias-metric)
    (begin
        ;; Test: Update metric for agent
        (let
            (
                (result (as-contract TEST_AGENT
                    (contract-call? .ethics-framework update-bias-metric
                        u1
                        50)))
            )
            (asserts! (is-ok result) (err "Failed to update bias metric"))
            
            ;; Verify metric was updated
            (let
                (
                    (stored-metric (contract-call? .ethics-framework get-bias-metrics 
                        TEST_AGENT
                        u1))
                )
                (asserts! (is-ok stored-metric) (err "Failed to retrieve metric"))
                (asserts! (is-eq (get value (unwrap-panic stored-metric)) 50)
                    (err "Invalid metric value"))
            )
            (ok true)
        )
    )
)

;; Review Decision Tests
(define-public (test-review-decision)
    (begin
        ;; Setup: Grant reviewer permissions
        (try! (contract-call? .authorization grant-permission TEST_REVIEWER "review-decisions"))
        
        ;; First record a decision that requires review
        (try! (as-contract TEST_AGENT
            (contract-call? .ethics-framework record-decision
                "high-impact-action"
                (list u1)
                "pending"
                true)))
        
        ;; Test: Review the decision
        (let
            (
                (result (as-contract TEST_REVIEWER
                    (contract-call? .ethics-framework review-decision
                        u1
                        true
                        "Decision aligns with ethical principles")))
            )
            (asserts! (is-ok result) (err "Failed to review decision"))
            
            ;; Verify review was recorded
            (let
                (
                    (stored-decision (contract-call? .ethics-framework get-decision u1))
                )
                (asserts! (is-ok stored-decision) (err "Failed to retrieve decision"))
                (asserts! (is-some (get review-notes (unwrap-panic stored-decision)))
                    (err "Review notes not recorded"))
            )
            (ok true)
        )
    )
)

;; Edge Cases and Error Tests
(define-public (test-error-cases)
    (begin
        ;; Test: Add principle without permission
        (let
            (
                (result (as-contract TEST_AGENT
                    (contract-call? .ethics-framework add-ethical-principle
                        "test"
                        "test"
                        (list)
                        u1
                        "test")))
            )
            (asserts! (is-err result) (err "Should fail without permission"))
            
            ;; Test: Review non-existent decision
            (let
                (
                    (result (as-contract TEST_REVIEWER
                        (contract-call? .ethics-framework review-decision
                            u999
                            true
                            "test")))
                )
                (asserts! (is-err result) (err "Should fail for non-existent decision"))
                (ok true)
            )
        )
    )
)

;; Run all tests
(define-public (run-all-tests)
    (begin
        (try! (test-add-ethical-principle))
        (try! (test-record-decision))
        (try! (test-update-bias-metric))
        (try! (test-review-decision))
        (try! (test-error-cases))
        (ok true)
    )
)