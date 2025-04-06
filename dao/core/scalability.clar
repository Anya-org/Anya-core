;; Scalability Module
;; Manages system growth and dynamic resource allocation

(use-trait auth-trait .authorization-trait.authorization-trait)
(use-trait perf-trait .performance-monitor.performance-monitor-trait)

;; Growth Parameters
(define-map scaling-thresholds
    { resource: (string-ascii 50) }
    {
        current-capacity: uint,
        max-capacity: uint,
        growth-rate: uint,
        last-scaled: uint,
        cool-down-period: uint,
        auto-scale: bool
    }
)

;; Resource Allocation
(define-map resource-allocations
    { component: (string-ascii 50) }
    {
        min-resources: uint,
        max-resources: uint,
        current-allocation: uint,
        priority: uint,
        last-adjusted: uint
    }
)

;; Scaling History
(define-map scaling-events
    { event-id: uint }
    {
        resource: (string-ascii 50),
        old-capacity: uint,
        new-capacity: uint,
        reason: (string-ascii 100),
        timestamp: uint,
        triggered-by: principal
    }
)

;; Data Variables
(define-data-var event-counter uint u0)
(define-data-var total-capacity uint u0)

;; Public Functions
(define-public (set-scaling-threshold
    (resource (string-ascii 50))
    (capacity uint)
    (max-cap uint)
    (growth uint)
    (cooldown uint)
    (auto bool))
    (begin
        (asserts! (has-permission tx-sender "manage-scaling") (err ERR_UNAUTHORIZED))
        
        (map-set scaling-thresholds
            { resource: resource }
            {
                current-capacity: capacity,
                max-capacity: max-cap,
                growth-rate: growth,
                last-scaled: block-height,
                cool-down-period: cooldown,
                auto-scale: auto
            }
        )
        (var-set total-capacity (+ (var-get total-capacity) capacity))
        (ok true)
    )
)

(define-public (allocate-resources
    (component (string-ascii 50))
    (min uint)
    (max uint)
    (priority uint))
    (begin
        (asserts! (has-permission tx-sender "manage-resources") (err ERR_UNAUTHORIZED))
        
        (map-set resource-allocations
            { component: component }
            {
                min-resources: min,
                max-resources: max,
                current-allocation: min,
                priority: priority,
                last-adjusted: block-height
            }
        )
        (ok true)
    )
)

(define-public (trigger-auto-scaling (resource (string-ascii 50)))
    (let
        (
            (threshold (unwrap! (map-get? scaling-thresholds { resource: resource })
                (err ERR_RESOURCE_NOT_FOUND)))
        )
        (asserts! (has-permission tx-sender "trigger-scaling") (err ERR_UNAUTHORIZED))
        (asserts! (get auto-scale threshold) (err ERR_AUTO_SCALE_DISABLED))
        (asserts! (can-scale? threshold) (err ERR_SCALING_COOLDOWN))
        
        (if (should-scale-up? resource)
            (scale-up resource threshold)
            (scale-down resource threshold))
    )
)

(define-public (adjust-allocation
    (component (string-ascii 50))
    (new-allocation uint))
    (let
        (
            (allocation (unwrap! (map-get? resource-allocations { component: component })
                (err ERR_COMPONENT_NOT_FOUND)))
        )
        (asserts! (has-permission tx-sender "adjust-resources") (err ERR_UNAUTHORIZED))
        (asserts! (and 
            (>= new-allocation (get min-resources allocation))
            (<= new-allocation (get max-resources allocation)))
            (err ERR_INVALID_ALLOCATION))
        
        (map-set resource-allocations
            { component: component }
            (merge allocation {
                current-allocation: new-allocation,
                last-adjusted: block-height
            })
        )
        (ok true)
    )
)

;; Private Helper Functions
(define-private (can-scale? (threshold {
    current-capacity: uint,
    max-capacity: uint,
    growth-rate: uint,
    last-scaled: uint,
    cool-down-period: uint,
    auto-scale: bool
}))
    (> block-height (+ (get last-scaled threshold) (get cool-down-period threshold)))
)

(define-private (should-scale-up? (resource (string-ascii 50)))
    (contract-call? .performance-monitor is-resource-constrained resource)
)

(define-private (scale-up
    (resource (string-ascii 50))
    (threshold {
        current-capacity: uint,
        max-capacity: uint,
        growth-rate: uint,
        last-scaled: uint,
        cool-down-period: uint,
        auto-scale: bool
    }))
    (let
        (
            (new-capacity (min 
                (+ (get current-capacity threshold) 
                   (* (get current-capacity threshold) (get growth-rate threshold)))
                (get max-capacity threshold)))
        )
        (if (> new-capacity (get current-capacity threshold))
            (begin
                (map-set scaling-thresholds
                    { resource: resource }
                    (merge threshold {
                        current-capacity: new-capacity,
                        last-scaled: block-height
                    })
                )
                (record-scaling-event 
                    resource 
                    (get current-capacity threshold)
                    new-capacity
                    "auto-scale-up")
                (ok new-capacity))
            (ok (get current-capacity threshold)))
    )
)

(define-private (scale-down
    (resource (string-ascii 50))
    (threshold {
        current-capacity: uint,
        max-capacity: uint,
        growth-rate: uint,
        last-scaled: uint,
        cool-down-period: uint,
        auto-scale: bool
    }))
    (let
        (
            (min-capacity (/ (get max-capacity threshold) u10))
            (new-capacity (max
                (- (get current-capacity threshold)
                   (* (get current-capacity threshold) (get growth-rate threshold)))
                min-capacity))
        )
        (if (< new-capacity (get current-capacity threshold))
            (begin
                (map-set scaling-thresholds
                    { resource: resource }
                    (merge threshold {
                        current-capacity: new-capacity,
                        last-scaled: block-height
                    })
                )
                (record-scaling-event
                    resource
                    (get current-capacity threshold)
                    new-capacity
                    "auto-scale-down")
                (ok new-capacity))
            (ok (get current-capacity threshold)))
    )
)

(define-private (record-scaling-event
    (resource (string-ascii 50))
    (old-cap uint)
    (new-cap uint)
    (reason (string-ascii 100)))
    (let
        (
            (event-id (+ u1 (var-get event-counter)))
        )
        (map-set scaling-events
            { event-id: event-id }
            {
                resource: resource,
                old-capacity: old-cap,
                new-capacity: new-cap,
                reason: reason,
                timestamp: block-height,
                triggered-by: tx-sender
            }
        )
        (var-set event-counter event-id)
        true)
)

;; Read-only Functions
(define-read-only (get-resource-threshold (resource (string-ascii 50)))
    (ok (unwrap! (map-get? scaling-thresholds { resource: resource })
        (err ERR_RESOURCE_NOT_FOUND)))
)

(define-read-only (get-component-allocation (component (string-ascii 50)))
    (ok (unwrap! (map-get? resource-allocations { component: component })
        (err ERR_COMPONENT_NOT_FOUND)))
)

(define-read-only (get-scaling-event (event-id uint))
    (ok (unwrap! (map-get? scaling-events { event-id: event-id })
        (err ERR_EVENT_NOT_FOUND)))
)

;; Helper Functions
(define-private (has-permission (caller principal) (permission (string-ascii 50)))
    (contract-call? .authorization caller permission)
)