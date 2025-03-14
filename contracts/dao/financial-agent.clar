 ;; Financial Agent Contract - Automated Treasury Operations with ML integration
;; [AIR-3][AIS-3][AIT-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_PARAMETER u402)
(define-constant ERR_BELOW_THRESHOLD u403)
(define-constant ERR_ABOVE_THRESHOLD u404)
(define-constant ERR_INSUFFICIENT_SIGNATURES u405)
(define-constant ERR_COOLDOWN_ACTIVE u406)
(define-constant ERR_OPERATION_FAILED u407)
(define-constant ERR_SIMULATION_FAILED u408)
(define-constant ERR_RISK_TOO_HIGH u409)
(define-constant ERR_EMERGENCY_ACTIVE u410)
(define-constant ERR_NOT_FOUND u411)
(define-constant ERR_ALREADY_SIGNED u412)
(define-constant ERR_AGENT_DISABLED u413)

;; Contract references
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance)
(define-constant TREASURY_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury-management)
(define-constant METRICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.metrics-oracle)
(define-constant OPERATIONS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.operations-manager)
(define-constant REPORTING_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.reporting-system)

;; Agent state data
(define-data-var agent-enabled bool true)
(define-data-var risk-tolerance (string-ascii 10) "medium") ;; low, medium, high
(define-data-var max-operation-size uint u500) ;; 0.5% of treasury in basis points
(define-data-var operation-cooldown uint u100) ;; blocks between operations
(define-data-var multi-sig-threshold uint u2) ;; required signatures
(define-data-var last-operation-block uint u0)
(define-data-var current-signers uint u0)
(define-data-var emergency-mode bool false)
(define-data-var simulation-depth uint u1000) ;; simulation iterations

;; Admin lists
(define-map administrators principal bool)
(define-map signers principal bool)
(define-map ml-providers principal bool)

;; Initialize administrators and signers
(map-set administrators tx-sender true)
(map-set signers tx-sender true)

;; Operation types
(define-constant OPERATION_RESERVE_ADJUSTMENT u1)
(define-constant OPERATION_LIQUIDITY_ADJUSTMENT u2)
(define-constant OPERATION_BUYBACK u3)
(define-constant OPERATION_EMISSION_ADJUSTMENT u4)
(define-constant OPERATION_INCENTIVE_ADJUSTMENT u5)

;; Agent registry
(define-map agents 
  { id: (string-ascii 64) }
  {
    description: (string-ascii 256),
    operation-type: uint,
    trigger-condition: (string-ascii 256),
    parameters: (list 10 {name: (string-ascii 64), value: uint}),
    enabled: bool,
    last-execution: uint,
    execution-count: uint,
    success-count: uint
  }
)

;; Operation history
(define-map operations
  uint ;; operation ID
  {
    agent-id: (string-ascii 64),
    operation-type: uint,
    amount: uint,
    executed-at: uint,
    executed-by: principal,
    signers: (list 5 principal),
    status: (string-ascii 20),
    result: (string-ascii 256),
    simulation-result: (string-ascii 256),
    ml-confidence: uint,
    risk-score: uint
  }
)

;; Pending operations
(define-map pending-operations
  uint ;; operation ID
  {
    agent-id: (string-ascii 64),
    operation-type: uint,
    amount: uint,
    proposed-at: uint,
    proposed-by: principal,
    signers: (list 5 principal),
    expires-at: uint,
    simulation-result: (string-ascii 256),
    ml-confidence: uint,
    risk-score: uint
  }
)

;; ML recommendations
(define-map ml-recommendations
  { metric: (string-ascii 64) }
  {
    recommendation: (string-ascii 256),
    confidence: uint,
    provider: principal,
    timestamp: uint,
    parameters: (list 10 {name: (string-ascii 64), value: uint})
  }
)

;; Simulation results
(define-map simulation-results
  { operation-type: uint, simulation-id: uint }
  {
    parameters: (list 10 {name: (string-ascii 64), value: uint}),
    result: (string-ascii 256),
    impact-score: uint,
    risk-score: uint,
    success: bool,
    timestamp: uint
  }
)

