;; Regulatory Compliance Module
;; Handles compliance checks and regulatory reporting requirements

(use-trait auth-trait .authorization-trait.authorization-trait)

;; Compliance Status Types
(define-constant STATUS_COMPLIANT "compliant")
(define-constant STATUS_REVIEW_NEEDED "review-needed")
(define-constant STATUS_NON_COMPLIANT "non-compliant")

;; Compliance Requirements
(define-map compliance-requirements
    { requirement-id: uint }
    {
        name: (string-ascii 100),
        description: (string-utf8 500),
        required-checks: (list 10 (string-ascii 50)),
        verification-frequency: uint,
        last-verified: uint,
        status: (string-ascii 20),
        jurisdiction: (string-ascii 50)
    }
)

;; Compliance History
(define-map compliance-history
    { requirement-id: uint, period: uint }
    {
        status: (string-ascii 20),
        verification-date: uint,
        verifier: principal,
        notes: (string-utf8 500),
        supporting-data: (optional (buff 1024))
    }
)

;; Regulatory Reports
(define-map regulatory-reports
    { report-id: uint }
    {
        report-type: (string-ascii 50),
        submission-date: uint,
        period-start: uint,
        period-end: uint,
        jurisdiction: (string-ascii 50),
        status: (string-ascii 20),
        data-hash: (buff 32)
    }
)

;; Public Functions
(define-public (add-compliance-requirement
    (name (string-ascii 100))
    (description (string-utf8 500))
    (checks (list 10 (string-ascii 50)))
    (frequency uint)
    (jurisdiction (string-ascii 50)))
    (let
        (
            (requirement-id (+ u1 (var-get requirement-counter)))
        )
        (asserts! (has-permission tx-sender "manage-compliance") (err ERR_UNAUTHORIZED))
        
        (map-set compliance-requirements
            { requirement-id: requirement-id }
            {
                name: name,
                description: description,
                required-checks: checks,
                verification-frequency: frequency,
                last-verified: u0,
                status: STATUS_REVIEW_NEEDED,
                jurisdiction: jurisdiction
            }
        )
        (var-set requirement-counter requirement-id)
        (ok requirement-id)
    )
)

(define-public (verify-compliance
    (requirement-id uint)
    (notes (string-utf8 500))
    (supporting-data (optional (buff 1024))))
    (let
        (
            (requirement (unwrap! (map-get? compliance-requirements { requirement-id: requirement-id })
                (err ERR_REQUIREMENT_NOT_FOUND)))
        )
        (asserts! (has-permission tx-sender "verify-compliance") (err ERR_UNAUTHORIZED))
        
        ;; Record verification in history
        (map-set compliance-history
            { 
                requirement-id: requirement-id,
                period: (/ block-height (get verification-frequency requirement))
            }
            {
                status: STATUS_COMPLIANT,
                verification-date: block-height,
                verifier: tx-sender,
                notes: notes,
                supporting-data: supporting-data
            }
        )
        
        ;; Update requirement status
        (map-set compliance-requirements
            { requirement-id: requirement-id }
            (merge requirement {
                last-verified: block-height,
                status: STATUS_COMPLIANT
            })
        )
        (ok true)
    )
)

(define-public (submit-regulatory-report
    (report-type (string-ascii 50))
    (period-start uint)
    (period-end uint)
    (jurisdiction (string-ascii 50))
    (data-hash (buff 32)))
    (let
        (
            (report-id (+ u1 (var-get report-counter)))
        )
        (asserts! (has-permission tx-sender "submit-reports") (err ERR_UNAUTHORIZED))
        
        (map-set regulatory-reports
            { report-id: report-id }
            {
                report-type: report-type,
                submission-date: block-height,
                period-start: period-start,
                period-end: period-end,
                jurisdiction: jurisdiction,
                status: "submitted",
                data-hash: data-hash
            }
        )
        (var-set report-counter report-id)
        (ok report-id)
    )
)

;; Read-only Functions
(define-read-only (get-compliance-status (requirement-id uint))
    (ok (unwrap! (map-get? compliance-requirements { requirement-id: requirement-id })
        (err ERR_REQUIREMENT_NOT_FOUND)))
)

(define-read-only (get-verification-history 
    (requirement-id uint)
    (period uint))
    (ok (unwrap! (map-get? compliance-history 
        { requirement-id: requirement-id, period: period })
        (err ERR_NO_HISTORY)))
)

(define-read-only (get-report-status (report-id uint))
    (ok (unwrap! (map-get? regulatory-reports { report-id: report-id })
        (err ERR_REPORT_NOT_FOUND)))
)

;; Data Variables
(define-data-var requirement-counter uint u0)
(define-data-var report-counter uint u0)

;; Helper Functions
(define-private (has-permission (caller principal) (permission (string-ascii 50)))
    (contract-call? .authorization caller permission)
)