;; Metrics Oracle Contract
;; [AIR-3][AIS-3][AIM-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_PARAMETER u402)
(define-constant ERR_STALE_DATA u403)
(define-constant ERR_RATE_LIMITED u404)
(define-constant ERR_ORACLE_OFFLINE u405)

;; Contract references
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance)
(define-constant TREASURY_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury-management)
(define-constant TOKENOMICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.tokenomics)

;; Data vars
(define-data-var metrics-enabled bool true)
(define-data-var update-interval uint u100) ;; Every 100 blocks
(define-data-var data-freshness-threshold uint u500) ;; Max 500 blocks old
(define-data-var last-updated uint u0)
(define-data-var oracle-status (string-ascii 20) "ACTIVE")

;; Admin list
(define-map administrators principal bool)
(define-map oracle-providers principal bool)

;; Initialize administrators
(map-set administrators tx-sender true)
(map-set oracle-providers tx-sender true)

;; Metric Types
(define-constant METRIC_TYPE_TOKEN u1)
(define-constant METRIC_TYPE_TREASURY u2)
(define-constant METRIC_TYPE_GOVERNANCE u3)
(define-constant METRIC_TYPE_MARKET u4)

;; Token Metrics
(define-map token-metrics
  { metric-name: (string-ascii 64) }
  {
    value: uint,
    last-updated: uint,
    provider: principal,
    confidence: uint,
    source: (string-ascii 64)
  }
)

;; Treasury Metrics
(define-map treasury-metrics
  { metric-name: (string-ascii 64) }
  {
    value: uint,
    last-updated: uint,
    provider: principal,
    confidence: uint,
    source: (string-ascii 64)
  }
)

;; Governance Metrics
(define-map governance-metrics
  { metric-name: (string-ascii 64) }
  {
    value: uint,
    last-updated: uint,
    provider: principal,
    confidence: uint,
    source: (string-ascii 64)
  }
)

;; Market Metrics
(define-map market-metrics
  { metric-name: (string-ascii 64) }
  {
    value: uint,
    last-updated: uint,
    provider: principal,
    confidence: uint,
    source: (string-ascii 64)
  }
)

;; Metric names registry
(define-map valid-metric-names
  { metric-name: (string-ascii 64) }
  { metric-type: uint }
)

;; Initialize common metrics
(map-set valid-metric-names { metric-name: "circulating_supply" } { metric-type: METRIC_TYPE_TOKEN })
(map-set valid-metric-names { metric-name: "total_minted" } { metric-type: METRIC_TYPE_TOKEN })
(map-set valid-metric-names { metric-name: "total_burned" } { metric-type: METRIC_TYPE_TOKEN })
(map-set valid-metric-names { metric-name: "velocity" } { metric-type: METRIC_TYPE_TOKEN })
(map-set valid-metric-names { metric-name: "active_wallets" } { metric-type: METRIC_TYPE_TOKEN })
(map-set valid-metric-names { metric-name: "holder_distribution" } { metric-type: METRIC_TYPE_TOKEN })
(map-set valid-metric-names { metric-name: "median_balance" } { metric-type: METRIC_TYPE_TOKEN })

(map-set valid-metric-names { metric-name: "reserves_ratio" } { metric-type: METRIC_TYPE_TREASURY })
(map-set valid-metric-names { metric-name: "pol_percentage" } { metric-type: METRIC_TYPE_TREASURY })
(map-set valid-metric-names { metric-name: "protocol_revenue" } { metric-type: METRIC_TYPE_TREASURY })
(map-set valid-metric-names { metric-name: "buyback_volume" } { metric-type: METRIC_TYPE_TREASURY })
(map-set valid-metric-names { metric-name: "runway_months" } { metric-type: METRIC_TYPE_TREASURY })
(map-set valid-metric-names { metric-name: "investment_performance" } { metric-type: METRIC_TYPE_TREASURY })
(map-set valid-metric-names { metric-name: "collateralization_ratio" } { metric-type: METRIC_TYPE_TREASURY })

