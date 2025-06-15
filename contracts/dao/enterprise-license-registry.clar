;; [AIR-3][AIS-3][BPC-3][DAO-3]
;; Enterprise License Registry Contract
;; Manages enterprise license issuance and validation
;; Created: 2025-06-15

(impl-trait .dao-agent-trait.dao-agent-trait)

;; Constants
(define-constant CONTRACT_OWNER tx-sender)
(define-constant ERR_UNAUTHORIZED (err u401))
(define-constant ERR_INVALID_PARAMS (err u400))
(define-constant ERR_NOT_FOUND (err u404))
(define-constant ERR_ALREADY_EXISTS (err u409))
(define-constant ERR_EXPIRED (err u410))
(define-constant ERR_LEGAL_COMPLIANCE (err u415))

;; Enterprise license tiers
(define-constant TIER_STANDARD u1)
(define-constant TIER_PROFESSIONAL u2)
(define-constant TIER_ENTERPRISE u3)
(define-constant TIER_CUSTOM u4)

;; Data Variables
(define-data-var contract-active bool true)
(define-data-var last-operation-id uint u0)
(define-data-var agent-version (string-ascii 10) "1.0.0")

;; Enterprise license features
(define-map enterprise-license-features
  { tier: uint }
  {
    max-users: uint,
    max-api-calls: uint,
    support-level: (string-ascii 20),
    sla-response-time: uint,  ;; in minutes
    custom-modules: (list 10 (string-ascii 32)),
    white-label: bool,
    analytics-access: bool
  })

;; License registry 
(define-map enterprise-licenses
  { license-id: (string-ascii 64) }
  {
    customer: principal,
    company-name: (string-ascii 100),
    tier: uint,
    issue-date: uint,
    expiration-date: uint,
    renewal-type: (string-ascii 20),
    payment-status: (string-ascii 20),
    payment-history: (list 10 {amount: uint, date: uint}),
    legal-entity-id: (string-ascii 64),
    jurisdiction: (string-ascii 32),
    contract-hash: (buff 32)
  })

;; License access logs
(define-map license-access-logs
  { license-id: (string-ascii 64), timestamp: uint }
  {
    accessor: principal,
    operation: (string-ascii 32),
    result: (string-ascii 16)
  })

;; Private Functions
(define-private (is-authorized (caller principal))
  (or 
    (is-eq caller CONTRACT_OWNER)
    (contract-call? .dao-governance is-dao-member caller)))

(define-private (create-license-approval-id (license-id (string-ascii 64)) (customer principal))
  (concat (concat "ent-license-" license-id) (to-ascii customer)))

(define-private (is-valid-tier (tier uint))
  (and (>= tier u1) (<= tier u4)))

(define-private (log-access (license-id (string-ascii 64)) (operation (string-ascii 32)) (result (string-ascii 16)))
  (map-set license-access-logs
    {license-id: license-id, timestamp: block-height}
    {
      accessor: tx-sender,
      operation: operation,
      result: result
    }))

;; Public Functions
;; Issue enterprise license (requires multi-sig and DAO approval)
(define-public (issue-enterprise-license
    (license-id (string-ascii 64))
    (customer principal)
    (company-name (string-ascii 100))
    (tier uint)
    (duration uint)
    (renewal-type (string-ascii 20))
    (legal-entity-id (string-ascii 64))
    (jurisdiction (string-ascii 32))
    (contract-hash (buff 32)))
  (begin
    (asserts! (is-authorized tx-sender) ERR_UNAUTHORIZED)
    (asserts! (is-valid-tier tier) ERR_INVALID_PARAMS)
    
    ;; Verify multi-sig approval
    (asserts!
      (is-ok (contract-call? .multisig-agent verify-approval
        (create-license-approval-id license-id customer)))
      (err u401))
    
    ;; Verify legal compliance
    (asserts!
      (is-ok (contract-call? .legal-agent verify-jurisdiction-compliance
        jurisdiction legal-entity-id))
      ERR_LEGAL_COMPLIANCE)
    
    ;; Register license
    (map-set enterprise-licenses
      { license-id: license-id }
      {
        customer: customer,
        company-name: company-name,
        tier: tier,
        issue-date: block-height,
        expiration-date: (+ block-height duration),
        renewal-type: renewal-type,
        payment-status: "paid",
        payment-history: (list {amount: u0, date: block-height}),
        legal-entity-id: legal-entity-id,
        jurisdiction: jurisdiction,
        contract-hash: contract-hash
      })
    
    ;; Set default features for this tier
    (try! (set-license-tier-features tier))
    
    ;; Log access
    (log-access license-id "issue" "success")
    
    ;; Emit event
    (print {
      event: "enterprise-license-issued",
      license-id: license-id,
      customer: customer,
      company: company-name,
      tier: tier
    })
    
    (ok true)))

