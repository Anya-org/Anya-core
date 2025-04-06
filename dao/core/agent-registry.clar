;; Agent Registry Contract
;; Manages autonomous agent registration, permissions, and monitoring

;; Constants
(define-constant MIN_STAKE_REQUIREMENT u100000)
(define-constant PERFORMANCE_THRESHOLD u75)
(define-constant COOLDOWN_PERIOD u144)

;; Agent Status Types
(define-constant STATUS_ACTIVE "active")
(define-constant STATUS_PROBATION "probation")
(define-constant STATUS_SUSPENDED "suspended")

;; Agent Registry
(define-map agents
    principal
    {
        stake: uint,
        status: (string-ascii 20),
        performance-score: uint,
        last-action: uint,
        total-actions: uint,
        successful-actions: uint,
        permissions: (list 10 (string-ascii 50)),
        suspension-end: (optional uint)
    }
)

;; Performance History
(define-map agent-history
    { agent: principal, period: uint }
    {
        actions-performed: uint,
        success-rate: uint,
        avg-gas-used: uint,
        stake-changes: (list 5 {amount: uint, block: uint})
    }
)

;; Registration Functions
(define-public (register-agent (stake uint))
    (let
        (
            (caller tx-sender)
            (existing-agent (map-get? agents caller))
        )
        (asserts! (>= stake MIN_STAKE_REQUIREMENT) (err ERR_INSUFFICIENT_STAKE))
        (asserts! (is-none existing-agent) (err ERR_ALREADY_REGISTERED))
        
        (map-set agents caller
            {
                stake: stake,
                status: STATUS_ACTIVE,
                performance-score: u100,
                last-action: block-height,
                total-actions: u0,
                successful-actions: u0,
                permissions: (list "basic"),
                suspension-end: none
            }
        )
        (ok true)
    )
)

;; Permission Management
(define-public (grant-permission (agent principal) (permission (string-ascii 50)))
    (let
        (
            (agent-data (unwrap! (map-get? agents agent) (err ERR_AGENT_NOT_FOUND)))
        )
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        (asserts! (is-eq (get status agent-data) STATUS_ACTIVE) (err ERR_AGENT_NOT_ACTIVE))
        
        (map-set agents agent
            (merge agent-data
                {
                    permissions: (unwrap! (as-max-len? 
                        (append (get permissions agent-data) permission)
                        u10)
                        (err ERR_TOO_MANY_PERMISSIONS))
                }
            )
        )
        (ok true)
    )
)

;; Performance Tracking
(define-public (record-action-result 
    (agent principal) 
    (success bool) 
    (gas-used uint))
    (let
        (
            (agent-data (unwrap! (map-get? agents agent) (err ERR_AGENT_NOT_FOUND)))
            (current-period (/ block-height COOLDOWN_PERIOD))
        )
        (asserts! (is-authorized tx-sender) (err ERR_UNAUTHORIZED))
        
        ;; Update agent stats
        (map-set agents agent
            (merge agent-data
                {
                    total-actions: (+ (get total-actions agent-data) u1),
                    successful-actions: (+ (get successful-actions agent-data) (if success u1 u0)),
                    performance-score: (calculate-new-score 
                        (get performance-score agent-data)
                        success
                        gas-used)
                }
            )
        )
        
        ;; Update history
        (update-agent-history agent current-period success gas-used)
        
        ;; Check for automatic status changes
        (check-status-changes agent)
        
        (ok true)
    )
)

;; Helper Functions
(define-private (calculate-new-score (current-score uint) (success bool) (gas-used uint))
    (let
        (
            (performance-impact (if success u5 (- u0 u5)))
            (gas-impact (if (> gas-used u1000000) (- u0 u2) u0))
        )
        (max (min (+ current-score performance-impact gas-impact) u100) u0)
    )
)

(define-private (update-agent-history (agent principal) (period uint) (success bool) (gas-used uint))
    (let
        (
            (history (default-to
                {
                    actions-performed: u0,
                    success-rate: u100,
                    avg-gas-used: u0,
                    stake-changes: (list)
                }
                (map-get? agent-history { agent: agent, period: period })))
        )
        (map-set agent-history
            { agent: agent, period: period }
            (merge history
                {
                    actions-performed: (+ (get actions-performed history) u1),
                    success-rate: (calculate-success-rate 
                        (get success-rate history)
                        (get actions-performed history)
                        success),
                    avg-gas-used: (/ (+ (* (get avg-gas-used history) 
                                         (get actions-performed history))
                                      gas-used)
                                   (+ (get actions-performed history) u1))
                }
            )
        )
    )
)

(define-private (check-status-changes (agent principal))
    (let
        (
            (agent-data (unwrap! (map-get? agents agent) false))
        )
        (if (< (get performance-score agent-data) PERFORMANCE_THRESHOLD)
            (update-agent-status agent STATUS_PROBATION)
            false)
    )
)

(define-private (update-agent-status (agent principal) (new-status (string-ascii 20)))
    (let
        (
            (agent-data (unwrap! (map-get? agents agent) false))
        )
        (map-set agents agent
            (merge agent-data
                {
                    status: new-status,
                    suspension-end: (if (is-eq new-status STATUS_SUSPENDED)
                                     (some (+ block-height COOLDOWN_PERIOD))
                                     none)
                }
            )
        )
    )
)

;; Read-only Functions
(define-read-only (get-agent-info (agent principal))
    (ok (unwrap! (map-get? agents agent) (err ERR_AGENT_NOT_FOUND)))
)

(define-read-only (get-agent-history (agent principal) (period uint))
    (ok (unwrap! (map-get? agent-history { agent: agent, period: period })
        (err ERR_NO_HISTORY)))
)

(define-read-only (has-permission (agent principal) (permission (string-ascii 50)))
    (match (map-get? agents agent)
        agent-data (ok (contains? permission (get permissions agent-data)))
        (ok false))
)