;; Reporting System Contract - Decentralized Version
;; [AIR-3][AIS-3][AIM-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)
(use-trait multi-sig-trait .governance-traits.multi-sig-trait)
(use-trait governance-trait .governance-traits.governance-trait)

;; Import shared constants
(use-contract dao-constants .shared.dao-constants)

;; Contract references
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant GOVERNANCE_CONTRACT .multi-sig-governance)
(define-constant TREASURY_CONTRACT .decentralized-treasury-management)
(define-constant METRICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.metrics-oracle)
(define-constant FINANCIAL_AGENT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.financial-agent)

;; Data vars
(define-data-var reporting-enabled bool true)
(define-data-var reporting-interval uint u1000) ;; Generate reports every 1000 blocks
(define-data-var data-retention-blocks uint u1000000) ;; Keep reports for 1,000,000 blocks
(define-data-var last-report-generation uint u0)
(define-data-var privacy-level (string-ascii 20) "aggregated")

;; Report generators map
(define-map report-generators principal bool)

;; Initialize report generators (governance contract as default)
(map-set report-generators GOVERNANCE_CONTRACT true)

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
  })

;; Report type definitions
(define-map report-types
  { report-type: uint }
  {
    name: (string-ascii 64),
    description: (string-utf8 256),
    default-components: (list 10 (string-ascii 64)),
    default-frequency: uint,
    is-enabled: bool
  })

;; Report data (metrics snapshots)
(define-map report-data
  { report-id: uint, metric-name: (string-ascii 64) }
  {
    value: uint,
    timestamp: uint
  })

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
      "runway_analysis"),
    default-frequency: FREQUENCY_WEEKLY,
    is-enabled: true
  })

(map-set report-types
  { report-type: REPORT_TYPE_TOKENOMICS }
  {
    name: "Tokenomics Health Report",
    description: "Analysis of token distribution, velocity, and market behavior",
    default-components: (list
      "distribution_analysis"
      "velocity_metrics"
      "emission_efficiency"
      "holder_analysis"),
    default-frequency: FREQUENCY_MONTHLY,
    is-enabled: true
  })

(map-set report-types
  { report-type: REPORT_TYPE_GOVERNANCE }
  {
    name: "Governance Activity Report",
    description: "Summary of governance proposals, voting patterns, and participation",
    default-components: (list
      "proposal_analytics"
      "voting_patterns"
      "execution_results"
      "participation_trends"),
    default-frequency: FREQUENCY_BIWEEKLY,
    is-enabled: true
  })

(map-set report-types
  { report-type: REPORT_TYPE_OPERATIONS }
  {
    name: "Financial Operations Report",
    description: "Detailed record of financial agent activities and treasury operations",
    default-components: (list
      "transaction_history"
      "agent_activities"
      "financial_impact"
      "risk_assessment"),
    default-frequency: FREQUENCY_MONTHLY,
    is-enabled: true
  })

;; Private functions

;; Check if caller is the governance contract
(define-private (is-governance-contract)
  (is-eq tx-sender GOVERNANCE_CONTRACT))

;; Check if principal is a report generator
(define-private (is-report-generator (address principal))
  (default-to false (map-get? report-generators address)))

;; Collect metrics for a report
(define-private (collect-metrics-for-report (report-type uint) (metrics (list 10 (string-ascii 64))))
  ;; Implementation would collect metrics from various sources
  ;; For this example, just return a hash
  (sha256 (concat (to-uint report-type) (sha256 metrics))))

;; Generate a URI for a report
(define-private (generate-report-uri (report-id uint) (report-type uint))
  (concat (concat "ipfs://report/" (to-uint report-id)) (concat "/" (to-uint report-type))))

;; Add a metric to a report
(define-private (add-report-metric (metric-name (string-ascii 64)) (report-id uint))
  ;; Implementation would add actual metric data
  (map-set report-data
    { report-id: report-id, metric-name: metric-name }
    {
      value: u100, ;; Example value
      timestamp: block-height
    }))