;; Set license tier features
(define-public (set-license-tier-features (tier uint))
  (begin
    (asserts! (is-authorized tx-sender) ERR_UNAUTHORIZED)
    (asserts! (is-valid-tier tier) ERR_INVALID_PARAMS)
    
    (map-set enterprise-license-features
      { tier: tier }
      {
        max-users: (match tier
                     TIER_STANDARD u10
                     TIER_PROFESSIONAL u50
                     TIER_ENTERPRISE u250
                     TIER_CUSTOM u1000),
        max-api-calls: (match tier
                        TIER_STANDARD u10000
                        TIER_PROFESSIONAL u100000
                        TIER_ENTERPRISE u1000000
                        TIER_CUSTOM u10000000),
        support-level: (match tier
                        TIER_STANDARD "basic"
                        TIER_PROFESSIONAL "standard"
                        TIER_ENTERPRISE "premium"
                        TIER_CUSTOM "white-glove"),
        sla-response-time: (match tier
                           TIER_STANDARD u1440  ;; 24 hours
                           TIER_PROFESSIONAL u480  ;; 8 hours
                           TIER_ENTERPRISE u120  ;; 2 hours
                           TIER_CUSTOM u30),  ;; 30 minutes
        custom-modules: (match tier
                        TIER_STANDARD (list )
                        TIER_PROFESSIONAL (list "analytics")
                        TIER_ENTERPRISE (list "analytics" "advanced-reports")
                        TIER_CUSTOM (list "analytics" "advanced-reports" "custom-integrations" "private-deployment")),
        white-label: (match tier
                      TIER_STANDARD false
                      TIER_PROFESSIONAL false
                      TIER_ENTERPRISE true
                      TIER_CUSTOM true),
        analytics-access: (match tier
                          TIER_STANDARD false
                          TIER_PROFESSIONAL true
                          TIER_ENTERPRISE true
                          TIER_CUSTOM true)
      })
    
    (ok tier)))

;; Validate enterprise license
(define-public (validate-enterprise-license
    (license-id (string-ascii 64)))
  (let ((license (map-get? enterprise-licenses {license-id: license-id})))
    (if (is-some license)
      (let ((license-data (unwrap-panic license)))
        (if (> (get expiration-date license-data) block-height)
          (begin
            ;; Log access
            (log-access license-id "validate" "valid")
            (ok true))
          (begin
            ;; Log access
            (log-access license-id "validate" "expired")
            (err ERR_EXPIRED))))
      (begin
        ;; Log access
        (log-access license-id "validate" "not-found")
        (err ERR_NOT_FOUND)))))

;; Get license details
(define-public (get-license-details
    (license-id (string-ascii 64)))
  (begin
    ;; Log access
    (log-access license-id "get-details" "queried")
    (ok (map-get? enterprise-licenses {license-id: license-id}))))

;; Get license tier features
(define-public (get-license-tier-features
    (tier uint))
  (begin
    (asserts! (is-valid-tier tier) ERR_INVALID_PARAMS)
    (ok (map-get? enterprise-license-features {tier: tier}))))

;; Update payment status and add to history
(define-public (record-license-payment
    (license-id (string-ascii 64))
    (payment-amount uint))
  (let ((license (map-get? enterprise-licenses {license-id: license-id})))
    (asserts! (is-some license) ERR_NOT_FOUND)
    (asserts! (is-authorized tx-sender) ERR_UNAUTHORIZED)
    
    (let ((license-data (unwrap-panic license)))
      ;; Update payment history
      (let ((current-history (get payment-history license-data))
            (new-payment {amount: payment-amount, date: block-height})
            (new-history (unwrap-panic (as-max-len? (append current-history new-payment) u10))))
        
        ;; Update license data
        (map-set enterprise-licenses
          {license-id: license-id}
          (merge license-data
            {
              payment-status: "paid",
              payment-history: new-history
            })
        )
        
        ;; Log access
        (log-access license-id "payment" "recorded")
        
        (ok true)))))

;; Renew enterprise license
(define-public (renew-enterprise-license
    (license-id (string-ascii 64))
    (duration uint))
  (let ((license (map-get? enterprise-licenses {license-id: license-id})))
    (asserts! (is-some license) ERR_NOT_FOUND)
    (asserts! (is-authorized tx-sender) ERR_UNAUTHORIZED)
    
    (let ((license-data (unwrap-panic license)))
      ;; Update license data
      (map-set enterprise-licenses
        {license-id: license-id}
        (merge license-data
          {
            expiration-date: (+ block-height duration),
            payment-status: "renewal-paid"
          })
      )
      
      ;; Log access
      (log-access license-id "renew" "success")
      
      (ok true)))))