;; Operation counter
(define-data-var operation-counter uint u0)
(define-data-var simulation-counter uint u0)

;; Initialize standard agents

;; Reserve manager agent
(map-set agents
  { id: "reserve_manager" }
  {
    description: "Maintains treasury reserve ratio within target bounds",
    operation-type: OPERATION_RESERVE_ADJUSTMENT,
    trigger-condition: "reserve_ratio < 0.13 or reserve_ratio > 0.20",
    parameters: (list 
      {name: "target_ratio", value: u150} ;; 15.0% target
      {name: "max_adjustment", value: u200} ;; 0.2% max adjustment
    ),
    enabled: true,
    last-execution: u0,
    execution-count: u0,
    success-count: u0
  }
)

;; Liquidity manager agent
(map-set agents
  { id: "liquidity_manager" }
  {
    description: "Optimizes protocol-owned liquidity",
    operation-type: OPERATION_LIQUIDITY_ADJUSTMENT,
    trigger-condition: "pol_percentage < 0.12 or pol_percentage > 0.18",
    parameters: (list 
      {name: "target_ratio", value: u150} ;; 15.0% target
      {name: "max_slippage", value: u50} ;; 0.5% max slippage
    ),
    enabled: true,
    last-execution: u0,
    execution-count: u0,
    success-count: u0
  }
)

;; Buyback agent
(map-set agents
  { id: "buyback_agent" }
  {
    description: "Executes token buybacks based on market conditions",
    operation-type: OPERATION_BUYBACK,
    trigger-condition: "price_7d_change < -0.15 and reserve_ratio > 0.16",
    parameters: (list 
      {name: "max_size", value: u100} ;; 0.1% max size
      {name: "target_impact", value: u50} ;; 0.5% target impact
    ),
    enabled: true,
    last-execution: u0,
    execution-count: u0,
    success-count: u0
  }
)

;; Velocity controller agent
(map-set agents
  { id: "velocity_controller" }
  {
    description: "Adjusts emission rate based on token velocity",
    operation-type: OPERATION_EMISSION_ADJUSTMENT,
    trigger-condition: "token_velocity > 4.0 or token_velocity < 0.5",
    parameters: (list 
      {name: "max_adjustment", value: u1000} ;; 10% max adjustment
      {name: "cooling_period", value: u10000} ;; 10,000 blocks cooling period
    ),
    enabled: true,
    last-execution: u0,
    execution-count: u0,
    success-count: u0
  }
)

;; Participation optimizer agent
(map-set agents
  { id: "participation_optimizer" }
  {
    description: "Optimizes incentives based on governance participation",
    operation-type: OPERATION_INCENTIVE_ADJUSTMENT,
    trigger-condition: "voting_participation < 0.20",
    parameters: (list 
      {name: "target_boost", value: u2500} ;; 25% target boost
      {name: "duration", value: u5000} ;; 5,000 blocks duration
    ),
    enabled: true,
    last-execution: u0,
    execution-count: u0,
    success-count: u0
  }
)

;; Public Functions

;; Check if agent conditions are met and execute operation if true
(define-public (check-and-execute-agent (agent-id (string-ascii 64)))
  (let (
    (agent (unwrap! (map-get? agents {id: agent-id}) (err ERR_NOT_FOUND)))
  )
    ;; Check agent is enabled
    (asserts! (get enabled agent) (err ERR_AGENT_DISABLED))
    
    ;; Check if global agent system is enabled
    (asserts! (var-get agent-enabled) (err ERR_AGENT_DISABLED))
    
    ;; Check cooldown period
    (asserts! (>= (- block-height (get last-execution agent)) (var-get operation-cooldown)) (err ERR_COOLDOWN_ACTIVE))
    
    ;; Check trigger conditions
    (if (agent-condition-met agent-id)
        (create-operation agent-id)
        (err u0)) ;; No error, but didn't trigger
  ))

