;; Ethics Framework Module
;; Implements ethical guidelines and bias monitoring for AI agents

(use-trait auth-trait .authorization-trait.authorization-trait)

;; Ethical Guidelines
(define-map ethical-guidelines
    { principle-id: uint }
    {
        name: (string-ascii 100),
        description: (string-utf8 500),
        checks: (list 10 (string-ascii 50)),
        severity: uint,
        category: (string-ascii 50)
    }
)

;; Bias Monitoring
(define-map bias-metrics
    { agent: principal, metric-id: uint }
    {
        category: (string-ascii 50),
        value: int,
        samples: uint,
        last-updated: uint,
        threshold: int,
        requires-review: bool
    }
)

;; Decision Records
(define-map ethical-decisions
    { decision-id: uint }
    {
        agent: principal,
        action: (string-ascii 100),
        principles-checked: (list 10 uint),
        outcome: (string-ascii 50),
        timestamp: uint,
        human-review-required: bool,
        review-notes: (optional (string-utf8 500))
    }
)

;; Data Variables
(define-data-var principle-counter uint u0)
(define-data-var decision-counter uint u0)

;; Public Functions
(define-public (add-ethical-principle
    (name (string-ascii 100))
    (description (string-utf8 500))
    (checks (list 10 (string-ascii 50)))
    (severity uint)
    (category (string-ascii 50)))
    (let
        (
            (principle-id (+ u1 (var-get principle-counter)))
        )
        (asserts! (has-permission tx-sender "manage-ethics") (err ERR_UNAUTHORIZED))
        
        (map-set ethical-guidelines
            { principle-id: principle-id }
            {
                name: name,
                description: description,
                checks: checks,
                severity: severity,
                category: category
            }
        )
        (var-set principle-counter principle-id)
        (ok principle-id)
    )
)

(define-public (record-decision
    (action (string-ascii 100))
    (principles (list 10 uint))
    (outcome (string-ascii 50))
    (requires-review bool))
    (let
        (
            (decision-id (+ u1 (var-get decision-counter)))
        )
        (asserts! (has-permission tx-sender "record-decisions") (err ERR_UNAUTHORIZED))
        
        (map-set ethical-decisions
            { decision-id: decision-id }
            {
                agent: tx-sender,
                action: action,
                principles-checked: principles,
                outcome: outcome,
                timestamp: block-height,
                human-review-required: requires-review,
                review-notes: none
            }
        )
        (var-set decision-counter decision-id)
        (ok decision-id)
    )
)

(define-public (update-bias-metric
    (metric-id uint)
    (value int))
    (let
        (
            (current-metric (default-to
                {
                    category: "general",
                    value: 0,
                    samples: u0,
                    last-updated: u0,
                    threshold: 100,
                    requires-review: false
                }
                (map-get? bias-metrics { agent: tx-sender, metric-id: metric-id })))
        )
        (map-set bias-metrics
            { agent: tx-sender, metric-id: metric-id }
            (merge current-metric {
                value: (/ (+ (* (get value current-metric) 
                               (to-int (get samples current-metric)))
                            value)
                         (to-int (+ u1 (get samples current-metric)))),
                samples: (+ u1 (get samples current-metric)),
                last-updated: block-height,
                requires-review: (> (abs value) (get threshold current-metric))
            })
        )
        (ok true)
    )
)

(define-public (review-decision
    (decision-id uint)
    (approved bool)
    (notes (string-utf8 500)))
    (let
        (
            (decision (unwrap! (map-get? ethical-decisions { decision-id: decision-id })
                (err ERR_DECISION_NOT_FOUND)))
        )
        (asserts! (has-permission tx-sender "review-decisions") (err ERR_UNAUTHORIZED))
        (asserts! (get human-review-required decision) (err ERR_REVIEW_NOT_REQUIRED))
        
        (map-set ethical-decisions
            { decision-id: decision-id }
            (merge decision {
                review-notes: (some notes)
            })
        )
        (ok true)
    )
)

;; Read-only Functions
(define-read-only (get-ethical-principle (principle-id uint))
    (ok (unwrap! (map-get? ethical-guidelines { principle-id: principle-id })
        (err ERR_PRINCIPLE_NOT_FOUND)))
)

(define-read-only (get-bias-metrics (agent principal) (metric-id uint))
    (ok (unwrap! (map-get? bias-metrics { agent: agent, metric-id: metric-id })
        (err ERR_METRIC_NOT_FOUND)))
)

(define-read-only (get-decision (decision-id uint))
    (ok (unwrap! (map-get? ethical-decisions { decision-id: decision-id })
        (err ERR_DECISION_NOT_FOUND)))
)

;; Helper Functions
(define-private (has-permission (caller principal) (permission (string-ascii 50)))
    (contract-call? .authorization caller permission)
)

(define-private (abs (x int))
    (if (< x 0) (* x -1) x)
)