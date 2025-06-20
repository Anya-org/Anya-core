;; [AIR-3][AIS-3][BPC-3][DAO-3]
;; License Manager Contract
;; Manages different license types and their validation
;; Created: 2025-06-15

(impl-trait .dao-agent-trait.dao-agent-trait)

;; Constants
(define-constant CONTRACT_OWNER tx-sender)
(define-constant ERR_UNAUTHORIZED (err u401))
(define-constant ERR_INVALID_PARAMS (err u400))
(define-constant ERR_NOT_FOUND (err u404))
(define-constant ERR_ALREADY_EXISTS (err u409))
(define-constant ERR_EXPIRED (err u410))

;; License types
(define-constant LICENSE_TYPE_OPEN u1)
(define-constant LICENSE_TYPE_COMMUNITY u2)
(define-constant LICENSE_TYPE_ENTERPRISE u3)
(define-constant LICENSE_TYPE_ENTERPRISE_PLUS u4)

;; Data Variables
(define-data-var contract-active bool true)
(define-data-var last-operation-id uint u0)
(define-data-var agent-version (string-ascii 10) "1.0.0")

;; License data structure
(define-map licenses
  { license-id: (string-ascii 64) }
  {
    owner: principal,
    license-type: uint,
    issue-date: uint,
    expiration-date: uint,
    features: (list 10 (string-ascii 32)),
    status: (string-ascii 16),
    signature: (string-ascii 128)
  })

;; License type configuration
(define-map license-type-config
  { license-type: uint }
  {
    name: (string-ascii 32),
    base-duration: uint,
    features: (list 10 (string-ascii 32)),
    base-price: uint,
    max-users: uint,
    renewable: bool,
    dao-revenue-share: uint  ;; In basis points (10000 = 100%)
  })

;; License validation logs
(define-map license-validations
  { license-id: (string-ascii 64), timestamp: uint }
  {
    validator: principal,
    is-valid: bool,
    feature-requested: (optional (string-ascii 32)),
    result: (string-ascii 16)
  })

;; Private Functions
(define-private (is-authorized (caller principal))
  (or 
    (is-eq caller CONTRACT_OWNER)
    (contract-call? .dao-governance is-dao-member caller)))

(define-private (is-valid-license-type (license-type uint))
  (is-some (map-get? license-type-config {license-type: license-type})))

(define-private (generate-license-signature (license-id (string-ascii 64)) (licensee principal))
  (concat (concat license-id "-") (to-ascii licensee)))

(define-private (create-license-proposal-id (license-id (string-ascii 64)) (licensee principal))
  (concat (concat "license-" license-id) (to-ascii licensee)))

(define-private (get-dao-share (license-type uint))
  (default-to u6500  ;; Default 65%
    (get dao-revenue-share
      (default-to 
        {name: "", base-duration: u0, features: (list ), base-price: u0, max-users: u0, renewable: false, dao-revenue-share: u6500}
        (map-get? license-type-config {license-type: license-type})))))

(define-private (get-dev-share (license-type uint))
  (default-to u2000  ;; Default 20%
    (if (is-eq license-type LICENSE_TYPE_ENTERPRISE_PLUS) u2500 u2000)))

(define-private (get-protocol-share (license-type uint))
  (default-to u1500  ;; Default 15%
    (if (is-eq license-type LICENSE_TYPE_ENTERPRISE_PLUS) u1000 u1500)))

(define-private (mul-down (a uint) (b uint))
  (/ (* a b) u10000))

