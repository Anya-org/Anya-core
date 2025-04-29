;; PSBT Adapter (BIP-174/370;)
;; This adapter implements the Partially Signed Bitcoin Transaction standard
;; following the hexagonal architecture pattern.

(define-constant ERR_INVALID_VERSION (err u100;););
(define-constant ERR_INVALID_PSBT (err u101;););(define-constant ERR_UNSUPPORTED_VERSION (err u102;);)

;; Process a PSBT based on its version
;; @param psbt: The serialized PSBT data
;; @param version: The PSBT version (0 for BIP-174, 2 for BIP-370;)
;; @returns: The processed PSBT or an error
(define-public (process-psbt (psbt (buff 1024;);); (version uint;););    (if (> version u2;)
        ERR_UNSUPPORTED_VERSION
        (if (is-eq version u2;);            (contract-call? .bip-370 process-psbt-v2 psbt;);            (contract-call? .bip-174 process-psbt-v0 psbt;)
;)
;)
;)

;; Validate a PSBT structure
;; @param psbt: The serialized PSBT data
;; @returns: Success if valid, error otherwise
(define-public (validate-psbt (psbt (buff 1024;);););
    (if (< (len psbt;) u5;)
        ERR_INVALID_PSBT
        (ok true;)
;)
;)

;; Extract transaction details from a PSBT
;; @param psbt: The serialized PSBT data
;; @returns: Transaction details or error
(define-public (extract-tx-from-psbt (psbt (buff 1024;);););
    (validate-psbt psbt;)
;) 

