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