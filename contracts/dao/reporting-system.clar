;; Reporting System Contract
;; [AIR-3][AIS-3][AIM-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_PARAMETER u402)
(define-constant ERR_REPORT_DISABLED u403)
(define-constant ERR_REPORT_NOT_FOUND u404)
(define-constant ERR_DATA_NOT_AVAILABLE u405)

;; Contract references
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance)
(define-constant TREASURY_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury-management)
(define-constant METRICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.metrics-oracle)
(define-constant FINANCIAL_AGENT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.financial-agent)

;; Data vars
(define-data-var reporting-enabled bool true)
(define-data-var reporting-interval uint u1000) ;; Generate reports every 1000 blocks
(define-data-var data-retention-blocks uint u1000000) ;; Keep reports for 1,000,000 blocks
(define-data-var last-report-generation uint u0)
(define-data-var privacy-level (string-ascii 20) "aggregated")

;; Admin list
(define-map administrators principal bool)
(define-map report-generators principal bool)

;; Initialize administrators
(map-set administrators tx-sender true)
(map-set report-generators tx-sender true)

;; Report Types
(define-constant REPORT_TYPE_TREASURY u1)
(define-constant REPORT_TYPE_TOKENOMICS u2)
(define-constant REPORT_TYPE_GOVERNANCE u3)
(define-constant REPORT_TYPE_OPERATIONS u4)

;; Report Frequencies
(define-constant FREQUENCY_DAILY u1)
(define-constant FREQUENCY_WEEKLY u7)
(define-constant FREQUENCY_BIWEEKLY u14)
(define-constant FREQUENCY_MONTHLY u30)
(define-constant FREQUENCY_QUARTERLY u90)

;; Report data
(define-map reports
  uint ;; report ID
  {
    report-type: uint,
    title: (string-ascii 64),
    created-at: uint,
    creator: principal,
    data-hash: (buff 32),
    metrics-included: (list 10 (string-ascii 64)),
    is-public: bool,
    frequency: uint,
    uri: (string-utf8 256)
  }
)

;; Report type definitions
(define-map report-types
  { report-type: uint }
  {
    name: (string-ascii 64),
    description: (string-utf8 256),
    default-components: (list 10 (string-ascii 64)),
    default-frequency: uint,
    is-enabled: bool
  }
)

;; Report data (metrics snapshots)
(define-map report-data
  { report-id: uint, metric-name: (string-ascii 64) }
  {
    value: uint,
    timestamp: uint
  }
)

;; Report counter
(define-data-var report-counter uint u0)

;; Initialize report types
(map-set report-types
  { report-type: REPORT_TYPE_TREASURY }
  {
    name: "Treasury Status Report",
    description: "Comprehensive overview of treasury assets, reserves, and financial position",
    default-components: (list
      "asset_allocation"
      "reserve_ratio"
      "pol_percentage"
      "runway_analysis"
    ),
    default-frequency: FREQUENCY_WEEKLY,
    is-enabled: true
  }
)

(map-set report-types
  { report-type: REPORT_TYPE_TOKENOMICS }
  {
    name: "Tokenomics Health Report",
    description: "Analysis of token distribution, velocity, and market behavior",
    default-components: (list
      "distribution_analysis"
      "velocity_metrics"
      "emission_efficiency"
      "holder_analysis"
    ),
    default-frequency: FREQUENCY_MONTHLY,
    is-enabled: true
  }
)

(map-set report-types
  { report-type: REPORT_TYPE_GOVERNANCE }
  {
    name: "Governance Activity Report",
    description: "Summary of governance proposals, voting patterns, and participation",
    default-components: (list
      "proposal_analytics"
      "voting_patterns"
      "execution_results"
      "participation_trends"
    ),
    default-frequency: FREQUENCY_BIWEEKLY,
    is-enabled: true
  }
)

(map-set report-types
  { report-type: REPORT_TYPE_OPERATIONS }
  {
    name: "Financial Operations Report",
    description: "Detailed record of financial agent activities and treasury operations",
    default-components: (list
      "transaction_history"
      "agent_activities"
      "financial_impact"
      "risk_assessment"
    ),
    default-frequency: FREQUENCY_MONTHLY,
    is-enabled: true
  }
)

;; Public Functions