;; Public Functions
(define-public (validate-license 
    (license-id (string-ascii 64)) 
    (feature (optional (string-ascii 32))))
  (let ((license-data (map-get? licenses {license-id: license-id})))
    (if (is-some license-data)
      (let ((license (unwrap-panic license-data)))
        (if (and 
              (is-eq (get status license) "active")
              (> (get expiration-date license) block-height)
              (or 
                (is-none feature) 
                (default-to true 
                  (some (includes? 
                    (default-to "" (get feature))
                    (get features license))))))
            (begin
              ;; Log validation
              (map-set license-validations
                {license-id: license-id, timestamp: block-height}
                {
                  validator: tx-sender,
                  is-valid: true,
                  feature-requested: feature,
                  result: "valid"
                })
              (ok true))
            (begin
              ;; Log failed validation
              (map-set license-validations
                {license-id: license-id, timestamp: block-height}
                {
                  validator: tx-sender,
                  is-valid: false,
                  feature-requested: feature,
                  result: (if (> (get expiration-date license) block-height)
                           "expired"
                           "feature-denied")
                })
              (err (if (> (get expiration-date license) block-height)
                     ERR_EXPIRED
                     ERR_UNAUTHORIZED)))))
      (begin
        ;; Log unknown license
        (map-set license-validations
          {license-id: license-id, timestamp: block-height}
          {
            validator: tx-sender,
            is-valid: false,
            feature-requested: feature,
            result: "not-found"
          })
        (err ERR_NOT_FOUND))))

;; License issuance function - requires DAO approval
(define-public (issue-license 
    (license-id (string-ascii 64))
    (licensee principal)
    (license-type uint)
    (duration uint)
    (features (list 10 (string-ascii 32))))
  (begin
    (asserts! (is-authorized tx-sender) ERR_UNAUTHORIZED)
    (asserts! (is-valid-license-type license-type) ERR_INVALID_PARAMS)
    
    ;; Check if license proposal was approved
    (asserts! 
      (contract-call? .dao-governance is-proposal-approved
        (create-license-proposal-id license-id licensee))
      (err u401))
    
    ;; Set license data
    (map-set licenses
      { license-id: license-id }
      {
        owner: licensee,
        license-type: license-type,
        issue-date: block-height,
        expiration-date: (+ block-height duration),
        features: features,
        status: "active",
        signature: (generate-license-signature license-id licensee)
      })
    (ok true)))

;; Set license configuration
(define-public (set-license-type-config
    (license-type uint)
    (name (string-ascii 32))
    (base-duration uint)
    (features (list 10 (string-ascii 32)))
    (base-price uint)
    (max-users uint)
    (renewable bool)
    (dao-revenue-share uint))
  (begin
    (asserts! (is-authorized tx-sender) ERR_UNAUTHORIZED)
    (asserts! (and (>= license-type u1) (<= license-type u4)) ERR_INVALID_PARAMS)
    (asserts! (<= dao-revenue-share u10000) ERR_INVALID_PARAMS)
    
    (map-set license-type-config
      {license-type: license-type}
      {
        name: name,
        base-duration: base-duration,
        features: features,
        base-price: base-price,
        max-users: max-users,
        renewable: renewable,
        dao-revenue-share: dao-revenue-share
      })
    (ok true)))

;; Get license data (public read)
(define-public (get-license (license-id (string-ascii 64)))
  (ok (map-get? licenses {license-id: license-id})))

;; Get license type config (public read)
(define-public (get-license-type-config (license-type uint))
  (ok (map-get? license-type-config {license-type: license-type})))

;; Revenue distribution function
(define-public (distribute-license-revenue (amount uint) (license-type uint))
  (let (
    (dao-share (get-dao-share license-type))
    (dev-share (get-dev-share license-type))
    (protocol-share (get-protocol-share license-type))
  )
    (asserts! (is-authorized tx-sender) ERR_UNAUTHORIZED)
    
    ;; Transfer shares to appropriate wallets/contracts
    (try! (contract-call? .token transfer
      (mul-down amount dao-share) 
      tx-sender 
      .dao-treasury))
      
    (try! (contract-call? .token transfer
      (mul-down amount dev-share) 
      tx-sender 
      .dev-fund))
      
    (try! (contract-call? .token transfer
      (mul-down amount protocol-share) 
      tx-sender 
      .protocol-treasury))
      
    (ok true)))

;; Revoke license
(define-public (revoke-license (license-id (string-ascii 64)))
  (let ((license-data (map-get? licenses {license-id: license-id})))
    (asserts! (is-some license-data) ERR_NOT_FOUND)
    (asserts! (is-authorized tx-sender) ERR_UNAUTHORIZED)
    
    (map-set licenses
      {license-id: license-id}
      (merge (unwrap-panic license-data)
        {status: "revoked"})
    )
    (ok true)))

;; Renew license
(define-public (renew-license 
    (license-id (string-ascii 64))
    (duration uint))
  (let ((license-data (map-get? licenses {license-id: license-id})))
    (asserts! (is-some license-data) ERR_NOT_FOUND)
    (asserts! (is-authorized tx-sender) ERR_UNAUTHORIZED)
    
    (let ((license (unwrap-panic license-data)))
      ;; Check if renewable based on license type
      (asserts! 
        (default-to false
          (get renewable 
            (default-to 
              {name: "", base-duration: u0, features: (list ), base-price: u0, max-users: u0, renewable: false, dao-revenue-share: u6500}
              (map-get? license-type-config {license-type: (get license-type license)}))))
        ERR_INVALID_PARAMS)
      
      (map-set licenses
        {license-id: license-id}
        (merge license
          {
            expiration-date: (+ block-height duration),
            status: "active"
          })
      )
      (ok true))))
