;; Performance Monitor Tests
(use-trait auth-trait .authorization-trait.authorization-trait)

;; Test setup
(define-constant TEST_ADMIN 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant TEST_MONITOR 'ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG)

;; Test Cases

;; Record Metric Tests
(define-public (test-record-metric)
    (begin
        ;; Setup: Grant monitoring permissions
        (try! (contract-call? .authorization grant-permission TEST_MONITOR "record-metrics"))
        
        ;; Test: Record new metric
        (let
            (
                (result (as-contract TEST_MONITOR
                    (contract-call? .performance-monitor record-metric
                        "cpu-usage"
                        u75
                        u90)))
            )
            (asserts! (is-ok result) (err "Failed to record metric"))
            
            ;; Verify metric was recorded
            (let
                (
                    (stored-metric (contract-call? .performance-monitor get-metric
                        (unwrap-panic result)))
                )
                (asserts! (is-ok stored-metric) (err "Failed to retrieve metric"))
                (asserts! (is-eq (get current-value (unwrap-panic stored-metric)) u75)
                    (err "Invalid metric value"))
                (asserts! (is-eq (get threshold (unwrap-panic stored-metric)) u90)
                    (err "Invalid threshold value"))
            )
            (ok true)
        )
    )
)

;; Update Resource Usage Tests
(define-public (test-update-resource-usage)
    (begin
        ;; Test: Update resource consumption
        (let
            (
                (result (as-contract TEST_MONITOR
                    (contract-call? .performance-monitor update-resource-usage
                        "memory"
                        u1000
                        u600)))
            )
            (asserts! (is-ok result) (err "Failed to update resource usage"))
            
            ;; Verify resource usage was updated
            (let
                (
                    (stored-usage (contract-call? .performance-monitor get-resource-usage
                        "memory"))
                )
                (asserts! (is-ok stored-usage) (err "Failed to retrieve resource usage"))
                (asserts! (is-eq (get allocated (unwrap-panic stored-usage)) u1000)
                    (err "Invalid allocated amount"))
                (asserts! (is-eq (get used (unwrap-panic stored-usage)) u600)
                    (err "Invalid used amount"))
                (asserts! (is-eq (get available (unwrap-panic stored-usage)) u400)
                    (err "Invalid available amount"))
            )
            (ok true)
        )
    )
)

;; Optimize Resources Tests
(define-public (test-optimize-resources)
    (begin
        ;; Setup: Grant optimization permissions
        (try! (contract-call? .authorization grant-permission TEST_ADMIN "optimize-resources"))
        
        ;; Test: Optimize system resources
        (let
            (
                (result (as-contract TEST_ADMIN
                    (contract-call? .performance-monitor optimize-resources)))
            )
            (asserts! (is-ok result) (err "Failed to optimize resources"))
            
            ;; Verify optimization effects
            (let
                (
                    (memory-usage (contract-call? .performance-monitor get-resource-usage
                        "memory"))
                )
                (asserts! (is-ok memory-usage) (err "Failed to retrieve resource usage"))
                (ok true)
            )
        )
    )
)

;; Alert Resolution Tests
(define-public (test-resolve-alert)
    (begin
        ;; Setup: Grant alert management permissions
        (try! (contract-call? .authorization grant-permission TEST_ADMIN "manage-alerts"))
        
        ;; Create a test alert first
        (try! (as-contract TEST_MONITOR
            (contract-call? .performance-monitor record-metric
                "disk-space"
                u95
                u90)))
        
        ;; Test: Resolve the alert
        (let
            (
                (result (as-contract TEST_ADMIN
                    (contract-call? .performance-monitor resolve-alert u1)))
            )
            (asserts! (is-ok result) (err "Failed to resolve alert"))
            
            ;; Verify alert was resolved
            (let
                (
                    (stored-alert (contract-call? .performance-monitor get-alert u1))
                )
                (asserts! (is-ok stored-alert) (err "Failed to retrieve alert"))
                (asserts! (get resolved (unwrap-panic stored-alert))
                    (err "Alert not marked as resolved"))
            )
            (ok true)
        )
    )
)

;; Edge Cases and Error Tests
(define-public (test-error-cases)
    (begin
        ;; Test: Record metric without permission
        (let
            (
                (result (contract-call? .performance-monitor record-metric
                    "test"
                    u1
                    u100))
            )
            (asserts! (is-err result) (err "Should fail without permission"))
            
            ;; Test: Resolve non-existent alert
            (let
                (
                    (result (as-contract TEST_ADMIN
                        (contract-call? .performance-monitor resolve-alert u999)))
                )
                (asserts! (is-err result) (err "Should fail for non-existent alert"))
                
                ;; Test: Update invalid resource
                (let
                    (
                        (result (as-contract TEST_MONITOR
                            (contract-call? .performance-monitor update-resource-usage
                                "invalid"
                                u0
                                u1)))
                    )
                    (asserts! (is-err result) (err "Should fail for invalid resource"))
                    (ok true)
                )
            )
        )
    )
)

;; Run all tests
(define-public (run-all-tests)
    (begin
        (try! (test-record-metric))
        (try! (test-update-resource-usage))
        (try! (test-optimize-resources))
        (try! (test-resolve-alert))
        (try! (test-error-cases))
        (ok true)
    )
)