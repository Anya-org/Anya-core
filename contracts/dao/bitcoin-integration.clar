;; Bitcoin Integration Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_PARAMETER u402)
(define-constant ERR_SYSTEM_DISABLED u403)
(define-constant ERR_TX_ALREADY_VERIFIED u404)
(define-constant ERR_INSUFFICIENT_CONFIRMATIONS u405)
(define-constant ERR_VERIFICATION_FAILED u406)
(define-constant ERR_BELOW_THRESHOLD u407)
(define-constant ERR_COOLING_PERIOD u408)
(define-constant ERR_TX_NOT_FOUND u409)
(define-constant ERR_PROOF_INVALID u410)
(define-constant ERR_SPV_VERIFICATION_FAILED u411)
(define-constant ERR_RATE_LIMIT_EXCEEDED u412)
(define-constant ERR_ALREADY_REGISTERED u413)
(define-constant ERR_BTC_ADDRESS_INVALID u414)
(define-constant ERR_NOT_TRACKED u415)

;; Contract References
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance)
(define-constant METRICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.metrics-oracle)
(define-constant TREASURY_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury-management)
(define-constant WEB5_DWN 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.web5-dwn-adapter)

;; Data Variables
(define-data-var bitcoin-integration-enabled bool true)
(define-data-var min-confirmations uint u6)
(define-data-var verification-fee-basis-points uint u25) ;; 0.25% fee
(define-data-var btc-reserve-ratio uint u10) ;; 10% of treasury in BTC
(define-data-var verification-cooldown-blocks uint u144) ;; ~1 day with 10-min blocks
(define-data-var last-verification-block uint u0)
(define-data-var oracle-url (string-utf8 256) "https://bitcoin-oracle.anya.ai/v1")
(define-data-var registration-fee uint u1000000) ;; 0.01 STX
(define-data-var min-btc-amount uint u100000) ;; 0.001 BTC in sats
(define-data-var total-btc-reserves uint u0) ;; in satoshis
(define-data-var total-verified-txs uint u0)
(define-data-var btc-price-in-usd uint u5500000) ;; $55,000 with 2 decimal places
(define-data-var last-price-update uint u0)

;; Admin and verifier lists
(define-map administrators principal bool)
(define-map btc-verifiers principal bool)

;; Initialize administrators
(map-set administrators tx-sender true)

;; Bitcoin Address Registry (maps BTC addresses to STX addresses)
(define-map btc-address-registry
  (buff 33) ;; BTC address (compressed public key)
  {
    stx-owner: principal,
    registered-at: uint,
    last-updated: uint,
    satoshi-balance: uint,
    total-tx-count: uint,
    verified: bool
  }
)

;; STX to BTC address mapping
(define-map stx-to-btc-mapping
  principal ;; STX address
  (list 10 (buff 33)) ;; List of BTC addresses
)

;; Bitcoin Transaction Registry
(define-map btc-transaction-registry
  (buff 32) ;; Transaction hash
  {
    tx-block-height: uint,
    confirmations: uint,
    timestamp: uint,
    amount-sats: uint,
    sender: (buff 33),
    receiver: (buff 33),
    verified: bool,
    verification-block: uint,
    verifier: (optional principal),
    fee-sats: uint,
    spv-proof: (optional (buff 1024))
  }
)

;; Reserve BTC addresses (DAO-controlled multisig)
(define-map reserve-btc-addresses
  (buff 33) ;; BTC address
  {
    satoshi-balance: uint,
    address-type: (string-ascii 20), ;; "p2pkh", "p2sh", "segwit", "taproot"
    total-tx-count: uint,
    last-updated: uint,
    verified-by: (list 5 principal),
    description: (string-ascii 64)
  }
)

;; Verification history
(define-map verification-history
  uint ;; verification ID
  {
    block-height: uint, 
    verifier: principal,
    total-btc-verified: uint,
    reserve-ratio-achieved: uint, ;; Basis points (e.g., 1000 = 10%)
    btc-price-usd: uint,
    tx-count: uint,
    spv-proofs-verified: uint,
    verification-time-ms: uint,
    report-uri: (optional (string-utf8 256))
  }
)