;; Public Functions

;; Add a report generator (only through governance)
(define-public (add-report-generator (address principal))
  (begin
    (asserts! (is-governance-contract) (get-error-unauthorized dao-constants))
    (map-set report-generators address true)
    (ok true)))

;; Remove a report generator (only through governance)
(define-public (remove-report-generator (address principal))
  (begin
    (asserts! (is-governance-contract) (get-error-unauthorized dao-constants))
    (map-delete report-generators address)
    (ok true)))

;; Update reporting settings (only through governance)
(define-public (update-reporting-settings 
                (new-reporting-enabled bool) 
                (new-reporting-interval uint) 
                (new-data-retention uint)
                (new-privacy-level (string-ascii 20)))
  (begin
    (asserts! (is-governance-contract) (get-error-unauthorized dao-constants))
    (var-set reporting-enabled new-reporting-enabled)
    (var-set reporting-interval new-reporting-interval)
    (var-set data-retention-blocks new-data-retention)
    (var-set privacy-level new-privacy-level)
    (ok true)))

;; Generate a report
(define-public (generate-report 
                (report-type uint) 
                (is-public bool) 
                (metrics-to-include (list 10 (string-ascii 64))))
  (let (
    (report-type-info (unwrap! (map-get? report-types {report-type: report-type}) (get-error-invalid-parameter dao-constants)))
    (report-id (+ (var-get report-counter) u1))
  )
    ;; Check report generator authorization
    (asserts! (is-report-generator tx-sender) (get-error-unauthorized dao-constants))
    
    ;; Check if reporting is enabled
    (asserts! (var-get reporting-enabled) (get-error-invalid-parameter dao-constants))
    
    ;; Check if report type is enabled
    (asserts! (get is-enabled report-type-info) (get-error-invalid-parameter dao-constants))
    
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
        })
      
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
    (asserts! (is-report-generator tx-sender) (get-error-unauthorized dao-constants))
    
    ;; Check if reporting is enabled
    (asserts! (var-get reporting-enabled) (get-error-invalid-parameter dao-constants))
    
    ;; Check if it's time to generate reports
    (asserts! (>= (- block-height (var-get last-report-generation)) (var-get reporting-interval)) (get-error-invalid-parameter dao-constants))
    
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
    report-type-info (if (get is-enabled report-type-info)
                      (generate-report report-type is-public metrics)
                      (ok u0))
    (ok u0)))

;; Update report type (only through governance)
(define-public (update-report-type 
                (report-type uint) 
                (name (string-ascii 64)) 
                (description (string-utf8 256))
                (components (list 10 (string-ascii 64)))
                (frequency uint)
                (enabled bool))
  (begin
    (asserts! (is-governance-contract) (get-error-unauthorized dao-constants))
    (map-set report-types
      { report-type: report-type }
      {
        name: name,
        description: description,
        default-components: components,
        default-frequency: frequency,
        is-enabled: enabled
      })
    (ok true)))

;; Read-only functions

;; Get report details
(define-read-only (get-report (report-id uint))
  (map-get? reports report-id))

;; Get report data for a specific metric
(define-read-only (get-report-metric (report-id uint) (metric-name (string-ascii 64)))
  (map-get? report-data { report-id: report-id, metric-name: metric-name }))

;; Get report type details
(define-read-only (get-report-type (report-type uint))
  (map-get? report-types { report-type: report-type }))

;; Check if reporting is enabled
(define-read-only (is-reporting-enabled)
  (var-get reporting-enabled))

;; Get reporting interval
(define-read-only (get-reporting-interval)
  (var-get reporting-interval))

;; Check if a principal is authorized as a report generator
(define-read-only (check-report-generator (address principal))
  (is-report-generator address))

;; Get the last report generation height
(define-read-only (get-last-report-generation)
  (var-get last-report-generation))

;; Get the current privacy level
(define-read-only (get-privacy-level)
  (var-get privacy-level))
