;; Cross-Chain Bridge Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;;
;; This contract implements the cross-chain bridge functionality for
;; integrating SIP-010 tokens on Stacks with SRC-20 tokens on Bitcoin
;; and tBTC on Ethereum.

;; Define bridge trait
(define-trait bridge-trait
  (
    ;; Initiate transfer from Stacks to Bitcoin
    (initiate-bitcoin-transfer (uint (buff 33)) (response (buff 32) uint))
    
    ;; Initiate transfer from Stacks to Ethereum
    (initiate-ethereum-transfer (uint (buff 20)) (response (buff 32) uint))
    
    ;; Complete inbound transfer from Bitcoin to Stacks
    (complete-bitcoin-transfer ((buff 32) (buff 33) uint) (response bool uint))
    
    ;; Complete inbound transfer from Ethereum to Stacks
    (complete-ethereum-transfer ((buff 32) (buff 20) uint) (response bool uint))
  )
)

;; Import token trait
(use-trait ft-trait .ft-token-trait)

;; Constants
(define-constant CONTRACT_OWNER 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant AUTHORIZED_ORACLES (list 
  'ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG 
  'ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC
))
(define-constant TOKEN_CONTRACT .token)
(define-constant TREASURY_CONTRACT .treasury-management)

;; Error codes
(define-constant ERR_UNAUTHORIZED (err u401))
(define-constant ERR_ALREADY_PROCESSED (err u402))
(define-constant ERR_INVALID_AMOUNT (err u403))
(define-constant ERR_INSUFFICIENT_FUNDS (err u404))
(define-constant ERR_TRANSFER_FAILED (err u405))

;; Confirmation parameters
(define-data-var bitcoin-confirmations uint u6)
(define-data-var ethereum-confirmations uint u12)

;; Operational parameters
(define-data-var bridge-paused bool false)
(define-data-var last-nonce uint u0)

;; Map to track pending outbound transfers
(define-map outbound-transfers
  { tx-hash: (buff 32) }
  {
    recipient: (optional (buff 33)),
    eth-recipient: (optional (buff 20)),
    amount: uint,
    status: (string-ascii 10),
    timestamp: uint,
    signature: (optional (buff 65))
  }
)

;; Map to track processed inbound transfers
(define-map inbound-transfers
  { source-tx: (buff 32) }
  {
    recipient: principal,
    amount: uint,
    bridge-type: (string-ascii 20),
    processed-height: uint
  }
)

;; Map to manage authorized bridge operators
(define-map authorized-operators principal bool)

;; Initialize authorized operators
(map-set authorized-operators CONTRACT_OWNER true)

;; Public functions

;; Initiate a transfer from Stacks to Bitcoin (SIP-010 to SRC-20)
(define-public (initiate-bitcoin-transfer (amount uint) (bitcoin-recipient (buff 33)))
  (let (
    (nonce (+ (var-get last-nonce) u1))
    (tx-hash (generate-tx-hash nonce amount))
  )
    ;; Ensure bridge is not paused
    (asserts! (not (var-get bridge-paused)) ERR_UNAUTHORIZED)
    
    ;; Ensure amount is positive
    (asserts! (> amount u0) ERR_INVALID_AMOUNT)
    
    ;; Record the outbound transfer
    (map-set outbound-transfers
      { tx-hash: tx-hash }
      {
        recipient: (some bitcoin-recipient),
        eth-recipient: none,
        amount: amount,
        status: "pending",
        timestamp: block-height,
        signature: none
      }
    )
    
    ;; Update nonce
    (var-set last-nonce nonce)
    
    ;; Return the transaction hash
    (ok tx-hash)
  )
)

;; Initiate a transfer from Stacks to Ethereum (SIP-010 to tBTC)
(define-public (initiate-ethereum-transfer (amount uint) (ethereum-recipient (buff 20)))
  (let (
    (nonce (+ (var-get last-nonce) u1))
    (tx-hash (generate-tx-hash nonce amount))
  )
    ;; Ensure bridge is not paused
    (asserts! (not (var-get bridge-paused)) ERR_UNAUTHORIZED)
    
    ;; Ensure amount is positive
    (asserts! (> amount u0) ERR_INVALID_AMOUNT)
    
    ;; Record the outbound transfer
    (map-set outbound-transfers
      { tx-hash: tx-hash }
      {
        recipient: none,
        eth-recipient: (some ethereum-recipient),
        amount: amount,
        status: "pending",
        timestamp: block-height,
        signature: none
      }
    )
    
    ;; Update nonce
    (var-set last-nonce nonce)
    
    ;; Return the transaction hash
    (ok tx-hash)
  )
)

