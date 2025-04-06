;; Performance Monitoring Module
;; Tracks system performance metrics and resource optimization

(use-trait auth-trait .authorization-trait.authorization-trait)

;; Performance Metrics
(define-map system-metrics
    { metric-id: uint }
    {
        name: (string-ascii 50),
        current-value: uint,
        historical-values: (list 100 uint),
        threshold: uint,
        alert-triggered: bool,
        last-updated: uint
    }
)

;; Resource Usage
(define-map resource-consumption
    { resource: (string-ascii 50) }
    {
        allocated: uint,
        used: uint,
        available: uint,
        efficiency: uint,
        last-optimized: uint
    }
)

;; Performance Alerts
(define-map performance-alerts
    { alert-id: uint }
    {
        metric-id: uint,
        severity: uint,
        timestamp: uint,
        description: (string-utf8 500),
        resolved: bool,
        resolution-time: (optional uint)
    }
)

;; Data Variables
(define-data-var metric-counter uint u0)
(define-data-var alert-counter uint u0)

;; Public Functions
(define-public (record-metric
    (name (string-ascii 50))
    (value uint)
    (threshold uint))
    (let
        (
            (metric-id (+ u1 (var-get metric-counter)))
            (current-metric (map-get? system-metrics { metric-id: metric-id }))
        )
        (asserts! (has-permission tx-sender "record-metrics") (err ERR_UNAUTHORIZED))
        
        (match current-metric
            existing (update-existing-metric metric-id existing value)
            (create-new-metric metric-id name value threshold))
    )
)

(define-public (update-resource-usage
    (resource (string-ascii 50))
    (allocated uint)
    (used uint))
    (let
        (
            (current-usage (default-to
                {
                    allocated: u0,
                    used: u0,
                    available: u0,
                    efficiency: u100,
                    last-optimized: u0
                }
                (map-get? resource-consumption { resource: resource })))
        )
        (map-set resource-consumption
            { resource: resource }
            (merge current-usage {
                allocated: allocated,
                used: used,
                available: (- allocated used),
                efficiency: (calculate-efficiency allocated used),
                last-optimized: block-height
            })
        )
        (ok true)
    )
)

(define-public (optimize-resources)
    (begin
        (asserts! (has-permission tx-sender "optimize-resources") (err ERR_UNAUTHORIZED))
        
        ;; Implement resource optimization logic here
        ;; This is a placeholder for the actual optimization algorithm
        (let
            (
                (resources (get-all-resources))
            )
            (map optimize-single-resource resources)
            (ok true)
        )
    )
)

(define-public (resolve-alert (alert-id uint))
    (let
        (
            (alert (unwrap! (map-get? performance-alerts { alert-id: alert-id })
                (err ERR_ALERT_NOT_FOUND)))
        )
        (asserts! (has-permission tx-sender "manage-alerts") (err ERR_UNAUTHORIZED))
        
        (map-set performance-alerts
            { alert-id: alert-id }
            (merge alert {
                resolved: true,
                resolution-time: (some block-height)
            })
        )
        (ok true)
    )
)

;; Private Helper Functions
(define-private (create-new-metric (metric-id uint) (name (string-ascii 50)) (value uint) (threshold uint))
    (begin
        (map-set system-metrics
            { metric-id: metric-id }
            {
                name: name,
                current-value: value,
                historical-values: (list value),
                threshold: threshold,
                alert-triggered: false,
                last-updated: block-height
            }
        )
        (var-set metric-counter metric-id)
        (ok metric-id)
    )
)

(define-private (update-existing-metric (metric-id uint) (existing {
    name: (string-ascii 50),
    current-value: uint,
    historical-values: (list 100 uint),
    threshold: uint,
    alert-triggered: bool,
    last-updated: uint
}) (new-value uint))
    (begin
        (map-set system-metrics
            { metric-id: metric-id }
            (merge existing {
                current-value: new-value,
                historical-values: (unwrap! 
                    (as-max-len? 
                        (append (get historical-values existing) new-value)
                        u100)
                    (err ERR_LIST_FULL)),
                alert-triggered: (> new-value (get threshold existing)),
                last-updated: block-height
            })
        )
        (ok metric-id)
    )
)

(define-private (calculate-efficiency (allocated uint) (used uint))
    (if (is-eq allocated u0)
        u0
        (* (/ used allocated) u100))
)

(define-private (get-all-resources)
    (map unwrap-resource
        (map-keys resource-consumption))
)

(define-private (unwrap-resource (key { resource: (string-ascii 50) }))
    (get resource key)
)

(define-private (optimize-single-resource (resource (string-ascii 50)))
    (let
        (
            (usage (unwrap! (map-get? resource-consumption { resource: resource })
                false))
        )
        (if (< (get efficiency usage) u80)
            (adjust-resource-allocation resource usage)
            true)
    )
)

(define-private (adjust-resource-allocation (resource (string-ascii 50)) (usage {
    allocated: uint,
    used: uint,
    available: uint,
    efficiency: uint,
    last-optimized: uint
}))
    (map-set resource-consumption
        { resource: resource }
        (merge usage {
            allocated: (optimal-allocation (get used usage)),
            last-optimized: block-height
        })
    )
)

(define-private (optimal-allocation (used uint))
    (+ used (/ used u10)) ;; Add 10% buffer
)

;; Read-only Functions
(define-read-only (get-metric (metric-id uint))
    (ok (unwrap! (map-get? system-metrics { metric-id: metric-id })
        (err ERR_METRIC_NOT_FOUND)))
)

(define-read-only (get-resource-usage (resource (string-ascii 50)))
    (ok (unwrap! (map-get? resource-consumption { resource: resource })
        (err ERR_RESOURCE_NOT_FOUND)))
)

(define-read-only (get-alert (alert-id uint))
    (ok (unwrap! (map-get? performance-alerts { alert-id: alert-id })
        (err ERR_ALERT_NOT_FOUND)))
)

;; Helper Functions
(define-private (has-permission (caller principal) (permission (string-ascii 50)))
    (contract-call? .authorization caller permission)
)