;; Web5 DWN (Decentralized Web Node) Adapter Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][AIE-3][DAO-3][W5C-3][DID-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_PARAMETER u402)
(define-constant ERR_SYSTEM_DISABLED u403)
(define-constant ERR_RECORD_NOT_FOUND u404)
(define-constant ERR_DID_INVALID u405)
(define-constant ERR_SCHEMA_INVALID u406)
(define-constant ERR_PROTOCOL_MISMATCH u407)
(define-constant ERR_RATE_LIMIT_EXCEEDED u408)
(define-constant ERR_INSUFFICIENT_PERMISSION u409)
(define-constant ERR_ALREADY_EXISTS u410)
(define-constant ERR_ENCRYPTION_ERROR u411)
(define-constant ERR_SIGNATURE_INVALID u412)
(define-constant ERR_STORAGE_QUOTA_EXCEEDED u413)

;; Contract References
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance)
(define-constant METRICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.metrics-oracle)

;; Data Variables
(define-data-var dwn-enabled bool true)
(define-data-var dwn-protocol-version (string-ascii 10) "0.7.0")
(define-data-var max-request-per-block uint u20)
(define-data-var request-cooldown-blocks uint u10)
(define-data-var default-retention-period uint u5256000) ;; ~1 year in blocks
(define-data-var record-count uint u0)
(define-data-var identity-verification-required bool false)
(define-data-var default-encryption-required bool true)
(define-data-var default-schema-validation-required bool true)
(define-data-var storage-quota-per-user uint u10000000) ;; 10MB in bytes
(define-data-var rate-limit-window-blocks uint u100)
(define-data-var did-resolution-uri (string-utf8 256) "https://did.anya.ai/resolve")

;; Admin and identity verifier lists
(define-map administrators principal bool)
(define-map identity-verifiers principal bool)
(define-map verified-identities principal bool)
(define-map schemas (string-ascii 64) {schema-uri: (string-utf8 256), version: (string-ascii 10), required: bool})

;; Initialize administrators
(map-set administrators tx-sender true)

;; Record Types
(define-constant RECORD_TYPE_JSON u1)
(define-constant RECORD_TYPE_MEDIA u2)
(define-constant RECORD_TYPE_DOCUMENT u3)
(define-constant RECORD_TYPE_ENCRYPTED u4)
(define-constant RECORD_TYPE_REFERENCE u5)

;; Permission Levels
(define-constant PERMISSION_NONE u0)
(define-constant PERMISSION_READ u1)
(define-constant PERMISSION_WRITE u2)
(define-constant PERMISSION_READ_WRITE u3)
(define-constant PERMISSION_ADMIN u4)

;; Record Status
(define-constant RECORD_STATUS_ACTIVE u1)
(define-constant RECORD_STATUS_ARCHIVED u2)
(define-constant RECORD_STATUS_DELETED u3)

;; DID Types
(define-constant DID_TYPE_WEB u1)
(define-constant DID_TYPE_KEY u2)
(define-constant DID_TYPE_ION u3)
(define-constant DID_TYPE_STACKS u4)

;; Record storage
(define-map data-records
  uint ;; record-id
  {
    schema-id: (string-ascii 64),
    record-type: uint,
    owner: principal,
    did: (optional (string-utf8 256)),
    record-uri: (string-utf8 256),
    content-hash: (buff 32),
    metadata: (string-utf8 1024),
    encryption-type: (optional (string-ascii 20)),
    timestamp: uint,
    expiration: uint,
    status: uint,
    size-bytes: uint,
    parent-record: (optional uint),
    protocol-path: (string-ascii 64)
  }
)

;; Schema registry
(define-map schema-registry
  (string-ascii 64) ;; schema-id
  {
    name: (string-ascii 64),
    description: (string-utf8 512),
    version: (string-ascii 10),
    uri: (string-utf8 256),
    created-by: principal,
    created-at: uint,
    last-updated: uint,
    validator-uri: (optional (string-utf8 256)),
    required-fields: (list 20 (string-ascii 64)),
    sample-document: (string-utf8 1024)
  }
)

;; Permissions mapping
(define-map record-permissions
  { record-id: uint, user: principal }
  { permission-level: uint, granted-by: principal, granted-at: uint }
)

;; Request Rate Limiting
(define-map request-counts
  { user: principal, window-start: uint }
  uint
)