(map-set valid-metric-names { metric-name: "proposal_volume" } { metric-type: METRIC_TYPE_GOVERNANCE })
(map-set valid-metric-names { metric-name: "voting_participation" } { metric-type: METRIC_TYPE_GOVERNANCE })
(map-set valid-metric-names { metric-name: "vote_concentration" } { metric-type: METRIC_TYPE_GOVERNANCE })
(map-set valid-metric-names { metric-name: "execution_success_rate" } { metric-type: METRIC_TYPE_GOVERNANCE })
(map-set valid-metric-names { metric-name: "voter_retention" } { metric-type: METRIC_TYPE_GOVERNANCE })
(map-set valid-metric-names { metric-name: "proposal_category_distribution" } { metric-type: METRIC_TYPE_GOVERNANCE })

(map-set valid-metric-names { metric-name: "price_7d_change" } { metric-type: METRIC_TYPE_MARKET })
(map-set valid-metric-names { metric-name: "liquidity_depth" } { metric-type: METRIC_TYPE_MARKET })
(map-set valid-metric-names { metric-name: "volume_24h" } { metric-type: METRIC_TYPE_MARKET })
(map-set valid-metric-names { metric-name: "volatility_index" } { metric-type: METRIC_TYPE_MARKET })
(map-set valid-metric-names { metric-name: "correlation_btc" } { metric-type: METRIC_TYPE_MARKET })
(map-set valid-metric-names { metric-name: "sentiment_score" } { metric-type: METRIC_TYPE_MARKET })

;; Public Functions