;; Generate a report
(define-public (generate-report (report-type uint) (is-public bool) (metrics-to-include (list 10 (string-ascii 64))))
  (let (
    (report-type-info (unwrap! (map-get? report-types {report-type: report-type}) (err ERR_INVALID_PARAMETER)))
    (report-id (+ (var-get report-counter) u1))
  )
    ;; Check report generator authorization
    (asserts! (is-report-generator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if reporting is enabled
    (asserts! (var-get reporting-enabled) (err ERR_REPORT_DISABLED))
    
    ;; Check if report type is enabled
    (asserts! (get is-enabled report-type-info) (err ERR_REPORT_DISABLED))
    
    ;; Collect metrics for the report
    (let (
      (collected-metrics (collect-metrics-for-report report-type metrics-to-include))
      (data-hash (hash collected-metrics))
      (metrics-list (if (> (len metrics-to-include) u0)
                       metrics-to-include
                       (get default-components report-type-info)))
    )
      ;; Create the report
      (map-set reports
        report-id
        {
          report-type: report-type,
          title: (get name report-type-info),
          created-at: block-height,
          creator: tx-sender,
          data-hash: data-hash,
          metrics-included: metrics-list,
          is-public: is-public,
          frequency: (get default-frequency report-type-info),
          uri: (generate-report-uri report-id report-type)
        }
      )
      
      ;; Store report data
      (map add-report-metric metrics-list report-id)
      
      ;; Update report counter
      (var-set report-counter report-id)
      
      ;; Update last report generation
      (var-set last-report-generation block-height)
      
      (ok report-id)
    )
  ))

;; Automated report generation (called by a scheduler)
(define-public (generate-scheduled-reports)
  (begin
    ;; Check report generator authorization
    (asserts! (is-report-generator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Check if reporting is enabled
    (asserts! (var-get reporting-enabled) (err ERR_REPORT_DISABLED))
    
    ;; Check if it's time to generate reports
    (asserts! (>= (- block-height (var-get last-report-generation)) (var-get reporting-interval)) (err ERR_INVALID_PARAMETER))
    
    ;; Generate each enabled report type
    (let (
      (treasury-result (generate-report-if-enabled REPORT_TYPE_TREASURY true (list)))
      (tokenomics-result (generate-report-if-enabled REPORT_TYPE_TOKENOMICS true (list)))
      (governance-result (generate-report-if-enabled REPORT_TYPE_GOVERNANCE true (list)))
      (operations-result (generate-report-if-enabled REPORT_TYPE_OPERATIONS true (list)))
    )
      (ok (var-get report-counter))
    )
  ))

;; Helper function to generate a report if its type is enabled
(define-private (generate-report-if-enabled (report-type uint) (is-public bool) (metrics (list 10 (string-ascii 64))))
  (match (map-get? report-types {report-type: report-type})
    type-info (if (get is-enabled type-info)
                 (generate-report report-type is-public metrics)
                 (ok u0))
    (ok u0)
  )
)

;; Subscribe to a report
(define-public (subscribe-to-report (report-type uint))
  (begin
    ;; Check if report type exists and is enabled
    (match (map-get? report-types {report-type: report-type})
      type-info (if (get is-enabled type-info)
                    (ok true)
                    (err ERR_REPORT_DISABLED))
      (err ERR_INVALID_PARAMETER)
    )
  ))

;; Query report by ID
(define-public (get-report-details (report-id uint))
  (match (map-get? reports report-id)
    report (if (or (get is-public report) (is-report-generator tx-sender))
                (ok report)
                (err ERR_UNAUTHORIZED))
    (err ERR_REPORT_NOT_FOUND)
  ))

;; Get report metrics
(define-public (get-report-metrics (report-id uint))
  (match (map-get? reports report-id)
    report (if (or (get is-public report) (is-report-generator tx-sender))
                (ok (get-metrics-for-report report-id (get metrics-included report)))
                (err ERR_UNAUTHORIZED))
    (err ERR_REPORT_NOT_FOUND)
  ))

;; Helper Functions

;; Collect metrics for a report
(define-private (collect-metrics-for-report (report-type uint) (metrics-to-include (list 10 (string-ascii 64))))
  (match report-type
    REPORT_TYPE_TREASURY (collect-treasury-metrics metrics-to-include)
    REPORT_TYPE_TOKENOMICS (collect-tokenomics-metrics metrics-to-include)
    REPORT_TYPE_GOVERNANCE (collect-governance-metrics metrics-to-include)
    REPORT_TYPE_OPERATIONS (collect-operations-metrics metrics-to-include)
    "unknown" ;; Default value if report type is invalid
  ))

;; Collect treasury metrics
(define-private (collect-treasury-metrics (metrics-to-include (list 10 (string-ascii 64))))
  (concat 
    (concat 
      (unwrap-panic (contract-call? METRICS_CONTRACT get-treasury-metric "reserves_ratio"))
      (unwrap-panic (contract-call? METRICS_CONTRACT get-treasury-metric "pol_percentage"))
    )
    (concat 
      (unwrap-panic (contract-call? METRICS_CONTRACT get-treasury-metric "protocol_revenue"))
      (unwrap-panic (contract-call? METRICS_CONTRACT get-treasury-metric "runway_months"))
    )
  )
)

;; Collect tokenomics metrics
(define-private (collect-tokenomics-metrics (metrics-to-include (list 10 (string-ascii 64))))
  (concat 
    (concat 
      (unwrap-panic (contract-call? METRICS_CONTRACT get-token-metric "circulating_supply"))
      (unwrap-panic (contract-call? METRICS_CONTRACT get-token-metric "velocity"))
    )
    (concat 
      (unwrap-panic (contract-call? METRICS_CONTRACT get-token-metric "active_wallets"))
      (unwrap-panic (contract-call? METRICS_CONTRACT get-token-metric "holder_distribution"))
    )
  )
)

;; Collect governance metrics
(define-private (collect-governance-metrics (metrics-to-include (list 10 (string-ascii 64))))
  (concat 
    (concat 
      (unwrap-panic (contract-call? METRICS_CONTRACT get-governance-metric "proposal_volume"))
      (unwrap-panic (contract-call? METRICS_CONTRACT get-governance-metric "voting_participation"))
    )
    (concat 
      (unwrap-panic (contract-call? METRICS_CONTRACT get-governance-metric "execution_success_rate"))
      (unwrap-panic (contract-call? METRICS_CONTRACT get-governance-metric "voter_retention"))
    )
  )
)

;; Collect operations metrics
(define-private (collect-operations-metrics (metrics-to-include (list 10 (string-ascii 64))))
  "operations_metrics" ;; Placeholder until actual metrics are available
)

;; Add a metric to a report
(define-private (add-report-metric (metric-name (string-ascii 64)) (report-id uint))
  (let (
    (metric-value (get-metric-value metric-name))
  )
    (map-set report-data
      { report-id: report-id, metric-name: metric-name }
      {
        value: metric-value,
        timestamp: block-height
      }
    )
  )
)

;; Get the value of a metric by name
(define-private (get-metric-value (metric-name (string-ascii 64)))
  (let (
    (token-metric (contract-call? METRICS_CONTRACT get-token-metric metric-name))
    (treasury-metric (contract-call? METRICS_CONTRACT get-treasury-metric metric-name))
    (governance-metric (contract-call? METRICS_CONTRACT get-governance-metric metric-name))
    (market-metric (contract-call? METRICS_CONTRACT get-market-metric metric-name))
  )
    (if (is-ok token-metric)
        (get value (unwrap-panic token-metric))
        (if (is-ok treasury-metric)
            (get value (unwrap-panic treasury-metric))
            (if (is-ok governance-metric)
                (get value (unwrap-panic governance-metric))
                (if (is-ok market-metric)
                    (get value (unwrap-panic market-metric))
                    u0) ;; Default value if metric not found
            )
        )
    )
  )
)

;; Get metrics for a report
(define-private (get-metrics-for-report (report-id uint) (metrics (list 10 (string-ascii 64))))
  (map (lambda (metric-name) 
          {
            name: metric-name, 
            data: (default-to 
                    { value: u0, timestamp: u0 } 
                    (map-get? report-data { report-id: report-id, metric-name: metric-name })
                  )
          }
       ) 
       metrics)
)

;; Generate a URI for a report
(define-private (generate-report-uri (report-id uint) (report-type uint))
  (let (
    (type-name (match report-type
                 REPORT_TYPE_TREASURY "treasury"
                 REPORT_TYPE_TOKENOMICS "tokenomics"
                 REPORT_TYPE_GOVERNANCE "governance"
                 REPORT_TYPE_OPERATIONS "operations"
                 "unknown"))
  )
    (concat 
      (concat "https://dao.anya.ai/reports/" type-name "/")
      (concat (uint-to-string report-id) ".json")
    )
  )
)

;; Convert uint to string (simple implementation)
(define-private (uint-to-string (value uint))
  (match value
    u0 "0"
    u1 "1"
    u2 "2"
    u3 "3"
    u4 "4"
    u5 "5"
    u6 "6"
    u7 "7"
    u8 "8"
    u9 "9"
    "other"
  )
)

;; Compute a hash of data (placeholder implementation)
(define-private (hash (data (string-ascii 64)))
  0x0000000000000000000000000000000000000000000000000000000000000000
)

;; Read-Only Functions

;; Get list of available report types
(define-read-only (get-available-report-types)
  (let (
    (treasury (map-get? report-types {report-type: REPORT_TYPE_TREASURY}))
    (tokenomics (map-get? report-types {report-type: REPORT_TYPE_TOKENOMICS}))
    (governance (map-get? report-types {report-type: REPORT_TYPE_GOVERNANCE}))
    (operations (map-get? report-types {report-type: REPORT_TYPE_OPERATIONS}))
  )
    (filter remove-disabled (list
      (some {
        id: REPORT_TYPE_TREASURY,
        name: (get name (default-to 
          {name: "", description: "", default-components: (list), default-frequency: u0, is-enabled: false} 
          treasury)),
        enabled: (get is-enabled (default-to 
          {name: "", description: "", default-components: (list), default-frequency: u0, is-enabled: false} 
          treasury))
      })
      (some {
        id: REPORT_TYPE_TOKENOMICS,
        name: (get name (default-to 
          {name: "", description: "", default-components: (list), default-frequency: u0, is-enabled: false} 
          tokenomics)),
        enabled: (get is-enabled (default-to 
          {name: "", description: "", default-components: (list), default-frequency: u0, is-enabled: false} 
          tokenomics))
      })
      (some {
        id: REPORT_TYPE_GOVERNANCE,
        name: (get name (default-to 
          {name: "", description: "", default-components: (list), default-frequency: u0, is-enabled: false} 
          governance)),
        enabled: (get is-enabled (default-to 
          {name: "", description: "", default-components: (list), default-frequency: u0, is-enabled: false} 
          governance))
      })
      (some {
        id: REPORT_TYPE_OPERATIONS,
        name: (get name (default-to 
          {name: "", description: "", default-components: (list), default-frequency: u0, is-enabled: false} 
          operations)),
        enabled: (get is-enabled (default-to 
          {name: "", description: "", default-components: (list), default-frequency: u0, is-enabled: false} 
          operations))
      })
    ))
  )
)

;; Filter function to remove disabled report types
(define-private (remove-disabled (report (optional {id: uint, name: (string-ascii 64), enabled: bool})))
  (match report
    value (get enabled value)
    false
  )
)

;; Get report type details
(define-read-only (get-report-type-details (report-type uint))
  (map-get? report-types {report-type: report-type}))

;; Get recent reports
(define-read-only (get-recent-reports (limit uint))
  (let (
    (current-id (var-get report-counter))
    (start-id (if (> current-id limit) (- current-id limit) u1))
  )
    (get-reports-in-range start-id current-id)
  )
)

;; Get reports in a specific range
(define-read-only (get-reports-in-range (start-id uint) (end-id uint))
  (map get-public-report-or-none (sequence start-id end-id))
)

;; Helper function for report retrieval
(define-private (get-public-report-or-none (id uint))
  (match (map-get? reports id)
    report (if (get is-public report)
               (some report)
               none)
    none
  )
)

;; Create a sequence of integers
(define-private (sequence (start uint) (end uint))
  (if (<= start end)
      (append (list start) (sequence (+ start u1) end))
      (list)
  )
)

;; Get reporting system status
(define-read-only (get-reporting-status)
  {
    enabled: (var-get reporting-enabled),
    interval: (var-get reporting-interval),
    data-retention: (var-get data-retention-blocks),
    last-generation: (var-get last-report-generation),
    privacy-level: (var-get privacy-level),
    report-count: (var-get report-counter),
    blocks-since-generation: (- block-height (var-get last-report-generation))
  })

;; Check if account is a report generator
(define-read-only (is-report-generator (account principal))
  (default-to false (map-get? report-generators account)))

;; Check if account is an administrator
(define-read-only (is-administrator (account principal))
  (default-to false (map-get? administrators account)))

;; Administrative Functions

;; Add a report type
(define-public (add-report-type (report-type uint) 
                              (name (string-ascii 64))
                              (description (string-utf8 256))
                              (components (list 10 (string-ascii 64)))
                              (frequency uint))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    (map-set report-types
      {report-type: report-type}
      {
        name: name,
        description: description,
        default-components: components,
        default-frequency: frequency,
        is-enabled: true
      }
    )
    
    (ok true)
  ))

;; Toggle report type enabled status
(define-public (toggle-report-type (report-type uint) (enabled bool))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    (match (map-get? report-types {report-type: report-type})
      type-info (begin
        (map-set report-types
          {report-type: report-type}
          (merge type-info {is-enabled: enabled})
        )
        (ok true)
      )
      (err ERR_INVALID_PARAMETER)
    )
  ))

;; Add a report generator
(define-public (add-report-generator (generator principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set report-generators generator true)
    (ok true)
  ))

;; Remove a report generator
(define-public (remove-report-generator (generator principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set report-generators generator false)
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

;; Update reporting settings
(define-public (update-reporting-settings 
               (new-enabled (optional bool))
               (new-interval (optional uint))
               (new-retention (optional uint))
               (new-privacy-level (optional (string-ascii 20))))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Update enabled status if provided
    (match new-enabled
      status (var-set reporting-enabled status)
      true)
    
    ;; Update interval if provided
    (match new-interval
      interval (var-set reporting-interval interval)
      true)
    
    ;; Update retention period if provided
    (match new-retention
      retention (var-set data-retention-blocks retention)
      true)
    
    ;; Update privacy level if provided
    (match new-privacy-level
      level (var-set privacy-level level)
      true)
    
    (ok true)
  )) 