;; Create a pending operation that requires signatures
(define-public (create-operation (agent-id (string-ascii 64)))
  (let (
    (agent (unwrap! (map-get? agents {id: agent-id}) (err ERR_NOT_FOUND)))
    (operation-id (+ (var-get operation-counter) u1))
    (amount (get-operation-amount agent))
    (simulation-result (run-operation-simulation agent-id amount))
    (ml-confidence (get-ml-confidence agent-id))
    (risk-score (calculate-risk-score agent-id amount ml-confidence))
  )
    ;; Validate risk is within tolerance
    (asserts! (risk-within-tolerance risk-score) (err ERR_RISK_TOO_HIGH))
    
    ;; Reset signature tracking
    (var-set current-signers u0)
    
    ;; Create pending operation
    (map-set pending-operations
      operation-id
      {
        agent-id: agent-id,
        operation-type: (get operation-type agent),
        amount: amount,
        proposed-at: block-height,
        proposed-by: tx-sender,
        signers: (list tx-sender),
        expires-at: (+ block-height u144), ;; 1 day expiration
        simulation-result: simulation-result,
        ml-confidence: ml-confidence,
        risk-score: risk-score
      }
    )
    
    ;; Update counter
    (var-set operation-counter operation-id)
    
    ;; Sign it automatically from creator
    (add-signature operation-id)
    
    ;; If threshold is 1, execute directly
    (if (is-eq (var-get multi-sig-threshold) u1)
        (execute-operation operation-id)
        (ok operation-id))
  ))

