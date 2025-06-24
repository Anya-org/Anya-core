;; SIP-010 Token Contract
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;;
;; This contract implements the SIP-010 fungible token standard
;; for the Anya Governance Token (AGT).

(impl-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Token Constants
(define-constant TOKEN_NAME "Anya Governance Token")
(define-constant TOKEN_SYMBOL "AGT")
(define-constant TOKEN_DECIMALS u8)
(define-constant TOKEN_TOTAL_SUPPLY u21000000000000000) ;; 21B with 8 decimals

;; Error codes
(define-constant ERR_OWNER_ONLY (err u100))
(define-constant ERR_NOT_TOKEN_OWNER (err u101))
(define-constant ERR_TRANSFER_FAILED (err u102))

;; Principal constants
(define-constant CONTRACT_OWNER 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant TREASURY_PRINCIPAL 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant COMMUNITY_PRINCIPAL 'ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC)

;; Data variables
(define-data-var token-uri (optional (string-utf8 256)) (some u"https://anya-core.org/token-metadata.json"))

;; Initialize the contract
(define-fungible-token agt TOKEN_TOTAL_SUPPLY)

;; Initial token distribution
(begin
  ;; 35% to Protocol Treasury
  (ft-mint? agt (* TOKEN_TOTAL_SUPPLY u35 (/ u1 u100)) TREASURY_PRINCIPAL)
  ;; 25% to Liquidity Provision (initially in treasury)
  (ft-mint? agt (* TOKEN_TOTAL_SUPPLY u25 (/ u1 u100)) TREASURY_PRINCIPAL)
  ;; 20% to Team & Development
  (ft-mint? agt (* TOKEN_TOTAL_SUPPLY u20 (/ u1 u100)) 'ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG)
  ;; 15% to Community Incentives
  (ft-mint? agt (* TOKEN_TOTAL_SUPPLY u15 (/ u1 u100)) COMMUNITY_PRINCIPAL)
  ;; 5% to Strategic Partners & Advisors
  (ft-mint? agt (* TOKEN_TOTAL_SUPPLY u5 (/ u1 u100)) 'ST2NEB84ASENDXKYGJPQW86YXQCEFEX2ZQPG87ND)
)

;; SIP-010 required functions

;; Transfer tokens to a recipient
(define-public (transfer (amount uint) (sender principal) (recipient principal) (memo (optional (buff 34))))
  (begin
    ;; Check that the sender is the tx-sender
    (asserts! (is-eq tx-sender sender) ERR_NOT_TOKEN_OWNER)
    
    ;; Transfer tokens
    (try! (ft-transfer? agt amount sender recipient))
    
    ;; Handle memo if provided
    (match memo
      memo-data (print memo-data)
      true
    )
    
    ;; Return success
    (ok true)
  )
)

;; Get the token balance of an account
(define-read-only (get-balance (owner principal))
  (ok (ft-get-balance agt owner))
)

;; Get the total supply of tokens
(define-read-only (get-total-supply)
  (ok (ft-get-supply agt))
)

;; Get the token name
(define-read-only (get-name)
  (ok TOKEN_NAME)
)

;; Get the token symbol
(define-read-only (get-symbol)
  (ok TOKEN_SYMBOL)
)

;; Get the token decimals
(define-read-only (get-decimals)
  (ok TOKEN_DECIMALS)
)

;; Get the token URI
(define-read-only (get-token-uri)
  (ok (var-get token-uri))
)

;; Admin functions

;; Set the token URI
(define-public (set-token-uri (new-uri (optional (string-utf8 256))))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_OWNER_ONLY)
    (var-set token-uri new-uri)
    (ok true)
  )
)

;; Mint new tokens - only authorized for the contract owner
(define-public (mint (amount uint) (recipient principal))
  (begin
    (asserts! (is-eq tx-sender CONTRACT_OWNER) ERR_OWNER_ONLY)
    (ft-mint? agt amount recipient)
  )
)