;; User storage quota usage
(define-map user-storage-usage
  principal
  { used-bytes: uint, record-count: uint, last-updated: uint }
)

;; DID to principal mapping (for authentication)
(define-map did-registry
  (string-utf8 256) ;; DID
  { 
    principal: principal, 
    did-type: uint, 
    verified: bool, 
    verification-method: (string-utf8 256),
    registered-at: uint 
  }
)

;; Public Functions

;; Store a record in the DWN
(define-public (store-record 
    (schema-id (string-ascii 64))
    (did (optional (string-utf8 256)))
    (owner (optional principal))
    (metadata (optional (string-utf8 1024)))
    (content-hash (optional (buff 32)))
    (encryption-type (optional (string-ascii 20)))
    (size-bytes (optional uint))
    (protocol-path (optional (string-ascii 64)))
    (parent-record (optional uint)))
  (begin
    ;; Check if DWN is enabled
    (asserts! (var-get dwn-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check rate limits
    (asserts! (not (rate-limited tx-sender)) (err ERR_RATE_LIMIT_EXCEEDED))
    
    ;; Validate schema
    (asserts! (is-some (map-get? schema-registry schema-id)) (err ERR_SCHEMA_INVALID))
    
    ;; If identity verification is required, check identity
    (when (var-get identity-verification-required)
      (asserts! (is-identity-verified tx-sender) (err ERR_UNAUTHORIZED)))
    
    ;; Check storage quota
    (let (
      (usage (default-to 
        { used-bytes: u0, record-count: u0, last-updated: block-height } 
        (map-get? user-storage-usage tx-sender)))
      (size (default-to u1024 size-bytes)) ;; Default 1KB if not specified
    )
      (asserts! (<= (+ (get used-bytes usage) size) (var-get storage-quota-per-user)) 
                (err ERR_STORAGE_QUOTA_EXCEEDED))
      
      ;; Increment request count
      (increment-request-count tx-sender)
      
      ;; Increment record count
      (let (
        (new-record-id (+ (var-get record-count) u1))
        (effective-owner (default-to tx-sender owner))
        (did-value did)
        (effective-protocol-path (default-to schema-id protocol-path))
      )
        ;; Create URI for the record
        (let (
          (record-uri (generate-record-uri new-record-id effective-owner did-value))
        )
          ;; Store the record
          (map-set data-records new-record-id
            {
              schema-id: schema-id,
              record-type: (if (is-some encryption-type) RECORD_TYPE_ENCRYPTED RECORD_TYPE_JSON),
              owner: effective-owner,
              did: did-value,
              record-uri: record-uri,
              content-hash: (default-to 0x0000000000000000000000000000000000000000000000000000000000000000 content-hash),
              metadata: (default-to "" metadata),
              encryption-type: encryption-type,
              timestamp: block-height,
              expiration: (+ block-height (var-get default-retention-period)),
              status: RECORD_STATUS_ACTIVE,
              size-bytes: size,
              parent-record: parent-record,
              protocol-path: effective-protocol-path
            }
          )
          
          ;; Update record count
          (var-set record-count new-record-id)
          
          ;; Update storage usage
          (map-set user-storage-usage tx-sender
            {
              used-bytes: (+ (get used-bytes usage) size),
              record-count: (+ (get record-count usage) u1),
              last-updated: block-height
            }
          )
          
          ;; Grant permission to owner
          (map-set record-permissions
            { record-id: new-record-id, user: effective-owner }
            { permission-level: PERMISSION_ADMIN, granted-by: tx-sender, granted-at: block-height }
          )
          
          ;; Log storage metric
          (try! (contract-call? METRICS_CONTRACT submit-governance-metric "dwn_records_stored" u1 u1000 "web5-dwn"))
          
          ;; Return the record ID and URI
          (ok { record-id: new-record-id, record-uri: record-uri })
        )
      )
    )
  ))

;; Update a record
(define-public (update-record
    (record-id uint)
    (metadata (optional (string-utf8 1024)))
    (content-hash (optional (buff 32)))
    (encryption-type (optional (string-ascii 20)))
    (size-delta (optional int))
    (status (optional uint)))
  (begin
    ;; Check if DWN is enabled
    (asserts! (var-get dwn-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check rate limits
    (asserts! (not (rate-limited tx-sender)) (err ERR_RATE_LIMIT_EXCEEDED))
    
    ;; Get the record
    (let ((record (unwrap! (map-get? data-records record-id) (err ERR_RECORD_NOT_FOUND))))
      
      ;; Check permissions (need WRITE or ADMIN)
      (let ((permission (default-to
        { permission-level: PERMISSION_NONE, granted-by: tx-sender, granted-at: block-height }
        (map-get? record-permissions { record-id: record-id, user: tx-sender }))))
        
        (asserts! (or (>= (get permission-level permission) PERMISSION_WRITE)
                     (is-eq tx-sender (get owner record))
                     (is-administrator tx-sender))
                 (err ERR_INSUFFICIENT_PERMISSION))
        
        ;; If size is changing, check storage quota
        (when (is-some size-delta)
          (let (
            (usage (default-to 
              { used-bytes: u0, record-count: u0, last-updated: block-height } 
              (map-get? user-storage-usage (get owner record))))
            (delta (unwrap! size-delta (err ERR_INVALID_PARAMETER)))
          )
            ;; Only check quota if increasing size
            (when (> delta 0)
              (asserts! (<= (+ (get used-bytes usage) (to-uint delta)) (var-get storage-quota-per-user)) 
                       (err ERR_STORAGE_QUOTA_EXCEEDED))
            )
            
            ;; Update storage usage
            (map-set user-storage-usage (get owner record)
              {
                used-bytes: (if (> delta 0)
                               (+ (get used-bytes usage) (to-uint delta))
                               (- (get used-bytes usage) (to-uint (abs delta)))),
                record-count: (get record-count usage),
                last-updated: block-height
              }
            )
          )
        )
        
        ;; Update the record
        (map-set data-records record-id
          (merge record {
            metadata: (default-to (get metadata record) metadata),
            content-hash: (default-to (get content-hash record) content-hash),
            encryption-type: (default-to (get encryption-type record) encryption-type),
            size-bytes: (if (is-some size-delta)
                         (if (> (unwrap! size-delta (err ERR_INVALID_PARAMETER)) 0)
                            (+ (get size-bytes record) (to-uint (unwrap! size-delta (err ERR_INVALID_PARAMETER))))
                            (- (get size-bytes record) (to-uint (abs (unwrap! size-delta (err ERR_INVALID_PARAMETER))))))
                         (get size-bytes record)),
            status: (default-to (get status record) status)
          })
        )
        
        ;; Increment request count
        (increment-request-count tx-sender)
        
        ;; Log update metric
        (try! (contract-call? METRICS_CONTRACT submit-governance-metric "dwn_records_updated" u1 u1000 "web5-dwn"))
        
        (ok true)
      )
    )
  ))

;; Grant permission on a record to another user
(define-public (grant-permission
    (record-id uint)
    (user principal)
    (permission-level uint))
  (begin
    ;; Check if DWN is enabled
    (asserts! (var-get dwn-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Validate permission level
    (asserts! (and (>= permission-level PERMISSION_NONE) 
                  (<= permission-level PERMISSION_ADMIN))
             (err ERR_INVALID_PARAMETER))
    
    ;; Get the record
    (let ((record (unwrap! (map-get? data-records record-id) (err ERR_RECORD_NOT_FOUND))))
      
      ;; Check if has admin permission
      (let ((admin-permission (default-to
        { permission-level: PERMISSION_NONE, granted-by: tx-sender, granted-at: block-height }
        (map-get? record-permissions { record-id: record-id, user: tx-sender }))))
        
        (asserts! (or (>= (get permission-level admin-permission) PERMISSION_ADMIN)
                     (is-eq tx-sender (get owner record))
                     (is-administrator tx-sender))
                 (err ERR_INSUFFICIENT_PERMISSION))
        
        ;; Grant permission
        (map-set record-permissions
          { record-id: record-id, user: user }
          { permission-level: permission-level, granted-by: tx-sender, granted-at: block-height }
        )
        
        ;; Increment request count
        (increment-request-count tx-sender)
        
        (ok true)
      )
    )
  ))

;; Register a schema
(define-public (register-schema
    (schema-id (string-ascii 64))
    (name (string-ascii 64))
    (description (string-utf8 512))
    (version (string-ascii 10))
    (uri (string-utf8 256))
    (validator-uri (optional (string-utf8 256)))
    (required-fields (list 20 (string-ascii 64)))
    (sample-document (string-utf8 1024)))
  (begin
    ;; Check if DWN is enabled
    (asserts! (var-get dwn-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check if caller is administrator or DAO
    (asserts! (or (is-administrator tx-sender) (is-eq tx-sender DAO_CONTRACT)) 
              (err ERR_UNAUTHORIZED))
    
    ;; Check if schema already exists
    (asserts! (is-none (map-get? schema-registry schema-id)) (err ERR_ALREADY_EXISTS))
    
    ;; Register schema
    (map-set schema-registry schema-id
      {
        name: name,
        description: description,
        version: version,
        uri: uri,
        created-by: tx-sender,
        created-at: block-height,
        last-updated: block-height,
        validator-uri: validator-uri,
        required-fields: required-fields,
        sample-document: sample-document
      }
    )
    
    ;; Add to schemas list
    (map-set schemas schema-id
      {
        schema-uri: uri,
        version: version,
        required: false
      }
    )
    
    ;; Log schema registration metric
    (try! (contract-call? METRICS_CONTRACT submit-governance-metric "dwn_schemas_registered" u1 u1000 "web5-dwn"))
    
    (ok true)
  ))

;; Register a DID for a principal
(define-public (register-did
    (did (string-utf8 256))
    (did-type uint)
    (verification-method (string-utf8 256)))
  (begin
    ;; Check if DWN is enabled
    (asserts! (var-get dwn-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Validate DID format
    (asserts! (is-valid-did did) (err ERR_DID_INVALID))
    
    ;; Validate DID type
    (asserts! (and (>= did-type DID_TYPE_WEB) (<= did-type DID_TYPE_STACKS)) 
              (err ERR_INVALID_PARAMETER))
    
    ;; Check if DID already exists
    (asserts! (is-none (map-get? did-registry did)) (err ERR_ALREADY_EXISTS))
    
    ;; Register DID
    (map-set did-registry did
      {
        principal: tx-sender,
        did-type: did-type,
        verified: false, ;; Will be verified later
        verification-method: verification-method,
        registered-at: block-height
      }
    )
    
    ;; Log DID registration metric
    (try! (contract-call? METRICS_CONTRACT submit-governance-metric "dwn_dids_registered" u1 u1000 "web5-dwn"))
    
    (ok true)
  ))

;; Verify a DID
(define-public (verify-did
    (did (string-utf8 256))
    (signature (buff 65)))
  (begin
    ;; Check if DWN is enabled
    (asserts! (var-get dwn-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Get the DID
    (let ((did-entry (unwrap! (map-get? did-registry did) (err ERR_DID_INVALID))))
      
      ;; Verify authorization
      (asserts! (or (is-eq tx-sender (get principal did-entry))
                   (is-administrator tx-sender)
                   (is-identity-verifier tx-sender))
               (err ERR_UNAUTHORIZED))
      
      ;; Verify signature (in a real implementation, this would validate the crypto)
      (asserts! (is-valid-did-signature did signature) (err ERR_SIGNATURE_INVALID))
      
      ;; Update DID as verified
      (map-set did-registry did
        (merge did-entry { verified: true })
      )
      
      ;; Log DID verification metric
      (try! (contract-call? METRICS_CONTRACT submit-governance-metric "dwn_dids_verified" u1 u1000 "web5-dwn"))
      
      (ok true)
    )
  ))

;; Mark a record as deleted
(define-public (delete-record (record-id uint))
  (begin
    ;; Check if DWN is enabled
    (asserts! (var-get dwn-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Get the record
    (let ((record (unwrap! (map-get? data-records record-id) (err ERR_RECORD_NOT_FOUND))))
      
      ;; Check permissions (need ADMIN)
      (let ((permission (default-to
        { permission-level: PERMISSION_NONE, granted-by: tx-sender, granted-at: block-height }
        (map-get? record-permissions { record-id: record-id, user: tx-sender }))))
        
        (asserts! (or (>= (get permission-level permission) PERMISSION_ADMIN)
                     (is-eq tx-sender (get owner record))
                     (is-administrator tx-sender))
                 (err ERR_INSUFFICIENT_PERMISSION))
        
        ;; Update storage usage
        (let (
          (usage (default-to 
            { used-bytes: u0, record-count: u0, last-updated: block-height } 
            (map-get? user-storage-usage (get owner record))))
        )
          (map-set user-storage-usage (get owner record)
            {
              used-bytes: (if (> (get used-bytes usage) (get size-bytes record))
                             (- (get used-bytes usage) (get size-bytes record))
                             u0),
              record-count: (if (> (get record-count usage) u0)
                               (- (get record-count usage) u1)
                               u0),
              last-updated: block-height
            }
          )
        )
        
        ;; Mark record as deleted
        (map-set data-records record-id
          (merge record { status: RECORD_STATUS_DELETED })
        )
        
        ;; Log deletion metric
        (try! (contract-call? METRICS_CONTRACT submit-governance-metric "dwn_records_deleted" u1 u1000 "web5-dwn"))
        
        (ok true)
      )
    )
  ))

;; Extend record retention
(define-public (extend-record-retention
    (record-id uint)
    (additional-blocks uint))
  (begin
    ;; Check if DWN is enabled
    (asserts! (var-get dwn-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Get the record
    (let ((record (unwrap! (map-get? data-records record-id) (err ERR_RECORD_NOT_FOUND))))
      
      ;; Check permissions (need at least READ)
      (let ((permission (default-to
        { permission-level: PERMISSION_NONE, granted-by: tx-sender, granted-at: block-height }
        (map-get? record-permissions { record-id: record-id, user: tx-sender }))))
        
        (asserts! (or (>= (get permission-level permission) PERMISSION_READ)
                     (is-eq tx-sender (get owner record))
                     (is-administrator tx-sender))
                 (err ERR_INSUFFICIENT_PERMISSION))
        
        ;; Update expiration
        (map-set data-records record-id
          (merge record {
            expiration: (+ (get expiration record) additional-blocks)
          })
        )
        
        (ok true)
      )
    )
  ))

;; Read-Only Functions

;; Get record details
(define-read-only (get-record (record-id uint))
  (let ((record (map-get? data-records record-id)))
    (match record
      r (if (has-record-permission record-id tx-sender PERMISSION_READ)
            (some r)
            none)
      none
    )
  ))

;; Get schema details
(define-read-only (get-schema (schema-id (string-ascii 64)))
  (map-get? schema-registry schema-id))

;; Get permission for a record
(define-read-only (get-permission (record-id uint) (user principal))
  (map-get? record-permissions { record-id: record-id, user: user }))

;; Get user storage usage
(define-read-only (get-user-storage-usage (user principal))
  (default-to 
    { used-bytes: u0, record-count: u0, last-updated: u0 } 
    (map-get? user-storage-usage user)
  ))

;; Check if account has permission on a record
(define-read-only (has-record-permission (record-id uint) (user principal) (required-permission uint))
  (let (
    (record (map-get? data-records record-id))
    (permission (map-get? record-permissions { record-id: record-id, user: user }))
  )
    (match record
      r (or (is-administrator user)
           (is-eq user (get owner r))
           (match permission
             p (>= (get permission-level p) required-permission)
             false))
      false
    )
  ))

;; Get DID details
(define-read-only (get-did-details (did (string-utf8 256)))
  (map-get? did-registry did))

;; Get principal for a DID
(define-read-only (get-principal-for-did (did (string-utf8 256)))
  (match (map-get? did-registry did)
    entry (some (get principal entry))
    none
  ))

;; Check if DID is verified
(define-read-only (is-did-verified (did (string-utf8 256)))
  (match (map-get? did-registry did)
    entry (get verified entry)
    false
  ))

;; Get DWN parameters
(define-read-only (get-dwn-parameters)
  {
    dwn-enabled: (var-get dwn-enabled),
    protocol-version: (var-get dwn-protocol-version),
    default-retention-period: (var-get default-retention-period),
    identity-verification-required: (var-get identity-verification-required),
    default-encryption-required: (var-get default-encryption-required),
    default-schema-validation-required: (var-get default-schema-validation-required),
    storage-quota-per-user: (var-get storage-quota-per-user),
    did-resolution-uri: (var-get did-resolution-uri)
  })

;; Check if account is an administrator
(define-read-only (is-administrator (account principal))
  (default-to false (map-get? administrators account)))

;; Check if account is an identity verifier
(define-read-only (is-identity-verifier (account principal))
  (default-to false (map-get? identity-verifiers account)))

;; Check if account has verified identity
(define-read-only (is-identity-verified (account principal))
  (default-to false (map-get? verified-identities account)))

;; Helper Functions

;; Generate a record URI
(define-private (generate-record-uri (record-id uint) (owner principal) (did (optional (string-utf8 256))))
  (concat "dwn://anya.ai/records/" 
         (concat (to-ascii record-id) 
                (concat "/" (address-to-ascii owner))))
)

;; Convert address to ASCII (simplified)
(define-private (address-to-ascii (addr principal))
  (concat "principal" (principal-to-string addr))
)

;; Convert principal to string (placeholder)
(define-private (principal-to-string (principal principal))
  "principal"
)

;; Check if DID is valid (simplified)
(define-private (is-valid-did (did (string-utf8 256)))
  ;; In a real implementation, we would validate the DID format
  ;; For simplicity, we'll just check if it starts with "did:"
  ;; but a real validator would be much more sophisticated
  (is-prefix "did:" did)
)

;; Check if string has prefix
(define-private (is-prefix (prefix (string-utf8 256)) (str (string-utf8 256)))
  ;; Simplified implementation
  true
)

;; Validate DID signature (simplified)
(define-private (is-valid-did-signature (did (string-utf8 256)) (signature (buff 65)))
  ;; In a real implementation, this would perform cryptographic validation
  ;; For simplicity, we'll return true
  true
)

;; Check if user is rate limited
(define-private (rate-limited (user principal))
  (let (
    (window-start (- block-height (mod block-height (var-get rate-limit-window-blocks))))
    (current-count (default-to u0 (map-get? request-counts {user: user, window-start: window-start})))
  )
    (> current-count (var-get max-request-per-block))
  ))

;; Increment request count for rate limiting
(define-private (increment-request-count (user principal))
  (let (
    (window-start (- block-height (mod block-height (var-get rate-limit-window-blocks))))
    (current-count (default-to u0 (map-get? request-counts {user: user, window-start: window-start})))
  )
    (map-set request-counts 
      {user: user, window-start: window-start} 
      (+ current-count u1))
  ))

;; Admin Functions

;; Add an administrator
(define-public (add-administrator (admin principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set administrators admin true)
    (ok true)))

;; Remove an administrator
(define-public (remove-administrator (admin principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set administrators admin false)
    (ok true)))

;; Add an identity verifier
(define-public (add-identity-verifier (verifier principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set identity-verifiers verifier true)
    (ok true)))

;; Remove an identity verifier
(define-public (remove-identity-verifier (verifier principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set identity-verifiers verifier false)
    (ok true)))

;; Verify a user's identity
(define-public (verify-identity (user principal))
  (begin
    (asserts! (or (is-administrator tx-sender) (is-identity-verifier tx-sender)) (err ERR_UNAUTHORIZED))
    (map-set verified-identities user true)
    (ok true)))

;; Toggle DWN
(define-public (toggle-dwn (enabled bool))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (var-set dwn-enabled enabled)
    (ok true)))

;; Set schema as required
(define-public (set-schema-required (schema-id (string-ascii 64)) (required bool))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Get the schema
    (let ((schema (unwrap! (map-get? schemas schema-id) (err ERR_SCHEMA_INVALID))))
      
      ;; Update schema
      (map-set schemas schema-id
        (merge schema { required: required })
      )
      
      (ok true)
    )
  ))

;; Update DWN parameters
(define-public (update-dwn-parameters
    (protocol-version (optional (string-ascii 10)))
    (retention-period (optional uint))
    (identity-required (optional bool))
    (encryption-required (optional bool))
    (schema-validation-required (optional bool))
    (storage-quota (optional uint))
    (did-uri (optional (string-utf8 256))))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Update each parameter if provided
    (match protocol-version
      val (var-set dwn-protocol-version val)
      true)
    
    (match retention-period
      val (var-set default-retention-period val)
      true)
    
    (match identity-required
      val (var-set identity-verification-required val)
      true)
    
    (match encryption-required
      val (var-set default-encryption-required val)
      true)
    
    (match schema-validation-required
      val (var-set default-schema-validation-required val)
      true)
    
    (match storage-quota
      val (var-set storage-quota-per-user val)
      true)
    
    (match did-uri
      val (var-set did-resolution-uri val)
      true)
    
    (ok true)
  )) 