;; Add signature to a pending operation
(define-public (add-signature (operation-id uint))
  (let (
    (pending-op (unwrap! (map-get? pending-operations operation-id) (err ERR_NOT_FOUND)))
    (current-signatures (get signers pending-op))
  )
    ;; Check caller is a signer
    (asserts! (is-signer tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check operation hasn't expired
    (asserts! (<= block-height (get expires-at pending-op)) (err ERR_NOT_FOUND))
    
    ;; Check caller hasn't already signed
    (asserts! (not (is-some (index-of current-signatures tx-sender))) (err ERR_ALREADY_SIGNED))
    
    ;; Add signature
    (map-set pending-operations
      operation-id
      (merge pending-op {signers: (append current-signatures (list tx-sender))})
    )
    
    ;; Update current signers count
    (var-set current-signers (+ (var-get current-signers) u1))
    
    ;; Check if we've reached the threshold
    (if (>= (var-get current-signers) (var-get multi-sig-threshold))
        (execute-operation operation-id)
        (ok operation-id))
  ))

;; Reset signatures for a pending operation
(define-public (reset-signatures (operation-id uint))
  (let (
    (pending-op (unwrap! (map-get? pending-operations operation-id) (err ERR_NOT_FOUND)))
  )
    ;; Only the proposer or an admin can reset signatures
    (asserts! (or (is-eq tx-sender (get proposed-by pending-op)) (is-administrator tx-sender)) (err ERR_UNAUTHORIZED))
    
    ;; Reset signatures to just the proposer
    (map-set pending-operations
      operation-id
      (merge pending-op {signers: (list (get proposed-by pending-op))})
    )
    
    ;; Reset counter
    (var-set current-signers u1)
    
    (ok true)
  ))

;; Execute a pending operation that has reached the signature threshold
(define-public (execute-operation (operation-id uint))
  (let (
    (pending-op (unwrap! (map-get? pending-operations operation-id) (err ERR_NOT_FOUND)))
    (agent-id (get agent-id pending-op))
    (agent (unwrap! (map-get? agents {id: agent-id}) (err ERR_NOT_FOUND)))
    (op-type (get operation-type pending-op))
    (amount (get amount pending-op))
    (signers-list (get signers pending-op))
  )
    ;; Check enough signatures
    (asserts! (>= (len signers-list) (var-get multi-sig-threshold)) (err ERR_INSUFFICIENT_SIGNATURES))
    
    ;; Check operation hasn't expired
    (asserts! (<= block-height (get expires-at pending-op)) (err ERR_NOT_FOUND))
    
    ;; Check global cooldown
    (asserts! (>= (- block-height (var-get last-operation-block)) (var-get operation-cooldown)) (err ERR_COOLDOWN_ACTIVE))
    
    ;; Check emergency mode
    (asserts! (not (var-get emergency-mode)) (err ERR_EMERGENCY_ACTIVE))
    
    ;; Execute operation based on type
    (let (
      (result (execute-by-type op-type amount))
    )
      ;; Record operation
      (map-set operations
        operation-id
        {
          agent-id: agent-id,
          operation-type: op-type,
          amount: amount,
          executed-at: block-height,
          executed-by: tx-sender,
          signers: signers-list,
          status: (if (is-ok result) "SUCCESS" "FAILED"),
          result: (if (is-ok result) (unwrap-panic result) "Operation execution failed"),
          simulation-result: (get simulation-result pending-op),
          ml-confidence: (get ml-confidence pending-op),
          risk-score: (get risk-score pending-op)
        }
      )
      
      ;; Clean up pending operation
      (map-delete pending-operations operation-id)
      
      ;; Update agent execution stats
      (map-set agents
        {id: agent-id}
        (merge agent
          {
            last-execution: block-height,
            execution-count: (+ (get execution-count agent) u1),
            success-count: (if (is-ok result)
                             (+ (get success-count agent) u1)
                             (get success-count agent))
          }
        )
      )
      
      ;; Update last operation block
      (var-set last-operation-block block-height)
      
      ;; Generate report
      (try! (as-contract (contract-call? REPORTING_CONTRACT generate-report u3 true (list))))
      
      result
    )
  ))

;; Execute operation by type
(define-private (execute-by-type (operation-type uint) (amount uint))
  (match operation-type
    OPERATION_RESERVE_ADJUSTMENT (execute-reserve-adjustment amount)
    OPERATION_LIQUIDITY_ADJUSTMENT (execute-liquidity-adjustment amount)
    OPERATION_BUYBACK (execute-buyback amount)
    OPERATION_EMISSION_ADJUSTMENT (execute-emission-adjustment amount)
    OPERATION_INCENTIVE_ADJUSTMENT (execute-incentive-adjustment amount)
    (err ERR_INVALID_PARAMETER)
  ))

;; Execute a reserve adjustment
(define-private (execute-reserve-adjustment (amount uint))
  (as-contract (contract-call? TREASURY_CONTRACT adjust-reserves amount))
)

;; Execute a liquidity adjustment
(define-private (execute-liquidity-adjustment (amount uint))
  (as-contract (contract-call? TREASURY_CONTRACT adjust-protocol-liquidity amount))
)

;; Execute a buyback operation
(define-private (execute-buyback (amount uint))
  (as-contract (contract-call? TREASURY_CONTRACT execute-buyback amount))
)

;; Execute an emission adjustment
(define-private (execute-emission-adjustment (amount uint))
  (as-contract (contract-call? DAO_CONTRACT propose-emission-adjustment amount))
)

;; Execute an incentive adjustment
(define-private (execute-incentive-adjustment (amount uint))
  (as-contract (contract-call? DAO_CONTRACT adjust-governance-incentives amount))
)

;; Simulation Functions

;; Run simulation for an operation
(define-public (run-operation-simulation (agent-id (string-ascii 64)) (amount uint))
  (let (
    (agent (unwrap! (map-get? agents {id: agent-id}) (err ERR_NOT_FOUND)))
    (operation-type (get operation-type agent))
    (simulation-id (+ (var-get simulation-counter) u1))
    (parameters (get parameters agent))
  )
    ;; Store simulation result
    (map-set simulation-results
      {operation-type: operation-type, simulation-id: simulation-id}
      {
        parameters: parameters,
        result: (simulate-by-type operation-type amount),
        impact-score: (calculate-impact-score operation-type amount),
        risk-score: (calculate-risk-score agent-id amount u500), ;; Default confidence
        success: true,
        timestamp: block-height
      }
    )
    
    ;; Update counter
    (var-set simulation-counter simulation-id)
    
    (concat "SIM-" (to-uint simulation-id))
  ))

;; Simulate operation by type
(define-private (simulate-by-type (operation-type uint) (amount uint))
  (match operation-type
    OPERATION_RESERVE_ADJUSTMENT (simulate-reserve-adjustment amount)
    OPERATION_LIQUIDITY_ADJUSTMENT (simulate-liquidity-adjustment amount)
    OPERATION_BUYBACK (simulate-buyback amount)
    OPERATION_EMISSION_ADJUSTMENT (simulate-emission-adjustment amount)
    OPERATION_INCENTIVE_ADJUSTMENT (simulate-incentive-adjustment amount)
    "UNKNOWN_OPERATION_TYPE"
  ))

;; Simulate reserve adjustment
(define-private (simulate-reserve-adjustment (amount uint))
  ;; Would use complex simulation in full implementation
  ;; Simplified version for now
  (concat "Reserve adjustment of " (to-uint amount) " simulated successfully")
)

;; Simulate liquidity adjustment
(define-private (simulate-liquidity-adjustment (amount uint))
  ;; Would use complex simulation in full implementation
  ;; Simplified version for now
  (concat "Liquidity adjustment of " (to-uint amount) " simulated successfully")
)

;; Simulate buyback
(define-private (simulate-buyback (amount uint))
  ;; Would use complex simulation in full implementation
  ;; Simplified version for now
  (concat "Buyback of " (to-uint amount) " simulated successfully")
)

;; Simulate emission adjustment
(define-private (simulate-emission-adjustment (amount uint))
  ;; Would use complex simulation in full implementation
  ;; Simplified version for now
  (concat "Emission adjustment of " (to-uint amount) " simulated successfully")
)

;; Simulate incentive adjustment
(define-private (simulate-incentive-adjustment (amount uint))
  ;; Would use complex simulation in full implementation
  ;; Simplified version for now
  (concat "Incentive adjustment of " (to-uint amount) " simulated successfully")
)

;; ML Integration Functions

;; Submit ML recommendation
(define-public (submit-ml-recommendation 
                (metric (string-ascii 64))
                (recommendation (string-ascii 256))
                (confidence uint)
                (parameters (list 10 {name: (string-ascii 64), value: uint})))
  (begin
    ;; Check caller is an ML provider
    (asserts! (is-ml-provider tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Validate confidence (0-1000 basis points, 0-10%)
    (asserts! (<= confidence u1000) (err ERR_INVALID_PARAMETER))
    
    ;; Store recommendation
    (map-set ml-recommendations
      {metric: metric}
      {
        recommendation: recommendation,
        confidence: confidence,
        provider: tx-sender,
        timestamp: block-height,
        parameters: parameters
      }
    )
    
    (ok true)
  ))

;; Get ML recommendation confidence for an agent
(define-private (get-ml-confidence (agent-id (string-ascii 64)))
  (let (
    (agent (unwrap! (map-get? agents {id: agent-id}) none))
    (metric-name (get-agent-primary-metric agent-id))
  )
    (match (map-get? ml-recommendations {metric: metric-name})
      recommendation (get confidence recommendation)
      u500 ;; Default medium confidence (50%)
    )
  ))

;; Get agent's primary metric
(define-private (get-agent-primary-metric (agent-id (string-ascii 64)))
  (match agent-id
    "reserve_manager" "reserve_ratio"
    "liquidity_manager" "pol_percentage"
    "buyback_agent" "price_7d_change"
    "velocity_controller" "token_velocity"
    "participation_optimizer" "voting_participation"
    "unknown_metric"
  ))

;; Get agent's operation amount
(define-private (get-operation-amount (agent {
                                              description: (string-ascii 256),
                                              operation-type: uint,
                                              trigger-condition: (string-ascii 256),
                                              parameters: (list 10 {name: (string-ascii 64), value: uint}),
                                              enabled: bool,
                                              last-execution: uint,
                                              execution-count: uint,
                                              success-count: uint
                                            }))
  ;; For simple operations, return the max amount from parameters
  ;; In a full implementation, this would dynamically calculate based on treasury size and parameters
  (let (
    (operation-type (get operation-type agent))
  )
    (match operation-type
      OPERATION_RESERVE_ADJUSTMENT (get-parameter-value (get parameters agent) "max_adjustment")
      OPERATION_LIQUIDITY_ADJUSTMENT (get-parameter-value (get parameters agent) "max_adjustment")
      OPERATION_BUYBACK (get-parameter-value (get parameters agent) "max_size")
      OPERATION_EMISSION_ADJUSTMENT (get-parameter-value (get parameters agent) "max_adjustment")
      OPERATION_INCENTIVE_ADJUSTMENT (get-parameter-value (get parameters agent) "target_boost")
      u100 ;; Default value
    )
  ))

;; Get parameter value from list
(define-private (get-parameter-value (params (list 10 {name: (string-ascii 64), value: uint})) (param-name (string-ascii 64)))
  (default-to u100
    (get value
      (default-to {name: "", value: u100}
        (find (lambda (param) (is-eq (get name param) param-name)) params)
      )
    )
  ))

;; Calculate impact score for an operation
(define-private (calculate-impact-score (operation-type uint) (amount uint))
  ;; In a real implementation, this would use complex impact calculation logic
  ;; For now, we'll use a simplified approach
  (let (
    (base-score u500) ;; 5% base impact
    (amount-factor (/ (* amount u1000) u10000)) ;; Scale based on amount
  )
    (+ base-score amount-factor)
  ))

;; Calculate risk score for an operation
(define-private (calculate-risk-score (agent-id (string-ascii 64)) (amount uint) (ml-confidence uint))
  ;; In a real implementation, this would be a complex risk model
  ;; For now, we'll use a simplified approach
  (let (
    (base-risk u300) ;; 3% base risk
    (amount-factor (/ (* amount u1000) u20000)) ;; Scale based on amount, less than impact
    (confidence-factor (/ (* (- u1000 ml-confidence) u500) u1000)) ;; Lower confidence increases risk
  )
    (+ (+ base-risk amount-factor) confidence-factor)
  ))

;; Check if a risk score is within the current tolerance
(define-private (risk-within-tolerance (risk-score uint))
  (let (
    (tolerance (var-get risk-tolerance))
  )
    (match tolerance
      "low" (<= risk-score u500) ;; Max 5% risk
      "medium" (<= risk-score u750) ;; Max 7.5% risk
      "high" (<= risk-score u1000) ;; Max 10% risk
      false
    )
  ))

;; Check if an agent's condition is met
(define-private (agent-condition-met (agent-id (string-ascii 64)))
  ;; In a full implementation, this would parse and evaluate the trigger condition
  ;; For demonstration, we'll check the primary metric against a threshold
  (let (
    (metric-name (get-agent-primary-metric agent-id))
    (current-value (get-metric-value metric-name))
    (threshold (get-agent-threshold agent-id))
    (comparison-type (get-agent-comparison-type agent-id))
  )
    (match comparison-type
      ">" (> current-value threshold)
      "<" (< current-value threshold)
      "=" (is-eq current-value threshold)
      ">=" (>= current-value threshold)
      "<=" (<= current-value threshold)
      "!=" (not (is-eq current-value threshold))
      false
    )
  ))

;; Get metric value
(define-private (get-metric-value (metric-name (string-ascii 64)))
  ;; In a full implementation, this would query the metrics oracle
  ;; For demonstration, return placeholder values
  (match metric-name
    "reserve_ratio" u140 ;; 14.0% reserve ratio
    "pol_percentage" u130 ;; 13.0% protocol-owned liquidity
    "price_7d_change" (- u200) ;; -2.0% price change
    "token_velocity" u450 ;; 4.5 velocity
    "voting_participation" u180 ;; 18.0% participation
    u100 ;; Default value
  ))

;; Get agent threshold
(define-private (get-agent-threshold (agent-id (string-ascii 64)))
  (match agent-id
    "reserve_manager" u150 ;; 15.0% target
    "liquidity_manager" u150 ;; 15.0% target
    "buyback_agent" (- u150) ;; -1.5% price change trigger
    "velocity_controller" u400 ;; 4.0 velocity threshold
    "participation_optimizer" u200 ;; 20.0% participation threshold
    u100 ;; Default value
  ))

;; Get agent comparison type
(define-private (get-agent-comparison-type (agent-id (string-ascii 64)))
  (match agent-id
    "reserve_manager" "!=" ;; Not equal to target
    "liquidity_manager" "!=" ;; Not equal to target
    "buyback_agent" "<" ;; Less than threshold
    "velocity_controller" "!=" ;; Not equal to threshold
    "participation_optimizer" "<" ;; Less than threshold
    "=" ;; Default comparison
  ))

;; Emergency Functions

;; Activate emergency mode
(define-public (activate-emergency-mode)
  (begin
    ;; Only administrators can activate emergency mode
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Set emergency mode
    (var-set emergency-mode true)
    
    ;; Notify operations manager
    (try! (as-contract (contract-call? OPERATIONS_CONTRACT activate-emergency)))
    
    (ok true)
  ))

;; Deactivate emergency mode
(define-public (deactivate-emergency-mode)
  (begin
    ;; Only administrators can deactivate emergency mode
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Clear emergency mode
    (var-set emergency-mode false)
    
    ;; Notify operations manager
    (try! (as-contract (contract-call? OPERATIONS_CONTRACT deactivate-emergency)))
    
    (ok true)
  ))

;; Read-Only Functions

;; Get agent details
(define-read-only (get-agent (agent-id (string-ascii 64)))
  (map-get? agents {id: agent-id}))

;; Get all agent IDs
(define-read-only (get-agent-ids)
  (list 
    "reserve_manager"
    "liquidity_manager"
    "buyback_agent"
    "velocity_controller"
    "participation_optimizer"
  ))

;; Get operation details
(define-read-only (get-operation (operation-id uint))
  (map-get? operations operation-id))

;; Get pending operation details
(define-read-only (get-pending-operation (operation-id uint))
  (map-get? pending-operations operation-id))

;; Get ML recommendation for a metric
(define-read-only (get-ml-recommendation (metric (string-ascii 64)))
  (map-get? ml-recommendations {metric: metric}))

;; Get simulation result
(define-read-only (get-simulation-result (operation-type uint) (simulation-id uint))
  (map-get? simulation-results {operation-type: operation-type, simulation-id: simulation-id}))

;; Get recent operations
(define-read-only (get-recent-operations (limit uint))
  (let (
    (current-id (var-get operation-counter))
    (start-id (if (> current-id limit) (- current-id limit) u1))
  )
    (get-operations-in-range start-id current-id)
  ))

;; Get operations in a range
(define-read-only (get-operations-in-range (start-id uint) (end-id uint))
  (map get-operation-or-none (sequence start-id end-id))
)

;; Get operation or none
(define-private (get-operation-or-none (id uint))
  (map-get? operations id)
)

;; Create a sequence of integers
(define-private (sequence (start uint) (end uint))
  (if (<= start end)
      (append (list start) (sequence (+ start u1) end))
      (list)
  )
)

;; Get agent success rate
(define-read-only (get-agent-success-rate (agent-id (string-ascii 64)))
  (let (
    (agent (default-to 
             {
               description: "",
               operation-type: u0,
               trigger-condition: "",
               parameters: (list),
               enabled: false,
               last-execution: u0,
               execution-count: u0,
               success-count: u0
             } 
             (map-get? agents {id: agent-id})))
    (execution-count (get execution-count agent))
    (success-count (get success-count agent))
  )
    (if (> execution-count u0)
        (/ (* success-count u10000) execution-count) ;; Basis points with 2 decimals
        u0)
  ))

;; Get financial agent status
(define-read-only (get-agent-status)
  {
    enabled: (var-get agent-enabled),
    risk-tolerance: (var-get risk-tolerance),
    max-operation-size: (var-get max-operation-size),
    operation-cooldown: (var-get operation-cooldown),
    multi-sig-threshold: (var-get multi-sig-threshold),
    last-operation-block: (var-get last-operation-block),
    current-signers: (var-get current-signers),
    emergency-mode: (var-get emergency-mode),
    simulation-depth: (var-get simulation-depth),
    operation-count: (var-get operation-counter),
    simulation-count: (var-get simulation-counter)
  })

;; Check if account is an administrator
(define-read-only (is-administrator (account principal))
  (default-to false (map-get? administrators account)))

;; Check if account is a signer
(define-read-only (is-signer (account principal))
  (default-to false (map-get? signers account)))

;; Check if account is an ML provider
(define-read-only (is-ml-provider (account principal))
  (default-to false (map-get? ml-providers account)))

;; Administrative Functions

;; Add an agent
(define-public (add-agent (agent-id (string-ascii 64))
                          (description (string-ascii 256))
                          (operation-type uint)
                          (trigger-condition (string-ascii 256))
                          (parameters (list 10 {name: (string-ascii 64), value: uint})))
  (begin
    ;; Only administrators can add agents
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Add the agent
    (map-set agents
      {id: agent-id}
      {
        description: description,
        operation-type: operation-type,
        trigger-condition: trigger-condition,
        parameters: parameters,
        enabled: true,
        last-execution: u0,
        execution-count: u0,
        success-count: u0
      }
    )
    
    (ok true)
  ))

;; Update an agent
(define-public (update-agent (agent-id (string-ascii 64))
                            (description (string-ascii 256))
                            (trigger-condition (string-ascii 256))
                            (parameters (list 10 {name: (string-ascii 64), value: uint})))
  (begin
    ;; Only administrators can update agents
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Get existing agent
    (let (
      (agent (unwrap! (map-get? agents {id: agent-id}) (err ERR_NOT_FOUND)))
    )
      ;; Update the agent
      (map-set agents
        {id: agent-id}
        (merge agent {
          description: description,
          trigger-condition: trigger-condition,
          parameters: parameters
        })
      )
    )
    
    (ok true)
  ))

;; Enable or disable an agent
(define-public (toggle-agent (agent-id (string-ascii 64)) (enabled bool))
  (begin
    ;; Only administrators can toggle agents
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Get existing agent
    (let (
      (agent (unwrap! (map-get? agents {id: agent-id}) (err ERR_NOT_FOUND)))
    )
      ;; Update the agent
      (map-set agents
        {id: agent-id}
        (merge agent {enabled: enabled})
      )
    )
    
    (ok true)
  ))

;; Enable or disable the entire agent system
(define-public (toggle-agent-system (enabled bool))
  (begin
    ;; Only administrators can toggle the system
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Update the setting
    (var-set agent-enabled enabled)
    
    (ok true)
  ))

;; Update agent settings
(define-public (update-agent-settings
               (new-risk-tolerance (optional (string-ascii 10)))
               (new-max-operation-size (optional uint))
               (new-operation-cooldown (optional uint))
               (new-multi-sig-threshold (optional uint))
               (new-simulation-depth (optional uint)))
  (begin
    ;; Only administrators can update settings
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Update risk tolerance if provided
    (match new-risk-tolerance
      tolerance (var-set risk-tolerance tolerance)
      true)
    
    ;; Update max operation size if provided
    (match new-max-operation-size
      size (var-set max-operation-size size)
      true)
    
    ;; Update operation cooldown if provided
    (match new-operation-cooldown
      cooldown (var-set operation-cooldown cooldown)
      true)
    
    ;; Update multi-signature threshold if provided
    (match new-multi-sig-threshold
      threshold (var-set multi-sig-threshold threshold)
      true)
    
    ;; Update simulation depth if provided
    (match new-simulation-depth
      depth (var-set simulation-depth depth)
      true)
    
    (ok true)
  ))