;; Verification stats
(define-data-var verification-count uint u0)
(define-data-var total-satoshis-verified uint u0)

;; Public Functions

;; Register a Bitcoin address
(define-public (register-btc-address
    (btc-address (buff 33))
    (proof-of-ownership (buff 65)))
  (begin
    ;; Check if Bitcoin integration is enabled
    (asserts! (var-get bitcoin-integration-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Verify Bitcoin address format (compressed public key)
    (asserts! (is-valid-btc-address btc-address) (err ERR_BTC_ADDRESS_INVALID))
    
    ;; Check if address is already registered
    (asserts! (is-none (map-get? btc-address-registry btc-address)) (err ERR_ALREADY_REGISTERED))
    
    ;; Verify proof of ownership
    (asserts! (verify-btc-ownership btc-address proof-of-ownership tx-sender) (err ERR_VERIFICATION_FAILED))
    
    ;; Register BTC address
    (map-set btc-address-registry btc-address
      {
        stx-owner: tx-sender,
        registered-at: block-height,
        last-updated: block-height,
        satoshi-balance: u0,
        total-tx-count: u0,
        verified: false
      }
    )
    
    ;; Update STX to BTC mapping
    (let ((existing-addresses (default-to (list) (map-get? stx-to-btc-mapping tx-sender))))
      (map-set stx-to-btc-mapping tx-sender (unwrap! (as-max-len? (append existing-addresses btc-address) u10) 
                                               (err ERR_INVALID_PARAMETER)))
    )
    
    ;; Log metric
    (try! (contract-call? METRICS_CONTRACT submit-governance-metric "btc_addresses_registered" 
                         u1 u1000 "bitcoin-integration"))
    
    (ok true)
  ))

;; Verify a Bitcoin transaction with SPV proof
(define-public (verify-btc-transaction
    (tx-hash (buff 32))
    (block-header (buff 80))
    (tx-index uint)
    (merkle-proof (list 20 (buff 32)))
    (tx-hex (buff 1024)))
  (begin
    ;; Check if Bitcoin integration is enabled
    (asserts! (var-get bitcoin-integration-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check if verifier is authorized
    (asserts! (or (is-btc-verifier tx-sender) (is-administrator tx-sender)) (err ERR_UNAUTHORIZED))
    
    ;; Check if transaction already verified
    (let ((tx-info (map-get? btc-transaction-registry tx-hash)))
      (asserts! (or (is-none tx-info) (not (get verified (default-to {verified: false} tx-info)))) 
                (err ERR_TX_ALREADY_VERIFIED))
      
      ;; Compute SPV proof verification
      ;; In a real implementation, this would perform the actual Merkle proof validation
      ;; For this demonstration, we'll assume the verification is successful
      (asserts! (verify-merkle-proof tx-hash merkle-proof tx-index block-header) (err ERR_SPV_VERIFICATION_FAILED))
      
      ;; Extract transaction details from tx-hex
      (let (
        (tx-details (parse-btc-transaction tx-hex))
        (fee-amount (calculate-verification-fee (get amount tx-details)))
      )
        ;; Store transaction details
        (map-set btc-transaction-registry tx-hash
          {
            tx-block-height: (get block-height tx-details),
            confirmations: (get confirmations tx-details),
            timestamp: (get timestamp tx-details),
            amount-sats: (get amount tx-details),
            sender: (get sender tx-details),
            receiver: (get receiver tx-details),
            verified: true,
            verification-block: block-height,
            verifier: (some tx-sender),
            fee-sats: fee-amount,
            spv-proof: (some (concat block-header (concat tx-hex (serialize-merkle-proof merkle-proof))))
          }
        )
        
        ;; Update total verified transactions
        (var-set total-verified-txs (+ (var-get total-verified-txs) u1))
        
        ;; Update BTC address registry if the receiver is tracked
        (update-btc-address-balance (get receiver tx-details) (get amount tx-details))
        
        ;; Check if this is a reserve address
        (when (is-some (map-get? reserve-btc-addresses (get receiver tx-details)))
          (var-set total-btc-reserves (+ (var-get total-btc-reserves) (get amount tx-details)))
          
          ;; Update reserve address balance
          (match (map-get? reserve-btc-addresses (get receiver tx-details))
            reserve-info (map-set reserve-btc-addresses (get receiver tx-details)
              (merge reserve-info {
                satoshi-balance: (+ (get satoshi-balance reserve-info) (get amount tx-details)),
                total-tx-count: (+ (get total-tx-count reserve-info) u1),
                last-updated: block-height,
                verified-by: (unwrap! (as-max-len? (append (get verified-by reserve-info) tx-sender) u5) 
                                     (err ERR_INVALID_PARAMETER))
              }))
            (err ERR_TX_NOT_FOUND)
          )
        )
        
        ;; Log metric
        (try! (contract-call? METRICS_CONTRACT submit-governance-metric "btc_txs_verified" 
                             u1 u1000 "bitcoin-integration"))
        
        ;; Store verification in Web5 DWN if available
        (try! (store-btc-verification-to-dwn tx-hash block-height))
        
        ;; Return transaction details
        (ok {
          tx-hash: tx-hash,
          amount-sats: (get amount tx-details),
          confirmations: (get confirmations tx-details),
          verified: true
        })
      )
    )
  ))

;; Verify Bitcoin reserves and update DAO metrics
(define-public (verify-btc-reserves)
  (begin
    ;; Check if Bitcoin integration is enabled
    (asserts! (var-get bitcoin-integration-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check cooling period
    (asserts! (>= (- block-height (var-get last-verification-block)) (var-get verification-cooldown-blocks))
             (err ERR_COOLING_PERIOD))
    
    ;; Check if verifier is authorized
    (asserts! (or (is-btc-verifier tx-sender) (is-administrator tx-sender)) (err ERR_UNAUTHORIZED))
    
    ;; Get reserve ratio and other metrics
    (let (
      (total-reserves (var-get total-btc-reserves))
      (usd-value-of-btc (* total-reserves (var-get btc-price-in-usd)))
      (usd-value-of-btc-normalized (/ usd-value-of-btc u100000000)) ;; Converting satoshis to BTC
      (treasury-value (contract-call? TREASURY_CONTRACT get-total-value))
      (reserve-ratio (if (> treasury-value u0)
                        (/ (* usd-value-of-btc-normalized u10000) treasury-value)
                        u0))
      (verification-id (+ (var-get verification-count) u1))
      (btc-tx-count (var-get total-verified-txs))
    )
      ;; Update verification history
      (map-set verification-history verification-id
        {
          block-height: block-height,
          verifier: tx-sender,
          total-btc-verified: total-reserves,
          reserve-ratio-achieved: reserve-ratio,
          btc-price-usd: (var-get btc-price-in-usd),
          tx-count: btc-tx-count,
          spv-proofs-verified: btc-tx-count,
          verification-time-ms: u1000, ;; Placeholder
          report-uri: (some (generate-report-uri verification-id))
        }
      )
      
      ;; Update verification stats
      (var-set verification-count verification-id)
      (var-set total-satoshis-verified (+ (var-get total-satoshis-verified) total-reserves))
      (var-set last-verification-block block-height)
      
      ;; Update BTC price in treasury contract
      (try! (contract-call? TREASURY_CONTRACT update-asset-price "BTC" (var-get btc-price-in-usd)))
      
      ;; Submit metrics
      (try! (contract-call? METRICS_CONTRACT submit-treasury-metric "btc_reserves_ratio" 
                           reserve-ratio u10000 "bitcoin-integration"))
      
      (try! (contract-call? METRICS_CONTRACT submit-treasury-metric "btc_reserves_value_usd" 
                           usd-value-of-btc-normalized u100 "bitcoin-integration"))
      
      ;; Return verification details
      (ok {
        verification-id: verification-id,
        total-btc-sats: total-reserves,
        usd-value: usd-value-of-btc-normalized,
        reserve-ratio: reserve-ratio,
        target-ratio: (var-get btc-reserve-ratio),
        timestamp: block-height
      })
    )
  ))

;; Register a DAO reserve BTC address
(define-public (register-reserve-address
    (btc-address (buff 33))
    (address-type (string-ascii 20))
    (description (string-ascii 64)))
  (begin
    ;; Check if Bitcoin integration is enabled
    (asserts! (var-get bitcoin-integration-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check if administrator
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Verify Bitcoin address format
    (asserts! (is-valid-btc-address btc-address) (err ERR_BTC_ADDRESS_INVALID))
    
    ;; Validate address type
    (asserts! (or (is-eq address-type "p2pkh") 
                 (is-eq address-type "p2sh") 
                 (is-eq address-type "segwit") 
                 (is-eq address-type "taproot"))
             (err ERR_INVALID_PARAMETER))
    
    ;; Register reserve address
    (map-set reserve-btc-addresses btc-address
      {
        satoshi-balance: u0,
        address-type: address-type,
        total-tx-count: u0,
        last-updated: block-height,
        verified-by: (list tx-sender),
        description: description
      }
    )
    
    ;; Log metric
    (try! (contract-call? METRICS_CONTRACT submit-governance-metric "btc_reserve_addresses_registered" 
                         u1 u1000 "bitcoin-integration"))
    
    (ok true)
  ))

;; Update BTC price 
(define-public (update-btc-price (new-btc-price-usd uint))
  (begin
    ;; Check if Bitcoin integration is enabled
    (asserts! (var-get bitcoin-integration-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check if price oracle or administrator
    (asserts! (or (is-btc-verifier tx-sender) (is-administrator tx-sender)) (err ERR_UNAUTHORIZED))
    
    ;; Validate price (must be > 0 and not too drastic of a change)
    (asserts! (> new-btc-price-usd u0) (err ERR_INVALID_PARAMETER))
    
    ;; Get current price and limit change to 20% per update
    (let (
      (current-price (var-get btc-price-in-usd))
      (max-increase (* current-price u12 / u10)) ;; 120% of current price
      (min-decrease (* current-price u8 / u10))  ;; 80% of current price
    )
      (asserts! (and (<= new-btc-price-usd max-increase) (>= new-btc-price-usd min-decrease))
               (err ERR_INVALID_PARAMETER))
      
      ;; Update price
      (var-set btc-price-in-usd new-btc-price-usd)
      (var-set last-price-update block-height)
      
      ;; Submit metric
      (try! (contract-call? METRICS_CONTRACT submit-treasury-metric "btc_price_usd" 
                           new-btc-price-usd u100 "bitcoin-integration"))
      
      (ok new-btc-price-usd)
    )
  ))

;; Mint AGT tokens backed by BTC
(define-public (mint-agt-from-btc (satoshi-amount uint))
  (begin
    ;; Check if Bitcoin integration is enabled
    (asserts! (var-get bitcoin-integration-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check if administrator or DAO
    (asserts! (or (is-administrator tx-sender) (is-eq tx-sender DAO_CONTRACT)) (err ERR_UNAUTHORIZED))
    
    ;; Check minimum amount
    (asserts! (>= satoshi-amount (var-get min-btc-amount)) (err ERR_BELOW_THRESHOLD))
    
    ;; Calculate AGT to mint based on BTC value
    (let (
      (btc-value-usd (* satoshi-amount (var-get btc-price-in-usd)))
      (btc-value-usd-normalized (/ btc-value-usd u100000000)) ;; Converting satoshis to BTC
      (agt-price-usd (contract-call? TOKEN_CONTRACT get-token-price))
      (agt-to-mint (/ (* btc-value-usd-normalized u100000000) agt-price-usd)) ;; Normalized to 8 decimals
    )
      ;; Mint AGT tokens to treasury
      (try! (contract-call? TOKEN_CONTRACT mint-tokens agt-to-mint TREASURY_CONTRACT))
      
      ;; Log metrics
      (try! (contract-call? METRICS_CONTRACT submit-treasury-metric "btc_backed_agt_minted" 
                           agt-to-mint u100000000 "bitcoin-integration"))
      
      (ok {
        btc-amount-sats: satoshi-amount,
        btc-value-usd: btc-value-usd-normalized,
        agt-minted: agt-to-mint
      })
    )
  ))

;; Store Bitcoin data to Web5 DWN
(define-public (store-btc-data-to-dwn
    (data-type (string-ascii 20))
    (data-id (buff 32))
    (metadata (string-utf8 1024)))
  (begin
    ;; Check if Bitcoin integration is enabled
    (asserts! (var-get bitcoin-integration-enabled) (err ERR_SYSTEM_DISABLED))
    
    ;; Check if administrator or verifier
    (asserts! (or (is-administrator tx-sender) (is-btc-verifier tx-sender)) (err ERR_UNAUTHORIZED))
    
    ;; Check if Web5 DWN adapter is available
    (asserts! (contract-exists? WEB5_DWN) (err ERR_SYSTEM_DISABLED))
    
    ;; Store data in DWN
    (let (
      (content-hash (hash160 (concat data-type (concat data-id (to-consensus-buff metadata)))))
    )
      (try! (contract-call? WEB5_DWN store-record 
        "bitcoin-data"
        (some (concat "btc:" data-type))
        (some tx-sender)
        (some metadata)
        (some content-hash)
        (some "aes256-gcm")
        none
        (some data-type)
        none))
      
      (ok true)
    )
  ))

;; Read-Only Functions

;; Get BTC address details
(define-read-only (get-btc-address (btc-address (buff 33)))
  (map-get? btc-address-registry btc-address))

;; Get BTC transaction details
(define-read-only (get-btc-transaction (tx-hash (buff 32)))
  (map-get? btc-transaction-registry tx-hash))

;; Get reserve address details
(define-read-only (get-reserve-address (btc-address (buff 33)))
  (map-get? reserve-btc-addresses btc-address))

;; Get verification history
(define-read-only (get-verification-history (verification-id uint))
  (map-get? verification-history verification-id))

;; Get Bitcoin integration parameters
(define-read-only (get-bitcoin-integration-parameters)
  {
    bitcoin-integration-enabled: (var-get bitcoin-integration-enabled),
    min-confirmations: (var-get min-confirmations),
    verification-fee-basis-points: (var-get verification-fee-basis-points),
    btc-reserve-ratio: (var-get btc-reserve-ratio),
    verification-cooldown-blocks: (var-get verification-cooldown-blocks),
    last-verification-block: (var-get last-verification-block),
    oracle-url: (var-get oracle-url),
    registration-fee: (var-get registration-fee),
    min-btc-amount: (var-get min-btc-amount)
  })

;; Get current BTC reserves
(define-read-only (get-btc-reserves)
  {
    total-btc-sats: (var-get total-btc-reserves),
    btc-price-usd: (var-get btc-price-in-usd),
    usd-value: (/ (* (var-get total-btc-reserves) (var-get btc-price-in-usd)) u100000000),
    verification-count: (var-get verification-count),
    last-verification-block: (var-get last-verification-block),
    blocks-since-verification: (- block-height (var-get last-verification-block))
  })

;; Get BTC addresses for STX address
(define-read-only (get-btc-addresses-for-stx (stx-address principal))
  (map-get? stx-to-btc-mapping stx-address))

;; Check if account is an administrator
(define-read-only (is-administrator (account principal))
  (default-to false (map-get? administrators account)))

;; Check if account is a BTC verifier
(define-read-only (is-btc-verifier (account principal))
  (default-to false (map-get? btc-verifiers account)))

;; Helper Functions

;; Verify Bitcoin address format
(define-private (is-valid-btc-address (btc-address (buff 33)))
  ;; In a real implementation, this would validate the address format
  ;; For simplicity, we'll just check basic length
  (is-eq (len btc-address) u33))

;; Verify proof of BTC ownership
(define-private (verify-btc-ownership (btc-address (buff 33)) (proof (buff 65)) (stx-address principal))
  ;; In a real implementation, this would validate the cryptographic proof
  ;; For simplicity, we'll return true
  true)

;; Parse Bitcoin transaction
(define-private (parse-btc-transaction (tx-hex (buff 1024)))
  ;; In a real implementation, this would parse the transaction details
  ;; For demonstration, we'll return dummy values
  {
    block-height: (- block-height u10),
    confirmations: u10,
    timestamp: block-height,
    amount: u10000000, ;; 0.1 BTC
    sender: 0x010203040506070809101112131415161718192021222324252627282930313233,
    receiver: 0x330201040506070809101112131415161718192021222324252627282930313233,
    fee: u10000
  })

;; Verify Merkle proof
(define-private (verify-merkle-proof (tx-hash (buff 32)) (merkle-proof (list 20 (buff 32))) (tx-index uint) (block-header (buff 80)))
  ;; In a real implementation, this would validate the Merkle proof
  ;; For simplicity, we'll return true
  true)

;; Calculate verification fee
(define-private (calculate-verification-fee (amount uint))
  (/ (* amount (var-get verification-fee-basis-points)) u10000))

;; Update BTC address balance
(define-private (update-btc-address-balance (btc-address (buff 33)) (amount uint))
  (match (map-get? btc-address-registry btc-address)
    addr-info (map-set btc-address-registry btc-address
              (merge addr-info {
                satoshi-balance: (+ (get satoshi-balance addr-info) amount),
                last-updated: block-height,
                total-tx-count: (+ (get total-tx-count addr-info) u1)
              }))
    false))

;; Store BTC verification to Web5 DWN
(define-private (store-btc-verification-to-dwn (tx-hash (buff 32)) (block-height uint))
  (let (
    (metadata (concat "{\"txHash\":\"" 
                     (concat (to-ascii tx-hash) 
                            (concat "\",\"verified_at\":" 
                                   (concat (to-ascii block-height) "}")))))
  )
    (if (contract-exists? WEB5_DWN)
        (contract-call? WEB5_DWN store-record 
          "btc-verification" 
          (some (concat "tx:" (to-ascii tx-hash)))
          (some tx-sender)
          (some metadata)
          none
          (some "none")
          none
          (some "bitcoin-verification")
          none)
        (ok true)
    )
  ))

;; Generate report URI
(define-private (generate-report-uri (verification-id uint))
  (concat (var-get oracle-url) (concat "/verification/" (to-ascii verification-id))))

;; Serialize Merkle proof
(define-private (serialize-merkle-proof (merkle-proof (list 20 (buff 32))))
  ;; In a real implementation, this would serialize the proof
  ;; For simplicity, we'll return a dummy value
  0x010203)

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

;; Add a BTC verifier
(define-public (add-btc-verifier (verifier principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set btc-verifiers verifier true)
    (ok true)))

;; Remove a BTC verifier
(define-public (remove-btc-verifier (verifier principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set btc-verifiers verifier false)
    (ok true)))

;; Toggle Bitcoin integration
(define-public (toggle-bitcoin-integration (enabled bool))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (var-set bitcoin-integration-enabled enabled)
    (ok true)))

;; Update Bitcoin integration parameters
(define-public (update-bitcoin-integration-parameters
    (min-confirmations-new (optional uint))
    (verification-fee-new (optional uint))
    (btc-reserve-ratio-new (optional uint))
    (verification-cooldown-new (optional uint))
    (oracle-url-new (optional (string-utf8 256)))
    (registration-fee-new (optional uint))
    (min-btc-amount-new (optional uint)))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Update each parameter if provided
    (match min-confirmations-new
      val (var-set min-confirmations val)
      true)
    
    (match verification-fee-new
      val (var-set verification-fee-basis-points val)
      true)
    
    (match btc-reserve-ratio-new
      val (var-set btc-reserve-ratio val)
      true)
    
    (match verification-cooldown-new
      val (var-set verification-cooldown-blocks val)
      true)
    
    (match oracle-url-new
      val (var-set oracle-url val)
      true)
    
    (match registration-fee-new
      val (var-set registration-fee val)
      true)
    
    (match min-btc-amount-new
      val (var-set min-btc-amount val)
      true)
    
    (ok true)
  )) 