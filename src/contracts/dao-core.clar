(impl-trait .bip-compliance.bip-341-342-trait)
(impl-trait .bip-compliance.bip-174-trait)

;; Add Taproot verification to DAO proposals
(define-public (create-proposal (title (string-ascii 256)) (content (buff 1024)) (sig (buff 64)) (pubkey (buff 32)))
  (let (
    (caller (contract-caller))
    (verified (unwrap! (contract-call? .bip-compliance verify-taproot-signature content sig pubkey) (err u401)))
  )
  ;; Rest of proposal creation logic
  (ok true)
  )
)

;; Add explicit BIP-342 (Tapscript) implementation
(define-public (execute-tapscript (script (buff 256)) (params (list 10 (buff 256))) (signature (buff 64)) (pubkey (buff 32)))
  (let (
    (verified (unwrap! (contract-call? .bip-compliance verify-tapscript script params signature pubkey) (err u403)))
    (caller (contract-caller))
  )
  ;; Execute verified script
  (match (contract-call? .bip-compliance execute-verified-script script params)
    (ok result) (ok result)
    (err code) (err code)
  )
  )
)

;; Enhance PSBT handling with version checks
(define-public (process-treasury-psbt (psbt (buff 1024)))
  (let (
    (processed (unwrap! (contract-call? .bip-compliance process-psbt psbt) (err u402)))
    (version (unwrap! (contract-call? .bip-compliance get-psbt-version processed) (err u404)))
  )
  (asserts! (<= version u2) (err u405)) ;; Enforce PSBT v2 max
  (ok processed)
  )
)

;; Add Taproot verification to critical functions
(define-public (execute-proposal (action (buff 256)) (sig (buff 64)) (pubkey (buff 32)))
  (let (
    (caller (contract-caller))
    (verified (unwrap! (contract-call? .bip-compliance verify-taproot-signature (sha256 action) sig pubkey) (err u601)))
  )
  ;; Proposal execution logic
  (ok true)
  )
)

;; Implement PSBT v2 compliance
(define-public (process-treasury-transaction (psbt (buff 1024)))
  (let (
    (processed (unwrap! (contract-call? .bip-compliance validate-psbt-v2 psbt) (err u602)))
  )
  (ok processed)
  )
)

;; Add enhanced indexing and search capabilities
(define-map proposals-index
    {creator: principal, category: (string-ascii 50)}
    uint
)

(define-read-only (search-proposals
    (creator: (optional principal))
    (category: (optional (string-ascii 50)))
    (keywords: (list 10 (string-ascii 100)))
))
    (let (
        (filtered-proposals 
            (filter 
                (map (lambda ((prop-id uint)) 
                    (let ((prop (unwrap! (map-get? proposals prop-id) (err u404))))
                        (if (and
                                (or (is-none creator) (eq (get creator prop) (unwrap! creator (err u400))))
                                (or (is-none category) (eq (get category prop) (unwrap! category (err u401))))
                                (any (lambda ((kw (string-ascii 100))) 
                                    (contains? kw (get description prop))
                                ) keywords)
                            )
                            (some prop-id)
                            none
                        )
                    )
                ) (map-keys proposals))
            )
        )
    )
    (ok (map (lambda ((id uint)) (map-get? proposals id)) filtered-proposals))
)

;; Update proposal submission to maintain index
(define-public (submit-proposal
    (title (string-ascii 150))
    (description (string-ascii 1000))
    (category (string-ascii 50))
))
    (let (
        (prop-id (+ (var-get proposal-counter) u1))
        (new-proposal 
            { 
                creator: tx-sender, 
                title: title, 
                description: description,
                category: category,
                status: "pending",
                votes-for: u0,
                votes-against: u0
            }
        )
    )
    (map-set proposals prop-id new-proposal)
    (map-set proposals-index {creator: tx-sender, category: category} prop-id)
    (var-set proposal-counter prop-id)
    (ok prop-id)
) 