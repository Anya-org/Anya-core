;; Test Harness for Agentic DAO Modules
;; Runs all tests and aggregates results

;; Import traits
(use-trait auth-trait .authorization-trait.authorization-trait)
(use-trait ft-trait .sip-010-trait.sip-010-trait)
(use-trait perf-trait .performance-monitor.performance-monitor-trait)

;; Test Results Structure
(define-map test-results
    { module: (string-ascii 50) }
    {
        passed: uint,
        failed: uint,
        skipped: uint,
        error-messages: (list 10 (string-utf8 500))
    }
)

;; Run Core Module Tests
(define-public (run-core-tests)
    (begin
        ;; Ethics Framework Tests
        (match (contract-call? .ethics-framework-test run-all-tests)
            success (record-test-results "ethics-framework" true none)
            error (record-test-results "ethics-framework" false (some "Ethics framework tests failed")))

        ;; Performance Monitor Tests
        (match (contract-call? .performance-monitor-test run-all-tests)
            success (record-test-results "performance-monitor" true none)
            error (record-test-results "performance-monitor" false (some "Performance monitor tests failed")))

        ;; Scalability Tests
        (match (contract-call? .scalability-test run-all-tests)
            success (record-test-results "scalability" true none)
            error (record-test-results "scalability" false (some "Scalability tests failed")))

        ;; Node Incentives Tests
        (match (contract-call? .node-incentives-test run-all-tests)
            success (record-test-results "node-incentives" true none)
            error (record-test-results "node-incentives" false (some "Node incentives tests failed")))

        (ok true)
    )
)

;; Record Test Results
(define-private (record-test-results
    (module (string-ascii 50))
    (passed bool)
    (error-msg (optional (string-utf8 500))))
    (let
        (
            (current-results (default-to
                {
                    passed: u0,
                    failed: u0,
                    skipped: u0,
                    error-messages: (list)
                }
                (map-get? test-results { module: module })))
        )
        (map-set test-results
            { module: module }
            (merge current-results
                {
                    passed: (+ (get passed current-results) (if passed u1 u0)),
                    failed: (+ (get failed current-results) (if passed u0 u1)),
                    error-messages: (match error-msg
                        msg (unwrap! (as-max-len?
                            (append (get error-messages current-results) msg)
                            u10)
                            (get error-messages current-results))
                        (get error-messages current-results))
                }))
        true)
)

;; Get Test Results
(define-read-only (get-module-results (module (string-ascii 50)))
    (ok (unwrap! (map-get? test-results { module: module })
        (err "No results found")))
)

;; Get All Test Results
(define-read-only (get-all-results)
    (ok (map unwrap-module-result
        (list
            "ethics-framework"
            "performance-monitor"
            "scalability"
            "node-incentives")))
)

;; Helper Functions
(define-private (unwrap-module-result (module (string-ascii 50)))
    (unwrap-panic (map-get? test-results { module: module }))
)