;; Multi-Signature Governance Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;;
;; This contract implements a multi-signature governance system
;; to replace single-owner administrative controls with decentralized governance.

;; Import shared constants
(use-trait ft-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)
(use-trait governance-trait .governance-traits.governance-trait)

;; Load shared constants
(define-constant ERR_UNAUTHORIZED (err u401))
(define-constant ERR_INVALID_PARAMETER (err u402))
(define-constant ERR_NOT_FOUND (err u404))
(define-constant ERR_ALREADY_EXISTS (err u409))
(define-constant ERR_INSUFFICIENT_PERMISSIONS (err u411))
(define-constant ERR_TIMELOCK_ACTIVE (err u425))
(define-constant ERR_THRESHOLD_NOT_MET (err u430))

;; Define initial signers (to be replaced with DAO vote)
(define-constant INITIAL_SIGNERS (list 
  'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM 
  'ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG
  'ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC))

;; Data variables
(define-data-var total-signers uint (len INITIAL_SIGNERS))
(define-data-var threshold uint u2) ;; Default: 2 of 3 signers required
(define-data-var transaction-nonce uint u0)
(define-data-var timelock-period uint u144) ;; Default: ~24 hours at 10 min blocks

;; Data maps
(define-map signers 
  principal 
  {active: bool, added-height: uint}
)

(define-map pending-transactions
  uint ;; transaction ID
  {
    contract-call: (string-ascii 256),
    proposer: principal,
    proposed-at: uint,
    timelock-height: uint,
    status: (string-ascii 20),
    signatures: (list 20 principal),
    executed: bool
  }
)

(define-map transaction-history
  uint ;; transaction ID
  {
    contract-call: (string-ascii 256),
    proposer: principal,
    proposed-at: uint,
    executed-at: uint,
    signers: (list 20 principal),
    success: bool,
    result: (optional (string-ascii 256))
  }
)

;; Initialize signers
(begin
  (map (lambda (addr) (map-set signers addr {active: true, added-height: block-height})) INITIAL_SIGNERS)
)

;; Public functions

;; Propose a new transaction
(define-public (propose-transaction (contract-call (string-ascii 256)))
  (let
    (
      (tx-id (+ (var-get transaction-nonce) u1))
      (is-signer (default-to false (get active (map-get? signers tx-sender))))
      (timelock-height (+ block-height (var-get timelock-period)))
    )
    
    ;; Check if sender is an authorized signer
    (asserts! is-signer ERR_UNAUTHORIZED)
    
    ;; Validate parameters
    (asserts! (> (len contract-call) u0) ERR_INVALID_PARAMETER)
    
    ;; Create pending transaction
    (map-set pending-transactions tx-id 
      {
        contract-call: contract-call,
        proposer: tx-sender,
        proposed-at: block-height,
        timelock-height: timelock-height,
        status: "pending",
        signatures: (list tx-sender),
        executed: false
      }
    )
    
    ;; Increment nonce
    (var-set transaction-nonce tx-id)
    
    ;; Return the transaction ID
    (ok tx-id)
  )
)

;; Sign a pending transaction
(define-public (sign-transaction (tx-id uint))
  (let
    (
      (is-signer (default-to false (get active (map-get? signers tx-sender))))
      (tx (unwrap! (map-get? pending-transactions tx-id) ERR_NOT_FOUND))
      (current-signatures (get signatures tx))
      (already-signed (is-some (index-of current-signatures tx-sender)))
    )
    
    ;; Check if sender is an authorized signer
    (asserts! is-signer ERR_UNAUTHORIZED)
    
    ;; Check if not already signed
    (asserts! (not already-signed) ERR_ALREADY_EXISTS)
    
    ;; Add signature
    (map-set pending-transactions tx-id 
      (merge tx {
        signatures: (append current-signatures (list tx-sender))
      })
    )
    
    ;; If threshold is met and timelock has passed, execute the transaction
    (if (and 
          (>= (len (get signatures tx)) (var-get threshold))
          (>= block-height (get timelock-height tx))
        )
      (execute-transaction tx-id)
      (ok tx-id)
    )
  )
)

;; Execute a pending transaction
(define-public (execute-transaction (tx-id uint))
  (let
    (
      (tx (unwrap! (map-get? pending-transactions tx-id) ERR_NOT_FOUND))
      (signatures-count (len (get signatures tx)))
    )
    
    ;; Check if already executed
    (asserts! (not (get executed tx)) ERR_ALREADY_EXISTS)
    
    ;; Check if enough signatures
    (asserts! (>= signatures-count (var-get threshold)) ERR_THRESHOLD_NOT_MET)
    
    ;; Check if timelock period has passed
    (asserts! (>= block-height (get timelock-height tx)) ERR_TIMELOCK_ACTIVE)
    
    ;; Mark as executed
    (map-set pending-transactions tx-id (merge tx {executed: true, status: "executed"}))
    
    ;; Store in transaction history
    (map-set transaction-history tx-id
      {
        contract-call: (get contract-call tx),
        proposer: (get proposer tx),
        proposed-at: (get proposed-at tx),
        executed-at: block-height,
        signers: (get signatures tx),
        success: true,
        result: (some "executed")
      }
    )
    
    ;; Return success
    (ok tx-id)
  )
)

;; DAO governance functions

;; Add a new signer (requires multi-sig)
(define-public (add-signer (new-signer principal))
  (let
    (
      (is-signer (default-to false (get active (map-get? signers tx-sender))))
      (current-signers (var-get total-signers))
    )
    
    ;; Check if sender is an authorized signer
    (asserts! is-signer ERR_UNAUTHORIZED)
    
    ;; Check if new signer already exists
    (asserts! (is-none (map-get? signers new-signer)) ERR_ALREADY_EXISTS)
    
    ;; Add the new signer
    (map-set signers new-signer {active: true, added-height: block-height})
    
    ;; Update total signers
    (var-set total-signers (+ current-signers u1))
    
    ;; Return the total number of signers
    (ok (var-get total-signers))
  )
)

;; Remove a signer (requires multi-sig)
(define-public (remove-signer (old-signer principal))
  (let
    (
      (is-signer (default-to false (get active (map-get? signers tx-sender))))
      (signer-data (unwrap! (map-get? signers old-signer) ERR_NOT_FOUND))
      (current-signers (var-get total-signers))
    )
    
    ;; Check if sender is an authorized signer
    (asserts! is-signer ERR_UNAUTHORIZED)
    
    ;; Check if not removing self
    (asserts! (not (is-eq tx-sender old-signer)) ERR_INVALID_PARAMETER)
    
    ;; Check if we maintain minimum number of signers
    (asserts! (> current-signers u2) ERR_INVALID_PARAMETER)
    
    ;; Remove the signer
    (map-set signers old-signer (merge signer-data {active: false}))
    
    ;; Update total signers
    (var-set total-signers (- current-signers u1))
    
    ;; Return the total number of signers
    (ok (var-get total-signers))
  )
)

;; Change the signature threshold
(define-public (change-threshold (new-threshold uint))
  (let
    (
      (is-signer (default-to false (get active (map-get? signers tx-sender))))
      (current-signers (var-get total-signers))
    )
    
    ;; Check if sender is an authorized signer
    (asserts! is-signer ERR_UNAUTHORIZED)
    
    ;; Check if the threshold is valid
    (asserts! (and (> new-threshold u0) (<= new-threshold current-signers)) ERR_INVALID_PARAMETER)
    
    ;; Update threshold
    (var-set threshold new-threshold)
    
    ;; Return the new threshold
    (ok new-threshold)
  )
)

;; Change the timelock period
(define-public (change-timelock (new-timelock uint))
  (let
    (
      (is-signer (default-to false (get active (map-get? signers tx-sender))))
    )
    
    ;; Check if sender is an authorized signer
    (asserts! is-signer ERR_UNAUTHORIZED)
    
    ;; Check if the timelock is valid
    (asserts! (> new-timelock u0) ERR_INVALID_PARAMETER)
    
    ;; Update timelock
    (var-set timelock-period new-timelock)
    
    ;; Return the new timelock
    (ok new-timelock)
  )
)

;; Read-only functions

;; Check if an address is a valid signer
(define-read-only (is-valid-signer (address principal))
  (default-to false (get active (map-get? signers address)))
)

;; Get the total number of signers
(define-read-only (get-total-signers)
  (var-get total-signers)
)

;; Get the current threshold
(define-read-only (get-threshold)
  (var-get threshold)
)

;; Get the current timelock period
(define-read-only (get-timelock-period)
  (var-get timelock-period)
)

;; Get a pending transaction
(define-read-only (get-pending-transaction (tx-id uint))
  (map-get? pending-transactions tx-id)
)

;; Get a transaction from history
(define-read-only (get-transaction-history (tx-id uint))
  (map-get? transaction-history tx-id)
)