;; Complete a transfer from Bitcoin to Stacks (SRC-20 to SIP-010)
(define-public (complete-bitcoin-transfer (bitcoin-tx (buff 32)) (recipient principal) (amount uint))
  (let (
    ;; Check if transfer has already been processed
    (processed (is-some (map-get? inbound-transfers { source-tx: bitcoin-tx })))
  )
    ;; Verify caller is authorized
    (asserts! (is-authorized-oracle) ERR_UNAUTHORIZED)
    
    ;; Ensure bridge is not paused
    (asserts! (not (var-get bridge-paused)) ERR_UNAUTHORIZED)
    
    ;; Ensure transfer has not been processed already
    (asserts! (not processed) ERR_ALREADY_PROCESSED)
    
    ;; Ensure amount is positive
    (asserts! (> amount u0) ERR_INVALID_AMOUNT)
    
    ;; Process the transfer and mint/transfer tokens
    (unwrap! (contract-call? TOKEN_CONTRACT transfer amount TREASURY_CONTRACT recipient none) ERR_TRANSFER_FAILED)
    
    ;; Record the completed inbound transfer
    (map-set inbound-transfers
      { source-tx: bitcoin-tx }
      {
        recipient: recipient,
        amount: amount,
        bridge-type: "bitcoin-to-stacks",
        processed-height: block-height
      }
    )
    
    ;; Return success
    (ok true)
  )
)

;; Complete a transfer from Ethereum to Stacks (tBTC to SIP-010)
(define-public (complete-ethereum-transfer (ethereum-tx (buff 32)) (recipient principal) (amount uint))
  (let (
    ;; Check if transfer has already been processed
    (processed (is-some (map-get? inbound-transfers { source-tx: ethereum-tx })))
  )
    ;; Verify caller is authorized
    (asserts! (is-authorized-oracle) ERR_UNAUTHORIZED)
    
    ;; Ensure bridge is not paused
    (asserts! (not (var-get bridge-paused)) ERR_UNAUTHORIZED)
    
    ;; Ensure transfer has not been processed already
    (asserts! (not processed) ERR_ALREADY_PROCESSED)
    
    ;; Ensure amount is positive
    (asserts! (> amount u0) ERR_INVALID_AMOUNT)
    
    ;; Process the transfer and mint/transfer tokens
    (unwrap! (contract-call? TOKEN_CONTRACT transfer amount TREASURY_CONTRACT recipient none) ERR_TRANSFER_FAILED)
    
    ;; Record the completed inbound transfer
    (map-set inbound-transfers
      { source-tx: ethereum-tx }
      {
        recipient: recipient,
        amount: amount,
        bridge-type: "ethereum-to-stacks",
        processed-height: block-height
      }
    )
    
    ;; Return success
    (ok true)
  )
)

;; Admin functions

;; Set confirmation requirements
(define-public (set-confirmations (bitcoin-confirms uint) (ethereum-confirms uint))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (var-set bitcoin-confirmations bitcoin-confirms)
    (var-set ethereum-confirmations ethereum-confirms)
    (ok { bitcoin: bitcoin-confirms, ethereum: ethereum-confirms })
  )
)

;; Pause/unpause the bridge
(define-public (set-bridge-paused (paused bool))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (var-set bridge-paused paused)
    (ok paused)
  )
)

;; Add authorized operator
(define-public (add-authorized-operator (operator principal))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (map-set authorized-operators operator true)
    (ok true)
  )
)

;; Remove authorized operator
(define-public (remove-authorized-operator (operator principal))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_UNAUTHORIZED)
    (map-delete authorized-operators operator)
    (ok true)
  )
)

;; Update outbound transfer status
(define-public (update-transfer-status (tx-hash (buff 32)) (status (string-ascii 10)) (signature (buff 65)))
  (let (
    (transfer (unwrap! (map-get? outbound-transfers { tx-hash: tx-hash }) ERR_INVALID_PARAMETER))
  )
    ;; Verify caller is authorized
    (asserts! (is-authorized-oracle) ERR_UNAUTHORIZED)
    
    ;; Update the transfer status
    (map-set outbound-transfers
      { tx-hash: tx-hash }
      (merge transfer 
        {
          status: status,
          signature: (some signature)
        }
      )
    )
    
    (ok true)
  )
)

;; Helper functions

;; Check if caller is an authorized oracle
(define-private (is-authorized-oracle)
  (or 
    (is-eq tx-sender CONTRACT_OWNER)
    (default-to false (map-get? authorized-operators tx-sender))
    (is-some (index-of AUTHORIZED_ORACLES tx-sender))
  )
)

;; Generate a deterministic transaction hash
(define-private (generate-tx-hash (nonce uint) (amount uint))
  (sha256 
    (append 
      (append 
        (unwrap-panic (to-consensus-buff? nonce))
        (unwrap-panic (to-consensus-buff? amount))
      )
      (unwrap-panic (to-consensus-buff? block-height))
    )
  )
)

;; Read functions

;; Get outbound transfer details
(define-read-only (get-outbound-transfer (tx-hash (buff 32)))
  (map-get? outbound-transfers { tx-hash: tx-hash })
)

;; Get inbound transfer details
(define-read-only (get-inbound-transfer (source-tx (buff 32)))
  (map-get? inbound-transfers { source-tx: source-tx })
)

;; Check if bridge is paused
(define-read-only (is-bridge-paused)
  (var-get bridge-paused)
)

;; Get confirmation requirements
(define-read-only (get-confirmation-requirements)
  (tuple 
    (bitcoin (var-get bitcoin-confirmations))
    (ethereum (var-get ethereum-confirmations)))
)

;; Check if an address is an authorized operator
(define-read-only (is-operator (address principal))
  (default-to false (map-get? authorized-operators address))
)
