;; Node Incentivization Module
;; Manages rewards and penalties for network participants

(use-trait ft-trait .sip-010-trait.sip-010-trait)
(use-trait auth-trait .authorization-trait.authorization-trait)

;; Constants
(define-constant REWARD_CYCLE_LENGTH u144) ;; ~24 hours in blocks
(define-constant MIN_PERFORMANCE_THRESHOLD u80)
(define-constant MAX_PENALTY_RATE u20) ;; 20% maximum penalty

;; Reward Pools
(define-map reward-pools
    { pool-id: uint }
    {
        total-amount: uint,
        distributed-amount: uint,
        start-block: uint,
        end-block: uint,
        is-active: bool,
        reward-token: principal
    }
)

;; Node Performance
(define-map node-performance
    { node: principal }
    {
        total-rewards: uint,
        current-cycle-rewards: uint,
        performance-score: uint,
        penalty-rate: uint,
        last-reward-block: uint,
        consecutive-penalties: uint
    }
)

;; Reward Distribution History
(define-map distribution-history
    { cycle: uint, node: principal }
    {
        amount: uint,
        base-amount: uint,
        performance-multiplier: uint,
        penalty-applied: uint,
        timestamp: uint
    }
)

;; Data Variables
(define-data-var pool-counter uint u0)
(define-data-var current-cycle uint u0)

;; Public Functions
(define-public (create-reward-pool
    (amount uint)
    (duration uint)
    (reward-token principal))
    (let
        (
            (pool-id (+ u1 (var-get pool-counter)))
        )
        (asserts! (has-permission tx-sender "manage-rewards") (err ERR_UNAUTHORIZED))
        
        (try! (contract-call? reward-token transfer amount tx-sender (as-contract tx-sender)))
        
        (map-set reward-pools
            { pool-id: pool-id }
            {
                total-amount: amount,
                distributed-amount: u0,
                start-block: block-height,
                end-block: (+ block-height duration),
                is-active: true,
                reward-token: reward-token
            }
        )
        (var-set pool-counter pool-id)
        (ok pool-id)
    )
)

(define-public (distribute-rewards (pool-id uint))
    (let
        (
            (pool (unwrap! (map-get? reward-pools { pool-id: pool-id })
                (err ERR_POOL_NOT_FOUND)))
            (cycle (/ block-height REWARD_CYCLE_LENGTH))
        )
        (asserts! (has-permission tx-sender "distribute-rewards") (err ERR_UNAUTHORIZED))
        (asserts! (get is-active pool) (err ERR_POOL_INACTIVE))
        
        (var-set current-cycle cycle)
        (process-cycle-rewards pool-id pool cycle)
    )
)

(define-public (update-node-performance 
    (node principal)
    (performance uint))
    (let
        (
            (current-perf (default-to
                {
                    total-rewards: u0,
                    current-cycle-rewards: u0,
                    performance-score: u100,
                    penalty-rate: u0,
                    last-reward-block: u0,
                    consecutive-penalties: u0
                }
                (map-get? node-performance { node: node })))
        )
        (asserts! (has-permission tx-sender "manage-performance") (err ERR_UNAUTHORIZED))
        
        (map-set node-performance
            { node: node }
            (merge current-perf {
                performance-score: performance,
                penalty-rate: (calculate-penalty-rate 
                    performance 
                    (get consecutive-penalties current-perf)),
                consecutive-penalties: (if (< performance MIN_PERFORMANCE_THRESHOLD)
                    (+ (get consecutive-penalties current-perf) u1)
                    u0)
            })
        )
        (ok true)
    )
)

(define-public (claim-rewards 
    (pool-id uint)
    (cycle uint))
    (let
        (
            (distribution (unwrap! (map-get? distribution-history 
                { cycle: cycle, node: tx-sender })
                (err ERR_NO_REWARDS)))
            (pool (unwrap! (map-get? reward-pools { pool-id: pool-id })
                (err ERR_POOL_NOT_FOUND)))
        )
        (asserts! (<= cycle (var-get current-cycle)) (err ERR_FUTURE_CYCLE))
        
        (try! (contract-call? 
            (unwrap! (contract-call? .ft-trait (get reward-token pool))
                (err ERR_INVALID_TOKEN))
            transfer
            (get amount distribution)
            (as-contract tx-sender)
            tx-sender))
        
        (map-set distribution-history
            { cycle: cycle, node: tx-sender }
            (merge distribution { amount: u0 })
        )
        (ok true)
    )
)

;; Private Helper Functions
(define-private (process-cycle-rewards (pool-id uint) (pool {
    total-amount: uint,
    distributed-amount: uint,
    start-block: uint,
    end-block: uint,
    is-active: bool,
    reward-token: principal
}) (cycle uint))
    (let
        (
            (active-nodes (get-active-nodes))
            (cycle-rewards (calculate-cycle-rewards 
                (get total-amount pool)
                (- (get end-block pool) (get start-block pool))))
        )
        (map (distribute-node-rewards pool-id cycle cycle-rewards) active-nodes)
        (ok true)
    )
)

(define-private (distribute-node-rewards (pool-id uint) (cycle uint) (base-reward uint))
    (lambda (node principal)
        (let
            (
                (perf (unwrap! (map-get? node-performance { node: node })
                    false))
            )
            (if perf
                (let
                    (
                        (multiplier (/ (get performance-score perf) u100))
                        (penalty (get penalty-rate perf))
                        (final-reward (calculate-final-reward base-reward multiplier penalty))
                    )
                    (map-set distribution-history
                        { cycle: cycle, node: node }
                        {
                            amount: final-reward,
                            base-amount: base-reward,
                            performance-multiplier: multiplier,
                            penalty-applied: penalty,
                            timestamp: block-height
                        }
                    )
                    (map-set node-performance
                        { node: node }
                        (merge perf {
                            total-rewards: (+ (get total-rewards perf) final-reward),
                            current-cycle-rewards: final-reward,
                            last-reward-block: block-height
                        })
                    ))
                false)
        )
    )
)

(define-private (calculate-penalty-rate (performance uint) (consecutive-penalties uint))
    (min (* consecutive-penalties u5) MAX_PENALTY_RATE)
)

(define-private (calculate-cycle-rewards (total uint) (total-blocks uint))
    (/ total (/ total-blocks REWARD_CYCLE_LENGTH))
)

(define-private (calculate-final-reward (base uint) (multiplier uint) (penalty uint))
    (* base (- u100 penalty) multiplier)
)

(define-private (get-active-nodes)
    ;; In a full implementation, this would query the node registry
    ;; This is a placeholder that returns an empty list
    (list)
)

;; Read-only Functions
(define-read-only (get-node-stats (node principal))
    (ok (unwrap! (map-get? node-performance { node: node })
        (err ERR_NODE_NOT_FOUND)))
)

(define-read-only (get-pool-info (pool-id uint))
    (ok (unwrap! (map-get? reward-pools { pool-id: pool-id })
        (err ERR_POOL_NOT_FOUND)))
)

(define-read-only (get-distribution (cycle uint) (node principal))
    (ok (unwrap! (map-get? distribution-history { cycle: cycle, node: node })
        (err ERR_NO_DISTRIBUTION)))
)

;; Helper Functions
(define-private (has-permission (caller principal) (permission (string-ascii 50)))
    (contract-call? .authorization caller permission)
)