;; Submit a token metric
(define-public (submit-token-metric (metric-name (string-ascii 64)) (value uint) (confidence uint) (source (string-ascii 64)))
  (begin
    ;; Check oracle provider authorization
    (asserts! (is-oracle-provider tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if metrics are enabled
    (asserts! (var-get metrics-enabled) (err ERR_ORACLE_OFFLINE))
    
    ;; Validate metric name
    (asserts! (is-valid-metric metric-name METRIC_TYPE_TOKEN) (err ERR_INVALID_PARAMETER))
    
    ;; Store the metric
    (map-set token-metrics
      { metric-name: metric-name }
      {
        value: value,
        last-updated: block-height,
        provider: tx-sender,
        confidence: confidence,
        source: source
      }
    )
    
    ;; Update last updated timestamp
    (var-set last-updated block-height)
    
    (ok true)
  ))

;; Submit a treasury metric
(define-public (submit-treasury-metric (metric-name (string-ascii 64)) (value uint) (confidence uint) (source (string-ascii 64)))
  (begin
    ;; Check oracle provider authorization
    (asserts! (is-oracle-provider tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if metrics are enabled
    (asserts! (var-get metrics-enabled) (err ERR_ORACLE_OFFLINE))
    
    ;; Validate metric name
    (asserts! (is-valid-metric metric-name METRIC_TYPE_TREASURY) (err ERR_INVALID_PARAMETER))
    
    ;; Store the metric
    (map-set treasury-metrics
      { metric-name: metric-name }
      {
        value: value,
        last-updated: block-height,
        provider: tx-sender,
        confidence: confidence,
        source: source
      }
    )
    
    ;; Update last updated timestamp
    (var-set last-updated block-height)
    
    (ok true)
  ))

;; Submit a governance metric
(define-public (submit-governance-metric (metric-name (string-ascii 64)) (value uint) (confidence uint) (source (string-ascii 64)))
  (begin
    ;; Check oracle provider authorization
    (asserts! (is-oracle-provider tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if metrics are enabled
    (asserts! (var-get metrics-enabled) (err ERR_ORACLE_OFFLINE))
    
    ;; Validate metric name
    (asserts! (is-valid-metric metric-name METRIC_TYPE_GOVERNANCE) (err ERR_INVALID_PARAMETER))
    
    ;; Store the metric
    (map-set governance-metrics
      { metric-name: metric-name }
      {
        value: value,
        last-updated: block-height,
        provider: tx-sender,
        confidence: confidence,
        source: source
      }
    )
    
    ;; Update last updated timestamp
    (var-set last-updated block-height)
    
    (ok true)
  ))

;; Submit a market metric
(define-public (submit-market-metric (metric-name (string-ascii 64)) (value uint) (confidence uint) (source (string-ascii 64)))
  (begin
    ;; Check oracle provider authorization
    (asserts! (is-oracle-provider tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if metrics are enabled
    (asserts! (var-get metrics-enabled) (err ERR_ORACLE_OFFLINE))
    
    ;; Validate metric name
    (asserts! (is-valid-metric metric-name METRIC_TYPE_MARKET) (err ERR_INVALID_PARAMETER))
    
    ;; Store the metric
    (map-set market-metrics
      { metric-name: metric-name }
      {
        value: value,
        last-updated: block-height,
        provider: tx-sender,
        confidence: confidence,
        source: source
      }
    )
    
    ;; Update last updated timestamp
    (var-set last-updated block-height)
    
    (ok true)
  ))

;; Batch submit metrics (for gas efficiency)
(define-public (batch-submit-metrics 
                (token-metrics-list (list 10 {name: (string-ascii 64), value: uint, confidence: uint}))
                (treasury-metrics-list (list 10 {name: (string-ascii 64), value: uint, confidence: uint}))
                (governance-metrics-list (list 10 {name: (string-ascii 64), value: uint, confidence: uint}))
                (market-metrics-list (list 10 {name: (string-ascii 64), value: uint, confidence: uint}))
                (source (string-ascii 64)))
  (begin
    ;; Check oracle provider authorization
    (asserts! (is-oracle-provider tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if metrics are enabled
    (asserts! (var-get metrics-enabled) (err ERR_ORACLE_OFFLINE))
    
    ;; Process each list of metrics
    (map process-token-metric token-metrics-list)
    (map process-treasury-metric treasury-metrics-list)
    (map process-governance-metric governance-metrics-list)
    (map process-market-metric market-metrics-list)
    
    ;; Update last updated timestamp
    (var-set last-updated block-height)
    
    (ok true)
  ))

;; Helper functions for batch processing
(define-private (process-token-metric (metric {name: (string-ascii 64), value: uint, confidence: uint}))
  (if (is-valid-metric (get name metric) METRIC_TYPE_TOKEN)
      (map-set token-metrics
        { metric-name: (get name metric) }
        {
          value: (get value metric),
          last-updated: block-height,
          provider: tx-sender,
          confidence: (get confidence metric),
          source: "batch-update"
        }
      )
      false
  ))

(define-private (process-treasury-metric (metric {name: (string-ascii 64), value: uint, confidence: uint}))
  (if (is-valid-metric (get name metric) METRIC_TYPE_TREASURY)
      (map-set treasury-metrics
        { metric-name: (get name metric) }
        {
          value: (get value metric),
          last-updated: block-height,
          provider: tx-sender,
          confidence: (get confidence metric),
          source: "batch-update"
        }
      )
      false
  ))

(define-private (process-governance-metric (metric {name: (string-ascii 64), value: uint, confidence: uint}))
  (if (is-valid-metric (get name metric) METRIC_TYPE_GOVERNANCE)
      (map-set governance-metrics
        { metric-name: (get name metric) }
        {
          value: (get value metric),
          last-updated: block-height,
          provider: tx-sender,
          confidence: (get confidence metric),
          source: "batch-update"
        }
      )
      false
  ))

(define-private (process-market-metric (metric {name: (string-ascii 64), value: uint, confidence: uint}))
  (if (is-valid-metric (get name metric) METRIC_TYPE_MARKET)
      (map-set market-metrics
        { metric-name: (get name metric) }
        {
          value: (get value metric),
          last-updated: block-height,
          provider: tx-sender,
          confidence: (get confidence metric),
          source: "batch-update"
        }
      )
      false
  ))

;; Read-Only Functions

;; Get token metric
(define-read-only (get-token-metric (metric-name (string-ascii 64)))
  (let ((metric (map-get? token-metrics { metric-name: metric-name })))
    (if (and 
          (is-some metric) 
          (< (- block-height (get last-updated (unwrap-panic metric))) (var-get data-freshness-threshold))
        )
        (ok (unwrap-panic metric))
        (err ERR_STALE_DATA)
    )
  ))

;; Get treasury metric
(define-read-only (get-treasury-metric (metric-name (string-ascii 64)))
  (let ((metric (map-get? treasury-metrics { metric-name: metric-name })))
    (if (and 
          (is-some metric) 
          (< (- block-height (get last-updated (unwrap-panic metric))) (var-get data-freshness-threshold))
        )
        (ok (unwrap-panic metric))
        (err ERR_STALE_DATA)
    )
  ))

;; Get governance metric
(define-read-only (get-governance-metric (metric-name (string-ascii 64)))
  (let ((metric (map-get? governance-metrics { metric-name: metric-name })))
    (if (and 
          (is-some metric) 
          (< (- block-height (get last-updated (unwrap-panic metric))) (var-get data-freshness-threshold))
        )
        (ok (unwrap-panic metric))
        (err ERR_STALE_DATA)
    )
  ))

;; Get market metric
(define-read-only (get-market-metric (metric-name (string-ascii 64)))
  (let ((metric (map-get? market-metrics { metric-name: metric-name })))
    (if (and 
          (is-some metric) 
          (< (- block-height (get last-updated (unwrap-panic metric))) (var-get data-freshness-threshold))
        )
        (ok (unwrap-panic metric))
        (err ERR_STALE_DATA)
    )
  ))

;; Get multiple token metrics in one call
(define-read-only (get-multiple-token-metrics (metric-names (list 10 (string-ascii 64))))
  (ok (map get-token-metric-unwrapped metric-names)))

;; Helper function that returns the metric or a default
(define-private (get-token-metric-unwrapped (metric-name (string-ascii 64)))
  (default-to 
    {
      value: u0,
      last-updated: u0,
      provider: tx-sender,
      confidence: u0,
      source: "not-found"
    }
    (map-get? token-metrics { metric-name: metric-name })
  ))

;; Get oracle status and metrics
(define-read-only (get-oracle-status)
  {
    status: (var-get oracle-status),
    enabled: (var-get metrics-enabled),
    last-updated: (var-get last-updated),
    update-interval: (var-get update-interval),
    data-freshness-threshold: (var-get data-freshness-threshold),
    blocks-since-update: (- block-height (var-get last-updated))
  })

;; Check if metric is valid
(define-read-only (is-valid-metric (metric-name (string-ascii 64)) (expected-type uint))
  (match (map-get? valid-metric-names { metric-name: metric-name })
    metric-info (is-eq (get metric-type metric-info) expected-type)
    false
  ))

;; Check if account is an oracle provider
(define-read-only (is-oracle-provider (account principal))
  (default-to false (map-get? oracle-providers account)))

;; Check if account is an administrator
(define-read-only (is-administrator (account principal))
  (default-to false (map-get? administrators account)))

;; Administrative Functions

;; Add a metric
(define-public (add-metric (metric-name (string-ascii 64)) (metric-type uint))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (asserts! (and (>= metric-type METRIC_TYPE_TOKEN) (<= metric-type METRIC_TYPE_MARKET)) (err ERR_INVALID_PARAMETER))
    
    (map-set valid-metric-names 
      { metric-name: metric-name } 
      { metric-type: metric-type }
    )
    
    (ok true)
  ))

;; Add an oracle provider
(define-public (add-oracle-provider (provider principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set oracle-providers provider true)
    (ok true)
  ))

;; Remove an oracle provider
(define-public (remove-oracle-provider (provider principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set oracle-providers provider false)
    (ok true)
  ))

;; Add an administrator
(define-public (add-administrator (admin principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set administrators admin true)
    (ok true)
  ))

;; Remove an administrator
(define-public (remove-administrator (admin principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set administrators admin false)
    (ok true)
  ))

;; Update oracle settings
(define-public (update-oracle-settings 
               (new-enabled (optional bool))
               (new-update-interval (optional uint))
               (new-data-freshness-threshold (optional uint))
               (new-oracle-status (optional (string-ascii 20))))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Update enabled status if provided
    (match new-enabled
      status (var-set metrics-enabled status)
      true)
    
    ;; Update interval if provided
    (match new-update-interval
      interval (var-set update-interval interval)
      true)
    
    ;; Update freshness threshold if provided
    (match new-data-freshness-threshold
      threshold (var-set data-freshness-threshold threshold)
      true)
    
    ;; Update oracle status if provided
    (match new-oracle-status
      status (var-set oracle-status status)
      true)
    
    (ok true)
  )) 