;; DAO Shared Constants and Error Codes
;; [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]
;;
;; This contract serves as a central library for common constants and error codes
;; used across the DAO smart contract system.

;; Common Error Codes
(define-constant ERR_UNAUTHORIZED (err u401))
(define-constant ERR_INVALID_PARAMETER (err u402))
(define-constant ERR_NOT_FOUND (err u404))
(define-constant ERR_ALREADY_EXISTS (err u409))
(define-constant ERR_INSUFFICIENT_BALANCE (err u410))
(define-constant ERR_INSUFFICIENT_PERMISSIONS (err u411))
(define-constant ERR_LIMIT_EXCEEDED (err u413))
(define-constant ERR_EXPIRED (err u419))
(define-constant ERR_LOCKED (err u423))
(define-constant ERR_TIMELOCK_ACTIVE (err u425))
(define-constant ERR_INVALID_STATE (err u428))
(define-constant ERR_QUORUM_NOT_REACHED (err u429))
(define-constant ERR_THRESHOLD_NOT_MET (err u430))
(define-constant ERR_OPERATION_FAILED (err u500))

;; Token Constants
(define-constant MAX_SUPPLY u21000000000000000) ;; 21B with 8 decimals
(define-constant HALVING_INTERVAL u105000) ;; PRODUCTION: 105,000 blocks (adaptive minimum)
(define-constant INITIAL_BLOCK_REWARD u1000000000) ;; 10,000 tokens per block
(define-constant COMMUNITY_PERCENTAGE u15) ;; 15% to Community Incentives

;; DAO Governance Constants
(define-constant MIN_QUORUM_PERCENTAGE u33) ;; 33% participation required
(define-constant MIN_APPROVAL_PERCENTAGE u51) ;; 51% approval required
(define-constant DEFAULT_TIMELOCK_BLOCKS u144) ;; ~24 hours at 10 min blocks

;; Multi-sig constants
(define-constant MIN_SIGNERS u3)
(define-constant DEFAULT_THRESHOLD u2) ;; 2 of 3 default threshold

;; Oracle network constants
(define-constant MIN_ORACLE_CONSENSUS u67) ;; 67% of oracles must agree
(define-constant MAX_ORACLE_COUNT u7) ;; Maximum number of oracles in the network

;; Read-only functions for accessing constants in other contracts

(define-read-only (get-error-unauthorized)
  ERR_UNAUTHORIZED)

(define-read-only (get-error-invalid-parameter)
  ERR_INVALID_PARAMETER)

(define-read-only (get-error-not-found)
  ERR_NOT_FOUND)

(define-read-only (get-error-already-exists)
  ERR_ALREADY_EXISTS)

(define-read-only (get-max-supply)
  MAX_SUPPLY)

(define-read-only (get-halving-interval)
  HALVING_INTERVAL)

(define-read-only (get-initial-block-reward)
  INITIAL_BLOCK_REWARD)

(define-read-only (get-community-percentage)
  COMMUNITY_PERCENTAGE)

(define-read-only (get-min-quorum-percentage)
  MIN_QUORUM_PERCENTAGE)

(define-read-only (get-min-approval-percentage)
  MIN_APPROVAL_PERCENTAGE)

(define-read-only (get-default-timelock-blocks)
  DEFAULT_TIMELOCK_BLOCKS)

(define-read-only (get-min-signers)
  MIN_SIGNERS)

(define-read-only (get-default-threshold)
  DEFAULT_THRESHOLD)

(define-read-only (get-min-oracle-consensus)
  MIN_ORACLE_CONSENSUS)

(define-read-only (get-max-oracle-count)
  MAX_ORACLE_COUNT)
