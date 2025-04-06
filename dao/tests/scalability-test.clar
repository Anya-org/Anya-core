;; Scalability Module Tests
(use-trait auth-trait .authorization-trait.authorization-trait)
(use-trait perf-trait .performance-monitor.performance-monitor-trait)

;; Test setup
(define-constant TEST_ADMIN 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant TEST_OPERATOR 'ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG)

;; Test Cases

;; Scaling Threshold Tests
(define-public (test-set-scaling-threshold)
    (begin
        ;; Setup: Grant scaling management permissions
        (try! (contract-call? .authorization grant-permission TEST_ADMIN "manage-scaling"))
        
        ;; Test: Set valid scaling threshold
        (let
            (
                (result (as-contract TEST_ADMIN
                    (contract-call? .scalability set-scaling-threshold
                        "compute"
                        u1000
                        u10000
                        u10
                        u144
                        true)))
            )
            (asserts! (is-ok result) (err "Failed to set threshold"))
            
            ;; Verify threshold was set
            (let
                (
                    (stored-threshold (contract-call? .scalability get-resource-threshold
                        "compute"))
                )
                (asserts! (is-ok stored-threshold) (err "Failed to retrieve threshold"))
                (asserts! (is-eq (get current-capacity (unwrap-panic stored-threshold)) u1000)
                    (err "Invalid capacity value"))
                (asserts! (is-eq (get max-capacity (unwrap-panic stored-threshold)) u10000)
                    (err "Invalid max capacity"))
            )
            (ok true)
        )
    )
)

;; Resource Allocation Tests
(define-public (test-allocate-resources)
    (begin
        ;; Setup: Grant resource management permissions
        (try! (contract-call? .authorization grant-permission TEST_OPERATOR "manage-resources"))
        
        ;; Test: Allocate resources to component
        (let
            (
                (result (as-contract TEST_OPERATOR
                    (contract-call? .scalability allocate-resources
                        "worker-node"
                        u100
                        u1000
                        u1)))
            )
            (asserts! (is-ok result) (err "Failed to allocate resources"))
            
            ;; Verify allocation
            (let
                (
                    (stored-allocation (contract-call? .scalability get-component-allocation
                        "worker-node"))
                )
                (asserts! (is-ok stored-allocation) (err "Failed to retrieve allocation"))
                (asserts! (is-eq (get min-resources (unwrap-panic stored-allocation)) u100)
                    (err "Invalid min resources"))
                (asserts! (is-eq (get max-resources (unwrap-panic stored-allocation)) u1000)
                    (err "Invalid max resources"))
            )
            (ok true)
        )
    )
)

;; Auto-scaling Tests
(define-public (test-auto-scaling)
    (begin
        ;; Setup: Grant scaling permissions
        (try! (contract-call? .authorization grant-permission TEST_OPERATOR "trigger-scaling"))
        
        ;; First set up a scaling threshold
        (try! (as-contract TEST_ADMIN
            (contract-call? .scalability set-scaling-threshold
                "memory"
                u1000
                u10000
                u10
                u1
                true)))
        
        ;; Test: Trigger auto-scaling
        (let
            (
                (result (as-contract TEST_OPERATOR
                    (contract-call? .scalability trigger-auto-scaling "memory")))
            )
            (asserts! (is-ok result) (err "Failed to trigger auto-scaling"))
            
            ;; Verify scaling occurred
            (let
                (
                    (new-threshold (contract-call? .scalability get-resource-threshold
                        "memory"))
                )
                (asserts! (is-ok new-threshold) (err "Failed to retrieve threshold"))
                ;; Scale up should have occurred due to our mock performance monitor
                (asserts! (> (get current-capacity (unwrap-panic new-threshold)) u1000)
                    (err "Scaling did not occur"))
            )
            (ok true)
        )
    )
)

;; Resource Adjustment Tests
(define-public (test-adjust-allocation)
    (begin
        ;; Setup: Grant resource adjustment permissions
        (try! (contract-call? .authorization grant-permission TEST_OPERATOR "adjust-resources"))
        
        ;; First create an initial allocation
        (try! (as-contract TEST_OPERATOR
            (contract-call? .scalability allocate-resources
                "api-server"
                u100
                u1000
                u1)))
        
        ;; Test: Adjust allocation within bounds
        (let
            (
                (result (as-contract TEST_OPERATOR
                    (contract-call? .scalability adjust-allocation
                        "api-server"
                        u500)))
            )
            (asserts! (is-ok result) (err "Failed to adjust allocation"))
            
            ;; Verify adjustment
            (let
                (
                    (stored-allocation (contract-call? .scalability get-component-allocation
                        "api-server"))
                )
                (asserts! (is-ok stored-allocation) (err "Failed to retrieve allocation"))
                (asserts! (is-eq (get current-allocation (unwrap-panic stored-allocation)) u500)
                    (err "Invalid allocation adjustment"))
            )
            (ok true)
        )
    )
)

;; Edge Cases and Error Tests
(define-public (test-error-cases)
    (begin
        ;; Test: Set threshold without permission
        (let
            (
                (result (contract-call? .scalability set-scaling-threshold
                    "test"
                    u1
                    u10
                    u1
                    u1
                    true))
            )
            (asserts! (is-err result) (err "Should fail without permission"))
            
            ;; Test: Allocate beyond max capacity
            (let
                (
                    (result (as-contract TEST_OPERATOR
                        (contract-call? .scalability allocate-resources
                            "test"
                            u10000
                            u100000
                            u1)))
                )
                (asserts! (is-err result) (err "Should fail for excessive allocation"))
                
                ;; Test: Adjust invalid component
                (let
                    (
                        (result (as-contract TEST_OPERATOR
                            (contract-call? .scalability adjust-allocation
                                "invalid"
                                u1)))
                    )
                    (asserts! (is-err result) (err "Should fail for invalid component"))
                    (ok true)
                )
            )
        )
    )
)

;; Run all tests
(define-public (run-all-tests)
    (begin
        (try! (test-set-scaling-threshold))
        (try! (test-allocate-resources))
        (try! (test-auto-scaling))
        (try! (test-adjust-allocation))
        (try! (test-error-cases))
        (ok true)
    )
)